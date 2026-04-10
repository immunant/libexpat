use ::c2rust_bitfields;

pub mod siphash_h {

    pub unsafe extern "C" fn sip_tokey(
        mut key: *mut crate::siphash_h::sipkey,
        mut src: *const ::core::ffi::c_void,
    ) -> *mut crate::siphash_h::sipkey {
        (*key).k[0] = (*(src as *const ::core::ffi::c_uchar).offset(0) as uint64_t) << 0
            | (*(src as *const ::core::ffi::c_uchar).offset(1) as uint64_t) << 8
            | (*(src as *const ::core::ffi::c_uchar).offset(2) as uint64_t) << 16
            | (*(src as *const ::core::ffi::c_uchar).offset(3) as uint64_t) << 24
            | (*(src as *const ::core::ffi::c_uchar).offset(4) as uint64_t) << 32
            | (*(src as *const ::core::ffi::c_uchar).offset(5) as uint64_t) << 40
            | (*(src as *const ::core::ffi::c_uchar).offset(6) as uint64_t) << 48
            | (*(src as *const ::core::ffi::c_uchar).offset(7) as uint64_t) << 56;
        (*key).k[1] = (*(src as *const ::core::ffi::c_uchar).offset(8).offset(0) as uint64_t) << 0
            | (*(src as *const ::core::ffi::c_uchar).offset(8).offset(1) as uint64_t) << 8
            | (*(src as *const ::core::ffi::c_uchar).offset(8).offset(2) as uint64_t) << 16
            | (*(src as *const ::core::ffi::c_uchar).offset(8).offset(3) as uint64_t) << 24
            | (*(src as *const ::core::ffi::c_uchar).offset(8).offset(4) as uint64_t) << 32
            | (*(src as *const ::core::ffi::c_uchar).offset(8).offset(5) as uint64_t) << 40
            | (*(src as *const ::core::ffi::c_uchar).offset(8).offset(6) as uint64_t) << 48
            | (*(src as *const ::core::ffi::c_uchar).offset(8).offset(7) as uint64_t) << 56;
        return key;
    }

    pub unsafe extern "C" fn sip_round(
        mut H: *mut crate::siphash_h::siphash,
        rounds: ::core::ffi::c_int,
    ) {
        let mut i: ::core::ffi::c_int = 0;
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

    pub unsafe extern "C" fn sip24_init(
        mut H: *mut crate::siphash_h::siphash,
        mut key: *const crate::siphash_h::sipkey,
    ) -> *mut crate::siphash_h::siphash {
        (*H).v0 = ((0x736f6d65u64) << 32 | 0x70736575) ^ (*key).k[0];
        (*H).v1 = ((0x646f7261u64) << 32 | 0x6e646f6d) ^ (*key).k[1];
        (*H).v2 = ((0x6c796765u64) << 32 | 0x6e657261) ^ (*key).k[0];
        (*H).v3 = ((0x74656462u64) << 32 | 0x79746573) ^ (*key).k[1];
        (*H).p = &raw mut (*H).buf as *mut ::core::ffi::c_uchar;
        (*H).c = 0u64;
        return H;
    }

    pub unsafe extern "C" fn sip24_update(
        mut H: *mut crate::siphash_h::siphash,
        mut src: *const ::core::ffi::c_void,
        mut len: size_t,
    ) -> *mut crate::siphash_h::siphash {
        let mut p: *const ::core::ffi::c_uchar = src as *const ::core::ffi::c_uchar;
        let mut pe: *const ::core::ffi::c_uchar = p.offset(len as isize);
        let mut m: uint64_t = 0;
        loop {
            while p < pe
                && (*H).p
                    < (&raw mut (*H).buf as *mut ::core::ffi::c_uchar).offset(
                        (::core::mem::size_of::<[::core::ffi::c_uchar; 8]>())
                            .wrapping_div(::core::mem::size_of::<::core::ffi::c_uchar>())
                            as isize,
                    )
            {
                let c2rust_fresh0 = p;
                p = p.offset(1);
                let c2rust_fresh1 = (*H).p;
                (*H).p = (*H).p.offset(1);
                *c2rust_fresh1 = *c2rust_fresh0;
            }
            if (*H).p
                < (&raw mut (*H).buf as *mut ::core::ffi::c_uchar).offset(
                    (::core::mem::size_of::<[::core::ffi::c_uchar; 8]>())
                        .wrapping_div(::core::mem::size_of::<::core::ffi::c_uchar>())
                        as isize,
                )
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
            (*H).p = &raw mut (*H).buf as *mut ::core::ffi::c_uchar;
            (*H).c = (*H).c.wrapping_add(8u64);
            if !(p < pe) {
                break;
            }
        }
        return H;
    }

    pub unsafe extern "C" fn sip24_final(mut H: *mut crate::siphash_h::siphash) -> uint64_t {
        let left: ::core::ffi::c_char = (*H)
            .p
            .offset_from(&raw mut (*H).buf as *mut ::core::ffi::c_uchar)
            as ::core::ffi::c_char;
        let mut b: uint64_t = (*H).c.wrapping_add(left as uint64_t) << 56;
        let mut c2rust_current_block_6: u64;
        match left as ::core::ffi::c_int {
            7 => {
                b |= ((*H).buf[6] as uint64_t) << 48;
                c2rust_current_block_6 = 15934335782010006484;
            }
            6 => {
                c2rust_current_block_6 = 15934335782010006484;
            }
            5 => {
                c2rust_current_block_6 = 927806753801690604;
            }
            4 => {
                c2rust_current_block_6 = 5149041459065675557;
            }
            3 => {
                c2rust_current_block_6 = 17473121293339793080;
            }
            2 => {
                c2rust_current_block_6 = 8772994252565639119;
            }
            1 => {
                c2rust_current_block_6 = 9642291731049860871;
            }
            0 | _ => {
                c2rust_current_block_6 = 5720623009719927633;
            }
        }
        match c2rust_current_block_6 {
            15934335782010006484 => {
                b |= ((*H).buf[5] as uint64_t) << 40;
                c2rust_current_block_6 = 927806753801690604;
            }
            _ => {}
        }
        match c2rust_current_block_6 {
            927806753801690604 => {
                b |= ((*H).buf[4] as uint64_t) << 32;
                c2rust_current_block_6 = 5149041459065675557;
            }
            _ => {}
        }
        match c2rust_current_block_6 {
            5149041459065675557 => {
                b |= ((*H).buf[3] as uint64_t) << 24;
                c2rust_current_block_6 = 17473121293339793080;
            }
            _ => {}
        }
        match c2rust_current_block_6 {
            17473121293339793080 => {
                b |= ((*H).buf[2] as uint64_t) << 16;
                c2rust_current_block_6 = 8772994252565639119;
            }
            _ => {}
        }
        match c2rust_current_block_6 {
            8772994252565639119 => {
                b |= ((*H).buf[1] as uint64_t) << 8;
                c2rust_current_block_6 = 9642291731049860871;
            }
            _ => {}
        }
        match c2rust_current_block_6 {
            9642291731049860871 => {
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

    pub unsafe extern "C" fn siphash24(
        mut src: *const ::core::ffi::c_void,
        mut len: size_t,
        mut key: *const crate::siphash_h::sipkey,
    ) -> uint64_t {
        let mut state: crate::siphash_h::siphash = crate::siphash_h::siphash {
            v0: 0u64,
            v1: 0u64,
            v2: 0u64,
            v3: 0u64,
            buf: [0u8, 0, 0, 0, 0, 0, 0, 0],
            p: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
            c: 0u64,
        };
        return sip24_final(sip24_update(sip24_init(&raw mut state, key), src, len));
    }

    pub unsafe extern "C" fn sip24_valid() -> ::core::ffi::c_int {
        pub static mut vectors: [[::core::ffi::c_uchar; 8]; 64] = [
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
        let mut in_0: [::core::ffi::c_uchar; 64] = [0; 64];
        let mut k: crate::siphash_h::sipkey = crate::siphash_h::sipkey { k: [0; 2] };
        let mut i: size_t = 0;
        sip_tokey(
            &raw mut k,
            b"\0\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0B\x0C\r\x0E\x0F\0".as_ptr()
                as *const ::core::ffi::c_void,
        );
        i = 0;
        while i < ::core::mem::size_of::<[::core::ffi::c_uchar; 64]>() {
            in_0[i] = i as ::core::ffi::c_uchar;
            if siphash24(&raw mut in_0 as *const ::core::ffi::c_void, i, &raw mut k)
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
    use crate::__stddef_size_t_h::size_t;
    use crate::stdlib::uint64_t;
}

pub use crate::__stddef_size_t_h::size_t;

pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_ptrdiff_t_h::ptrdiff_t;
pub use crate::expat_config_h::XML_CONTEXT_BYTES;
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
pub use crate::expat_h::XML_EntityDeclHandler;
pub use crate::expat_h::XML_Error;
pub use crate::expat_h::XML_ExternalEntityRefHandler;
pub use crate::expat_h::XML_Feature;
pub use crate::expat_h::XML_FeatureEnum;
pub use crate::expat_h::XML_Memory_Handling_Suite;
pub use crate::expat_h::XML_NotStandaloneHandler;
pub use crate::expat_h::XML_NotationDeclHandler;
pub use crate::expat_h::XML_ParamEntityParsing;
pub use crate::expat_h::XML_Parser;
pub use crate::expat_h::XML_ParserStruct;
pub use crate::expat_h::XML_Parsing;
pub use crate::expat_h::XML_ParsingStatus;
pub use crate::expat_h::XML_ProcessingInstructionHandler;
pub use crate::expat_h::XML_SkippedEntityHandler;
pub use crate::expat_h::XML_StartCdataSectionHandler;
pub use crate::expat_h::XML_StartDoctypeDeclHandler;
pub use crate::expat_h::XML_StartElementHandler;
pub use crate::expat_h::XML_Status;
pub use crate::expat_h::XML_UnknownEncodingHandler;
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
pub use crate::limits_h::INT_MAX;
pub use crate::siphash_h::siphash;
pub use crate::siphash_h::sipkey;
use crate::src::lib::xmlparse::g_bytesScanned;
use crate::src::lib::xmlparse::g_reparseDeferralEnabledDefault;
pub use crate::src::lib::xmlparse::XML_ErrorString;
pub use crate::src::lib::xmlparse::XML_ExternalEntityParserCreate;
pub use crate::src::lib::xmlparse::XML_FreeContentModel;
pub use crate::src::lib::xmlparse::XML_GetBase;
pub use crate::src::lib::xmlparse::XML_GetBuffer;
pub use crate::src::lib::xmlparse::XML_GetCurrentByteCount;
pub use crate::src::lib::xmlparse::XML_GetCurrentByteIndex;
pub use crate::src::lib::xmlparse::XML_GetCurrentColumnNumber;
pub use crate::src::lib::xmlparse::XML_GetCurrentLineNumber;
pub use crate::src::lib::xmlparse::XML_GetErrorCode;
pub use crate::src::lib::xmlparse::XML_GetFeatureList;
pub use crate::src::lib::xmlparse::XML_GetInputContext;
pub use crate::src::lib::xmlparse::XML_GetParsingStatus;
pub use crate::src::lib::xmlparse::XML_MemFree;
pub use crate::src::lib::xmlparse::XML_MemMalloc;
pub use crate::src::lib::xmlparse::XML_MemRealloc;
pub use crate::src::lib::xmlparse::XML_Parse;
pub use crate::src::lib::xmlparse::XML_ParseBuffer;
pub use crate::src::lib::xmlparse::XML_ParserCreate;
pub use crate::src::lib::xmlparse::XML_ParserCreate_MM;
pub use crate::src::lib::xmlparse::XML_ParserFree;
pub use crate::src::lib::xmlparse::XML_ParserReset;
pub use crate::src::lib::xmlparse::XML_ResumeParser;
pub use crate::src::lib::xmlparse::XML_SetAllocTrackerActivationThreshold;
pub use crate::src::lib::xmlparse::XML_SetAttlistDeclHandler;
pub use crate::src::lib::xmlparse::XML_SetBase;
pub use crate::src::lib::xmlparse::XML_SetCharacterDataHandler;
pub use crate::src::lib::xmlparse::XML_SetCommentHandler;
pub use crate::src::lib::xmlparse::XML_SetDefaultHandler;
pub use crate::src::lib::xmlparse::XML_SetDefaultHandlerExpand;
pub use crate::src::lib::xmlparse::XML_SetDoctypeDeclHandler;
pub use crate::src::lib::xmlparse::XML_SetElementDeclHandler;
pub use crate::src::lib::xmlparse::XML_SetElementHandler;
pub use crate::src::lib::xmlparse::XML_SetEncoding;
pub use crate::src::lib::xmlparse::XML_SetEndCdataSectionHandler;
pub use crate::src::lib::xmlparse::XML_SetEndDoctypeDeclHandler;
pub use crate::src::lib::xmlparse::XML_SetEndElementHandler;
pub use crate::src::lib::xmlparse::XML_SetEntityDeclHandler;
pub use crate::src::lib::xmlparse::XML_SetExternalEntityRefHandler;
pub use crate::src::lib::xmlparse::XML_SetExternalEntityRefHandlerArg;
pub use crate::src::lib::xmlparse::XML_SetHashSalt;
pub use crate::src::lib::xmlparse::XML_SetNotStandaloneHandler;
pub use crate::src::lib::xmlparse::XML_SetNotationDeclHandler;
pub use crate::src::lib::xmlparse::XML_SetParamEntityParsing;
pub use crate::src::lib::xmlparse::XML_SetProcessingInstructionHandler;
pub use crate::src::lib::xmlparse::XML_SetReparseDeferralEnabled;
pub use crate::src::lib::xmlparse::XML_SetSkippedEntityHandler;
pub use crate::src::lib::xmlparse::XML_SetStartCdataSectionHandler;
pub use crate::src::lib::xmlparse::XML_SetStartDoctypeDeclHandler;
pub use crate::src::lib::xmlparse::XML_SetStartElementHandler;
pub use crate::src::lib::xmlparse::XML_SetUnknownEncodingHandler;
pub use crate::src::lib::xmlparse::XML_SetUserData;
pub use crate::src::lib::xmlparse::XML_SetXmlDeclHandler;
pub use crate::src::lib::xmlparse::XML_UseForeignDTD;
pub use crate::src::lib::xmlparse::XML_UseParserAsHandlerArg;
use crate::src::lib::xmltok::_INTERNAL_trim_to_complete_utf8_characters;
pub use crate::src::tests::basic_tests::siphash_h::sip24_final;
pub use crate::src::tests::basic_tests::siphash_h::sip24_init;
pub use crate::src::tests::basic_tests::siphash_h::sip24_update;
pub use crate::src::tests::basic_tests::siphash_h::sip24_valid;
pub use crate::src::tests::basic_tests::siphash_h::sip_round;
pub use crate::src::tests::basic_tests::siphash_h::sip_tokey;
pub use crate::src::tests::basic_tests::siphash_h::siphash24;
pub use crate::src::tests::chardata::CharData;
pub use crate::src::tests::chardata::CharData_CheckXMLChars;
pub use crate::src::tests::chardata::CharData_Init;
pub use crate::src::tests::common::basic_teardown;
pub use crate::src::tests::common::g_abortable;
pub use crate::src::tests::common::g_chunkSize;
pub use crate::src::tests::common::g_parser;
pub use crate::src::tests::common::g_resumable;
pub use crate::src::tests::common::get_buffer_test_text;
pub use crate::src::tests::common::long_cdata_text;
pub use crate::src::tests::common::long_character_data_text;
pub use crate::src::tests::common::tcase_add_test__if_xml_ge;
pub use crate::src::tests::common::tcase_add_test__ifdef_xml_dtd;
pub use crate::src::tests::common::ExtTest;
pub use crate::src::tests::common::_XML_Parse_SINGLE_BYTES;
pub use crate::src::tests::common::_expect_failure;
pub use crate::src::tests::common::_run_attribute_check;
pub use crate::src::tests::common::_run_character_check;
pub use crate::src::tests::common::_run_ext_character_check;
pub use crate::src::tests::common::_xml_failure;
pub use crate::src::tests::dummy::dummy_attlist_decl_handler;
pub use crate::src::tests::dummy::dummy_cdata_handler;
pub use crate::src::tests::dummy::dummy_comment_handler;
pub use crate::src::tests::dummy::dummy_default_handler;
pub use crate::src::tests::dummy::dummy_element_decl_handler;
pub use crate::src::tests::dummy::dummy_end_cdata_handler;
pub use crate::src::tests::dummy::dummy_end_doctype_handler;
pub use crate::src::tests::dummy::dummy_end_element;
pub use crate::src::tests::dummy::dummy_entity_decl_handler;
pub use crate::src::tests::dummy::dummy_notation_decl_handler;
pub use crate::src::tests::dummy::dummy_pi_handler;
pub use crate::src::tests::dummy::dummy_skip_handler;
pub use crate::src::tests::dummy::dummy_start_cdata_handler;
pub use crate::src::tests::dummy::dummy_start_doctype_handler;
pub use crate::src::tests::dummy::dummy_start_element;
pub use crate::src::tests::dummy::dummy_xdecl_handler;
pub use crate::src::tests::dummy::get_dummy_handler_flags;
pub use crate::src::tests::dummy::init_dummy_handlers;
pub use crate::src::tests::dummy::DUMMY_ELEMENT_DECL_HANDLER_FLAG;
pub use crate::src::tests::dummy::DUMMY_NOTATION_DECL_HANDLER_FLAG;
pub use crate::src::tests::dummy::DUMMY_SKIP_HANDLER_FLAG;
pub use crate::src::tests::handlers::_handler_record_get;
pub use crate::src::tests::handlers::accept_not_standalone_handler;
pub use crate::src::tests::handlers::accumulate_and_suspend_comment_handler;
pub use crate::src::tests::handlers::accumulate_attribute;
pub use crate::src::tests::handlers::accumulate_char_data_and_suspend;
pub use crate::src::tests::handlers::accumulate_characters;
pub use crate::src::tests::handlers::accumulate_comment;
pub use crate::src::tests::handlers::accumulate_entity_decl;
pub use crate::src::tests::handlers::accumulate_pi_characters;
pub use crate::src::tests::handlers::attrInfo;
pub use crate::src::tests::handlers::byte_character_handler;
pub use crate::src::tests::handlers::checking_default_handler;
pub use crate::src::tests::handlers::clearing_aborting_character_handler;
pub use crate::src::tests::handlers::counting_start_element_handler;
pub use crate::src::tests::handlers::cr_cdata_handler;
pub use crate::src::tests::handlers::data_check_comment_handler;
pub use crate::src::tests::handlers::default_check;
pub use crate::src::tests::handlers::elementInfo;
pub use crate::src::tests::handlers::element_decl_suspender;
pub use crate::src::tests::handlers::end_element_event_handler;
pub use crate::src::tests::handlers::end_element_event_handler2;
pub use crate::src::tests::handlers::entity_suspending_xdecl_handler;
pub use crate::src::tests::handlers::ext2_accumulate_characters;
pub use crate::src::tests::handlers::ext_faults;
pub use crate::src::tests::handlers::ext_hdlr_data;
pub use crate::src::tests::handlers::external_entity_bad_cr_catcher;
pub use crate::src::tests::handlers::external_entity_cr_catcher;
pub use crate::src::tests::handlers::external_entity_devaluer;
pub use crate::src::tests::handlers::external_entity_faulter;
pub use crate::src::tests::handlers::external_entity_faulter2;
pub use crate::src::tests::handlers::external_entity_good_cdata_ascii;
pub use crate::src::tests::handlers::external_entity_load_ignore;
pub use crate::src::tests::handlers::external_entity_load_ignore_utf16;
pub use crate::src::tests::handlers::external_entity_load_ignore_utf16_be;
pub use crate::src::tests::handlers::external_entity_loader;
pub use crate::src::tests::handlers::external_entity_loader2;
pub use crate::src::tests::handlers::external_entity_not_standalone;
pub use crate::src::tests::handlers::external_entity_null_loader;
pub use crate::src::tests::handlers::external_entity_oneshot_loader;
pub use crate::src::tests::handlers::external_entity_param;
pub use crate::src::tests::handlers::external_entity_param_checker;
pub use crate::src::tests::handlers::external_entity_public;
pub use crate::src::tests::handlers::external_entity_ref_param_checker;
pub use crate::src::tests::handlers::external_entity_resetter;
pub use crate::src::tests::handlers::external_entity_rsqb_catcher;
pub use crate::src::tests::handlers::external_entity_suspend_xmldecl;
pub use crate::src::tests::handlers::external_entity_suspender;
pub use crate::src::tests::handlers::external_entity_suspending_faulter;
pub use crate::src::tests::handlers::external_entity_unfinished_attlist;
pub use crate::src::tests::handlers::external_entity_value_aborter;
pub use crate::src::tests::handlers::external_entity_valuer;
pub use crate::src::tests::handlers::g_comment_count;
pub use crate::src::tests::handlers::g_handler_data;
pub use crate::src::tests::handlers::g_skip_count;
pub use crate::src::tests::handlers::g_xdecl_count;
pub use crate::src::tests::handlers::get_param_entity_match_flag;
pub use crate::src::tests::handlers::handler_record_entry;
pub use crate::src::tests::handlers::handler_record_list;
pub use crate::src::tests::handlers::param_check_skip_handler;
pub use crate::src::tests::handlers::param_entity_match_handler;
pub use crate::src::tests::handlers::param_entity_match_init;
pub use crate::src::tests::handlers::parser_stop_character_handler;
pub use crate::src::tests::handlers::record_cdata_handler;
pub use crate::src::tests::handlers::record_cdata_nodefault_handler;
pub use crate::src::tests::handlers::record_default_handler;
pub use crate::src::tests::handlers::record_element_end_handler;
pub use crate::src::tests::handlers::record_element_start_handler;
pub use crate::src::tests::handlers::record_skip_handler;
pub use crate::src::tests::handlers::reject_not_standalone_handler;
pub use crate::src::tests::handlers::rsqb_handler;
pub use crate::src::tests::handlers::selective_aborting_default_handler;
pub use crate::src::tests::handlers::start_element_event_handler;
pub use crate::src::tests::handlers::start_element_event_handler2;
pub use crate::src::tests::handlers::start_element_suspender;
pub use crate::src::tests::handlers::suspending_comment_handler;
pub use crate::src::tests::handlers::suspending_end_handler;
pub use crate::src::tests::handlers::user_data_checking_unknown_encoding_handler;
pub use crate::src::tests::handlers::verify_attlist_decl_handler;
pub use crate::src::tests::handlers::xml_decl_handler;
pub use crate::src::tests::handlers::AttTest;
pub use crate::src::tests::handlers::AttrInfo;
pub use crate::src::tests::handlers::ByteTestData;
pub use crate::src::tests::handlers::DefaultCheck;
pub use crate::src::tests::handlers::ElementInfo;
pub use crate::src::tests::handlers::ExtFaults;
pub use crate::src::tests::handlers::ExtFaults2;
pub use crate::src::tests::handlers::ExtHdlrData;
pub use crate::src::tests::handlers::ExtTest2;
pub use crate::src::tests::handlers::MiscEncodingHandler;
pub use crate::src::tests::handlers::ParserAndElementInfo;
pub use crate::src::tests::handlers::ParserPlusStorage;
pub use crate::src::tests::handlers::StructParserAndElementInfo;
pub use crate::src::tests::handlers::UnknownEncodingHandler;
pub use crate::src::tests::handlers::UnrecognisedEncodingHandler;
pub use crate::src::tests::handlers::ENTITY_MATCH_FAIL;
pub use crate::src::tests::handlers::ENTITY_MATCH_NOT_FOUND;
pub use crate::src::tests::handlers::STRUCT_END_TAG;
pub use crate::src::tests::handlers::STRUCT_START_TAG;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
use crate::stdlib::__assert_fail;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::__uint64_t;
use crate::stdlib::fprintf;
use crate::stdlib::free;
pub use crate::stdlib::intptr_t;
use crate::stdlib::malloc;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
use crate::stdlib::printf;
use crate::stdlib::realloc;
use crate::stdlib::snprintf;
use crate::stdlib::stderr;
use crate::stdlib::strcmp;
use crate::stdlib::strlen;
pub use crate::stdlib::uint32_t;
pub use crate::stdlib::uint64_t;
pub use crate::stdlib::uintptr_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

pub use crate::src::tests::minicheck::set_subtest;
pub use crate::src::tests::minicheck::tcase_setup_function;
pub use crate::src::tests::minicheck::tcase_teardown_function;
pub use crate::src::tests::minicheck::tcase_test_function;
pub use crate::src::tests::minicheck::Suite;
pub use crate::src::tests::minicheck::TCase;
pub use crate::src::tests::minicheck::_check_set_test_info;
pub use crate::src::tests::minicheck::_fail;
pub use crate::src::tests::minicheck::suite_add_tcase;
pub use crate::src::tests::minicheck::tcase_add_checked_fixture;
pub use crate::src::tests::minicheck::tcase_add_test;
pub use crate::src::tests::minicheck::tcase_create;
pub use crate::src::tests::structdata::StructData;
pub use crate::src::tests::structdata::StructDataEntry;
pub use crate::src::tests::structdata::StructData_CheckItems;
pub use crate::src::tests::structdata::StructData_Dispose;
pub use crate::src::tests::structdata::StructData_Init;
pub use crate::stdbool_h::false_0;
pub use crate::stdbool_h::true_0;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct element_decl_data {
    pub parser: XML_Parser,
    pub count: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct C2Rust_Unnamed_10 {
    pub pre: *const ::core::ffi::c_char,
    pub post: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct test_case {
    pub goodName: bool,
    pub goodNameStart: bool,
    pub tagName: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct TestCase {
    pub doc: *const ::core::ffi::c_char,
    pub expectedStatus: XML_Status,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct bom_testdata {
    pub external: *const ::core::ffi::c_char,
    pub split: ::core::ffi::c_int,
    pub nested_callback_happened: XML_Bool,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct CaseData {
    pub text_bytes: size_t,
    pub text: *const ::core::ffi::c_char,
    pub expected_error: XML_Error,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct CaseData_0 {
    pub text: *const ::core::ffi::c_char,
    pub expectedError: XML_Error,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct TestCase_0 {
    pub doc: *const ::core::ffi::c_char,
    pub usesParameterEntities: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct TestCase_1 {
    pub expectedMovementInChars: ptrdiff_t,
    pub input: *const ::core::ffi::c_char,
}

unsafe extern "C" fn basic_setup() {
    g_parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
    if g_parser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            75i32,
            b"Parser not created.\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_nul_byte() {
    _check_set_test_info(
        b"test_nul_byte\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        82,
    );
    let mut text: [::core::ffi::c_char; 13] =
        ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"<doc>\0</doc>\0");
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        &raw mut text as *mut ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 13]>()).wrapping_sub(1usize)
            as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_OK
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            88i32,
            b"Parser did not report error on NUL-byte.\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_INVALID_TOKEN {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            90i32,
        );
    }
}

unsafe extern "C" fn test_u0000_char() {
    _check_set_test_info(
        b"test_u0000_char\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        94,
    );
    _expect_failure(
        b"<doc>&#0;</doc>\0".as_ptr() as *const ::core::ffi::c_char,
        XML_ERROR_BAD_CHAR_REF,
        b"Parser did not report error on NUL-byte.\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        97,
    );
}

unsafe extern "C" fn test_siphash_self() {
    _check_set_test_info(
        b"test_siphash_self\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        101,
    );
    if sip24_valid() == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            103i32,
            b"SipHash self-test failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_siphash_spec() {
    _check_set_test_info(
        b"test_siphash_spec\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        107,
    );
    let message: [::core::ffi::c_char; 16] =
        ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(
            *b"\0\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0B\x0C\r\x0E\0",
        );
    let len: size_t = (::core::mem::size_of::<[::core::ffi::c_char; 16]>()).wrapping_sub(1usize);
    let expected: uint64_t = (0xa129ca61) << 32 | 0x49be45e5;
    let mut state: siphash = siphash {
        v0: 0,
        v1: 0,
        v2: 0,
        v3: 0,
        buf: [0; 8],
        p: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        c: 0,
    };
    let mut key: sipkey = sipkey { k: [0; 2] };
    sip_tokey(
        &raw mut key,
        b"\0\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0B\x0C\r\x0E\x0F\0".as_ptr()
            as *const ::core::ffi::c_void,
    );
    sip24_init(&raw mut state, &raw mut key);
    sip24_update(
        &raw mut state,
        &raw const message as *const ::core::ffi::c_void,
        4,
    );
    sip24_update(
        &raw mut state,
        (&raw const message as *const ::core::ffi::c_char).offset(4) as *const ::core::ffi::c_void,
        len.wrapping_sub(4usize),
    );
    sip24_update(
        &raw mut state,
        &raw const message as *const ::core::ffi::c_void,
        0,
    );
    if sip24_final(&raw mut state) != expected {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            128i32,
            b"sip24_final failed spec test\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if siphash24(
        &raw const message as *const ::core::ffi::c_void,
        len,
        &raw mut key,
    ) != expected
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            132i32,
            b"siphash24 failed spec test\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_bom_utf8() {
    _check_set_test_info(
        b"test_bom_utf8\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        136,
    );
    let mut text: *const ::core::ffi::c_char =
        b"\xEF\xBB\xBF<e/>\0".as_ptr() as *const ::core::ffi::c_char;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            142i32,
        );
    }
}

unsafe extern "C" fn test_bom_utf16_be() {
    _check_set_test_info(
        b"test_bom_utf16_be\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        146,
    );
    let mut text: [::core::ffi::c_char; 11] =
        ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"\xFE\xFF\0<\0e\0/\0>\0");
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        &raw mut text as *mut ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 11]>()).wrapping_sub(1usize)
            as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            151i32,
        );
    }
}

unsafe extern "C" fn test_bom_utf16_le() {
    _check_set_test_info(
        b"test_bom_utf16_le\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        155,
    );
    let mut text: [::core::ffi::c_char; 11] =
        ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"\xFF\xFE<\0e\0/\0>\0\0");
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        &raw mut text as *mut ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 11]>()).wrapping_sub(1usize)
            as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            160i32,
        );
    }
}

unsafe extern "C" fn test_nobom_utf16_le() {
    _check_set_test_info(
        b"test_nobom_utf16_le\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        164,
    );
    let mut text: [::core::ffi::c_char; 11] =
        ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b" \0<\0e\0/\0>\0\0");
    if g_chunkSize == 1 {
        return;
    }
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        &raw mut text as *mut ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 11]>()).wrapping_sub(1usize)
            as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            175i32,
        );
    }
}

unsafe extern "C" fn test_hash_collision() {
    _check_set_test_info(
        b"test_hash_collision\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        179,
    );
    let mut text: *const ::core::ffi::c_char = b"<doc>\n<a1/><a2/><a3/><a4/><a5/><a6/><a7/><a8/>\n<b1></b1><b2 attr='foo'>This is a foo</b2><b3></b3><b4></b4>\n<b5></b5><b6></b6><b7></b7><b8></b8>\n<c1/><c2/><c3/><c4/><c5/><c6/><c7/><c8/>\n<d1/><d2/><d3/><d4/><d5/><d6/><d7/>\n<d8>This triggers the table growth and collides with b2</d8>\n</doc>\n\0"
        .as_ptr() as *const ::core::ffi::c_char;
    XML_SetHashSalt(g_parser, (0xffffffff) << 32 | 0xff99fc90);
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            202i32,
        );
    }
}

unsafe extern "C" fn test_danish_latin1() {
    _check_set_test_info(
        b"test_danish_latin1\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        208,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='iso-8859-1'?>\n<e>J\xF8rgen \xE6\xF8\xE5\xC6\xD8\xC5</e>\0"
            .as_ptr() as *const ::core::ffi::c_char;
    let mut expected: *const XML_Char =
        b"J\xC3\xB8rgen \xC3\xA6\xC3\xB8\xC3\xA5\xC3\x86\xC3\x98\xC3\x85\0".as_ptr()
            as *const XML_Char;
    _run_character_check(
        text,
        expected,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        218,
    );
}

unsafe extern "C" fn test_french_charref_hexidecimal() {
    _check_set_test_info(
        b"test_french_charref_hexidecimal\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        223,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='iso-8859-1'?>\n<doc>&#xE9;&#xE8;&#xE0;&#xE7;&#xEA;&#xC8;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut expected: *const XML_Char =
        b"\xC3\xA9\xC3\xA8\xC3\xA0\xC3\xA7\xC3\xAA\xC3\x88\0".as_ptr() as *const XML_Char;
    _run_character_check(
        text,
        expected,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        232,
    );
}

unsafe extern "C" fn test_french_charref_decimal() {
    _check_set_test_info(
        b"test_french_charref_decimal\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        236,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='iso-8859-1'?>\n<doc>&#233;&#232;&#224;&#231;&#234;&#200;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut expected: *const XML_Char =
        b"\xC3\xA9\xC3\xA8\xC3\xA0\xC3\xA7\xC3\xAA\xC3\x88\0".as_ptr() as *const XML_Char;
    _run_character_check(
        text,
        expected,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        245,
    );
}

unsafe extern "C" fn test_french_latin1() {
    _check_set_test_info(
        b"test_french_latin1\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        249,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='iso-8859-1'?>\n<doc>\xE9\xE8\xE0\xE7\xEA\xC8</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
    let mut expected: *const XML_Char =
        b"\xC3\xA9\xC3\xA8\xC3\xA0\xC3\xA7\xC3\xAA\xC3\x88\0".as_ptr() as *const XML_Char;
    _run_character_check(
        text,
        expected,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        258,
    );
}

unsafe extern "C" fn test_french_utf8() {
    _check_set_test_info(
        b"test_french_utf8\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        262,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='utf-8'?>\n<doc>\xC3\xA9</doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut expected: *const XML_Char = b"\xC3\xA9\0".as_ptr() as *const XML_Char;
    _run_character_check(
        text,
        expected,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        270,
    );
}

unsafe extern "C" fn test_utf8_false_rejection() {
    _check_set_test_info(
        b"test_utf8_false_rejection\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        279,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc>\xEF\xBA\xBF</doc>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut expected: *const XML_Char = b"\xEF\xBA\xBF\0".as_ptr() as *const XML_Char;
    _run_character_check(
        text,
        expected,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        286,
    );
}

unsafe extern "C" fn test_illegal_utf8() {
    _check_set_test_info(
        b"test_illegal_utf8\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        295,
    );
    let mut text: [::core::ffi::c_char; 100] = [0; 100];
    let mut i: ::core::ffi::c_int = 0;
    i = 128;
    while i <= 255 {
        snprintf(
            &raw mut text as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 100]>(),
            b"<e>%ccd</e>\0".as_ptr() as *const ::core::ffi::c_char,
            i,
        );
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            &raw mut text as *mut ::core::ffi::c_char,
            strlen(&raw mut text as *mut ::core::ffi::c_char) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) == XML_STATUS_OK
        {
            snprintf(
                &raw mut text as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 100]>(),
                b"expected token error for '%c' (ordinal %d) in UTF-8 text\0".as_ptr()
                    as *const ::core::ffi::c_char,
                i,
                i,
            );
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                306i32,
                &raw mut text as *mut ::core::ffi::c_char,
            );
        } else if XML_GetErrorCode(g_parser) != XML_ERROR_INVALID_TOKEN {
            _xml_failure(
                g_parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                308i32,
            );
        }
        XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
        i += 1;
    }
}

pub const UTF8_LEAD_1: [::core::ffi::c_char; 2] =
    unsafe { ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"\x7F\0") };

pub const UTF8_LEAD_2: [::core::ffi::c_char; 2] =
    unsafe { ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"\xDF\0") };

pub const UTF8_LEAD_3: [::core::ffi::c_char; 2] =
    unsafe { ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"\xEF\0") };

pub const UTF8_LEAD_4: [::core::ffi::c_char; 2] =
    unsafe { ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"\xF7\0") };

unsafe extern "C" fn test_utf8_auto_align() {
    _check_set_test_info(
        b"test_utf8_auto_align\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        322,
    );
    let mut cases: [TestCase_1; 11] = [
        TestCase_1 {
            expectedMovementInChars: 0isize,
            input: b"\0".as_ptr() as *const ::core::ffi::c_char,
        },
        TestCase_1 {
            expectedMovementInChars: 0isize,
            input: UTF8_LEAD_1.as_ptr(),
        },
        TestCase_1 {
            expectedMovementInChars: -1isize,
            input: UTF8_LEAD_2.as_ptr(),
        },
        TestCase_1 {
            expectedMovementInChars: 0isize,
            input: b"\xDF\xBF\0".as_ptr() as *const ::core::ffi::c_char,
        },
        TestCase_1 {
            expectedMovementInChars: -1isize,
            input: UTF8_LEAD_3.as_ptr(),
        },
        TestCase_1 {
            expectedMovementInChars: -2isize,
            input: b"\xEF\xBF\0".as_ptr() as *const ::core::ffi::c_char,
        },
        TestCase_1 {
            expectedMovementInChars: 0isize,
            input: b"\xEF\xBF\xBF\0".as_ptr() as *const ::core::ffi::c_char,
        },
        TestCase_1 {
            expectedMovementInChars: -1isize,
            input: UTF8_LEAD_4.as_ptr(),
        },
        TestCase_1 {
            expectedMovementInChars: -2isize,
            input: b"\xF7\xBF\0".as_ptr() as *const ::core::ffi::c_char,
        },
        TestCase_1 {
            expectedMovementInChars: -3isize,
            input: b"\xF7\xBF\xBF\0".as_ptr() as *const ::core::ffi::c_char,
        },
        TestCase_1 {
            expectedMovementInChars: 0isize,
            input: b"\xF7\xBF\xBF\xBF\0".as_ptr() as *const ::core::ffi::c_char,
        },
    ];
    let mut i: size_t = 0;
    let mut success: bool = true_0 != 0;
    while i
        < (::core::mem::size_of::<[TestCase_1; 11]>())
            .wrapping_div(::core::mem::size_of::<TestCase_1>())
    {
        let mut fromLim: *const ::core::ffi::c_char =
            cases[i].input.offset(strlen(cases[i].input) as isize);
        let fromLimInitially: *const ::core::ffi::c_char = fromLim;
        let mut actualMovementInChars: ptrdiff_t = 0;
        _INTERNAL_trim_to_complete_utf8_characters(cases[i].input, &raw mut fromLim);
        actualMovementInChars = fromLim.offset_from(fromLimInitially);
        if actualMovementInChars != cases[i].expectedMovementInChars {
            let mut j: size_t = 0;
            success = false_0 != 0;
            printf(
                b"[-] UTF-8 case %2u: Expected movement by %2d chars, actually moved by %2d chars: \"\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                i.wrapping_add(1usize) as ::core::ffi::c_uint,
                cases[i].expectedMovementInChars as ::core::ffi::c_int,
                actualMovementInChars as ::core::ffi::c_int,
            );
            while j < strlen(cases[i].input) {
                printf(
                    b"\\x%02x\0".as_ptr() as *const ::core::ffi::c_char,
                    *cases[i].input.offset(j as isize) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int,
                );
                j = j.wrapping_add(1);
            }
            printf(b"\"\n\0".as_ptr() as *const ::core::ffi::c_char);
        }
        i = i.wrapping_add(1);
    }
    if !success {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            371i32,
            b"UTF-8 auto-alignment is not bullet-proof\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_utf16() {
    _check_set_test_info(
        b"test_utf16\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        376,
    );
    let mut text: [::core::ffi::c_char; 141] = ::core::mem::transmute::<
        [u8; 141],
        [::core::ffi::c_char; 141],
    >(
        *b"\0<\0?\0x\0m\0l\0 \0v\0e\0r\0s\0i\0o\0n\0=\0'\x001\0.\x000\0'\0 \0e\0n\0c\0o\0d\0i\0n\0g\0=\0'\0U\0T\0F\0-\x001\x006\0'\0?\0>\0\n\0<\0d\0o\0c\0 \0a\0=\0'\x001\x002\x003\0'\0>\0s\0o\0m\0e\0 \xFF!\0 \0t\0e\0x\0t\0<\0/\0d\0o\0c\0>\0",
    );
    let mut expected: *const XML_Char = b"some \xEF\xBC\xA1 text\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        &raw mut text as *mut ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 141]>()).wrapping_sub(1usize)
            as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            402i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_utf16_le_epilog_newline() {
    _check_set_test_info(
        b"test_utf16_le_epilog_newline\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        407,
    );
    let mut first_chunk_bytes: ::core::ffi::c_uint = 17;
    let mut text: [::core::ffi::c_char; 19] = ::core::mem::transmute::<
        [u8; 19],
        [::core::ffi::c_char; 19],
    >(*b"\xFF\xFE<\0e\0/\0>\0\r\0\n\0\r\0\n\0\0");
    if first_chunk_bytes as usize
        >= (::core::mem::size_of::<[::core::ffi::c_char; 19]>()).wrapping_sub(1usize)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            414i32,
            b"bad value of first_chunk_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        &raw mut text as *mut ::core::ffi::c_char,
        first_chunk_bytes as ::core::ffi::c_int,
        XML_FALSE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            417i32,
        );
    } else {
        let mut rc: XML_Status = XML_STATUS_ERROR;
        rc = _XML_Parse_SINGLE_BYTES(
            g_parser,
            (&raw mut text as *mut ::core::ffi::c_char).offset(first_chunk_bytes as isize),
            (::core::mem::size_of::<[::core::ffi::c_char; 19]>())
                .wrapping_sub(first_chunk_bytes as usize)
                .wrapping_sub(1usize) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        );
        if rc == XML_STATUS_ERROR {
            _xml_failure(
                g_parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                424i32,
            );
        }
    };
}

unsafe extern "C" fn test_not_utf16() {
    _check_set_test_info(
        b"test_not_utf16\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        430,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='utf-16'?><doc>Hi</doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    XML_SetXmlDeclHandler(
        g_parser,
        Some(
            dummy_xdecl_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    _expect_failure(
        text,
        XML_ERROR_INCORRECT_ENCODING,
        b"UTF-16 declared in UTF-8 not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        437,
    );
}

unsafe extern "C" fn test_bad_encoding() {
    _check_set_test_info(
        b"test_bad_encoding\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        442,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc>Hi</doc>\0".as_ptr() as *const ::core::ffi::c_char;
    if XML_SetEncoding(g_parser, b"unknown-encoding\0".as_ptr() as *const XML_Char) as u64 == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            446i32,
            b"XML_SetEncoding failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    _expect_failure(
        text,
        XML_ERROR_UNKNOWN_ENCODING,
        b"Unknown encoding not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        448,
    );
}

unsafe extern "C" fn test_latin1_umlauts() {
    _check_set_test_info(
        b"test_latin1_umlauts\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        453,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='iso-8859-1'?>\n<e a='\xE4 \xF6 \xFC &#228; &#246; &#252; &#x00E4; &#x0F6; &#xFC; >'\n  >\xE4 \xF6 \xFC &#228; &#246; &#252; &#x00E4; &#x0F6; &#xFC; ></e>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut expected: *const XML_Char =
        b"\xC3\xA4 \xC3\xB6 \xC3\xBC \xC3\xA4 \xC3\xB6 \xC3\xBC \xC3\xA4 \xC3\xB6 \xC3\xBC >\0"
            .as_ptr() as *const XML_Char;
    _run_character_check(
        text,
        expected,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        468,
    );
    XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
    _run_attribute_check(
        text,
        expected,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        470,
    );
    XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
    XML_SetDefaultHandler(
        g_parser,
        Some(
            dummy_default_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    _run_character_check(
        text,
        expected,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        474,
    );
    XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
    XML_SetDefaultHandler(
        g_parser,
        Some(
            dummy_default_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    _run_attribute_check(
        text,
        expected,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        477,
    );
}

unsafe extern "C" fn test_long_utf8_character() {
    _check_set_test_info(
        b"test_long_utf8_character\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        482,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='utf-8'?>\n<do\xF0\x90\x80\x80/>\0".as_ptr()
            as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"4-byte UTF-8 character in element name not faulted\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        488,
    );
}

unsafe extern "C" fn test_long_latin1_attribute() {
    _check_set_test_info(
        b"test_long_latin1_attribute\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        495,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='iso-8859-1'?>\n<doc att='ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO\xE4'>\n</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut expected: *const XML_Char = b"ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO\xC3\xA4\0"
        .as_ptr() as *const XML_Char;
    _run_attribute_check(
        text,
        expected,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        545,
    );
}

unsafe extern "C" fn test_long_ascii_attribute() {
    _check_set_test_info(
        b"test_long_ascii_attribute\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        552,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='us-ascii'?>\n<doc att='ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP01234'>\n</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut expected: *const XML_Char = b"ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP01234\0"
        .as_ptr() as *const XML_Char;
    _run_attribute_check(
        text,
        expected,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        596,
    );
}

unsafe extern "C" fn test_line_number_after_parse() {
    _check_set_test_info(
        b"test_line_number_after_parse\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        601,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<tag>\n\n\n</tag>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut lineno: XML_Size = 0;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            609i32,
        );
    }
    lineno = XML_GetCurrentLineNumber(g_parser);
    if lineno != 4 {
        let mut buffer: [::core::ffi::c_char; 100] = [0; 100];
        snprintf(
            &raw mut buffer as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 100]>(),
            b"expected 4 lines, saw %lu\0".as_ptr() as *const ::core::ffi::c_char,
            lineno,
        );
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            615i32,
            &raw mut buffer as *mut ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_column_number_after_parse() {
    _check_set_test_info(
        b"test_column_number_after_parse\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        621,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<tag></tag>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut colno: XML_Size = 0;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            627i32,
        );
    }
    colno = XML_GetCurrentColumnNumber(g_parser);
    if colno != 11 {
        let mut buffer: [::core::ffi::c_char; 100] = [0; 100];
        snprintf(
            &raw mut buffer as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 100]>(),
            b"expected 11 columns, saw %lu\0".as_ptr() as *const ::core::ffi::c_char,
            colno,
        );
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            633i32,
            &raw mut buffer as *mut ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_line_and_column_numbers_inside_handlers() {
    _check_set_test_info(
        b"test_line_and_column_numbers_inside_handlers\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        639,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<a>\n  <b>\r\n    <c/>\r  </b>\n  <d>\n    <f/>\n  </d>\n</a>\0".as_ptr()
            as *const ::core::ffi::c_char;
    let expected: [StructDataEntry; 10] = [
        StructDataEntry {
            str: b"a\0".as_ptr() as *const XML_Char,
            data0: 0,
            data1: 1,
            data2: STRUCT_START_TAG,
        },
        StructDataEntry {
            str: b"b\0".as_ptr() as *const XML_Char,
            data0: 2,
            data1: 2,
            data2: STRUCT_START_TAG,
        },
        StructDataEntry {
            str: b"c\0".as_ptr() as *const XML_Char,
            data0: 4,
            data1: 3,
            data2: STRUCT_START_TAG,
        },
        StructDataEntry {
            str: b"c\0".as_ptr() as *const XML_Char,
            data0: 8,
            data1: 3,
            data2: STRUCT_END_TAG,
        },
        StructDataEntry {
            str: b"b\0".as_ptr() as *const XML_Char,
            data0: 2,
            data1: 4,
            data2: STRUCT_END_TAG,
        },
        StructDataEntry {
            str: b"d\0".as_ptr() as *const XML_Char,
            data0: 2,
            data1: 5,
            data2: STRUCT_START_TAG,
        },
        StructDataEntry {
            str: b"f\0".as_ptr() as *const XML_Char,
            data0: 4,
            data1: 6,
            data2: STRUCT_START_TAG,
        },
        StructDataEntry {
            str: b"f\0".as_ptr() as *const XML_Char,
            data0: 8,
            data1: 6,
            data2: STRUCT_END_TAG,
        },
        StructDataEntry {
            str: b"d\0".as_ptr() as *const XML_Char,
            data0: 2,
            data1: 7,
            data2: STRUCT_END_TAG,
        },
        StructDataEntry {
            str: b"a\0".as_ptr() as *const XML_Char,
            data0: 0,
            data1: 8,
            data2: STRUCT_END_TAG,
        },
    ];
    let expected_count: ::core::ffi::c_int = (::core::mem::size_of::<[StructDataEntry; 10]>())
        .wrapping_div(::core::mem::size_of::<StructDataEntry>())
        as ::core::ffi::c_int;
    let mut storage: StructData = StructData {
        count: 0,
        max_count: 0,
        entries: ::core::ptr::null_mut::<StructDataEntry>(),
    };
    StructData_Init(&raw mut storage);
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    XML_SetStartElementHandler(
        g_parser,
        Some(
            start_element_event_handler2
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetEndElementHandler(
        g_parser,
        Some(
            end_element_event_handler2
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            663i32,
        );
    }
    StructData_CheckItems(
        &raw mut storage,
        &raw const expected as *const StructDataEntry,
        expected_count,
    );
    StructData_Dispose(&raw mut storage);
}

unsafe extern "C" fn test_line_number_after_error() {
    _check_set_test_info(
        b"test_line_number_after_error\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        671,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<a>\n  <b>\n  </a>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut lineno: XML_Size = 0;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_ERROR
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            678i32,
            b"Expected a parse error\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    lineno = XML_GetCurrentLineNumber(g_parser);
    if lineno != 3 {
        let mut buffer: [::core::ffi::c_char; 100] = [0; 100];
        snprintf(
            &raw mut buffer as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 100]>(),
            b"expected 3 lines, saw %lu\0".as_ptr() as *const ::core::ffi::c_char,
            lineno,
        );
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            685i32,
            &raw mut buffer as *mut ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_column_number_after_error() {
    _check_set_test_info(
        b"test_column_number_after_error\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        691,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<a>\n  <b>\n  </a>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut colno: XML_Size = 0;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_ERROR
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            698i32,
            b"Expected a parse error\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    colno = XML_GetCurrentColumnNumber(g_parser);
    if colno != 4 {
        let mut buffer: [::core::ffi::c_char; 100] = [0; 100];
        snprintf(
            &raw mut buffer as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 100]>(),
            b"expected 4 columns, saw %lu\0".as_ptr() as *const ::core::ffi::c_char,
            colno,
        );
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            705i32,
            &raw mut buffer as *mut ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_really_long_lines() {
    _check_set_test_info(
        b"test_really_long_lines\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        711,
    );
    let mut text: *const ::core::ffi::c_char = b"<e>ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+</e>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            741i32,
        );
    }
}

unsafe extern "C" fn test_really_long_encoded_lines() {
    _check_set_test_info(
        b"test_really_long_encoded_lines\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        746,
    );
    let mut buffer: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='iso-8859-1'?><e>ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-+</e>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut parse_len: ::core::ffi::c_int = strlen(text) as ::core::ffi::c_int;
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            dummy_cdata_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    buffer = XML_GetBuffer(g_parser, parse_len);
    if buffer.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            781i32,
            b"Could not allocate parse buffer\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !buffer.is_null() {
    } else {
        __assert_fail(
            b"buffer != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            782u32,
            b"void test_really_long_encoded_lines(void)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    };
    memcpy(
        buffer,
        text as *const ::core::ffi::c_void,
        parse_len as size_t,
    );
    if XML_ParseBuffer(g_parser, parse_len, XML_TRUE as ::core::ffi::c_int) == XML_STATUS_ERROR {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            785i32,
        );
    }
}

unsafe extern "C" fn test_end_element_events() {
    _check_set_test_info(
        b"test_end_element_events\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        793,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<a><b><c/></b><d><f/></d></a>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut expected: *const XML_Char = b"/c/b/f/d/a\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    XML_SetEndElementHandler(
        g_parser,
        Some(
            end_element_event_handler
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            803i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn is_whitespace_normalized(
    mut s: *const XML_Char,
    mut is_cdata: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut blanks: ::core::ffi::c_int = 0;
    let mut at_start: ::core::ffi::c_int = 1;
    while *s != 0 {
        if *s as ::core::ffi::c_int == ' ' as i32 {
            blanks += 1;
        } else if *s as ::core::ffi::c_int == '\t' as i32
            || *s as ::core::ffi::c_int == '\n' as i32
            || *s as ::core::ffi::c_int == '\r' as i32
        {
            return 0i32;
        } else {
            if at_start != 0 {
                at_start = 0;
                if blanks != 0 && is_cdata == 0 {
                    return 0i32;
                }
            } else if blanks > 1 && is_cdata == 0 {
                return 0i32;
            }
            blanks = 0i32;
        }
        s = s.offset(1);
    }
    if blanks != 0 && is_cdata == 0 {
        return 0i32;
    }
    return 1;
}

unsafe extern "C" fn test_helper_is_whitespace_normalized() {
    _check_set_test_info(
        b"test_helper_is_whitespace_normalized\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        848,
    );
    if is_whitespace_normalized(b"abc\0".as_ptr() as *const XML_Char, 0) != 0 {
    } else {
        __assert_fail(
            b"is_whitespace_normalized(XCS(\"abc\"), 0)\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            849u32,
            b"void test_helper_is_whitespace_normalized(void)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    if is_whitespace_normalized(b"abc\0".as_ptr() as *const XML_Char, 1) != 0 {
    } else {
        __assert_fail(
            b"is_whitespace_normalized(XCS(\"abc\"), 1)\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            850u32,
            b"void test_helper_is_whitespace_normalized(void)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    if is_whitespace_normalized(b"abc def ghi\0".as_ptr() as *const XML_Char, 0) != 0 {
    } else {
        __assert_fail(
            b"is_whitespace_normalized(XCS(\"abc def ghi\"), 0)\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            851u32,
            b"void test_helper_is_whitespace_normalized(void)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    if is_whitespace_normalized(b"abc def ghi\0".as_ptr() as *const XML_Char, 1) != 0 {
    } else {
        __assert_fail(
            b"is_whitespace_normalized(XCS(\"abc def ghi\"), 1)\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            852u32,
            b"void test_helper_is_whitespace_normalized(void)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    if is_whitespace_normalized(b" abc def ghi\0".as_ptr() as *const XML_Char, 0) == 0 {
    } else {
        __assert_fail(
            b"! is_whitespace_normalized(XCS(\" abc def ghi\"), 0)\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            853u32,
            b"void test_helper_is_whitespace_normalized(void)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    if is_whitespace_normalized(b" abc def ghi\0".as_ptr() as *const XML_Char, 1) != 0 {
    } else {
        __assert_fail(
            b"is_whitespace_normalized(XCS(\" abc def ghi\"), 1)\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            854u32,
            b"void test_helper_is_whitespace_normalized(void)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    if is_whitespace_normalized(b"abc  def ghi\0".as_ptr() as *const XML_Char, 0) == 0 {
    } else {
        __assert_fail(
            b"! is_whitespace_normalized(XCS(\"abc  def ghi\"), 0)\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            855u32,
            b"void test_helper_is_whitespace_normalized(void)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    if is_whitespace_normalized(b"abc  def ghi\0".as_ptr() as *const XML_Char, 1) != 0 {
    } else {
        __assert_fail(
            b"is_whitespace_normalized(XCS(\"abc  def ghi\"), 1)\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            856u32,
            b"void test_helper_is_whitespace_normalized(void)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    if is_whitespace_normalized(b"abc def ghi \0".as_ptr() as *const XML_Char, 0) == 0 {
    } else {
        __assert_fail(
            b"! is_whitespace_normalized(XCS(\"abc def ghi \"), 0)\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            857u32,
            b"void test_helper_is_whitespace_normalized(void)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    if is_whitespace_normalized(b"abc def ghi \0".as_ptr() as *const XML_Char, 1) != 0 {
    } else {
        __assert_fail(
            b"is_whitespace_normalized(XCS(\"abc def ghi \"), 1)\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            858u32,
            b"void test_helper_is_whitespace_normalized(void)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    if is_whitespace_normalized(b" \0".as_ptr() as *const XML_Char, 0) == 0 {
    } else {
        __assert_fail(
            b"! is_whitespace_normalized(XCS(\" \"), 0)\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            859u32,
            b"void test_helper_is_whitespace_normalized(void)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    if is_whitespace_normalized(b" \0".as_ptr() as *const XML_Char, 1) != 0 {
    } else {
        __assert_fail(
            b"is_whitespace_normalized(XCS(\" \"), 1)\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            860u32,
            b"void test_helper_is_whitespace_normalized(void)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    if is_whitespace_normalized(b"\t\0".as_ptr() as *const XML_Char, 0) == 0 {
    } else {
        __assert_fail(
            b"! is_whitespace_normalized(XCS(\"\\t\"), 0)\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            861u32,
            b"void test_helper_is_whitespace_normalized(void)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    if is_whitespace_normalized(b"\t\0".as_ptr() as *const XML_Char, 1) == 0 {
    } else {
        __assert_fail(
            b"! is_whitespace_normalized(XCS(\"\\t\"), 1)\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            862u32,
            b"void test_helper_is_whitespace_normalized(void)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    if is_whitespace_normalized(b"\n\0".as_ptr() as *const XML_Char, 0) == 0 {
    } else {
        __assert_fail(
            b"! is_whitespace_normalized(XCS(\"\\n\"), 0)\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            863u32,
            b"void test_helper_is_whitespace_normalized(void)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    if is_whitespace_normalized(b"\n\0".as_ptr() as *const XML_Char, 1) == 0 {
    } else {
        __assert_fail(
            b"! is_whitespace_normalized(XCS(\"\\n\"), 1)\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            864u32,
            b"void test_helper_is_whitespace_normalized(void)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    if is_whitespace_normalized(b"\r\0".as_ptr() as *const XML_Char, 0) == 0 {
    } else {
        __assert_fail(
            b"! is_whitespace_normalized(XCS(\"\\r\"), 0)\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            865u32,
            b"void test_helper_is_whitespace_normalized(void)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    if is_whitespace_normalized(b"\r\0".as_ptr() as *const XML_Char, 1) == 0 {
    } else {
        __assert_fail(
            b"! is_whitespace_normalized(XCS(\"\\r\"), 1)\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            866u32,
            b"void test_helper_is_whitespace_normalized(void)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    if is_whitespace_normalized(b"abc\t def\0".as_ptr() as *const XML_Char, 1) == 0 {
    } else {
        __assert_fail(
            b"! is_whitespace_normalized(XCS(\"abc\\t def\"), 1)\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            867u32,
            b"void test_helper_is_whitespace_normalized(void)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
}

unsafe extern "C" fn check_attr_contains_normalized_whitespace(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const XML_Char,
    mut atts: *mut *const XML_Char,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0;
    while !(*atts.offset(i as isize)).is_null() {
        let mut attrname: *const XML_Char = *atts.offset(i as isize);
        let mut value: *const XML_Char = *atts.offset((i + 1) as isize);
        if strcmp(b"attr\0".as_ptr() as *const ::core::ffi::c_char, attrname) == 0
            || strcmp(b"ents\0".as_ptr() as *const ::core::ffi::c_char, attrname) == 0
            || strcmp(b"refs\0".as_ptr() as *const ::core::ffi::c_char, attrname) == 0
        {
            if is_whitespace_normalized(value, 0) == 0 {
                let mut buffer: [::core::ffi::c_char; 256] = [0; 256];
                snprintf(
                    &raw mut buffer as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                    b"attribute value not normalized: %s='%s'\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    attrname,
                    value,
                );
                _fail(
                    b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    889i32,
                    &raw mut buffer as *mut ::core::ffi::c_char,
                );
            }
        }
        i += 2;
    }
}

unsafe extern "C" fn test_attr_whitespace_normalization() {
    _check_set_test_info(
        b"test_attr_whitespace_normalization\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        895,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ATTLIST doc\n            attr NMTOKENS #REQUIRED\n            ents ENTITIES #REQUIRED\n            refs IDREFS   #REQUIRED>\n]>\n<doc attr='    a  b c\t\td\te\t' refs=' id-1   \t  id-2\t\t'  \n     ents=' ent-1   \t\r\n            ent-2  ' >\n  <e id='id-1'/>\n  <e id='id-2'/>\n</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    XML_SetStartElementHandler(
        g_parser,
        Some(
            check_attr_contains_normalized_whitespace
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut *const XML_Char,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            914i32,
        );
    }
}

unsafe extern "C" fn test_xmldecl_misplaced() {
    _check_set_test_info(
        b"test_xmldecl_misplaced\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        922,
    );
    _expect_failure(
        b"\n<?xml version='1.0'?>\n<a/>\0".as_ptr() as *const ::core::ffi::c_char,
        XML_ERROR_MISPLACED_XML_PI,
        b"failed to report misplaced XML declaration\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        927,
    );
}

unsafe extern "C" fn test_xmldecl_invalid() {
    _check_set_test_info(
        b"test_xmldecl_invalid\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        931,
    );
    _expect_failure(
        b"<?xml version='1.0' \xC3\xA7?>\n<doc/>\0".as_ptr() as *const ::core::ffi::c_char,
        XML_ERROR_XML_DECL,
        b"Failed to report invalid XML declaration\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        933,
    );
}

unsafe extern "C" fn test_xmldecl_missing_attr() {
    _check_set_test_info(
        b"test_xmldecl_missing_attr\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        937,
    );
    _expect_failure(
        b"<?xml ='1.0'?>\n<doc/>\n\0".as_ptr() as *const ::core::ffi::c_char,
        XML_ERROR_XML_DECL,
        b"Failed to report missing XML declaration attribute\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        939,
    );
}

unsafe extern "C" fn test_xmldecl_missing_value() {
    _check_set_test_info(
        b"test_xmldecl_missing_value\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        943,
    );
    _expect_failure(
        b"<?xml version='1.0' encoding='us-ascii' standalone?>\n<doc/>\0".as_ptr()
            as *const ::core::ffi::c_char,
        XML_ERROR_XML_DECL,
        b"Failed to report missing attribute value\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        947,
    );
}

unsafe extern "C" fn test_unknown_encoding_internal_entity() {
    _check_set_test_info(
        b"test_unknown_encoding_internal_entity\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        952,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='unsupported-encoding'?>\n<!DOCTYPE test [<!ENTITY foo 'bar'>]>\n<test a='&foo;'/>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        ::core::mem::transmute(Some(
            UnknownEncodingHandler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Encoding,
                ) -> ::core::ffi::c_int,
        )),
        NULL,
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            960i32,
        );
    }
}

unsafe extern "C" fn test_unrecognised_encoding_internal_entity() {
    _check_set_test_info(
        b"test_unrecognised_encoding_internal_entity\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        965,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='unsupported-encoding'?>\n<!DOCTYPE test [<!ENTITY foo 'bar'>]>\n<test a='&foo;'/>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        ::core::mem::transmute(Some(
            UnrecognisedEncodingHandler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Encoding,
                ) -> ::core::ffi::c_int,
        )),
        NULL,
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_ERROR
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            973i32,
            b"Unrecognised encoding not rejected\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_ext_entity_set_encoding() {
    _check_set_test_info(
        b"test_ext_entity_set_encoding\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        978,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut test_data: ExtTest = ExtTest {
        parse_text: b"<?xml encoding='iso-8859-3'?>\xC3\xA9\0".as_ptr()
            as *const ::core::ffi::c_char,
        encoding: b"utf-8\0".as_ptr() as *const XML_Char,
        storage: ::core::ptr::null_mut::<CharData>(),
    };
    let mut expected: *const XML_Char = b"\xC3\xA9\0".as_ptr() as *const XML_Char;
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    _run_ext_character_check(
        text,
        &raw mut test_data,
        expected,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        995,
    );
}

unsafe extern "C" fn test_ext_entity_no_handler() {
    _check_set_test_info(
        b"test_ext_entity_no_handler\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1000,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    XML_SetDefaultHandler(
        g_parser,
        Some(
            dummy_default_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    _run_character_check(
        text,
        b"\0".as_ptr() as *const XML_Char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1007,
    );
}

unsafe extern "C" fn test_ext_entity_set_bom() {
    _check_set_test_info(
        b"test_ext_entity_set_bom\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1012,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut test_data: ExtTest = ExtTest {
        parse_text: b"\xEF\xBB\xBF<?xml encoding='iso-8859-3'?>\xC3\xA9\0".as_ptr()
            as *const ::core::ffi::c_char,
        encoding: b"utf-8\0".as_ptr() as *const XML_Char,
        storage: ::core::ptr::null_mut::<CharData>(),
    };
    let mut expected: *const XML_Char = b"\xC3\xA9\0".as_ptr() as *const XML_Char;
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    _run_ext_character_check(
        text,
        &raw mut test_data,
        expected,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1028,
    );
}

unsafe extern "C" fn test_ext_entity_bad_encoding() {
    _check_set_test_info(
        b"test_ext_entity_bad_encoding\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1033,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut fault: ExtFaults = ext_faults {
        parse_text: b"<?xml encoding='iso-8859-3'?>u\0".as_ptr() as *const ::core::ffi::c_char,
        fail_text: b"Unsupported encoding not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        encoding: b"unknown\0".as_ptr() as *const XML_Char,
        error: XML_ERROR_UNKNOWN_ENCODING,
    };
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_faulter
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetUserData(g_parser, &raw mut fault as *mut ::core::ffi::c_void);
    _expect_failure(
        text,
        XML_ERROR_EXTERNAL_ENTITY_HANDLING,
        b"Bad encoding should not have been accepted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1045,
    );
}

unsafe extern "C" fn test_ext_entity_bad_encoding_2() {
    _check_set_test_info(
        b"test_ext_entity_bad_encoding_2\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1050,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='us-ascii'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut fault: ExtFaults = ext_faults {
        parse_text: b"<!ELEMENT doc (#PCDATA)*>\0".as_ptr() as *const ::core::ffi::c_char,
        fail_text: b"Unknown encoding not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        encoding: b"unknown-encoding\0".as_ptr() as *const XML_Char,
        error: XML_ERROR_UNKNOWN_ENCODING,
    };
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_faulter
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetUserData(g_parser, &raw mut fault as *mut ::core::ffi::c_void);
    _expect_failure(
        text,
        XML_ERROR_EXTERNAL_ENTITY_HANDLING,
        b"Bad encoding not faulted in external entity handler\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1062,
    );
}

unsafe extern "C" fn test_wfc_undeclared_entity_unread_external_subset() {
    _check_set_test_info(
        b"test_wfc_undeclared_entity_unread_external_subset\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1069,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1075i32,
        );
    }
}

unsafe extern "C" fn test_wfc_undeclared_entity_no_external_subset() {
    _check_set_test_info(
        b"test_wfc_undeclared_entity_no_external_subset\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1082,
    );
    _expect_failure(
        b"<doc>&entity;</doc>\0".as_ptr() as *const ::core::ffi::c_char,
        XML_ERROR_UNDEFINED_ENTITY,
        b"Parser did not report undefined entity w/out a DTD.\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1084,
    );
}

unsafe extern "C" fn test_wfc_undeclared_entity_standalone() {
    _check_set_test_info(
        b"test_wfc_undeclared_entity_standalone\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1091,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='us-ascii' standalone='yes'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_UNDEFINED_ENTITY,
        b"Parser did not report undefined entity (standalone).\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1098,
    );
}

unsafe extern "C" fn test_wfc_undeclared_entity_with_external_subset_standalone() {
    _check_set_test_info(
        b"test_wfc_undeclared_entity_with_external_subset_standalone\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1105,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='us-ascii' standalone='yes'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut test_data: ExtTest = ExtTest {
        parse_text: b"<!ELEMENT doc (#PCDATA)*>\0".as_ptr() as *const ::core::ffi::c_char,
        encoding: ::core::ptr::null::<XML_Char>(),
        storage: ::core::ptr::null_mut::<CharData>(),
    };
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    _expect_failure(
        text,
        XML_ERROR_UNDEFINED_ENTITY,
        b"Parser did not report undefined entity (external DTD).\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1116,
    );
}

unsafe extern "C" fn test_entity_with_external_subset_unless_standalone() {
    _check_set_test_info(
        b"test_entity_with_external_subset_unless_standalone\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1123,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='us-ascii' standalone='yes'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut test_data: ExtTest = ExtTest {
        parse_text: b"<!ENTITY entity 'bar'>\0".as_ptr() as *const ::core::ffi::c_char,
        encoding: ::core::ptr::null::<XML_Char>(),
        storage: ::core::ptr::null_mut::<CharData>(),
    };
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_UNLESS_STANDALONE);
    XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    _expect_failure(
        text,
        XML_ERROR_UNDEFINED_ENTITY,
        b"Parser did not report undefined entity\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1135,
    );
}

unsafe extern "C" fn test_wfc_undeclared_entity_with_external_subset() {
    _check_set_test_info(
        b"test_wfc_undeclared_entity_with_external_subset\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1142,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='us-ascii'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut test_data: ExtTest = ExtTest {
        parse_text: b"<!ELEMENT doc (#PCDATA)*>\0".as_ptr() as *const ::core::ffi::c_char,
        encoding: ::core::ptr::null::<XML_Char>(),
        storage: ::core::ptr::null_mut::<CharData>(),
    };
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    _run_ext_character_check(
        text,
        &raw mut test_data,
        b"\0".as_ptr() as *const XML_Char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1150,
    );
}

unsafe extern "C" fn test_not_standalone_handler_reject() {
    _check_set_test_info(
        b"test_not_standalone_handler_reject\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1155,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='us-ascii'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut test_data: ExtTest = ExtTest {
        parse_text: b"<!ELEMENT doc (#PCDATA)*>\0".as_ptr() as *const ::core::ffi::c_char,
        encoding: ::core::ptr::null::<XML_Char>(),
        storage: ::core::ptr::null_mut::<CharData>(),
    };
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetNotStandaloneHandler(
        g_parser,
        Some(
            reject_not_standalone_handler
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
    );
    _expect_failure(
        text,
        XML_ERROR_NOT_STANDALONE,
        b"NotStandalone handler failed to reject\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1166,
    );
    XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
    XML_SetNotStandaloneHandler(
        g_parser,
        Some(
            reject_not_standalone_handler
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
    );
    _expect_failure(
        text,
        XML_ERROR_NOT_STANDALONE,
        b"NotStandalone handler failed to reject\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1172,
    );
}

unsafe extern "C" fn test_not_standalone_handler_accept() {
    _check_set_test_info(
        b"test_not_standalone_handler_accept\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1177,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='us-ascii'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut test_data: ExtTest = ExtTest {
        parse_text: b"<!ELEMENT doc (#PCDATA)*>\0".as_ptr() as *const ::core::ffi::c_char,
        encoding: ::core::ptr::null::<XML_Char>(),
        storage: ::core::ptr::null_mut::<CharData>(),
    };
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetNotStandaloneHandler(
        g_parser,
        Some(
            accept_not_standalone_handler
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
    );
    _run_ext_character_check(
        text,
        &raw mut test_data,
        b"\0".as_ptr() as *const XML_Char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1186,
    );
    XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
    XML_SetNotStandaloneHandler(
        g_parser,
        Some(
            accept_not_standalone_handler
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
    );
    _run_character_check(
        text,
        b"\0".as_ptr() as *const XML_Char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1191,
    );
}

unsafe extern "C" fn test_entity_start_tag_level_greater_than_one() {
    _check_set_test_info(
        b"test_entity_start_tag_level_greater_than_one\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1195,
    );
    let text: *const ::core::ffi::c_char =
        b"<!DOCTYPE t1 [\n  <!ENTITY e1 'hello'>\n]>\n<t1>\n  <t2>&e1;</t2>\n</t1>\n\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
    if !(_XML_Parse_SINGLE_BYTES(parser, text, strlen(text) as ::core::ffi::c_int, 1)
        == XML_STATUS_OK)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            1206i32,
            b"check failed: _XML_Parse_SINGLE_BYTES(parser, text, (int)strlen(text), XML_TRUE) == XML_STATUS_OK\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
    XML_ParserFree(parser);
}

unsafe extern "C" fn test_wfc_no_recursive_entity_refs() {
    _check_set_test_info(
        b"test_wfc_no_recursive_entity_refs\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1211,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc [\n  <!ENTITY entity '&#38;entity;'>\n]>\n<doc>&entity;</doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_RECURSIVE_ENTITY_REF,
        b"Parser did not report recursive entity reference.\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1218,
    );
}

unsafe extern "C" fn test_no_indirectly_recursive_entity_refs() {
    _check_set_test_info(
        b"test_no_indirectly_recursive_entity_refs\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1222,
    );
    let cases: [TestCase_0; 3] = [
        TestCase_0 {
            doc: b"<!DOCTYPE a [\n  <!ENTITY e1 '&e2;'>\n  <!ENTITY e2 '&e1;'>\n]><a>&e2;</a>\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            usesParameterEntities: false_0 != 0,
        },
        TestCase_0 {
            doc: b"<!DOCTYPE a [\n  <!ENTITY e1 '&e2;'>\n  <!ENTITY e2 '&e1;'>\n]><a k1='&e2;' />\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            usesParameterEntities: false_0 != 0,
        },
        TestCase_0 {
            doc: b"<!DOCTYPE doc [\n  <!ENTITY % p1 '&#37;p2;'>\n  <!ENTITY % p2 '&#37;p1;'>\n  <!ENTITY % define_g \"<!ENTITY g '&#37;p2;'>\">\n  %define_g;\n]>\n<doc/>\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            usesParameterEntities: true_0 != 0,
        },
    ];
    let reset_or_not: [XML_Bool; 2] = [XML_TRUE, XML_FALSE];
    let mut i: size_t = 0;
    while i
        < (::core::mem::size_of::<[TestCase_0; 3]>())
            .wrapping_div(::core::mem::size_of::<TestCase_0>())
    {
        let mut j: size_t = 0;
        while j
            < (::core::mem::size_of::<[XML_Bool; 2]>())
                .wrapping_div(::core::mem::size_of::<XML_Bool>())
        {
            let reset_wanted: XML_Bool = reset_or_not[j];
            let doc: *const ::core::ffi::c_char = cases[i].doc;
            let usesParameterEntities: bool = cases[i].usesParameterEntities;
            set_subtest(
                b"[%i,reset=%i] %s\0".as_ptr() as *const ::core::ffi::c_char,
                i as ::core::ffi::c_int,
                j as ::core::ffi::c_int,
                doc,
            );
            let rejection_expected: bool = true_0 != 0;
            let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
            if usesParameterEntities {
                if !(XML_SetParamEntityParsing(parser, XML_PARAM_ENTITY_PARSING_ALWAYS) == 1) {
                    _fail(
                        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        1278i32,
                        b"check failed: XML_SetParamEntityParsing(parser, XML_PARAM_ENTITY_PARSING_ALWAYS) == 1\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            }
            let status: XML_Status = _XML_Parse_SINGLE_BYTES(
                parser,
                doc,
                strlen(doc) as ::core::ffi::c_int,
                XML_TRUE as ::core::ffi::c_int,
            );
            if rejection_expected {
                if !(status == XML_STATUS_ERROR) {
                    _fail(
                        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        1289i32,
                        b"check failed: status == XML_STATUS_ERROR\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
                if !(XML_GetErrorCode(parser) == XML_ERROR_RECURSIVE_ENTITY_REF) {
                    _fail(
                        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        1290i32,
                        b"check failed: XML_GetErrorCode(parser) == XML_ERROR_RECURSIVE_ENTITY_REF\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            } else if !(status == XML_STATUS_OK) {
                _fail(
                    b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    1292i32,
                    b"check failed: status == XML_STATUS_OK\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            if reset_wanted != 0 {
                XML_ParserReset(parser, ::core::ptr::null::<XML_Char>());
            }
            XML_ParserFree(parser);
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
}

unsafe extern "C" fn test_recursive_external_parameter_entity_2() {
    _check_set_test_info(
        b"test_recursive_external_parameter_entity_2\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1309,
    );
    let mut cases: [TestCase; 4] = [
        TestCase {
            doc: b"<!ENTITY % p1 '%p1;'>\0".as_ptr() as *const ::core::ffi::c_char,
            expectedStatus: XML_STATUS_ERROR,
        },
        TestCase {
            doc: b"<!ENTITY % p1 '%p1;'><!ENTITY % p1 'first declaration wins'>\0".as_ptr()
                as *const ::core::ffi::c_char,
            expectedStatus: XML_STATUS_ERROR,
        },
        TestCase {
            doc: b"<!ENTITY % p1 'first declaration wins'><!ENTITY % p1 '%p1;'>\0".as_ptr()
                as *const ::core::ffi::c_char,
            expectedStatus: XML_STATUS_OK,
        },
        TestCase {
            doc: b"<!ENTITY % p1 '&#37;p1;'>\0".as_ptr() as *const ::core::ffi::c_char,
            expectedStatus: XML_STATUS_OK,
        },
    ];
    let mut i: size_t = 0;
    while i
        < (::core::mem::size_of::<[TestCase; 4]>()).wrapping_div(::core::mem::size_of::<TestCase>())
    {
        let doc: *const ::core::ffi::c_char = cases[i].doc;
        let expectedStatus: XML_Status = cases[i].expectedStatus;
        set_subtest(b"%s\0".as_ptr() as *const ::core::ffi::c_char, doc);
        let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        if parser.is_null() {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1332i32,
                b"check failed: parser != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut ext_parser: XML_Parser = XML_ExternalEntityParserCreate(
            parser,
            ::core::ptr::null::<XML_Char>(),
            ::core::ptr::null::<XML_Char>(),
        );
        if ext_parser.is_null() {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1335i32,
                b"check failed: ext_parser != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let actualStatus: XML_Status = _XML_Parse_SINGLE_BYTES(
            ext_parser,
            doc,
            strlen(doc) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        );
        if !(actualStatus == expectedStatus) {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1340i32,
                b"check failed: actualStatus == expectedStatus\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if actualStatus != XML_STATUS_OK {
            if !(XML_GetErrorCode(ext_parser) == XML_ERROR_RECURSIVE_ENTITY_REF) {
                _fail(
                    b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    1343i32,
                    b"check failed: XML_GetErrorCode(ext_parser) == XML_ERROR_RECURSIVE_ENTITY_REF\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
        }
        XML_ParserFree(ext_parser);
        XML_ParserFree(parser);
        i = i.wrapping_add(1);
    }
}

unsafe extern "C" fn test_ext_entity_invalid_parse() {
    _check_set_test_info(
        b"test_ext_entity_invalid_parse\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1353,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let faults: [ExtFaults; 4] = [
        ext_faults {
            parse_text: b"<\0".as_ptr() as *const ::core::ffi::c_char,
            fail_text: b"Incomplete element declaration not faulted\0".as_ptr()
                as *const ::core::ffi::c_char,
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_UNCLOSED_TOKEN,
        },
        ext_faults {
            parse_text: b"<\xE2\x82\0".as_ptr() as *const ::core::ffi::c_char,
            fail_text: b"Incomplete character not faulted\0".as_ptr() as *const ::core::ffi::c_char,
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_PARTIAL_CHAR,
        },
        ext_faults {
            parse_text: b"<tag>\xE2\x82\0".as_ptr() as *const ::core::ffi::c_char,
            fail_text: b"Incomplete character in CDATA not faulted\0".as_ptr()
                as *const ::core::ffi::c_char,
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_PARTIAL_CHAR,
        },
        ext_faults {
            parse_text: ::core::ptr::null::<::core::ffi::c_char>(),
            fail_text: ::core::ptr::null::<::core::ffi::c_char>(),
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_NONE,
        },
    ];
    let mut fault: *const ExtFaults = &raw const faults as *const ExtFaults;
    while !(*fault).parse_text.is_null() {
        set_subtest(
            b"\"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
            (*fault).parse_text,
        );
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_faulter
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetUserData(g_parser, fault as *mut ::core::ffi::c_void);
        _expect_failure(
            text,
            XML_ERROR_EXTERNAL_ENTITY_HANDLING,
            b"Parser did not report external entity error\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1374,
        );
        XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
        fault = fault.offset(1);
    }
}

unsafe extern "C" fn test_dtd_default_handling() {
    _check_set_test_info(
        b"test_dtd_default_handling\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1381,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ENTITY e SYSTEM 'http://example.org/e'>\n<!NOTATION n SYSTEM 'http://example.org/n'>\n<!ELEMENT doc EMPTY>\n<!ATTLIST doc a CDATA #IMPLIED>\n<?pi in dtd?>\n<!--comment in dtd-->\n]><doc/>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    XML_SetDefaultHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetStartDoctypeDeclHandler(
        g_parser,
        Some(
            dummy_start_doctype_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetEndDoctypeDeclHandler(
        g_parser,
        Some(dummy_end_doctype_handler as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
    );
    XML_SetEntityDeclHandler(
        g_parser,
        Some(
            dummy_entity_decl_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                    *const XML_Char,
                    ::core::ffi::c_int,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetNotationDeclHandler(
        g_parser,
        Some(
            dummy_notation_decl_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetElementDeclHandler(
        g_parser,
        ::core::mem::transmute(Some(
            dummy_element_decl_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Content,
                ) -> (),
        )),
    );
    XML_SetAttlistDeclHandler(
        g_parser,
        Some(
            dummy_attlist_decl_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetProcessingInstructionHandler(
        g_parser,
        Some(
            dummy_pi_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetCommentHandler(
        g_parser,
        Some(
            dummy_comment_handler
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
        ),
    );
    XML_SetStartCdataSectionHandler(
        g_parser,
        Some(dummy_start_cdata_handler as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
    );
    XML_SetEndCdataSectionHandler(
        g_parser,
        Some(dummy_end_cdata_handler as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
    );
    _run_character_check(
        text,
        b"\n\n\n\n\n\n\n<doc/>\0".as_ptr() as *const XML_Char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1402,
    );
}

unsafe extern "C" fn test_dtd_attr_handling() {
    _check_set_test_info(
        b"test_dtd_attr_handling\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1407,
    );
    let mut prolog: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc [\n<!ELEMENT doc EMPTY>\n\0".as_ptr() as *const ::core::ffi::c_char;
    let mut attr_data: [AttTest; 5] = [
        AttTest {
    definition:   b"<!ATTLIST doc a ( one | two | three ) #REQUIRED>\n]><doc a='two'/>\0"
                .as_ptr() as *const ::core::ffi::c_char,
    element_name:   b"doc\0".as_ptr() as *const XML_Char,
    attr_name:   b"a\0".as_ptr() as *const XML_Char,
    attr_type:   b"(one|two|three)\0".as_ptr() as *const XML_Char,
    default_value:   ::core::ptr::null::<XML_Char>(),
    is_required:   XML_TRUE as ::core::ffi::c_int,
},
        AttTest {
    definition:   b"<!NOTATION foo SYSTEM 'http://example.org/foo'>\n<!ATTLIST doc a NOTATION (foo) #IMPLIED>\n]><doc/>\0"
                .as_ptr() as *const ::core::ffi::c_char,
    element_name:   b"doc\0".as_ptr() as *const XML_Char,
    attr_name:   b"a\0".as_ptr() as *const XML_Char,
    attr_type:   b"NOTATION(foo)\0".as_ptr() as *const XML_Char,
    default_value:   ::core::ptr::null::<XML_Char>(),
    is_required:   XML_FALSE as ::core::ffi::c_int,
},
        AttTest {
    definition:   b"<!ATTLIST doc a NOTATION (foo) 'bar'>\n]><doc/>\0".as_ptr()
                as *const ::core::ffi::c_char,
    element_name:   b"doc\0".as_ptr() as *const XML_Char,
    attr_name:   b"a\0".as_ptr() as *const XML_Char,
    attr_type:   b"NOTATION(foo)\0".as_ptr() as *const XML_Char,
    default_value:   b"bar\0".as_ptr() as *const XML_Char,
    is_required:   XML_FALSE as ::core::ffi::c_int,
},
        AttTest {
    definition:   b"<!ATTLIST doc a CDATA '\xDB\xB2'>\n]><doc/>\0".as_ptr()
                as *const ::core::ffi::c_char,
    element_name:   b"doc\0".as_ptr() as *const XML_Char,
    attr_name:   b"a\0".as_ptr() as *const XML_Char,
    attr_type:   b"CDATA\0".as_ptr() as *const XML_Char,
    default_value:   b"\xDB\xB2\0".as_ptr() as *const XML_Char,
    is_required:   XML_FALSE as ::core::ffi::c_int,
},
        AttTest {
    definition:   ::core::ptr::null::<::core::ffi::c_char>(),
    element_name:   ::core::ptr::null::<XML_Char>(),
    attr_name:   ::core::ptr::null::<XML_Char>(),
    attr_type:   ::core::ptr::null::<XML_Char>(),
    default_value:   ::core::ptr::null::<XML_Char>(),
    is_required:   XML_FALSE as ::core::ffi::c_int,
},
    ];
    let mut test: *mut AttTest = ::core::ptr::null_mut::<AttTest>();
    test = &raw mut attr_data as *mut AttTest;
    while !(*test).definition.is_null() {
        set_subtest(
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            (*test).definition,
        );
        XML_SetAttlistDeclHandler(
            g_parser,
            Some(
                verify_attlist_decl_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetUserData(g_parser, test as *mut ::core::ffi::c_void);
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            prolog,
            strlen(prolog) as ::core::ffi::c_int,
            XML_FALSE as ::core::ffi::c_int,
        ) == XML_STATUS_ERROR
        {
            _xml_failure(
                g_parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1446i32,
            );
        }
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            (*test).definition,
            strlen((*test).definition) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) == XML_STATUS_ERROR
        {
            _xml_failure(
                g_parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1450i32,
            );
        }
        XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
        test = test.offset(1);
    }
}

unsafe extern "C" fn test_empty_ns_without_namespaces() {
    _check_set_test_info(
        b"test_empty_ns_without_namespaces\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1462,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc xmlns:prefix='http://example.org/'>\n  <e xmlns:prefix=''/>\n</doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1469i32,
        );
    }
}

unsafe extern "C" fn test_ns_in_attribute_default_without_namespaces() {
    _check_set_test_info(
        b"test_ns_in_attribute_default_without_namespaces\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1477,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE e:element [\n  <!ATTLIST e:element\n    xmlns:e CDATA 'http://example.org/'>\n      ]>\n<e:element/>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1486i32,
        );
    }
}

unsafe extern "C" fn test_stop_parser_between_char_data_calls() {
    _check_set_test_info(
        b"test_stop_parser_between_char_data_calls\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1492,
    );
    let mut text: *const ::core::ffi::c_char = long_character_data_text;
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            clearing_aborting_character_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    g_resumable = XML_FALSE;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1505i32,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_ABORTED {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1507i32,
        );
    }
}

unsafe extern "C" fn test_suspend_parser_between_char_data_calls() {
    _check_set_test_info(
        b"test_suspend_parser_between_char_data_calls\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1513,
    );
    let mut text: *const ::core::ffi::c_char = long_character_data_text;
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            clearing_aborting_character_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    g_resumable = XML_TRUE;
    if XML_Parse(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_SUSPENDED
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1528i32,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_NONE {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1530i32,
        );
    }
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_ERROR
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1534i32,
            b"Attempt to continue parse while suspended not faulted\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_SUSPENDED {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1536i32,
            b"Suspended parse not faulted with correct error\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_repeated_stop_parser_between_char_data_calls() {
    _check_set_test_info(
        b"test_repeated_stop_parser_between_char_data_calls\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1541,
    );
    let mut text: *const ::core::ffi::c_char = long_character_data_text;
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            parser_stop_character_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    g_resumable = XML_FALSE;
    g_abortable = XML_FALSE;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_ERROR
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1549i32,
            b"Failed to double-stop parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            parser_stop_character_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    g_resumable = XML_TRUE;
    g_abortable = XML_FALSE;
    if XML_Parse(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_SUSPENDED
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1559i32,
            b"Failed to double-suspend parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            parser_stop_character_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    g_resumable = XML_TRUE;
    g_abortable = XML_TRUE;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_ERROR
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1567i32,
            b"Failed to suspend-abort parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_good_cdata_ascii() {
    _check_set_test_info(
        b"test_good_cdata_ascii\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1571,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<a><![CDATA[<greeting>Hello, world!</greeting>]]></a>\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut expected: *const XML_Char =
        b"<greeting>Hello, world!</greeting>\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetStartCdataSectionHandler(
        g_parser,
        Some(dummy_start_cdata_handler as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
    );
    XML_SetEndCdataSectionHandler(
        g_parser,
        Some(dummy_end_cdata_handler as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1585i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
    XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
    CharData_Init(&raw mut storage);
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetDefaultHandler(
        g_parser,
        Some(
            dummy_default_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1597i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_good_cdata_utf16() {
    _check_set_test_info(
        b"test_good_cdata_utf16\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1602,
    );
    let text: [::core::ffi::c_char; 129] = ::core::mem::transmute::<
        [u8; 129],
        [::core::ffi::c_char; 129],
    >(
        *b"\0<\0?\0x\0m\0l\0 \0v\0e\0r\0s\0i\0o\0n\0=\0'\x001\0.\x000\0'\0 \0e\0n\0c\0o\0d\0i\0n\0g\0=\0'\0u\0t\0f\0-\x001\x006\0'\0?\0>\0\n\0<\0a\0>\0<\0!\0[\0C\0D\0A\0T\0A\0[\0h\0e\0l\0l\0o\0]\0]\0>\0<\0/\0a\0>\0",
    );
    let mut expected: *const XML_Char = b"hello\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        &raw const text as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 129]>() as ::core::ffi::c_int - 1,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1624i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_good_cdata_utf16_le() {
    _check_set_test_info(
        b"test_good_cdata_utf16_le\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1629,
    );
    let text: [::core::ffi::c_char; 129] = ::core::mem::transmute::<
        [u8; 129],
        [::core::ffi::c_char; 129],
    >(
        *b"<\0?\0x\0m\0l\0 \0v\0e\0r\0s\0i\0o\0n\0=\0'\x001\0.\x000\0'\0 \0e\0n\0c\0o\0d\0i\0n\0g\0=\0'\0u\0t\0f\0-\x001\x006\0'\0?\0>\0\n\0<\0a\0>\0<\0!\0[\0C\0D\0A\0T\0A\0[\0h\0e\0l\0l\0o\0]\0]\0>\0<\0/\0a\0>\0\0",
    );
    let mut expected: *const XML_Char = b"hello\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        &raw const text as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 129]>() as ::core::ffi::c_int - 1,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1651i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_long_cdata_utf16() {
    _check_set_test_info(
        b"test_long_cdata_utf16\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1661,
    );
    let text: [::core::ffi::c_char; 2197] = ::core::mem::transmute::<
        [u8; 2197],
        [::core::ffi::c_char; 2197],
    >(
        *b"\0<\0?\0x\0m\0l\0 \0v\0e\0r\0s\0i\0o\0n\0=\0'\x001\0.\x000\0'\0 \0e\0n\0c\0o\0d\0i\0n\0g\0=\0'\0u\0t\0f\0-\x001\x006\0'\0?\0>\0<\0a\0>\0<\0!\0[\0C\0D\0A\0T\0A\0[\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0]\0]\0>\0<\0/\0a\0>\0",
    );
    let mut expected: *const XML_Char = b"ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\0"
        .as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    let mut buffer: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    CharData_Init(&raw mut storage);
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    buffer = XML_GetBuffer(
        g_parser,
        (::core::mem::size_of::<[::core::ffi::c_char; 2197]>()).wrapping_sub(1usize)
            as ::core::ffi::c_int,
    );
    if buffer.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1722i32,
            b"Could not allocate parse buffer\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !buffer.is_null() {
    } else {
        __assert_fail(
            b"buffer != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1723u32,
            b"void test_long_cdata_utf16(void)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    };
    memcpy(
        buffer,
        &raw const text as *const ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 2197]>()).wrapping_sub(1usize),
    );
    if XML_ParseBuffer(
        g_parser,
        (::core::mem::size_of::<[::core::ffi::c_char; 2197]>()).wrapping_sub(1usize)
            as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1726i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_multichar_cdata_utf16() {
    _check_set_test_info(
        b"test_multichar_cdata_utf16\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1732,
    );
    let text: [::core::ffi::c_char; 127] = ::core::mem::transmute::<
        [u8; 127],
        [::core::ffi::c_char; 127],
    >(
        *b"\0<\0?\0x\0m\0l\0 \0v\0e\0r\0s\0i\0o\0n\0=\0'\x001\0.\x000\0'\0 \0e\0n\0c\0o\0d\0i\0n\0g\0=\0'\0u\0t\0f\0-\x001\x006\0'\0?\0>\0\n\0<\0a\0>\0<\0!\0[\0C\0D\0A\0T\0A\0[\xD84\xDD^\xD84\xDD_\0]\0]\0>\0<\0/\0a\0>\0",
    );
    let mut expected: *const XML_Char =
        b"\xF0\x9D\x85\x9E\xF0\x9D\x85\x9F\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        &raw const text as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 127]>() as ::core::ffi::c_int - 1,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1766i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_utf16_bad_surrogate_pair() {
    _check_set_test_info(
        b"test_utf16_bad_surrogate_pair\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1772,
    );
    let text: [::core::ffi::c_char; 123] = ::core::mem::transmute::<
        [u8; 123],
        [::core::ffi::c_char; 123],
    >(
        *b"\0<\0?\0x\0m\0l\0 \0v\0e\0r\0s\0i\0o\0n\0=\0'\x001\0.\x000\0'\0 \0e\0n\0c\0o\0d\0i\0n\0g\0=\0'\0u\0t\0f\0-\x001\x006\0'\0?\0>\0\n\0<\0a\0>\0<\0!\0[\0C\0D\0A\0T\0A\0[\xDC\0\xD8\0\0]\0]\0>\0<\0/\0a\0>\0",
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        &raw const text as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 123]>() as ::core::ffi::c_int - 1,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_ERROR
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1793i32,
            b"Reversed UTF-16 surrogate pair not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_INVALID_TOKEN {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1795i32,
        );
    }
}

unsafe extern "C" fn test_bad_cdata() {
    _check_set_test_info(
        b"test_bad_cdata\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1799,
    );
    let mut cases: [CaseData_0; 21] = [
        CaseData_0 {
            text: b"<a><\0".as_ptr() as *const ::core::ffi::c_char,
            expectedError: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData_0 {
            text: b"<a><!\0".as_ptr() as *const ::core::ffi::c_char,
            expectedError: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData_0 {
            text: b"<a><![\0".as_ptr() as *const ::core::ffi::c_char,
            expectedError: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData_0 {
            text: b"<a><![C\0".as_ptr() as *const ::core::ffi::c_char,
            expectedError: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData_0 {
            text: b"<a><![CD\0".as_ptr() as *const ::core::ffi::c_char,
            expectedError: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData_0 {
            text: b"<a><![CDA\0".as_ptr() as *const ::core::ffi::c_char,
            expectedError: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData_0 {
            text: b"<a><![CDAT\0".as_ptr() as *const ::core::ffi::c_char,
            expectedError: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData_0 {
            text: b"<a><![CDATA\0".as_ptr() as *const ::core::ffi::c_char,
            expectedError: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData_0 {
            text: b"<a><![CDATA[\0".as_ptr() as *const ::core::ffi::c_char,
            expectedError: XML_ERROR_UNCLOSED_CDATA_SECTION,
        },
        CaseData_0 {
            text: b"<a><![CDATA[]\0".as_ptr() as *const ::core::ffi::c_char,
            expectedError: XML_ERROR_UNCLOSED_CDATA_SECTION,
        },
        CaseData_0 {
            text: b"<a><![CDATA[]]\0".as_ptr() as *const ::core::ffi::c_char,
            expectedError: XML_ERROR_UNCLOSED_CDATA_SECTION,
        },
        CaseData_0 {
            text: b"<a><!<a/>\0".as_ptr() as *const ::core::ffi::c_char,
            expectedError: XML_ERROR_INVALID_TOKEN,
        },
        CaseData_0 {
            text: b"<a><![<a/>\0".as_ptr() as *const ::core::ffi::c_char,
            expectedError: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData_0 {
            text: b"<a><![C<a/>\0".as_ptr() as *const ::core::ffi::c_char,
            expectedError: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData_0 {
            text: b"<a><![CD<a/>\0".as_ptr() as *const ::core::ffi::c_char,
            expectedError: XML_ERROR_INVALID_TOKEN,
        },
        CaseData_0 {
            text: b"<a><![CDA<a/>\0".as_ptr() as *const ::core::ffi::c_char,
            expectedError: XML_ERROR_INVALID_TOKEN,
        },
        CaseData_0 {
            text: b"<a><![CDAT<a/>\0".as_ptr() as *const ::core::ffi::c_char,
            expectedError: XML_ERROR_INVALID_TOKEN,
        },
        CaseData_0 {
            text: b"<a><![CDATA<a/>\0".as_ptr() as *const ::core::ffi::c_char,
            expectedError: XML_ERROR_INVALID_TOKEN,
        },
        CaseData_0 {
            text: b"<a><![CDATA[<a/>\0".as_ptr() as *const ::core::ffi::c_char,
            expectedError: XML_ERROR_UNCLOSED_CDATA_SECTION,
        },
        CaseData_0 {
            text: b"<a><![CDATA[]<a/>\0".as_ptr() as *const ::core::ffi::c_char,
            expectedError: XML_ERROR_UNCLOSED_CDATA_SECTION,
        },
        CaseData_0 {
            text: b"<a><![CDATA[]]<a/>\0".as_ptr() as *const ::core::ffi::c_char,
            expectedError: XML_ERROR_UNCLOSED_CDATA_SECTION,
        },
    ];
    let mut i: size_t = 0;
    while i
        < (::core::mem::size_of::<[CaseData_0; 21]>())
            .wrapping_div(::core::mem::size_of::<CaseData_0>())
    {
        set_subtest(
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            cases[i].text,
        );
        let actualStatus: XML_Status = _XML_Parse_SINGLE_BYTES(
            g_parser,
            cases[i].text,
            strlen(cases[i].text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        );
        let actualError: XML_Error = XML_GetErrorCode(g_parser);
        if actualStatus == XML_STATUS_ERROR {
        } else {
            __assert_fail(
                b"actualStatus == XML_STATUS_ERROR\0".as_ptr() as *const ::core::ffi::c_char,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1838u32,
                b"void test_bad_cdata(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if actualError != cases[i].expectedError {
            let mut message: [::core::ffi::c_char; 100] = [0; 100];
            snprintf(
                &raw mut message as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 100]>(),
                b"Expected error %d but got error %d for case %u: \"%s\"\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                cases[i].expectedError,
                actualError,
                (i as ::core::ffi::c_uint).wrapping_add(1u32),
                cases[i].text,
            );
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1846i32,
                &raw mut message as *mut ::core::ffi::c_char,
            );
        }
        XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
        i = i.wrapping_add(1);
    }
}

unsafe extern "C" fn test_bad_cdata_utf16() {
    _check_set_test_info(
        b"test_bad_cdata_utf16\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1855,
    );
    let prolog: [::core::ffi::c_char; 87] = ::core::mem::transmute::<
        [u8; 87],
        [::core::ffi::c_char; 87],
    >(
        *b"\0<\0?\0x\0m\0l\0 \0v\0e\0r\0s\0i\0o\0n\0=\0'\x001\0.\x000\0'\0 \0e\0n\0c\0o\0d\0i\0n\0g\0=\0'\0u\0t\0f\0-\x001\x006\0'\0?\0>\0\n\0<\0a\0>\0",
    );
    let mut cases: [CaseData; 24] = [
        CaseData {
            text_bytes: 1usize,
            text: b"\0\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 2usize,
            text: b"\0<\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 3usize,
            text: b"\0<\0\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 4usize,
            text: b"\0<\0!\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 5usize,
            text: b"\0<\0!\0\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 6usize,
            text: b"\0<\0!\0[\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 7usize,
            text: b"\0<\0!\0[\0\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 8usize,
            text: b"\0<\0!\0[\0C\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 9usize,
            text: b"\0<\0!\0[\0C\0\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 10usize,
            text: b"\0<\0!\0[\0C\0D\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 11usize,
            text: b"\0<\0!\0[\0C\0D\0\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 12usize,
            text: b"\0<\0!\0[\0C\0D\0A\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 13usize,
            text: b"\0<\0!\0[\0C\0D\0A\0\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 14usize,
            text: b"\0<\0!\0[\0C\0D\0A\0T\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 15usize,
            text: b"\0<\0!\0[\0C\0D\0A\0T\0\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 16usize,
            text: b"\0<\0!\0[\0C\0D\0A\0T\0A\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 17usize,
            text: b"\0<\0!\0[\0C\0D\0A\0T\0A\0\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_TOKEN,
        },
        CaseData {
            text_bytes: 18usize,
            text: b"\0<\0!\0[\0C\0D\0A\0T\0A\0[\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_CDATA_SECTION,
        },
        CaseData {
            text_bytes: 19usize,
            text: b"\0<\0!\0[\0C\0D\0A\0T\0A\0[\0\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_CDATA_SECTION,
        },
        CaseData {
            text_bytes: 20usize,
            text: b"\0<\0!\0[\0C\0D\0A\0T\0A\0[\0Z\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_CDATA_SECTION,
        },
        CaseData {
            text_bytes: 21usize,
            text: b"\0<\0!\0[\0C\0D\0A\0T\0A\0[\0Z\xD8\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_CDATA_SECTION,
        },
        CaseData {
            text_bytes: 22usize,
            text: b"\0<\0!\0[\0C\0D\0A\0T\0A\0[\0Z\xD84\0".as_ptr() as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_PARTIAL_CHAR,
        },
        CaseData {
            text_bytes: 23usize,
            text: b"\0<\0!\0[\0C\0D\0A\0T\0A\0[\0Z\xD84\xDD\0".as_ptr()
                as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_PARTIAL_CHAR,
        },
        CaseData {
            text_bytes: 24usize,
            text: b"\0<\0!\0[\0C\0D\0A\0T\0A\0[\0Z\xD84\xDD^\0".as_ptr()
                as *const ::core::ffi::c_char,
            expected_error: XML_ERROR_UNCLOSED_CDATA_SECTION,
        },
    ];
    let mut i: size_t = 0;
    i = 0;
    while i
        < (::core::mem::size_of::<[CaseData; 24]>())
            .wrapping_div(::core::mem::size_of::<CaseData>())
    {
        set_subtest(
            b"case %lu\0".as_ptr() as *const ::core::ffi::c_char,
            i.wrapping_add(1usize) as ::core::ffi::c_ulong,
        );
        let mut actual_status: XML_Status = XML_STATUS_ERROR;
        let mut actual_error: XML_Error = XML_ERROR_NONE;
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            &raw const prolog as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 87]>() as ::core::ffi::c_int - 1,
            XML_FALSE as ::core::ffi::c_int,
        ) == XML_STATUS_ERROR
        {
            _xml_failure(
                g_parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1908i32,
            );
        }
        actual_status = _XML_Parse_SINGLE_BYTES(
            g_parser,
            cases[i].text,
            cases[i].text_bytes as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        );
        if actual_status == XML_STATUS_ERROR {
        } else {
            __assert_fail(
                b"actual_status == XML_STATUS_ERROR\0".as_ptr() as *const ::core::ffi::c_char,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1911u32,
                b"void test_bad_cdata_utf16(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        actual_error = XML_GetErrorCode(g_parser);
        if actual_error != cases[i].expected_error {
            let mut message: [::core::ffi::c_char; 1024] = [0; 1024];
            snprintf(
                &raw mut message as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
                b"Expected error %d (%s), got %d (%s) for case %lu\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                cases[i].expected_error,
                XML_ErrorString(cases[i].expected_error),
                actual_error,
                XML_ErrorString(actual_error),
                i.wrapping_add(1usize) as ::core::ffi::c_ulong,
            );
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1922i32,
                &raw mut message as *mut ::core::ffi::c_char,
            );
        }
        XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
        i = i.wrapping_add(1);
    }
}

unsafe extern "C" fn test_stop_parser_between_cdata_calls() {
    _check_set_test_info(
        b"test_stop_parser_between_cdata_calls\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1930,
    );
    let mut text: *const ::core::ffi::c_char = long_cdata_text;
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            clearing_aborting_character_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    g_resumable = XML_FALSE;
    _expect_failure(
        text,
        XML_ERROR_ABORTED,
        b"Parse not aborted in CDATA handler\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1935,
    );
}

unsafe extern "C" fn test_suspend_parser_between_cdata_calls() {
    _check_set_test_info(
        b"test_suspend_parser_between_cdata_calls\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1940,
    );
    if g_chunkSize != 0 {
        return;
    }
    let mut text: *const ::core::ffi::c_char = long_cdata_text;
    let mut result: XML_Status = XML_STATUS_ERROR;
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            clearing_aborting_character_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    g_resumable = XML_TRUE;
    result = XML_Parse(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    );
    if result != XML_STATUS_SUSPENDED {
        if result == XML_STATUS_ERROR {
            _xml_failure(
                g_parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1956i32,
            );
        }
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1957i32,
            b"Parse not suspended in CDATA handler\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_NONE {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1960i32,
        );
    }
}

unsafe extern "C" fn test_memory_allocation() {
    _check_set_test_info(
        b"test_memory_allocation\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1965,
    );
    let mut buffer: *mut ::core::ffi::c_char =
        XML_MemMalloc(g_parser, 256) as *mut ::core::ffi::c_char;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if buffer.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1970i32,
            b"Allocation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        *buffer.offset(0) = 'T' as ::core::ffi::c_char;
        *buffer.offset(1) = 'E' as ::core::ffi::c_char;
        *buffer.offset(2) = 'S' as ::core::ffi::c_char;
        *buffer.offset(3) = 'T' as ::core::ffi::c_char;
        *buffer.offset(4) = '\0' as ::core::ffi::c_char;
        if strcmp(buffer, b"TEST\0".as_ptr() as *const ::core::ffi::c_char) != 0 {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1979i32,
                b"Memory not writable\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            p = XML_MemRealloc(g_parser, buffer as *mut ::core::ffi::c_void, 512)
                as *mut ::core::ffi::c_char;
            if p.is_null() {
                _fail(
                    b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    1983i32,
                    b"Reallocation failed\0".as_ptr() as *const ::core::ffi::c_char,
                );
            } else {
                buffer = p;
                *buffer.offset(0) = 'V' as ::core::ffi::c_char;
                if strcmp(buffer, b"VEST\0".as_ptr() as *const ::core::ffi::c_char) != 0 {
                    _fail(
                        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        1989i32,
                        b"Reallocated memory not writable\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
            }
        }
        XML_MemFree(g_parser, buffer as *mut ::core::ffi::c_void);
    };
}

unsafe extern "C" fn test_default_current() {
    _check_set_test_info(
        b"test_default_current\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1999,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc>hell]</doc>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut entity_text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc [\n<!ENTITY entity '&#37;'>\n]>\n<doc>&entity;</doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    set_subtest(b"with defaulting\0".as_ptr() as *const ::core::ffi::c_char);
    let mut storage: handler_record_list = handler_record_list {
        count: 0,
        entries: [handler_record_entry {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            arg: 0,
        }; 50],
    };
    storage.count = 0;
    XML_SetDefaultHandler(
        g_parser,
        Some(
            record_default_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            record_cdata_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2015i32,
        );
    }
    let mut i: ::core::ffi::c_int = 0;
    let c2rust_fresh2 = i;
    i = i + 1;
    let mut e: *const handler_record_entry = _handler_record_get(
        &raw mut storage as *const handler_record_list,
        c2rust_fresh2,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2017,
    );
    if !(strcmp(
        (*e).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2017i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e).arg == 5) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2017i32,
            b"check failed: e->arg == (5)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut cdata_len_remaining: ::core::ffi::c_int = 5;
    while cdata_len_remaining > 0 {
        let c2rust_fresh3 = i;
        i = i + 1;
        let mut c_entry: *const handler_record_entry = _handler_record_get(
            &raw mut storage as *const handler_record_list,
            c2rust_fresh3,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2022,
        );
        if !(strcmp(
            (*c_entry).name,
            b"record_cdata_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0)
        {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                2023i32,
                b"check failed: strcmp(c_entry->name, \"record_cdata_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*c_entry).arg > 0) {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                2024i32,
                b"check failed: c_entry->arg > 0\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if !((*c_entry).arg <= cdata_len_remaining) {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                2025i32,
                b"check failed: c_entry->arg <= cdata_len_remaining\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        cdata_len_remaining -= (*c_entry).arg;
        let c2rust_fresh4 = i;
        i = i + 1;
        let mut e_0: *const handler_record_entry = _handler_record_get(
            &raw mut storage as *const handler_record_list,
            c2rust_fresh4,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2029,
        );
        if !(strcmp(
            (*e_0).name,
            b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0)
        {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                2029i32,
                b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !((*e_0).arg == (*c_entry).arg) {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                2029i32,
                b"check failed: e->arg == (c_entry->arg)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
    let c2rust_fresh5 = i;
    i = i + 1;
    let mut e_1: *const handler_record_entry = _handler_record_get(
        &raw mut storage as *const handler_record_list,
        c2rust_fresh5,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2031,
    );
    if !(strcmp(
        (*e_1).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2031i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_1).arg == 6) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2031i32,
            b"check failed: e->arg == (6)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !(storage.count == i) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2032i32,
            b"check failed: storage.count == i\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    set_subtest(b"no defaulting\0".as_ptr() as *const ::core::ffi::c_char);
    let mut storage_0: handler_record_list = handler_record_list {
        count: 0,
        entries: [handler_record_entry {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            arg: 0,
        }; 50],
    };
    storage_0.count = 0;
    XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
    XML_SetDefaultHandler(
        g_parser,
        Some(
            record_default_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            record_cdata_nodefault_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut storage_0 as *mut ::core::ffi::c_void);
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2046i32,
        );
    }
    let mut i_0: ::core::ffi::c_int = 0;
    let c2rust_fresh6 = i_0;
    i_0 = i_0 + 1;
    let mut e_2: *const handler_record_entry = _handler_record_get(
        &raw mut storage_0 as *const handler_record_list,
        c2rust_fresh6,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2048,
    );
    if !(strcmp(
        (*e_2).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2048i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_2).arg == 5) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2048i32,
            b"check failed: e->arg == (5)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut cdata_len_remaining_0: ::core::ffi::c_int = 5;
    while cdata_len_remaining_0 > 0 {
        let c2rust_fresh7 = i_0;
        i_0 = i_0 + 1;
        let mut c_entry_0: *const handler_record_entry = _handler_record_get(
            &raw mut storage_0 as *const handler_record_list,
            c2rust_fresh7,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2053,
        );
        if !(strcmp(
            (*c_entry_0).name,
            b"record_cdata_nodefault_handler\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0)
        {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                2054i32,
                b"check failed: strcmp(c_entry->name, \"record_cdata_nodefault_handler\") == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if !((*c_entry_0).arg > 0) {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                2055i32,
                b"check failed: c_entry->arg > 0\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if !((*c_entry_0).arg <= cdata_len_remaining_0) {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                2056i32,
                b"check failed: c_entry->arg <= cdata_len_remaining\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        cdata_len_remaining_0 -= (*c_entry_0).arg;
    }
    let c2rust_fresh8 = i_0;
    i_0 = i_0 + 1;
    let mut e_3: *const handler_record_entry = _handler_record_get(
        &raw mut storage_0 as *const handler_record_list,
        c2rust_fresh8,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2059,
    );
    if !(strcmp(
        (*e_3).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2059i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_3).arg == 6) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2059i32,
            b"check failed: e->arg == (6)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !(storage_0.count == i_0) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2060i32,
            b"check failed: storage.count == i\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    set_subtest(b"with internal entity\0".as_ptr() as *const ::core::ffi::c_char);
    let mut storage_1: handler_record_list = handler_record_list {
        count: 0,
        entries: [handler_record_entry {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            arg: 0,
        }; 50],
    };
    storage_1.count = 0;
    XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
    XML_SetDefaultHandler(
        g_parser,
        Some(
            record_default_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            record_cdata_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut storage_1 as *mut ::core::ffi::c_void);
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        entity_text,
        strlen(entity_text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2075i32,
        );
    }
    let mut e_4: *const handler_record_entry = _handler_record_get(
        &raw mut storage_1 as *const handler_record_list,
        0,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2077,
    );
    if !(strcmp(
        (*e_4).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2077i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_4).arg == 9) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2077i32,
            b"check failed: e->arg == (9)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_5: *const handler_record_entry = _handler_record_get(
        &raw mut storage_1 as *const handler_record_list,
        1,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2078,
    );
    if !(strcmp(
        (*e_5).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2078i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_5).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2078i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_6: *const handler_record_entry = _handler_record_get(
        &raw mut storage_1 as *const handler_record_list,
        2,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2079,
    );
    if !(strcmp(
        (*e_6).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2079i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_6).arg == 3) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2079i32,
            b"check failed: e->arg == (3)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_7: *const handler_record_entry = _handler_record_get(
        &raw mut storage_1 as *const handler_record_list,
        3,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2080,
    );
    if !(strcmp(
        (*e_7).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2080i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_7).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2080i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_8: *const handler_record_entry = _handler_record_get(
        &raw mut storage_1 as *const handler_record_list,
        4,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2081,
    );
    if !(strcmp(
        (*e_8).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2081i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_8).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2081i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_9: *const handler_record_entry = _handler_record_get(
        &raw mut storage_1 as *const handler_record_list,
        5,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2082,
    );
    if !(strcmp(
        (*e_9).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2082i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_9).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2082i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_10: *const handler_record_entry = _handler_record_get(
        &raw mut storage_1 as *const handler_record_list,
        6,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2083,
    );
    if !(strcmp(
        (*e_10).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2083i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_10).arg == 8) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2083i32,
            b"check failed: e->arg == (8)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_11: *const handler_record_entry = _handler_record_get(
        &raw mut storage_1 as *const handler_record_list,
        7,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2084,
    );
    if !(strcmp(
        (*e_11).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2084i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_11).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2084i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_12: *const handler_record_entry = _handler_record_get(
        &raw mut storage_1 as *const handler_record_list,
        8,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2085,
    );
    if !(strcmp(
        (*e_12).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2085i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_12).arg == 6) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2085i32,
            b"check failed: e->arg == (6)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_13: *const handler_record_entry = _handler_record_get(
        &raw mut storage_1 as *const handler_record_list,
        9,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2086,
    );
    if !(strcmp(
        (*e_13).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2086i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_13).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2086i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_14: *const handler_record_entry = _handler_record_get(
        &raw mut storage_1 as *const handler_record_list,
        10,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2087,
    );
    if !(strcmp(
        (*e_14).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2087i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_14).arg == 7) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2087i32,
            b"check failed: e->arg == (7)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_15: *const handler_record_entry = _handler_record_get(
        &raw mut storage_1 as *const handler_record_list,
        11,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2088,
    );
    if !(strcmp(
        (*e_15).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2088i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_15).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2088i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_16: *const handler_record_entry = _handler_record_get(
        &raw mut storage_1 as *const handler_record_list,
        12,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2089,
    );
    if !(strcmp(
        (*e_16).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2089i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_16).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2089i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_17: *const handler_record_entry = _handler_record_get(
        &raw mut storage_1 as *const handler_record_list,
        13,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2090,
    );
    if !(strcmp(
        (*e_17).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2090i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_17).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2090i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_18: *const handler_record_entry = _handler_record_get(
        &raw mut storage_1 as *const handler_record_list,
        14,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2091,
    );
    if !(strcmp(
        (*e_18).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2091i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_18).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2091i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_19: *const handler_record_entry = _handler_record_get(
        &raw mut storage_1 as *const handler_record_list,
        15,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2092,
    );
    if !(strcmp(
        (*e_19).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2092i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_19).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2092i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_20: *const handler_record_entry = _handler_record_get(
        &raw mut storage_1 as *const handler_record_list,
        16,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2093,
    );
    if !(strcmp(
        (*e_20).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2093i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_20).arg == 5) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2093i32,
            b"check failed: e->arg == (5)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_21: *const handler_record_entry = _handler_record_get(
        &raw mut storage_1 as *const handler_record_list,
        17,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2094,
    );
    if !(strcmp(
        (*e_21).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2094i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_21).arg == 8) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2094i32,
            b"check failed: e->arg == (8)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_22: *const handler_record_entry = _handler_record_get(
        &raw mut storage_1 as *const handler_record_list,
        18,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2095,
    );
    if !(strcmp(
        (*e_22).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2095i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_22).arg == 6) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2095i32,
            b"check failed: e->arg == (6)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !(storage_1.count == 19) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2096i32,
            b"check failed: storage.count == 19\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    set_subtest(b"with skip handler\0".as_ptr() as *const ::core::ffi::c_char);
    let mut storage_2: handler_record_list = handler_record_list {
        count: 0,
        entries: [handler_record_entry {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            arg: 0,
        }; 50],
    };
    storage_2.count = 0;
    XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
    XML_SetDefaultHandler(
        g_parser,
        Some(
            record_default_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            record_cdata_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetSkippedEntityHandler(
        g_parser,
        Some(
            record_skip_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut storage_2 as *mut ::core::ffi::c_void);
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        entity_text,
        strlen(entity_text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2112i32,
        );
    }
    let mut e_23: *const handler_record_entry = _handler_record_get(
        &raw mut storage_2 as *const handler_record_list,
        0,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2114,
    );
    if !(strcmp(
        (*e_23).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2114i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_23).arg == 9) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2114i32,
            b"check failed: e->arg == (9)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_24: *const handler_record_entry = _handler_record_get(
        &raw mut storage_2 as *const handler_record_list,
        1,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2115,
    );
    if !(strcmp(
        (*e_24).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2115i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_24).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2115i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_25: *const handler_record_entry = _handler_record_get(
        &raw mut storage_2 as *const handler_record_list,
        2,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2116,
    );
    if !(strcmp(
        (*e_25).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2116i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_25).arg == 3) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2116i32,
            b"check failed: e->arg == (3)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_26: *const handler_record_entry = _handler_record_get(
        &raw mut storage_2 as *const handler_record_list,
        3,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2117,
    );
    if !(strcmp(
        (*e_26).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2117i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_26).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2117i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_27: *const handler_record_entry = _handler_record_get(
        &raw mut storage_2 as *const handler_record_list,
        4,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2118,
    );
    if !(strcmp(
        (*e_27).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2118i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_27).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2118i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_28: *const handler_record_entry = _handler_record_get(
        &raw mut storage_2 as *const handler_record_list,
        5,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2119,
    );
    if !(strcmp(
        (*e_28).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2119i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_28).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2119i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_29: *const handler_record_entry = _handler_record_get(
        &raw mut storage_2 as *const handler_record_list,
        6,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2120,
    );
    if !(strcmp(
        (*e_29).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2120i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_29).arg == 8) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2120i32,
            b"check failed: e->arg == (8)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_30: *const handler_record_entry = _handler_record_get(
        &raw mut storage_2 as *const handler_record_list,
        7,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2121,
    );
    if !(strcmp(
        (*e_30).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2121i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_30).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2121i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_31: *const handler_record_entry = _handler_record_get(
        &raw mut storage_2 as *const handler_record_list,
        8,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2122,
    );
    if !(strcmp(
        (*e_31).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2122i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_31).arg == 6) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2122i32,
            b"check failed: e->arg == (6)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_32: *const handler_record_entry = _handler_record_get(
        &raw mut storage_2 as *const handler_record_list,
        9,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2123,
    );
    if !(strcmp(
        (*e_32).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2123i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_32).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2123i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_33: *const handler_record_entry = _handler_record_get(
        &raw mut storage_2 as *const handler_record_list,
        10,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2124,
    );
    if !(strcmp(
        (*e_33).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2124i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_33).arg == 7) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2124i32,
            b"check failed: e->arg == (7)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_34: *const handler_record_entry = _handler_record_get(
        &raw mut storage_2 as *const handler_record_list,
        11,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2125,
    );
    if !(strcmp(
        (*e_34).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2125i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_34).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2125i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_35: *const handler_record_entry = _handler_record_get(
        &raw mut storage_2 as *const handler_record_list,
        12,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2126,
    );
    if !(strcmp(
        (*e_35).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2126i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_35).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2126i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_36: *const handler_record_entry = _handler_record_get(
        &raw mut storage_2 as *const handler_record_list,
        13,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2127,
    );
    if !(strcmp(
        (*e_36).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2127i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_36).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2127i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_37: *const handler_record_entry = _handler_record_get(
        &raw mut storage_2 as *const handler_record_list,
        14,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2128,
    );
    if !(strcmp(
        (*e_37).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2128i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_37).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2128i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_38: *const handler_record_entry = _handler_record_get(
        &raw mut storage_2 as *const handler_record_list,
        15,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2129,
    );
    if !(strcmp(
        (*e_38).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2129i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_38).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2129i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_39: *const handler_record_entry = _handler_record_get(
        &raw mut storage_2 as *const handler_record_list,
        16,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2130,
    );
    if !(strcmp(
        (*e_39).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2130i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_39).arg == 5) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2130i32,
            b"check failed: e->arg == (5)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_40: *const handler_record_entry = _handler_record_get(
        &raw mut storage_2 as *const handler_record_list,
        17,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2131,
    );
    if !(strcmp(
        (*e_40).name,
        b"record_skip_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2131i32,
            b"check failed: strcmp(e->name, \"record_skip_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_40).arg == 0) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2131i32,
            b"check failed: e->arg == (0)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_41: *const handler_record_entry = _handler_record_get(
        &raw mut storage_2 as *const handler_record_list,
        18,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2132,
    );
    if !(strcmp(
        (*e_41).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2132i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_41).arg == 6) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2132i32,
            b"check failed: e->arg == (6)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !(storage_2.count == 19) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2133i32,
            b"check failed: storage.count == 19\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    set_subtest(b"allow entity\0".as_ptr() as *const ::core::ffi::c_char);
    let mut storage_3: handler_record_list = handler_record_list {
        count: 0,
        entries: [handler_record_entry {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            arg: 0,
        }; 50],
    };
    storage_3.count = 0;
    XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
    XML_SetDefaultHandlerExpand(
        g_parser,
        Some(
            record_default_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            record_cdata_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut storage_3 as *mut ::core::ffi::c_void);
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        entity_text,
        strlen(entity_text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2148i32,
        );
    }
    let mut e_42: *const handler_record_entry = _handler_record_get(
        &raw mut storage_3 as *const handler_record_list,
        0,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2149,
    );
    if !(strcmp(
        (*e_42).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2149i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_42).arg == 9) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2149i32,
            b"check failed: e->arg == (9)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_43: *const handler_record_entry = _handler_record_get(
        &raw mut storage_3 as *const handler_record_list,
        1,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2150,
    );
    if !(strcmp(
        (*e_43).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2150i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_43).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2150i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_44: *const handler_record_entry = _handler_record_get(
        &raw mut storage_3 as *const handler_record_list,
        2,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2151,
    );
    if !(strcmp(
        (*e_44).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2151i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_44).arg == 3) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2151i32,
            b"check failed: e->arg == (3)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_45: *const handler_record_entry = _handler_record_get(
        &raw mut storage_3 as *const handler_record_list,
        3,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2152,
    );
    if !(strcmp(
        (*e_45).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2152i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_45).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2152i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_46: *const handler_record_entry = _handler_record_get(
        &raw mut storage_3 as *const handler_record_list,
        4,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2153,
    );
    if !(strcmp(
        (*e_46).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2153i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_46).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2153i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_47: *const handler_record_entry = _handler_record_get(
        &raw mut storage_3 as *const handler_record_list,
        5,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2154,
    );
    if !(strcmp(
        (*e_47).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2154i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_47).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2154i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_48: *const handler_record_entry = _handler_record_get(
        &raw mut storage_3 as *const handler_record_list,
        6,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2155,
    );
    if !(strcmp(
        (*e_48).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2155i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_48).arg == 8) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2155i32,
            b"check failed: e->arg == (8)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_49: *const handler_record_entry = _handler_record_get(
        &raw mut storage_3 as *const handler_record_list,
        7,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2156,
    );
    if !(strcmp(
        (*e_49).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2156i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_49).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2156i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_50: *const handler_record_entry = _handler_record_get(
        &raw mut storage_3 as *const handler_record_list,
        8,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2157,
    );
    if !(strcmp(
        (*e_50).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2157i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_50).arg == 6) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2157i32,
            b"check failed: e->arg == (6)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_51: *const handler_record_entry = _handler_record_get(
        &raw mut storage_3 as *const handler_record_list,
        9,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2158,
    );
    if !(strcmp(
        (*e_51).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2158i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_51).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2158i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_52: *const handler_record_entry = _handler_record_get(
        &raw mut storage_3 as *const handler_record_list,
        10,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2159,
    );
    if !(strcmp(
        (*e_52).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2159i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_52).arg == 7) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2159i32,
            b"check failed: e->arg == (7)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_53: *const handler_record_entry = _handler_record_get(
        &raw mut storage_3 as *const handler_record_list,
        11,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2160,
    );
    if !(strcmp(
        (*e_53).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2160i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_53).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2160i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_54: *const handler_record_entry = _handler_record_get(
        &raw mut storage_3 as *const handler_record_list,
        12,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2161,
    );
    if !(strcmp(
        (*e_54).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2161i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_54).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2161i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_55: *const handler_record_entry = _handler_record_get(
        &raw mut storage_3 as *const handler_record_list,
        13,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2162,
    );
    if !(strcmp(
        (*e_55).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2162i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_55).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2162i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_56: *const handler_record_entry = _handler_record_get(
        &raw mut storage_3 as *const handler_record_list,
        14,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2163,
    );
    if !(strcmp(
        (*e_56).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2163i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_56).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2163i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_57: *const handler_record_entry = _handler_record_get(
        &raw mut storage_3 as *const handler_record_list,
        15,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2164,
    );
    if !(strcmp(
        (*e_57).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2164i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_57).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2164i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_58: *const handler_record_entry = _handler_record_get(
        &raw mut storage_3 as *const handler_record_list,
        16,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2165,
    );
    if !(strcmp(
        (*e_58).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2165i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_58).arg == 5) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2165i32,
            b"check failed: e->arg == (5)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_59: *const handler_record_entry = _handler_record_get(
        &raw mut storage_3 as *const handler_record_list,
        17,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2166,
    );
    if !(strcmp(
        (*e_59).name,
        b"record_cdata_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2166i32,
            b"check failed: strcmp(e->name, \"record_cdata_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_59).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2166i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_60: *const handler_record_entry = _handler_record_get(
        &raw mut storage_3 as *const handler_record_list,
        18,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2167,
    );
    if !(strcmp(
        (*e_60).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2167i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_60).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2167i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_61: *const handler_record_entry = _handler_record_get(
        &raw mut storage_3 as *const handler_record_list,
        19,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2168,
    );
    if !(strcmp(
        (*e_61).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2168i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_61).arg == 6) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2168i32,
            b"check failed: e->arg == (6)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !(storage_3.count == 20) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2169i32,
            b"check failed: storage.count == 20\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    set_subtest(b"not passing cdata\0".as_ptr() as *const ::core::ffi::c_char);
    let mut storage_4: handler_record_list = handler_record_list {
        count: 0,
        entries: [handler_record_entry {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            arg: 0,
        }; 50],
    };
    storage_4.count = 0;
    XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
    XML_SetDefaultHandlerExpand(
        g_parser,
        Some(
            record_default_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            record_cdata_nodefault_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut storage_4 as *mut ::core::ffi::c_void);
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        entity_text,
        strlen(entity_text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2184i32,
        );
    }
    let mut e_62: *const handler_record_entry = _handler_record_get(
        &raw mut storage_4 as *const handler_record_list,
        0,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2185,
    );
    if !(strcmp(
        (*e_62).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2185i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_62).arg == 9) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2185i32,
            b"check failed: e->arg == (9)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_63: *const handler_record_entry = _handler_record_get(
        &raw mut storage_4 as *const handler_record_list,
        1,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2186,
    );
    if !(strcmp(
        (*e_63).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2186i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_63).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2186i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_64: *const handler_record_entry = _handler_record_get(
        &raw mut storage_4 as *const handler_record_list,
        2,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2187,
    );
    if !(strcmp(
        (*e_64).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2187i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_64).arg == 3) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2187i32,
            b"check failed: e->arg == (3)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_65: *const handler_record_entry = _handler_record_get(
        &raw mut storage_4 as *const handler_record_list,
        3,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2188,
    );
    if !(strcmp(
        (*e_65).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2188i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_65).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2188i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_66: *const handler_record_entry = _handler_record_get(
        &raw mut storage_4 as *const handler_record_list,
        4,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2189,
    );
    if !(strcmp(
        (*e_66).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2189i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_66).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2189i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_67: *const handler_record_entry = _handler_record_get(
        &raw mut storage_4 as *const handler_record_list,
        5,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2190,
    );
    if !(strcmp(
        (*e_67).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2190i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_67).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2190i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_68: *const handler_record_entry = _handler_record_get(
        &raw mut storage_4 as *const handler_record_list,
        6,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2191,
    );
    if !(strcmp(
        (*e_68).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2191i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_68).arg == 8) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2191i32,
            b"check failed: e->arg == (8)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_69: *const handler_record_entry = _handler_record_get(
        &raw mut storage_4 as *const handler_record_list,
        7,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2192,
    );
    if !(strcmp(
        (*e_69).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2192i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_69).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2192i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_70: *const handler_record_entry = _handler_record_get(
        &raw mut storage_4 as *const handler_record_list,
        8,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2193,
    );
    if !(strcmp(
        (*e_70).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2193i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_70).arg == 6) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2193i32,
            b"check failed: e->arg == (6)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_71: *const handler_record_entry = _handler_record_get(
        &raw mut storage_4 as *const handler_record_list,
        9,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2194,
    );
    if !(strcmp(
        (*e_71).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2194i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_71).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2194i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_72: *const handler_record_entry = _handler_record_get(
        &raw mut storage_4 as *const handler_record_list,
        10,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2195,
    );
    if !(strcmp(
        (*e_72).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2195i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_72).arg == 7) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2195i32,
            b"check failed: e->arg == (7)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_73: *const handler_record_entry = _handler_record_get(
        &raw mut storage_4 as *const handler_record_list,
        11,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2196,
    );
    if !(strcmp(
        (*e_73).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2196i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_73).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2196i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_74: *const handler_record_entry = _handler_record_get(
        &raw mut storage_4 as *const handler_record_list,
        12,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2197,
    );
    if !(strcmp(
        (*e_74).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2197i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_74).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2197i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_75: *const handler_record_entry = _handler_record_get(
        &raw mut storage_4 as *const handler_record_list,
        13,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2198,
    );
    if !(strcmp(
        (*e_75).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2198i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_75).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2198i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_76: *const handler_record_entry = _handler_record_get(
        &raw mut storage_4 as *const handler_record_list,
        14,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2199,
    );
    if !(strcmp(
        (*e_76).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2199i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_76).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2199i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_77: *const handler_record_entry = _handler_record_get(
        &raw mut storage_4 as *const handler_record_list,
        15,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2200,
    );
    if !(strcmp(
        (*e_77).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2200i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_77).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2200i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_78: *const handler_record_entry = _handler_record_get(
        &raw mut storage_4 as *const handler_record_list,
        16,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2201,
    );
    if !(strcmp(
        (*e_78).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2201i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_78).arg == 5) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2201i32,
            b"check failed: e->arg == (5)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_79: *const handler_record_entry = _handler_record_get(
        &raw mut storage_4 as *const handler_record_list,
        17,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2203,
    );
    if !(strcmp(
        (*e_79).name,
        b"record_cdata_nodefault_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2203i32,
            b"check failed: strcmp(e->name, \"record_cdata_nodefault_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_79).arg == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2203i32,
            b"check failed: e->arg == (1)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut e_80: *const handler_record_entry = _handler_record_get(
        &raw mut storage_4 as *const handler_record_list,
        18,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2204,
    );
    if !(strcmp(
        (*e_80).name,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2204i32,
            b"check failed: strcmp(e->name, \"record_default_handler\") == 0\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !((*e_80).arg == 6) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2204i32,
            b"check failed: e->arg == (6)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !(storage_4.count == 19) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2205i32,
            b"check failed: storage.count == 19\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_dtd_elements() {
    _check_set_test_info(
        b"test_dtd_elements\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2211,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ELEMENT doc (chapter)>\n<!ELEMENT chapter (#PCDATA)>\n]>\n<doc><chapter>Wombats are go</chapter></doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    XML_SetElementDeclHandler(
        g_parser,
        ::core::mem::transmute(Some(
            dummy_element_decl_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Content,
                ) -> (),
        )),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2221i32,
        );
    }
}

unsafe extern "C" fn element_decl_check_model(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const XML_Char,
    mut model: *mut XML_Content,
) {
    let mut errorFlags: uint32_t = 0;
    errorFlags = errorFlags
        | if strcmp(name, b"junk\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
            0
        } else {
            (1) << 0
        };
    errorFlags = errorFlags | if !model.is_null() { 0 } else { (1) << 1 };
    if !model.is_null() {
        errorFlags = errorFlags
            | if (*model.offset(0)).type_0 == XML_CTYPE_SEQ {
                0
            } else {
                (1) << 2
            };
        errorFlags = errorFlags
            | if (*model.offset(0)).quant == XML_CQUANT_NONE {
                0
            } else {
                (1) << 3
            };
        errorFlags = errorFlags
            | if (*model.offset(0)).numchildren == 2u32 {
                0
            } else {
                (1) << 4
            };
        errorFlags = errorFlags
            | if (*model.offset(0)).children == model.offset(1) {
                0
            } else {
                (1) << 5
            };
        errorFlags = errorFlags
            | if (*model.offset(0)).name.is_null() {
                0
            } else {
                (1) << 6
            };
        errorFlags = errorFlags
            | if (*model.offset(1)).type_0 == XML_CTYPE_CHOICE {
                0
            } else {
                (1) << 7
            };
        errorFlags = errorFlags
            | if (*model.offset(1)).quant == XML_CQUANT_NONE {
                0
            } else {
                (1) << 8
            };
        errorFlags = errorFlags
            | if (*model.offset(1)).numchildren == 3u32 {
                0
            } else {
                (1) << 9
            };
        errorFlags = errorFlags
            | if (*model.offset(1)).children == model.offset(3) {
                0
            } else {
                (1) << 10
            };
        errorFlags = errorFlags
            | if (*model.offset(1)).name.is_null() {
                0
            } else {
                (1) << 11
            };
        errorFlags = errorFlags
            | if (*model.offset(2)).type_0 == XML_CTYPE_NAME {
                0
            } else {
                (1) << 12
            };
        errorFlags = errorFlags
            | if (*model.offset(2)).quant == XML_CQUANT_REP {
                0
            } else {
                (1) << 13
            };
        errorFlags = errorFlags
            | if (*model.offset(2)).numchildren == 0u32 {
                0
            } else {
                (1) << 14
            };
        errorFlags = errorFlags
            | if (*model.offset(2)).children.is_null() {
                0
            } else {
                (1) << 15
            };
        errorFlags = errorFlags
            | if strcmp(
                (*model.offset(2)).name,
                b"zebra\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
            {
                0
            } else {
                (1) << 16
            };
        errorFlags = errorFlags
            | if (*model.offset(3)).type_0 == XML_CTYPE_NAME {
                0
            } else {
                (1) << 17
            };
        errorFlags = errorFlags
            | if (*model.offset(3)).quant == XML_CQUANT_NONE {
                0
            } else {
                (1) << 18
            };
        errorFlags = errorFlags
            | if (*model.offset(3)).numchildren == 0u32 {
                0
            } else {
                (1) << 19
            };
        errorFlags = errorFlags
            | if (*model.offset(3)).children.is_null() {
                0
            } else {
                (1) << 20
            };
        errorFlags = errorFlags
            | if strcmp(
                (*model.offset(3)).name,
                b"bar\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
            {
                0
            } else {
                (1) << 21
            };
        errorFlags = errorFlags
            | if (*model.offset(4)).type_0 == XML_CTYPE_NAME {
                0
            } else {
                (1) << 22
            };
        errorFlags = errorFlags
            | if (*model.offset(4)).quant == XML_CQUANT_NONE {
                0
            } else {
                (1) << 23
            };
        errorFlags = errorFlags
            | if (*model.offset(4)).numchildren == 0u32 {
                0
            } else {
                (1) << 24
            };
        errorFlags = errorFlags
            | if (*model.offset(4)).children.is_null() {
                0
            } else {
                (1) << 25
            };
        errorFlags = errorFlags
            | if strcmp(
                (*model.offset(4)).name,
                b"foo\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
            {
                0
            } else {
                (1) << 26
            };
        errorFlags = errorFlags
            | if (*model.offset(5)).type_0 == XML_CTYPE_NAME {
                0
            } else {
                (1) << 27
            };
        errorFlags = errorFlags
            | if (*model.offset(5)).quant == XML_CQUANT_PLUS {
                0
            } else {
                (1) << 28
            };
        errorFlags = errorFlags
            | if (*model.offset(5)).numchildren == 0u32 {
                0
            } else {
                (1) << 29
            };
        errorFlags = errorFlags
            | if (*model.offset(5)).children.is_null() {
                0
            } else {
                (1) << 30
            };
        errorFlags = errorFlags
            | if strcmp(
                (*model.offset(5isize)).name,
                b"xyz\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0i32
            {
                0u32
            } else {
                (1u32) << 31i32
            };
    }
    XML_SetUserData(g_parser, errorFlags as *mut ::core::ffi::c_void);
    XML_FreeContentModel(g_parser, model);
}

unsafe extern "C" fn test_dtd_elements_nesting() {
    _check_set_test_info(
        b"test_dtd_elements_nesting\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2285,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE foo [\n<!ELEMENT junk ((bar|foo|xyz+), zebra*)>\n]>\n<foo/>\0".as_ptr()
            as *const ::core::ffi::c_char;
    XML_SetUserData(g_parser, -1i32 as *mut ::core::ffi::c_void);
    XML_SetElementDeclHandler(
        g_parser,
        ::core::mem::transmute(Some(
            element_decl_check_model
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Content,
                ) -> (),
        )),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2297i32,
        );
    }
    if *(g_parser as *mut *mut ::core::ffi::c_void) as uint32_t != 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2300i32,
            b"Element declaration model regression detected\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_set_foreign_dtd() {
    _check_set_test_info(
        b"test_set_foreign_dtd\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2305,
    );
    let mut text1: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='us-ascii'?>\n\0".as_ptr() as *const ::core::ffi::c_char;
    let mut text2: *const ::core::ffi::c_char =
        b"<doc>&entity;</doc>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut test_data: ExtTest = ExtTest {
        parse_text: b"<!ELEMENT doc (#PCDATA)*>\0".as_ptr() as *const ::core::ffi::c_char,
        encoding: ::core::ptr::null::<XML_Char>(),
        storage: ::core::ptr::null_mut::<CharData>(),
    };
    XML_SetHashSalt(g_parser, 0x12345678);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetDefaultHandler(
        g_parser,
        Some(
            dummy_default_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    if XML_UseForeignDTD(g_parser, XML_TRUE) != XML_ERROR_NONE {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2318i32,
            b"Could not set foreign DTD\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text1,
        strlen(text1) as ::core::ffi::c_int,
        XML_FALSE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2321i32,
        );
    }
    if XML_UseForeignDTD(g_parser, XML_TRUE) != XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2328i32,
            b"Failed to reject late foreign DTD setting\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_SetHashSalt(g_parser, 0x23456789) != 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2331i32,
            b"Failed to reject late hash salt change\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text2,
        strlen(text2) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2336i32,
        );
    }
}

unsafe extern "C" fn test_foreign_dtd_not_standalone() {
    _check_set_test_info(
        b"test_foreign_dtd_not_standalone\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2341,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='us-ascii'?>\n<doc>&entity;</doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut test_data: ExtTest = ExtTest {
        parse_text: b"<!ELEMENT doc (#PCDATA)*>\0".as_ptr() as *const ::core::ffi::c_char,
        encoding: ::core::ptr::null::<XML_Char>(),
        storage: ::core::ptr::null_mut::<CharData>(),
    };
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetNotStandaloneHandler(
        g_parser,
        Some(
            reject_not_standalone_handler
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
    );
    if XML_UseForeignDTD(g_parser, XML_TRUE) != XML_ERROR_NONE {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2351i32,
            b"Could not set foreign DTD\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    _expect_failure(
        text,
        XML_ERROR_NOT_STANDALONE,
        b"NotStandalonehandler failed to reject\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2353,
    );
}

unsafe extern "C" fn test_invalid_foreign_dtd() {
    _check_set_test_info(
        b"test_invalid_foreign_dtd\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2358,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='us-ascii'?>\n<doc>&entity;</doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut test_data: ExtFaults = ext_faults {
        parse_text: b"$\0".as_ptr() as *const ::core::ffi::c_char,
        fail_text: b"Dollar not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        encoding: ::core::ptr::null::<XML_Char>(),
        error: XML_ERROR_INVALID_TOKEN,
    };
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_faulter
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_UseForeignDTD(g_parser, XML_TRUE);
    _expect_failure(
        text,
        XML_ERROR_EXTERNAL_ENTITY_HANDLING,
        b"Bad DTD should not have been accepted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2369,
    );
}

unsafe extern "C" fn test_foreign_dtd_with_doctype() {
    _check_set_test_info(
        b"test_foreign_dtd_with_doctype\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2374,
    );
    let mut text1: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='us-ascii'?>\n<!DOCTYPE doc [<!ENTITY entity 'hello world'>]>\n\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut text2: *const ::core::ffi::c_char =
        b"<doc>&entity;</doc>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut test_data: ExtTest = ExtTest {
        parse_text: b"<!ELEMENT doc (#PCDATA)*>\0".as_ptr() as *const ::core::ffi::c_char,
        encoding: ::core::ptr::null::<XML_Char>(),
        storage: ::core::ptr::null_mut::<CharData>(),
    };
    XML_SetHashSalt(g_parser, 0x12345678);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetDefaultHandler(
        g_parser,
        Some(
            dummy_default_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    if XML_UseForeignDTD(g_parser, XML_TRUE) != XML_ERROR_NONE {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2388i32,
            b"Could not set foreign DTD\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text1,
        strlen(text1) as ::core::ffi::c_int,
        XML_FALSE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2391i32,
        );
    }
    if XML_UseForeignDTD(g_parser, XML_TRUE) != XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2398i32,
            b"Failed to reject late foreign DTD setting\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_SetHashSalt(g_parser, 0x23456789) != 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2401i32,
            b"Failed to reject late hash salt change\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text2,
        strlen(text2) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2406i32,
        );
    }
}

unsafe extern "C" fn test_foreign_dtd_without_external_subset() {
    _check_set_test_info(
        b"test_foreign_dtd_without_external_subset\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2411,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc [<!ENTITY foo 'bar'>]>\n<doc>&foo;</doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetUserData(g_parser, NULL);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_null_loader
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_UseForeignDTD(g_parser, XML_TRUE);
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2421i32,
        );
    }
}

unsafe extern "C" fn test_empty_foreign_dtd() {
    _check_set_test_info(
        b"test_empty_foreign_dtd\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2425,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='us-ascii'?>\n<doc>&entity;</doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_null_loader
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_UseForeignDTD(g_parser, XML_TRUE);
    _expect_failure(
        text,
        XML_ERROR_UNDEFINED_ENTITY,
        b"Undefined entity not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2433,
    );
}

unsafe extern "C" fn test_set_base() {
    _check_set_test_info(
        b"test_set_base\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2438,
    );
    let mut old_base: *const XML_Char = ::core::ptr::null::<XML_Char>();
    let mut new_base: *const XML_Char = b"/local/file/name.xml\0".as_ptr() as *const XML_Char;
    old_base = XML_GetBase(g_parser);
    if XML_SetBase(g_parser, new_base) != XML_STATUS_OK {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2444i32,
            b"Unable to set base\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if strcmp(XML_GetBase(g_parser), new_base) != 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2446i32,
            b"Base setting not correct\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_SetBase(g_parser, ::core::ptr::null::<XML_Char>()) != XML_STATUS_OK {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2448i32,
            b"Unable to NULL base\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !XML_GetBase(g_parser).is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2450i32,
            b"Base setting not nulled\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    XML_SetBase(g_parser, old_base);
}

unsafe extern "C" fn test_attributes() {
    _check_set_test_info(
        b"test_attributes\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2456,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ELEMENT doc (tag)>\n<!ATTLIST doc id ID #REQUIRED>\n]><doc a='1' id='one' b='2'><tag c='3'/></doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut doc_info: [AttrInfo; 4] = [
        attrInfo {
            name: b"a\0".as_ptr() as *const XML_Char,
            value: b"1\0".as_ptr() as *const XML_Char,
        },
        attrInfo {
            name: b"b\0".as_ptr() as *const XML_Char,
            value: b"2\0".as_ptr() as *const XML_Char,
        },
        attrInfo {
            name: b"id\0".as_ptr() as *const XML_Char,
            value: b"one\0".as_ptr() as *const XML_Char,
        },
        attrInfo {
            name: ::core::ptr::null::<XML_Char>(),
            value: ::core::ptr::null::<XML_Char>(),
        },
    ];
    let mut tag_info: [AttrInfo; 2] = [
        attrInfo {
            name: b"c\0".as_ptr() as *const XML_Char,
            value: b"3\0".as_ptr() as *const XML_Char,
        },
        attrInfo {
            name: ::core::ptr::null::<XML_Char>(),
            value: ::core::ptr::null::<XML_Char>(),
        },
    ];
    let mut info: [ElementInfo; 3] = [
        elementInfo {
            name: b"doc\0".as_ptr() as *const XML_Char,
            attr_count: 3,
            id_name: b"id\0".as_ptr() as *const XML_Char,
            attributes: ::core::ptr::null_mut::<AttrInfo>(),
        },
        elementInfo {
            name: b"tag\0".as_ptr() as *const XML_Char,
            attr_count: 1,
            id_name: ::core::ptr::null::<XML_Char>(),
            attributes: ::core::ptr::null_mut::<AttrInfo>(),
        },
        elementInfo {
            name: ::core::ptr::null::<XML_Char>(),
            attr_count: 0,
            id_name: ::core::ptr::null::<XML_Char>(),
            attributes: ::core::ptr::null_mut::<AttrInfo>(),
        },
    ];
    info[0].attributes = &raw mut doc_info as *mut AttrInfo;
    info[1].attributes = &raw mut tag_info as *mut AttrInfo;
    let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
    if parser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2476i32,
            b"check failed: parser != NULL\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut parserAndElementInfos: ParserAndElementInfo = StructParserAndElementInfo {
        parser: parser,
        info: &raw mut info as *mut ElementInfo,
    };
    XML_SetStartElementHandler(
        parser,
        Some(
            counting_start_element_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetUserData(
        parser,
        &raw mut parserAndElementInfos as *mut ::core::ffi::c_void,
    );
    if _XML_Parse_SINGLE_BYTES(
        parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2486i32,
        );
    }
    XML_ParserFree(parser);
}

unsafe extern "C" fn test_reset_in_entity() {
    _check_set_test_info(
        b"test_reset_in_entity\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2495,
    );
    if g_chunkSize != 0 {
        return;
    }
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ENTITY wombat 'wom'>\n<!ENTITY entity 'hi &wom; there'>\n]>\n<doc>&entity;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut status: XML_ParsingStatus = XML_ParsingStatus {
        parsing: XML_INITIALIZED,
        finalBuffer: 0,
    };
    g_resumable = XML_TRUE;
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            clearing_aborting_character_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    if XML_Parse(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2514i32,
        );
    }
    XML_GetParsingStatus(g_parser, &raw mut status);
    if status.parsing != XML_SUSPENDED {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2517i32,
            b"Parsing status not SUSPENDED\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
    XML_GetParsingStatus(g_parser, &raw mut status);
    if status.parsing != XML_INITIALIZED {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2521i32,
            b"Parsing status doesn't reset to INITIALIZED\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_resume_invalid_parse() {
    _check_set_test_info(
        b"test_resume_invalid_parse\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2526,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc>Hello</doc\0".as_ptr() as *const ::core::ffi::c_char;
    g_resumable = XML_TRUE;
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            clearing_aborting_character_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    if XML_Parse(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2533i32,
        );
    }
    if XML_ResumeParser(g_parser) == XML_STATUS_OK {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2535i32,
            b"Resumed invalid parse not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_UNCLOSED_TOKEN {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2537i32,
            b"Invalid parse not correctly faulted\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_resume_resuspended() {
    _check_set_test_info(
        b"test_resume_resuspended\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2542,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc>Hello<meep/>world</doc>\0".as_ptr() as *const ::core::ffi::c_char;
    g_resumable = XML_TRUE;
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            clearing_aborting_character_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    if XML_Parse(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2549i32,
        );
    }
    g_resumable = XML_TRUE;
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            clearing_aborting_character_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    if XML_ResumeParser(g_parser) != XML_STATUS_SUSPENDED {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2553i32,
            b"Resumption not suspended\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_ResumeParser(g_parser) != XML_STATUS_OK {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2556i32,
        );
    }
}

unsafe extern "C" fn test_cdata_default() {
    _check_set_test_info(
        b"test_cdata_default\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2561,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc><![CDATA[Hello\nworld]]></doc>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut expected: *const XML_Char =
        b"<doc><![CDATA[Hello\nworld]]></doc>\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    XML_SetDefaultHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2572i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_subordinate_reset() {
    _check_set_test_info(
        b"test_subordinate_reset\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2578,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='us-ascii'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_resetter
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2587i32,
        );
    }
}

unsafe extern "C" fn test_subordinate_suspend() {
    _check_set_test_info(
        b"test_subordinate_suspend\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2592,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='us-ascii'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_suspender
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2601i32,
        );
    }
}

unsafe extern "C" fn test_subordinate_xdecl_suspend() {
    _check_set_test_info(
        b"test_subordinate_xdecl_suspend\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2608,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY entity SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&entity;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_suspend_xmldecl
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    g_resumable = XML_TRUE;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2620i32,
        );
    }
}

unsafe extern "C" fn test_subordinate_xdecl_abort() {
    _check_set_test_info(
        b"test_subordinate_xdecl_abort\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2624,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY entity SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&entity;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_suspend_xmldecl
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    g_resumable = XML_FALSE;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2636i32,
        );
    }
}

unsafe extern "C" fn test_ext_entity_invalid_suspended_parse() {
    _check_set_test_info(
        b"test_ext_entity_invalid_suspended_parse\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2641,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut faults: [ExtFaults; 3] = [
        ext_faults {
            parse_text: b"<?xml version='1.0' encoding='us-ascii'?><\0".as_ptr()
                as *const ::core::ffi::c_char,
            fail_text: b"Incomplete element declaration not faulted\0".as_ptr()
                as *const ::core::ffi::c_char,
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_UNCLOSED_TOKEN,
        },
        ext_faults {
            parse_text: b"<?xml version='1.0' encoding='utf-8'?>\xE2\x82\0".as_ptr()
                as *const ::core::ffi::c_char,
            fail_text: b"Incomplete character not faulted\0".as_ptr() as *const ::core::ffi::c_char,
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_PARTIAL_CHAR,
        },
        ext_faults {
            parse_text: ::core::ptr::null::<::core::ffi::c_char>(),
            fail_text: ::core::ptr::null::<::core::ffi::c_char>(),
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_NONE,
        },
    ];
    let mut fault: *mut ExtFaults = ::core::ptr::null_mut::<ExtFaults>();
    fault = (&raw mut faults as *mut ExtFaults).offset(0);
    while !(*fault).parse_text.is_null() {
        set_subtest(
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            (*fault).parse_text,
        );
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_suspending_faulter
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetUserData(g_parser, fault as *mut ::core::ffi::c_void);
        _expect_failure(
            text,
            XML_ERROR_EXTERNAL_ENTITY_HANDLING,
            b"Parser did not report external entity error\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2663,
        );
        XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
        fault = fault.offset(1);
    }
}

unsafe extern "C" fn test_explicit_encoding() {
    _check_set_test_info(
        b"test_explicit_encoding\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2670,
    );
    let mut text1: *const ::core::ffi::c_char =
        b"<doc>Hello \0".as_ptr() as *const ::core::ffi::c_char;
    let mut text2: *const ::core::ffi::c_char =
        b" World</doc>\0".as_ptr() as *const ::core::ffi::c_char;
    if XML_SetEncoding(g_parser, ::core::ptr::null::<XML_Char>()) != XML_STATUS_OK {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2676i32,
            b"Failed to initialise encoding to NULL\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_SetEncoding(g_parser, b"utf-8\0".as_ptr() as *const XML_Char) != XML_STATUS_OK {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2679i32,
            b"Failed to set explicit encoding\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text1,
        strlen(text1) as ::core::ffi::c_int,
        XML_FALSE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2682i32,
        );
    }
    if XML_SetEncoding(g_parser, b"us-ascii\0".as_ptr() as *const XML_Char) != XML_STATUS_ERROR {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2685i32,
            b"Allowed encoding change\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text2,
        strlen(text2) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2688i32,
        );
    }
    if XML_SetEncoding(g_parser, ::core::ptr::null::<XML_Char>()) != XML_STATUS_OK {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2691i32,
            b"Failed to unset encoding\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_trailing_cr() {
    _check_set_test_info(
        b"test_trailing_cr\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2696,
    );
    let mut text: *const ::core::ffi::c_char = b"<doc>\r\0".as_ptr() as *const ::core::ffi::c_char;
    let mut found_cr: ::core::ffi::c_int = 0;
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            cr_cdata_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut found_cr as *mut ::core::ffi::c_void);
    found_cr = 0;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_OK
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2706i32,
            b"Failed to fault unclosed doc\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if found_cr == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2708i32,
            b"Did not catch the carriage return\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
    XML_SetDefaultHandler(
        g_parser,
        Some(
            cr_cdata_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut found_cr as *mut ::core::ffi::c_void);
    found_cr = 0;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_OK
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2717i32,
            b"Failed to fault unclosed doc\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if found_cr == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2719i32,
            b"Did not catch default carriage return\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_ext_entity_trailing_cr() {
    _check_set_test_info(
        b"test_ext_entity_trailing_cr\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2724,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut found_cr: ::core::ffi::c_int = 0;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_cr_catcher
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetUserData(g_parser, &raw mut found_cr as *mut ::core::ffi::c_void);
    found_cr = 0;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_OK
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2737i32,
        );
    }
    if found_cr == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2739i32,
            b"No carriage return found\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_bad_cr_catcher
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetUserData(g_parser, &raw mut found_cr as *mut ::core::ffi::c_void);
    found_cr = 0;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_OK
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2749i32,
        );
    }
    if found_cr == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2751i32,
            b"No carriage return found\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_trailing_rsqb() {
    _check_set_test_info(
        b"test_trailing_rsqb\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2756,
    );
    let mut text8: *const ::core::ffi::c_char = b"<doc>]\0".as_ptr() as *const ::core::ffi::c_char;
    let text16: [::core::ffi::c_char; 15] = ::core::mem::transmute::<
        [u8; 15],
        [::core::ffi::c_char; 15],
    >(*b"\xFF\xFE<\0d\0o\0c\0>\0]\0\0");
    let mut found_rsqb: ::core::ffi::c_int = 0;
    let mut text8_len: ::core::ffi::c_int = strlen(text8) as ::core::ffi::c_int;
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            rsqb_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut found_rsqb as *mut ::core::ffi::c_void);
    found_rsqb = 0;
    if _XML_Parse_SINGLE_BYTES(g_parser, text8, text8_len, XML_TRUE as ::core::ffi::c_int)
        == XML_STATUS_OK
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2767i32,
            b"Failed to fault unclosed doc\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if found_rsqb == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2769i32,
            b"Did not catch the right square bracket\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            rsqb_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut found_rsqb as *mut ::core::ffi::c_void);
    found_rsqb = 0;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        &raw const text16 as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 15]>() as ::core::ffi::c_int - 1,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_OK
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2779i32,
            b"Failed to fault unclosed doc\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if found_rsqb == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2781i32,
            b"Did not catch the right square bracket\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
    XML_SetDefaultHandler(
        g_parser,
        Some(
            rsqb_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut found_rsqb as *mut ::core::ffi::c_void);
    found_rsqb = 0;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        &raw const text16 as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 15]>() as ::core::ffi::c_int - 1,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_OK
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2791i32,
            b"Failed to fault unclosed doc\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if found_rsqb == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2793i32,
            b"Did not catch the right square bracket\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_ext_entity_trailing_rsqb() {
    _check_set_test_info(
        b"test_ext_entity_trailing_rsqb\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2798,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut found_rsqb: ::core::ffi::c_int = 0;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_rsqb_catcher
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetUserData(g_parser, &raw mut found_rsqb as *mut ::core::ffi::c_void);
    found_rsqb = 0;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_OK
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2811i32,
        );
    }
    if found_rsqb == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2813i32,
            b"No right square bracket found\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_ext_entity_good_cdata() {
    _check_set_test_info(
        b"test_ext_entity_good_cdata\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2818,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_good_cdata_ascii
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_OK
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2828i32,
        );
    }
}

unsafe extern "C" fn test_user_parameters() {
    _check_set_test_info(
        b"test_user_parameters\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2833,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='us-ascii'?>\n<!-- Primary parse -->\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut epilog: *const ::core::ffi::c_char =
        b"<!-- Back to primary parser -->\n</doc>\0".as_ptr() as *const ::core::ffi::c_char;
    g_comment_count = 0;
    g_skip_count = 0;
    g_xdecl_count = 0;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetXmlDeclHandler(
        g_parser,
        Some(
            xml_decl_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_param_checker
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetCommentHandler(
        g_parser,
        Some(
            data_check_comment_handler
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
        ),
    );
    XML_SetSkippedEntityHandler(
        g_parser,
        Some(
            param_check_skip_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_UseParserAsHandlerArg(g_parser);
    XML_SetUserData(g_parser, 1i32 as *mut ::core::ffi::c_void);
    g_handler_data = g_parser as *const ::core::ffi::c_void;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_FALSE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2854i32,
        );
    }
    if XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_NEVER) != 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2857i32,
            b"Changed param entity parsing policy while parsing\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        epilog,
        strlen(epilog) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2860i32,
        );
    }
    if g_comment_count != 3 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2862i32,
            b"Comment handler not invoked enough times\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if g_skip_count != 1 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2864i32,
            b"Skip handler not invoked enough times\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if g_xdecl_count != 1 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2866i32,
            b"XML declaration handler not invoked\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_ext_entity_ref_parameter() {
    _check_set_test_info(
        b"test_ext_entity_ref_parameter\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2879,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='us-ascii'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_ref_param_checker
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetExternalEntityRefHandlerArg(g_parser, text as *mut ::core::ffi::c_void);
    g_handler_data = text as *const ::core::ffi::c_void;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2893i32,
        );
    }
    XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_ref_param_checker
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetExternalEntityRefHandlerArg(g_parser, NULL);
    g_handler_data = g_parser as *const ::core::ffi::c_void;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2903i32,
        );
    }
}

unsafe extern "C" fn test_empty_parse() {
    _check_set_test_info(
        b"test_empty_parse\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2908,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc></doc>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut partial: *const ::core::ffi::c_char = b"<doc>\0".as_ptr() as *const ::core::ffi::c_char;
    if XML_Parse(
        g_parser,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0,
        XML_FALSE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2913i32,
            b"Parsing empty string faulted\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_Parse(
        g_parser,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_ERROR
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2915i32,
            b"Parsing final empty string not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_NO_ELEMENTS {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2917i32,
            b"Parsing final empty string faulted for wrong reason\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_FALSE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2923i32,
        );
    }
    if XML_Parse(
        g_parser,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2925i32,
            b"Parsing final empty string faulted\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        partial,
        strlen(partial) as ::core::ffi::c_int,
        XML_FALSE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2932i32,
        );
    }
    if XML_Parse(
        g_parser,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_ERROR
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2934i32,
            b"Parsing final incomplete empty string not faulted\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_negative_len_parse() {
    _check_set_test_info(
        b"test_negative_len_parse\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2939,
    );
    let doc: *const ::core::ffi::c_char = b"<root/>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut isFinal: ::core::ffi::c_int = 0;
    while isFinal < 2 {
        set_subtest(
            b"isFinal=%d\0".as_ptr() as *const ::core::ffi::c_char,
            isFinal,
        );
        let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        if XML_GetErrorCode(parser) != XML_ERROR_NONE {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                2947i32,
                b"There was not supposed to be any initial parse error.\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        let status: XML_Status = XML_Parse(parser, doc, -1, isFinal);
        if status != XML_STATUS_ERROR {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                2952i32,
                b"Negative len was expected to fail the parse but did not.\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if XML_GetErrorCode(parser) != XML_ERROR_INVALID_ARGUMENT {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                2955i32,
                b"Parse error does not match XML_ERROR_INVALID_ARGUMENT.\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        XML_ParserFree(parser);
        isFinal += 1;
    }
}

unsafe extern "C" fn test_negative_len_parse_buffer() {
    _check_set_test_info(
        b"test_negative_len_parse_buffer\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2963,
    );
    let doc: *const ::core::ffi::c_char = b"<root/>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut isFinal: ::core::ffi::c_int = 0;
    while isFinal < 2 {
        set_subtest(
            b"isFinal=%d\0".as_ptr() as *const ::core::ffi::c_char,
            isFinal,
        );
        let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        if XML_GetErrorCode(parser) != XML_ERROR_NONE {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                2971i32,
                b"There was not supposed to be any initial parse error.\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        let buffer: *mut ::core::ffi::c_void =
            XML_GetBuffer(parser, strlen(doc) as ::core::ffi::c_int);
        if buffer.is_null() {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                2976i32,
                b"XML_GetBuffer failed.\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        memcpy(buffer, doc as *const ::core::ffi::c_void, strlen(doc));
        let status: XML_Status = XML_ParseBuffer(parser, -1, isFinal);
        if status != XML_STATUS_ERROR {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                2983i32,
                b"Negative len was expected to fail the parse but did not.\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if XML_GetErrorCode(parser) != XML_ERROR_INVALID_ARGUMENT {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                2986i32,
                b"Parse error does not match XML_ERROR_INVALID_ARGUMENT.\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        XML_ParserFree(parser);
        isFinal += 1;
    }
}

unsafe extern "C" fn get_feature(
    mut feature_id: XML_FeatureEnum,
    mut presult: *mut ::core::ffi::c_long,
) -> XML_Status {
    let mut feature: *const XML_Feature = XML_GetFeatureList();
    if feature.is_null() {
        return XML_STATUS_ERROR;
    }
    while (*feature).feature != XML_FEATURE_END {
        if (*feature).feature == feature_id {
            *presult = (*feature).value;
            return XML_STATUS_OK;
        }
        feature = feature.offset(1);
    }
    return XML_STATUS_ERROR;
}

unsafe extern "C" fn test_get_buffer_1() {
    _check_set_test_info(
        b"test_get_buffer_1\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3010,
    );
    let mut text: *const ::core::ffi::c_char = get_buffer_test_text;
    let mut buffer: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut context_bytes: ::core::ffi::c_long = 0;
    if !XML_GetBuffer(g_parser, -12).is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3017i32,
            b"Negative length buffer not failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    buffer = XML_GetBuffer(g_parser, 1536);
    if buffer.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3022i32,
            b"1.5K buffer failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !buffer.is_null() {
    } else {
        __assert_fail(
            b"buffer != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3023u32,
            b"void test_get_buffer_1(void)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    };
    memcpy(buffer, text as *const ::core::ffi::c_void, strlen(text));
    if XML_ParseBuffer(
        g_parser,
        strlen(text) as ::core::ffi::c_int,
        XML_FALSE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3027i32,
        );
    }
    if !XML_GetBuffer(g_parser, INT_MAX).is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3029i32,
            b"INT_MAX buffer not failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if get_feature(XML_FEATURE_CONTEXT_BYTES, &raw mut context_bytes) != XML_STATUS_OK {
        context_bytes = 0i64;
    }
    if !XML_GetBuffer(
        g_parser,
        (INT_MAX as ::core::ffi::c_long - (context_bytes + 1025)) as ::core::ffi::c_int,
    )
    .is_null()
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3044i32,
            b"INT_MAX- buffer not failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_GetBuffer(g_parser, 1000).is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3048i32,
            b"1000 buffer failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_get_buffer_2() {
    _check_set_test_info(
        b"test_get_buffer_2\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3053,
    );
    let mut text: *const ::core::ffi::c_char = get_buffer_test_text;
    let mut buffer: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    buffer = XML_GetBuffer(g_parser, 1536);
    if buffer.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3060i32,
            b"1.5K buffer failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !buffer.is_null() {
    } else {
        __assert_fail(
            b"buffer != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3061u32,
            b"void test_get_buffer_2(void)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    };
    memcpy(buffer, text as *const ::core::ffi::c_void, strlen(text));
    if XML_ParseBuffer(
        g_parser,
        strlen(text) as ::core::ffi::c_int,
        XML_FALSE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3065i32,
        );
    }
    if XML_GetBuffer(g_parser, 1024).is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3069i32,
            b"1024 buffer failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_get_buffer_3_overflow() {
    _check_set_test_info(
        b"test_get_buffer_3_overflow\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3075,
    );
    let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
    if !parser.is_null() {
    } else {
        __assert_fail(
            b"parser != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3077u32,
            b"void test_get_buffer_3_overflow(void)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    };
    let text: *const ::core::ffi::c_char = b"\n\0".as_ptr() as *const ::core::ffi::c_char;
    let expectedKeepValue: ::core::ffi::c_int = strlen(text) as ::core::ffi::c_int;
    if _XML_Parse_SINGLE_BYTES(
        parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_FALSE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3087i32,
        );
    }
    if expectedKeepValue > 0 {
    } else {
        __assert_fail(
            b"expectedKeepValue > 0\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3089u32,
            b"void test_get_buffer_3_overflow(void)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    };
    if !XML_GetBuffer(parser, INT_MAX - expectedKeepValue + 1).is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3091i32,
            b"enlarging buffer not failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    XML_ParserFree(parser);
}

unsafe extern "C" fn test_buffer_can_grow_to_max() {
    _check_set_test_info(
        b"test_buffer_can_grow_to_max\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3098,
    );
    let prefixes: [*const ::core::ffi::c_char; 6] = [
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        b"<\0".as_ptr() as *const ::core::ffi::c_char,
        b"<x a='\0".as_ptr() as *const ::core::ffi::c_char,
        b"<doc><x a='\0".as_ptr() as *const ::core::ffi::c_char,
        b"<document><x a='\0".as_ptr() as *const ::core::ffi::c_char,
        b"<averylongelementnamesuchthatitwillhopefullystretchacrossmultiplelinesandlookprettyridiculousitsalsoveryhardtoreadandifyouredoingitihavetowonderifyoureallydonthaveanythingbettertodoofcourseiguessicouldveputsomethingbadinherebutipromisethatididntheybtwhowgreatarespacesandpunctuationforhelpingwithreadabilityprettygreatithinkanywaysthisisprobablylongenoughbye><x a='\0"
            .as_ptr() as *const ::core::ffi::c_char,
    ];
    let num_prefixes: ::core::ffi::c_int =
        (::core::mem::size_of::<[*const ::core::ffi::c_char; 6]>())
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
            as ::core::ffi::c_int;
    let mut maxbuf: ::core::ffi::c_int = INT_MAX / 2 + (INT_MAX & 1);
    let mut i: ::core::ffi::c_int = 0;
    while i < num_prefixes {
        set_subtest(
            b"\"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
            prefixes[i as usize],
        );
        let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        if !(XML_SetAllocTrackerActivationThreshold(
            parser,
            -1i32 as size_t as ::core::ffi::c_ulonglong,
        ) as ::core::ffi::c_int
            == 1)
        {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                3128i32,
                b"check failed: XML_SetAllocTrackerActivationThreshold(parser, (size_t)-1) == XML_TRUE\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let prefix_len: ::core::ffi::c_int = strlen(prefixes[i as usize]) as ::core::ffi::c_int;
        let s: XML_Status = _XML_Parse_SINGLE_BYTES(
            parser,
            prefixes[i as usize],
            prefix_len,
            XML_FALSE as ::core::ffi::c_int,
        );
        if s != XML_STATUS_OK {
            _xml_failure(
                parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                3134i32,
            );
        }
        if XML_GetBuffer(parser, maxbuf - prefix_len).is_null() {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                3138i32,
                b"check failed: XML_GetBuffer(parser, maxbuf - prefix_len) != NULL\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !XML_GetBuffer(parser, maxbuf + 1).is_null() {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                3141i32,
                b"check failed: XML_GetBuffer(parser, maxbuf + 1) == NULL\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        XML_ParserFree(parser);
        i += 1;
    }
}

unsafe extern "C" fn test_getbuffer_allocates_on_zero_len() {
    _check_set_test_info(
        b"test_getbuffer_allocates_on_zero_len\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3147,
    );
    let mut first_len: ::core::ffi::c_int = 1;
    while first_len >= 0 {
        set_subtest(
            b"with len=%d first\0".as_ptr() as *const ::core::ffi::c_char,
            first_len,
        );
        let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        if parser.is_null() {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                3151i32,
                b"check failed: parser != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if XML_GetBuffer(parser, first_len).is_null() {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                3152i32,
                b"check failed: XML_GetBuffer(parser, first_len) != NULL\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if XML_GetBuffer(parser, 0).is_null() {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                3153i32,
                b"check failed: XML_GetBuffer(parser, 0) != NULL\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if XML_ParseBuffer(parser, 0, XML_FALSE as ::core::ffi::c_int) != XML_STATUS_OK {
            _xml_failure(
                parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                3155i32,
            );
        }
        XML_ParserFree(parser);
        first_len -= 1;
    }
}

unsafe extern "C" fn test_byte_info_at_end() {
    _check_set_test_info(
        b"test_byte_info_at_end\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3162,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc></doc>\0".as_ptr() as *const ::core::ffi::c_char;
    if XML_GetCurrentByteIndex(g_parser) != -1 || XML_GetCurrentByteCount(g_parser) != 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3167i32,
            b"Byte index/count incorrect at start of parse\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3170i32,
        );
    }
    if XML_GetCurrentByteCount(g_parser) != 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3173i32,
            b"Terminal byte count incorrect\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_GetCurrentByteIndex(g_parser) != strlen(text) as XML_Index {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3175i32,
            b"Terminal byte index incorrect\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

pub const PRE_ERROR_STR: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"<doc></\0") };

unsafe extern "C" fn test_byte_info_at_error() {
    _check_set_test_info(
        b"test_byte_info_at_error\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3182,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc></wombat></doc>\0".as_ptr() as *const ::core::ffi::c_char;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_OK
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3187i32,
            b"Syntax error not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_GetCurrentByteCount(g_parser) != 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3189i32,
            b"Error byte count incorrect\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_GetCurrentByteIndex(g_parser) as size_t != strlen(PRE_ERROR_STR.as_ptr()) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3191i32,
            b"Error byte index incorrect\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

pub const START_ELEMENT: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"<e>\0") };

pub const CDATA_TEXT: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"Hello\0") };

unsafe extern "C" fn test_byte_info_at_cdata() {
    _check_set_test_info(
        b"test_byte_info_at_cdata\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3201,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<e>Hello</e>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut offset: ::core::ffi::c_int = 0;
    let mut size: ::core::ffi::c_int = 0;
    let mut data: ByteTestData = ByteTestData {
        start_element_len: 0,
        cdata_len: 0,
        total_string_len: 0,
    };
    if !XML_GetInputContext(g_parser, &raw mut offset, &raw mut size).is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3208i32,
            b"Unexpected context at start of parse\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    data.start_element_len = strlen(START_ELEMENT.as_ptr()) as ::core::ffi::c_int;
    data.cdata_len = strlen(CDATA_TEXT.as_ptr()) as ::core::ffi::c_int;
    data.total_string_len = strlen(text) as ::core::ffi::c_int;
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            byte_character_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut data as *mut ::core::ffi::c_void);
    if XML_Parse(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_OK
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3216i32,
        );
    }
}

unsafe extern "C" fn test_predefined_entities() {
    _check_set_test_info(
        b"test_predefined_entities\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3224,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc>&lt;&gt;&amp;&quot;&apos;</doc>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut expected: *const XML_Char =
        b"<doc>&lt;&gt;&amp;&quot;&apos;</doc>\0".as_ptr() as *const XML_Char;
    let mut result: *const XML_Char = b"<>&\"'\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    XML_SetDefaultHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    CharData_Init(&raw mut storage);
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3238i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
    XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
    _run_character_check(
        text,
        result,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3244,
    );
}

unsafe extern "C" fn test_invalid_tag_in_dtd() {
    _check_set_test_info(
        b"test_invalid_tag_in_dtd\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3256,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc SYSTEM '004-1.ent'>\n<doc></doc>\n\0"
        .as_ptr() as *const ::core::ffi::c_char;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_param
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    _expect_failure(
        text,
        XML_ERROR_EXTERNAL_ENTITY_HANDLING,
        b"Invalid tag IN DTD external param not rejected\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3263,
    );
}

unsafe extern "C" fn test_not_predefined_entities() {
    _check_set_test_info(
        b"test_not_predefined_entities\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3268,
    );
    let mut text: [*const ::core::ffi::c_char; 5] = [
        b"<doc>&pt;</doc>\0".as_ptr() as *const ::core::ffi::c_char,
        b"<doc>&amo;</doc>\0".as_ptr() as *const ::core::ffi::c_char,
        b"<doc>&quid;</doc>\0".as_ptr() as *const ::core::ffi::c_char,
        b"<doc>&apod;</doc>\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    let mut i: ::core::ffi::c_int = 0;
    while !text[i as usize].is_null() {
        _expect_failure(
            text[i as usize],
            XML_ERROR_UNDEFINED_ENTITY,
            b"Undefined entity not rejected\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3275,
        );
        XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
        i += 1;
    }
}

unsafe extern "C" fn test_ignore_section() {
    _check_set_test_info(
        b"test_ignore_section\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3283,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc SYSTEM 'foo'>\n<doc><e>&entity;</e></doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut expected: *const XML_Char =
        b"<![IGNORE[<!ELEMENT e (#PCDATA)*>]]>\n&entity;\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_load_ignore
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetDefaultHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetStartDoctypeDeclHandler(
        g_parser,
        Some(
            dummy_start_doctype_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetEndDoctypeDeclHandler(
        g_parser,
        Some(dummy_end_doctype_handler as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
    );
    XML_SetElementDeclHandler(
        g_parser,
        ::core::mem::transmute(Some(
            dummy_element_decl_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Content,
                ) -> (),
        )),
    );
    XML_SetStartElementHandler(
        g_parser,
        Some(
            dummy_start_element
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetEndElementHandler(
        g_parser,
        Some(
            dummy_end_element
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3302i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_ignore_section_utf16() {
    _check_set_test_info(
        b"test_ignore_section_utf16\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3307,
    );
    let text: [::core::ffi::c_char; 85] = ::core::mem::transmute::<
        [u8; 85],
        [::core::ffi::c_char; 85],
    >(
        *b"<\0!\0D\0O\0C\0T\0Y\0P\0E\0 \0d\0 \0S\0Y\0S\0T\0E\0M\0 \0'\0s\0'\0>\0\n\0<\0d\0>\0<\0e\0>\0&\0e\0n\0;\0<\0/\0e\0>\0<\0/\0d\0>\0\0",
    );
    let mut expected: *const XML_Char =
        b"<![IGNORE[<!ELEMENT e (#PCDATA)*>]]>\n&en;\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_load_ignore_utf16
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetDefaultHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetStartDoctypeDeclHandler(
        g_parser,
        Some(
            dummy_start_doctype_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetEndDoctypeDeclHandler(
        g_parser,
        Some(dummy_end_doctype_handler as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
    );
    XML_SetElementDeclHandler(
        g_parser,
        ::core::mem::transmute(Some(
            dummy_element_decl_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Content,
                ) -> (),
        )),
    );
    XML_SetStartElementHandler(
        g_parser,
        Some(
            dummy_start_element
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetEndElementHandler(
        g_parser,
        Some(
            dummy_end_element
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        &raw const text as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 85]>() as ::core::ffi::c_int - 1,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3329i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_ignore_section_utf16_be() {
    _check_set_test_info(
        b"test_ignore_section_utf16_be\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3334,
    );
    let text: [::core::ffi::c_char; 85] = ::core::mem::transmute::<
        [u8; 85],
        [::core::ffi::c_char; 85],
    >(
        *b"\0<\0!\0D\0O\0C\0T\0Y\0P\0E\0 \0d\0 \0S\0Y\0S\0T\0E\0M\0 \0'\0s\0'\0>\0\n\0<\0d\0>\0<\0e\0>\0&\0e\0n\0;\0<\0/\0e\0>\0<\0/\0d\0>\0",
    );
    let mut expected: *const XML_Char =
        b"<![IGNORE[<!ELEMENT e (#PCDATA)*>]]>\n&en;\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_load_ignore_utf16_be
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetDefaultHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetStartDoctypeDeclHandler(
        g_parser,
        Some(
            dummy_start_doctype_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetEndDoctypeDeclHandler(
        g_parser,
        Some(dummy_end_doctype_handler as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
    );
    XML_SetElementDeclHandler(
        g_parser,
        ::core::mem::transmute(Some(
            dummy_element_decl_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Content,
                ) -> (),
        )),
    );
    XML_SetStartElementHandler(
        g_parser,
        Some(
            dummy_start_element
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetEndElementHandler(
        g_parser,
        Some(
            dummy_end_element
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        &raw const text as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 85]>() as ::core::ffi::c_int - 1,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3357i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_bad_ignore_section() {
    _check_set_test_info(
        b"test_bad_ignore_section\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3363,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc SYSTEM 'foo'>\n<doc><e>&entity;</e></doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut faults: [ExtFaults; 4] = [
        ext_faults {
            parse_text: b"<![IGNORE[<!ELEM\0".as_ptr() as *const ::core::ffi::c_char,
            fail_text: b"Broken-off declaration not faulted\0".as_ptr()
                as *const ::core::ffi::c_char,
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_SYNTAX,
        },
        ext_faults {
            parse_text: b"<![IGNORE[\x01]]>\0".as_ptr() as *const ::core::ffi::c_char,
            fail_text: b"Invalid XML character not faulted\0".as_ptr()
                as *const ::core::ffi::c_char,
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_INVALID_TOKEN,
        },
        ext_faults {
            parse_text: b"<![IGNORE[\xE2\x82\0".as_ptr() as *const ::core::ffi::c_char,
            fail_text: b"Partial XML character not faulted\0".as_ptr()
                as *const ::core::ffi::c_char,
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_PARTIAL_CHAR,
        },
        ext_faults {
            parse_text: ::core::ptr::null::<::core::ffi::c_char>(),
            fail_text: ::core::ptr::null::<::core::ffi::c_char>(),
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_NONE,
        },
    ];
    let mut fault: *mut ExtFaults = ::core::ptr::null_mut::<ExtFaults>();
    fault = (&raw mut faults as *mut ExtFaults).offset(0);
    while !(*fault).parse_text.is_null() {
        set_subtest(
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            (*fault).parse_text,
        );
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_faulter
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetUserData(g_parser, fault as *mut ::core::ffi::c_void);
        _expect_failure(
            text,
            XML_ERROR_EXTERNAL_ENTITY_HANDLING,
            b"Incomplete IGNORE section not failed\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3383,
        );
        XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
        fault = fault.offset(1);
    }
}

unsafe extern "C" fn external_bom_checker(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut text: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut ext_parser: XML_Parser =
        XML_ExternalEntityParserCreate(parser, context, ::core::ptr::null::<XML_Char>());
    if ext_parser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3406i32,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if strcmp(
        systemId,
        b"004-2.ent\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        let testdata: *mut bom_testdata =
            *(parser as *mut *mut ::core::ffi::c_void) as *mut bom_testdata;
        let external: *const ::core::ffi::c_char = (*testdata).external;
        let split: ::core::ffi::c_int = (*testdata).split;
        (*testdata).nested_callback_happened = XML_TRUE;
        if _XML_Parse_SINGLE_BYTES(ext_parser, external, split, XML_FALSE as ::core::ffi::c_int)
            != XML_STATUS_OK
        {
            _xml_failure(
                ext_parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                3417i32,
            );
        }
        text = external.offset(split as isize);
    } else if strcmp(
        systemId,
        b"004-1.ent\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        text = b"<!ELEMENT doc EMPTY>\n<!ENTITY % e1 SYSTEM '004-2.ent'>\n<!ENTITY % e2 '%e1;'>\n\0"
            .as_ptr() as *const ::core::ffi::c_char;
    } else {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3425i32,
            b"unknown systemId\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if _XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_OK
    {
        _xml_failure(
            ext_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3430i32,
        );
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK as ::core::ffi::c_int;
}

unsafe extern "C" fn test_external_bom_consumed() {
    _check_set_test_info(
        b"test_external_bom_consumed\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3437,
    );
    let text: *const ::core::ffi::c_char = b"<!DOCTYPE doc SYSTEM '004-1.ent'>\n<doc></doc>\n\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let external: *const ::core::ffi::c_char =
        b"\xEF\xBB\xBF<!ATTLIST doc a1 CDATA 'value'>\0".as_ptr() as *const ::core::ffi::c_char;
    let len: ::core::ffi::c_int = strlen(external) as ::core::ffi::c_int;
    let mut split: ::core::ffi::c_int = 0;
    while split <= len {
        set_subtest(
            b"split at byte %d\0".as_ptr() as *const ::core::ffi::c_char,
            split,
        );
        let mut testdata: bom_testdata = bom_testdata {
            external: ::core::ptr::null::<::core::ffi::c_char>(),
            split: 0,
            nested_callback_happened: 0,
        };
        testdata.external = external;
        testdata.split = split;
        testdata.nested_callback_happened = XML_FALSE;
        let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        if parser.is_null() {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                3452i32,
                b"Couldn't create parser\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        XML_SetParamEntityParsing(parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            parser,
            Some(
                external_bom_checker
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetUserData(parser, &raw mut testdata as *mut ::core::ffi::c_void);
        if _XML_Parse_SINGLE_BYTES(
            parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) == XML_STATUS_ERROR
        {
            _xml_failure(
                parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                3459i32,
            );
        }
        if testdata.nested_callback_happened == 0 {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                3461i32,
                b"ref handler not called\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        XML_ParserFree(parser);
        split += 1;
    }
}

unsafe extern "C" fn test_external_entity_values() {
    _check_set_test_info(
        b"test_external_entity_values\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3469,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc SYSTEM '004-1.ent'>\n<doc></doc>\n\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut data_004_2: [ExtFaults; 12] = [
        ext_faults {
            parse_text: b"<!ATTLIST doc a1 CDATA 'value'>\0".as_ptr() as *const ::core::ffi::c_char,
            fail_text: ::core::ptr::null::<::core::ffi::c_char>(),
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_NONE,
        },
        ext_faults {
            parse_text: b"<!ATTLIST $doc a1 CDATA 'value'>\0".as_ptr()
                as *const ::core::ffi::c_char,
            fail_text: b"Invalid token not faulted\0".as_ptr() as *const ::core::ffi::c_char,
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_INVALID_TOKEN,
        },
        ext_faults {
            parse_text: b"'wombat\0".as_ptr() as *const ::core::ffi::c_char,
            fail_text: b"Unterminated string not faulted\0".as_ptr() as *const ::core::ffi::c_char,
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_UNCLOSED_TOKEN,
        },
        ext_faults {
            parse_text: b"\xE2\x82\0".as_ptr() as *const ::core::ffi::c_char,
            fail_text: b"Partial UTF-8 character not faulted\0".as_ptr()
                as *const ::core::ffi::c_char,
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_PARTIAL_CHAR,
        },
        ext_faults {
            parse_text: b"<?xml version='1.0' encoding='utf-8'?>\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            fail_text: ::core::ptr::null::<::core::ffi::c_char>(),
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_NONE,
        },
        ext_faults {
            parse_text: b"<?xml?>\0".as_ptr() as *const ::core::ffi::c_char,
            fail_text: b"Malformed XML declaration not faulted\0".as_ptr()
                as *const ::core::ffi::c_char,
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_XML_DECL,
        },
        ext_faults {
            parse_text: b"\xEF\xBB\xBF<!ATTLIST doc a1 CDATA 'value'>\0".as_ptr()
                as *const ::core::ffi::c_char,
            fail_text: ::core::ptr::null::<::core::ffi::c_char>(),
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_NONE,
        },
        ext_faults {
            parse_text: b"<?xml version='1.0' encoding='utf-8'?>\n$\0".as_ptr()
                as *const ::core::ffi::c_char,
            fail_text: b"Invalid token after text declaration not faulted\0".as_ptr()
                as *const ::core::ffi::c_char,
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_INVALID_TOKEN,
        },
        ext_faults {
            parse_text: b"<?xml version='1.0' encoding='utf-8'?>\n'wombat\0".as_ptr()
                as *const ::core::ffi::c_char,
            fail_text: b"Unterminated string after text decl not faulted\0".as_ptr()
                as *const ::core::ffi::c_char,
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_UNCLOSED_TOKEN,
        },
        ext_faults {
            parse_text: b"<?xml version='1.0' encoding='utf-8'?>\n\xE2\x82\0".as_ptr()
                as *const ::core::ffi::c_char,
            fail_text: b"Partial UTF-8 character after text decl not faulted\0".as_ptr()
                as *const ::core::ffi::c_char,
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_PARTIAL_CHAR,
        },
        ext_faults {
            parse_text: b"%e1;\0".as_ptr() as *const ::core::ffi::c_char,
            fail_text: b"Recursive parameter entity not faulted\0".as_ptr()
                as *const ::core::ffi::c_char,
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_RECURSIVE_ENTITY_REF,
        },
        ext_faults {
            parse_text: ::core::ptr::null::<::core::ffi::c_char>(),
            fail_text: ::core::ptr::null::<::core::ffi::c_char>(),
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_NONE,
        },
    ];
    let mut i: ::core::ffi::c_int = 0;
    i = 0;
    while !data_004_2[i as usize].parse_text.is_null() {
        set_subtest(
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            data_004_2[i as usize].parse_text,
        );
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_valuer
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetUserData(
            g_parser,
            (&raw mut data_004_2 as *mut ExtFaults).offset(i as isize) as *mut ::core::ffi::c_void,
        );
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) == XML_STATUS_ERROR
        {
            _xml_failure(
                g_parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                3507i32,
            );
        }
        XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
        i += 1;
    }
}

unsafe extern "C" fn test_ext_entity_not_standalone() {
    _check_set_test_info(
        b"test_ext_entity_not_standalone\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3514,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc SYSTEM 'foo'>\n<doc></doc>\0".as_ptr() as *const ::core::ffi::c_char;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_not_standalone
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    _expect_failure(
        text,
        XML_ERROR_EXTERNAL_ENTITY_HANDLING,
        b"Standalone rejection not caught\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3521,
    );
}

unsafe extern "C" fn test_ext_entity_value_abort() {
    _check_set_test_info(
        b"test_ext_entity_value_abort\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3525,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc SYSTEM '004-1.ent'>\n<doc></doc>\n\0"
        .as_ptr() as *const ::core::ffi::c_char;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_value_aborter
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    g_resumable = XML_FALSE;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3534i32,
        );
    }
}

unsafe extern "C" fn test_bad_public_doctype() {
    _check_set_test_info(
        b"test_bad_public_doctype\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3538,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='utf-8'?>\n<!DOCTYPE doc PUBLIC '{BadName}' 'test'>\n<doc></doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    XML_SetDoctypeDeclHandler(
        g_parser,
        Some(
            dummy_start_doctype_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
        Some(dummy_end_doctype_handler as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
    );
    _expect_failure(
        text,
        XML_ERROR_PUBLICID,
        b"Bad Public ID not failed\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3546,
    );
}

unsafe extern "C" fn test_attribute_enum_value() {
    _check_set_test_info(
        b"test_attribute_enum_value\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3551,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' standalone='no'?>\n<!DOCTYPE animal SYSTEM 'test.dtd'>\n<animal>This is a \n    <a/>  \n\nyellow tiger</animal>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut dtd_data: ExtTest = ExtTest {
    parse_text:   b"<!ELEMENT animal (#PCDATA|a)*>\n<!ELEMENT a EMPTY>\n<!ATTLIST animal xml:space (default|preserve) 'preserve'>\0"
            .as_ptr() as *const ::core::ffi::c_char,
    encoding:   ::core::ptr::null::<XML_Char>(),
    storage:   ::core::ptr::null_mut::<CharData>(),
};
    let mut expected: *const XML_Char =
        b"This is a \n      \n\nyellow tiger\0".as_ptr() as *const XML_Char;
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetUserData(g_parser, &raw mut dtd_data as *mut ::core::ffi::c_void);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetAttlistDeclHandler(
        g_parser,
        Some(
            dummy_attlist_decl_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    _run_ext_character_check(
        text,
        &raw mut dtd_data,
        expected,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3567,
    );
}

unsafe extern "C" fn test_predefined_entity_redefinition() {
    _check_set_test_info(
        b"test_predefined_entity_redefinition\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3576,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc [\n<!ENTITY apos 'foo'>\n]>\n<doc>&apos;</doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    _run_character_check(
        text,
        b"'\0".as_ptr() as *const XML_Char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3581,
    );
}

unsafe extern "C" fn test_dtd_stop_processing() {
    _check_set_test_info(
        b"test_dtd_stop_processing\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3588,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc [\n%foo;\n<!ENTITY bar 'bas'>\n]><doc/>\0".as_ptr()
            as *const ::core::ffi::c_char;
    XML_SetEntityDeclHandler(
        g_parser,
        Some(
            dummy_entity_decl_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                    *const XML_Char,
                    ::core::ffi::c_int,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> (),
        ),
    );
    init_dummy_handlers();
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3598i32,
        );
    }
    if get_dummy_handler_flags() != 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3600i32,
            b"DTD processing still going after undefined PE\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_public_notation_no_sysid() {
    _check_set_test_info(
        b"test_public_notation_no_sysid\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3605,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc [\n<!NOTATION note PUBLIC 'foo'>\n<!ELEMENT doc EMPTY>\n]>\n<doc/>\0"
            .as_ptr() as *const ::core::ffi::c_char;
    init_dummy_handlers();
    XML_SetNotationDeclHandler(
        g_parser,
        Some(
            dummy_notation_decl_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3615i32,
        );
    }
    if get_dummy_handler_flags() != DUMMY_NOTATION_DECL_HANDLER_FLAG {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3617i32,
            b"Notation declaration handler not called\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_nested_groups() {
    _check_set_test_info(
        b"test_nested_groups\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3621,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ELEMENT doc (e,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?))))))))))))))))))))))))))))))))>\n<!ELEMENT e EMPTY>]>\n<doc><e/></doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetElementDeclHandler(
        g_parser,
        ::core::mem::transmute(Some(
            dummy_element_decl_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Content,
                ) -> (),
        )),
    );
    XML_SetStartElementHandler(
        g_parser,
        Some(
            record_element_start_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    init_dummy_handlers();
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3641i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, b"doce\0".as_ptr() as *const XML_Char);
    if get_dummy_handler_flags() != DUMMY_ELEMENT_DECL_HANDLER_FLAG {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3644i32,
            b"Element handler not fired\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_group_choice() {
    _check_set_test_info(
        b"test_group_choice\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3648,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ELEMENT doc (a|b|c)+>\n<!ELEMENT a EMPTY>\n<!ELEMENT b (#PCDATA)>\n<!ELEMENT c ANY>\n]>\n<doc>\n<a/>\n<b attr='foo'>This is a foo</b>\n<c></c>\n</doc>\n\0"
        .as_ptr() as *const ::core::ffi::c_char;
    XML_SetElementDeclHandler(
        g_parser,
        ::core::mem::transmute(Some(
            dummy_element_decl_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Content,
                ) -> (),
        )),
    );
    init_dummy_handlers();
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3665i32,
        );
    }
    if get_dummy_handler_flags() != DUMMY_ELEMENT_DECL_HANDLER_FLAG {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3667i32,
            b"Element handler flag not raised\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_standalone_parameter_entity() {
    _check_set_test_info(
        b"test_standalone_parameter_entity\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3671,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' standalone='yes'?>\n<!DOCTYPE doc SYSTEM 'http://example.org/' [\n<!ENTITY % entity '<!ELEMENT doc (#PCDATA)>'>\n%entity;\n]>\n<doc></doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut dtd_data: [::core::ffi::c_char; 22] =
        ::core::mem::transmute::<[u8; 22], [::core::ffi::c_char; 22]>(*b"<!ENTITY % e1 'foo'>\n\0");
    XML_SetUserData(g_parser, &raw mut dtd_data as *mut ::core::ffi::c_void);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_public
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3685i32,
        );
    }
}

unsafe extern "C" fn test_skipped_parameter_entity() {
    _check_set_test_info(
        b"test_skipped_parameter_entity\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3691,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0'?>\n<!DOCTYPE root SYSTEM 'http://example.org/dtd.ent' [\n<!ELEMENT root (#PCDATA|a)* >\n]>\n<root></root>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut dtd_data: ExtTest = ExtTest {
        parse_text: b"%pe2;\0".as_ptr() as *const ::core::ffi::c_char,
        encoding: ::core::ptr::null::<XML_Char>(),
        storage: ::core::ptr::null_mut::<CharData>(),
    };
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetUserData(g_parser, &raw mut dtd_data as *mut ::core::ffi::c_void);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetSkippedEntityHandler(
        g_parser,
        Some(
            dummy_skip_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    init_dummy_handlers();
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3706i32,
        );
    }
    if get_dummy_handler_flags() != DUMMY_SKIP_HANDLER_FLAG {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3708i32,
            b"Skip handler not executed\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_recursive_external_parameter_entity() {
    _check_set_test_info(
        b"test_recursive_external_parameter_entity\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3713,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0'?>\n<!DOCTYPE root SYSTEM 'http://example.org/dtd.ent' [\n<!ELEMENT root (#PCDATA|a)* >\n]>\n<root></root>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut dtd_data: ExtFaults = ext_faults {
        parse_text: b"<!ENTITY % pe2 '&#37;pe2;'>\n%pe2;\0".as_ptr() as *const ::core::ffi::c_char,
        fail_text: b"Recursive external parameter entity not faulted\0".as_ptr()
            as *const ::core::ffi::c_char,
        encoding: ::core::ptr::null::<XML_Char>(),
        error: XML_ERROR_RECURSIVE_ENTITY_REF,
    };
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_faulter
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetUserData(g_parser, &raw mut dtd_data as *mut ::core::ffi::c_void);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    _expect_failure(
        text,
        XML_ERROR_EXTERNAL_ENTITY_HANDLING,
        b"Recursive external parameter not spotted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3727,
    );
}

unsafe extern "C" fn test_undefined_ext_entity_in_external_dtd() {
    _check_set_test_info(
        b"test_undefined_ext_entity_in_external_dtd\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3732,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc SYSTEM 'foo'>\n<doc></doc>\n\0".as_ptr() as *const ::core::ffi::c_char;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_devaluer
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetUserData(g_parser, NULL);
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3741i32,
        );
    }
    XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_devaluer
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetUserData(g_parser, g_parser as *mut ::core::ffi::c_void);
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3752i32,
        );
    }
}

unsafe extern "C" fn test_suspend_xdecl() {
    _check_set_test_info(
        b"test_suspend_xdecl\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3757,
    );
    let mut text: *const ::core::ffi::c_char = long_character_data_text;
    XML_SetXmlDeclHandler(
        g_parser,
        Some(
            entity_suspending_xdecl_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, g_parser as *mut ::core::ffi::c_void);
    g_resumable = XML_TRUE;
    if XML_Parse(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_SUSPENDED
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3767i32,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_NONE {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3769i32,
        );
    }
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_ERROR
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3773i32,
            b"Attempt to parse while suspended not faulted\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_SUSPENDED {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3775i32,
            b"Suspended parse not faulted with correct error\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_abort_epilog() {
    _check_set_test_info(
        b"test_abort_epilog\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3780,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc></doc>\n\r\n\0".as_ptr() as *const ::core::ffi::c_char;
    let mut trigger_char: XML_Char = '\r' as XML_Char;
    XML_SetDefaultHandler(
        g_parser,
        Some(
            selective_aborting_default_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut trigger_char as *mut ::core::ffi::c_void);
    g_resumable = XML_FALSE;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_ERROR
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3789i32,
            b"Abort not triggered\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_ABORTED {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3791i32,
        );
    }
}

unsafe extern "C" fn test_abort_epilog_2() {
    _check_set_test_info(
        b"test_abort_epilog_2\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3796,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc></doc>\n\0".as_ptr() as *const ::core::ffi::c_char;
    let mut trigger_char: XML_Char = '\n' as XML_Char;
    XML_SetDefaultHandler(
        g_parser,
        Some(
            selective_aborting_default_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut trigger_char as *mut ::core::ffi::c_void);
    g_resumable = XML_FALSE;
    _expect_failure(
        text,
        XML_ERROR_ABORTED,
        b"Abort not triggered\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3803,
    );
}

unsafe extern "C" fn test_suspend_epilog() {
    _check_set_test_info(
        b"test_suspend_epilog\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3808,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc></doc>\n\0".as_ptr() as *const ::core::ffi::c_char;
    let mut trigger_char: XML_Char = '\n' as XML_Char;
    XML_SetDefaultHandler(
        g_parser,
        Some(
            selective_aborting_default_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut trigger_char as *mut ::core::ffi::c_void);
    g_resumable = XML_TRUE;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_SUSPENDED
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3817i32,
        );
    }
}

unsafe extern "C" fn test_suspend_in_sole_empty_tag() {
    _check_set_test_info(
        b"test_suspend_in_sole_empty_tag\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3821,
    );
    let mut text: *const ::core::ffi::c_char = b"<doc/>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut rc: XML_Status = XML_STATUS_ERROR;
    XML_SetEndElementHandler(
        g_parser,
        Some(
            suspending_end_handler
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
        ),
    );
    XML_SetUserData(g_parser, g_parser as *mut ::core::ffi::c_void);
    rc = _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    );
    if rc == XML_STATUS_ERROR {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3829i32,
        );
    } else if rc != XML_STATUS_SUSPENDED {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3831i32,
            b"Suspend not triggered\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    rc = XML_ResumeParser(g_parser);
    if rc == XML_STATUS_ERROR {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3834i32,
        );
    } else if rc != XML_STATUS_OK {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3836i32,
            b"Resume failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_unfinished_epilog() {
    _check_set_test_info(
        b"test_unfinished_epilog\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3840,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc></doc><\0".as_ptr() as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_UNCLOSED_TOKEN,
        b"Incomplete epilog entry not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3844,
    );
}

unsafe extern "C" fn test_partial_char_in_epilog() {
    _check_set_test_info(
        b"test_partial_char_in_epilog\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3848,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc></doc>\xE2\x82\0".as_ptr() as *const ::core::ffi::c_char;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_FALSE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3854i32,
        );
    }
    if XML_ParseBuffer(g_parser, 0, XML_TRUE as ::core::ffi::c_int) != XML_STATUS_ERROR {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3857i32,
            b"Partial character in epilog not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_PARTIAL_CHAR {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3859i32,
        );
    }
}

unsafe extern "C" fn test_suspend_resume_internal_entity() {
    _check_set_test_info(
        b"test_suspend_resume_internal_entity\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3864,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ENTITY foo '<suspend>Hi<suspend>Ho</suspend></suspend>'>\n]>\n<doc>&foo;</doc>\n\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut expected1: *const XML_Char = b"Hi\0".as_ptr() as *const XML_Char;
    let mut expected2: *const XML_Char = b"HiHo\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetStartElementHandler(
        g_parser,
        Some(
            start_element_suspender
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    if XML_Parse(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_SUSPENDED
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3882i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, b"\0".as_ptr() as *const XML_Char);
    if XML_ResumeParser(g_parser) != XML_STATUS_SUSPENDED {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3885i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected1);
    if XML_ResumeParser(g_parser) != XML_STATUS_OK {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3888i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected2);
}

unsafe extern "C" fn test_suspend_resume_internal_entity_issue_629() {
    _check_set_test_info(
        b"test_suspend_resume_internal_entity_issue_629\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3893,
    );
    let text: *const ::core::ffi::c_char = b"<!DOCTYPE a [<!ENTITY e '<!--COMMENT-->a'>]><a>&e;<b>\n<aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa/></b></a>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let firstChunkSizeBytes: size_t = 54;
    let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
    XML_SetUserData(parser, parser as *mut ::core::ffi::c_void);
    XML_SetCommentHandler(
        parser,
        Some(
            suspending_comment_handler
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
        ),
    );
    if XML_Parse(
        parser,
        text,
        firstChunkSizeBytes as ::core::ffi::c_int,
        XML_FALSE as ::core::ffi::c_int,
    ) != XML_STATUS_SUSPENDED
    {
        _xml_failure(
            parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3947i32,
        );
    }
    if XML_ResumeParser(parser) != XML_STATUS_OK {
        _xml_failure(
            parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3949i32,
        );
    }
    if _XML_Parse_SINGLE_BYTES(
        parser,
        text.offset(firstChunkSizeBytes as isize),
        strlen(text).wrapping_sub(firstChunkSizeBytes) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_OK
    {
        _xml_failure(
            parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3954i32,
        );
    }
    XML_ParserFree(parser);
}

unsafe extern "C" fn test_resume_entity_with_syntax_error() {
    _check_set_test_info(
        b"test_resume_entity_with_syntax_error\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3960,
    );
    if g_chunkSize != 0 {
        return;
    }
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc [\n<!ENTITY foo '<suspend>Hi</wombat>'>\n]>\n<doc>&foo;</doc>\n\0".as_ptr()
            as *const ::core::ffi::c_char;
    XML_SetStartElementHandler(
        g_parser,
        Some(
            start_element_suspender
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut *const XML_Char,
                ) -> (),
        ),
    );
    if XML_Parse(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_SUSPENDED
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3976i32,
        );
    }
    if XML_ResumeParser(g_parser) != XML_STATUS_ERROR {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3978i32,
            b"Syntax error in entity not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_TAG_MISMATCH {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            3980i32,
        );
    }
}

unsafe extern "C" fn test_suspend_resume_parameter_entity() {
    _check_set_test_info(
        b"test_suspend_resume_parameter_entity\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        3985,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ENTITY % foo '<!ELEMENT doc (#PCDATA)*>'>\n%foo;\n]>\n<doc>Hello, world</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut expected: *const XML_Char = b"Hello, world\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetElementDeclHandler(
        g_parser,
        ::core::mem::transmute(Some(
            element_decl_suspender
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Content,
                ) -> (),
        )),
    );
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    if XML_Parse(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_SUSPENDED
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4001i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, b"\0".as_ptr() as *const XML_Char);
    if XML_ResumeParser(g_parser) != XML_STATUS_OK {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4004i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_restart_on_error() {
    _check_set_test_info(
        b"test_restart_on_error\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4010,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<$doc><doc></doc>\0".as_ptr() as *const ::core::ffi::c_char;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_ERROR
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4015i32,
            b"Invalid tag name not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_INVALID_TOKEN {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4017i32,
        );
    }
    if XML_Parse(
        g_parser,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_ERROR
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4019i32,
            b"Restarting invalid parse not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_INVALID_TOKEN {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4021i32,
        );
    }
}

unsafe extern "C" fn test_reject_lt_in_attribute_value() {
    _check_set_test_info(
        b"test_reject_lt_in_attribute_value\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4026,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc [<!ATTLIST doc a CDATA '<bar>'>]>\n<doc></doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Bad attribute default not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4031,
    );
}

unsafe extern "C" fn test_reject_unfinished_param_in_att_value() {
    _check_set_test_info(
        b"test_reject_unfinished_param_in_att_value\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4035,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc [<!ATTLIST doc a CDATA '&foo'>]>\n<doc></doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Bad attribute default not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4040,
    );
}

unsafe extern "C" fn test_trailing_cr_in_att_value() {
    _check_set_test_info(
        b"test_trailing_cr_in_att_value\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4044,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc a='value\r'/>\0".as_ptr() as *const ::core::ffi::c_char;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4049i32,
        );
    }
}

unsafe extern "C" fn test_standalone_internal_entity() {
    _check_set_test_info(
        b"test_standalone_internal_entity\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4056,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' standalone='yes' ?>\n<!DOCTYPE doc [\n  <!ELEMENT doc (#PCDATA)>\n  <!ENTITY % pe '<!ATTLIST doc att2 CDATA \"&ge;\">'>\n  <!ENTITY ge 'AttDefaultValue'>\n  %pe;\n]>\n<doc att2='any'/>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4069i32,
        );
    }
}

unsafe extern "C" fn test_skipped_external_entity() {
    _check_set_test_info(
        b"test_skipped_external_entity\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4074,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc SYSTEM 'http://example.org/'>\n<doc></doc>\n\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut test_data: ExtTest = ExtTest {
        parse_text: b"<!ELEMENT doc EMPTY>\n<!ENTITY % e2 '%e1;'>\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        encoding: ::core::ptr::null::<XML_Char>(),
        storage: ::core::ptr::null_mut::<CharData>(),
    };
    XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4086i32,
        );
    }
}

unsafe extern "C" fn test_skipped_null_loaded_ext_entity() {
    _check_set_test_info(
        b"test_skipped_null_loaded_ext_entity\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4091,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc SYSTEM 'http://example.org/one.ent'>\n<doc />\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut test_data: ExtHdlrData = ext_hdlr_data {
    parse_text:   b"<!ENTITY % pe1 SYSTEM 'http://example.org/two.ent'>\n<!ENTITY % pe2 '%pe1;'>\n%pe2;\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    handler:   Some(
            external_entity_null_loader
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    storage:   ::core::ptr::null_mut::<CharData>(),
};
    XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_oneshot_loader
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4105i32,
        );
    }
}

unsafe extern "C" fn test_skipped_unloaded_ext_entity() {
    _check_set_test_info(
        b"test_skipped_unloaded_ext_entity\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4109,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc SYSTEM 'http://example.org/one.ent'>\n<doc />\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut test_data: ExtHdlrData = ext_hdlr_data {
    parse_text:   b"<!ENTITY % pe1 SYSTEM 'http://example.org/two.ent'>\n<!ENTITY % pe2 '%pe1;'>\n%pe2;\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    handler:   None,
    storage:   ::core::ptr::null_mut::<CharData>(),
};
    XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_oneshot_loader
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4123i32,
        );
    }
}

unsafe extern "C" fn test_param_entity_with_trailing_cr() {
    _check_set_test_info(
        b"test_param_entity_with_trailing_cr\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4130,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc SYSTEM 'http://example.org/'>\n<doc/>\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut test_data: ExtTest = ExtTest {
        parse_text: b"<!ENTITY % pe '<!ATTLIST doc att CDATA \"default\">\r'>\n%pe;\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        encoding: ::core::ptr::null::<XML_Char>(),
        storage: ::core::ptr::null_mut::<CharData>(),
    };
    XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetEntityDeclHandler(
        g_parser,
        Some(
            param_entity_match_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                    *const XML_Char,
                    ::core::ffi::c_int,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> (),
        ),
    );
    param_entity_match_init(
        b"pe\0".as_ptr() as *const XML_Char,
        b"<!ATTLIST doc att CDATA \"default\">\n\0".as_ptr() as *const XML_Char,
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4148i32,
        );
    }
    let mut entity_match_flag: ::core::ffi::c_int = get_param_entity_match_flag();
    if entity_match_flag == ENTITY_MATCH_FAIL {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4151i32,
            b"Parameter entity CR->NEWLINE conversion failed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else if entity_match_flag == ENTITY_MATCH_NOT_FOUND {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4153i32,
            b"Parameter entity not parsed\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_invalid_character_entity() {
    _check_set_test_info(
        b"test_invalid_character_entity\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4159,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc [\n  <!ENTITY entity '&#x110000;'>\n]>\n<doc>&entity;</doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_BAD_CHAR_REF,
        b"Out of range character reference not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4166,
    );
}

unsafe extern "C" fn test_invalid_character_entity_2() {
    _check_set_test_info(
        b"test_invalid_character_entity_2\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4170,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc [\n  <!ENTITY entity '&#xg0;'>\n]>\n<doc>&entity;</doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Out of range character reference not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4177,
    );
}

unsafe extern "C" fn test_invalid_character_entity_3() {
    _check_set_test_info(
        b"test_invalid_character_entity_3\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4181,
    );
    let text: [::core::ffi::c_char; 125] = ::core::mem::transmute::<
        [u8; 125],
        [::core::ffi::c_char; 125],
    >(
        *b"\0<\0!\0D\0O\0C\0T\0Y\0P\0E\0 \0d\0o\0c\0 \0[\0\n\0<\0!\0E\0N\0T\0I\0T\0Y\0 \0e\0n\0t\0i\0t\0y\0 \0'\0&\x0E\x04\x0E\x08\0;\0'\0>\0\n\0]\0>\0\n\0<\0d\0o\0c\0>\0&\0e\0n\0t\0i\0t\0y\0;\0<\0/\0d\0o\0c\0>\0",
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        &raw const text as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 125]>() as ::core::ffi::c_int - 1,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_ERROR
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4197i32,
            b"Invalid start of entity name not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_UNDEFINED_ENTITY {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4199i32,
        );
    }
}

unsafe extern "C" fn test_invalid_character_entity_4() {
    _check_set_test_info(
        b"test_invalid_character_entity_4\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4203,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc [\n  <!ENTITY entity '&#1114112;'>\n]>\n<doc>&entity;</doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_BAD_CHAR_REF,
        b"Out of range character reference not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4210,
    );
}

unsafe extern "C" fn test_pi_handled_in_default() {
    _check_set_test_info(
        b"test_pi_handled_in_default\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4215,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?test processing instruction?>\n<doc/>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut expected: *const XML_Char =
        b"<?test processing instruction?>\n<doc/>\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetDefaultHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4225i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_comment_handled_in_default() {
    _check_set_test_info(
        b"test_comment_handled_in_default\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4231,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!-- This is a comment -->\n<doc/>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut expected: *const XML_Char =
        b"<!-- This is a comment -->\n<doc/>\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetDefaultHandler(
        g_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4241i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_pi_yml() {
    _check_set_test_info(
        b"test_pi_yml\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4247,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?yml something like data?><doc/>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut expected: *const XML_Char = b"yml: something like data\n\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetProcessingInstructionHandler(
        g_parser,
        Some(
            accumulate_pi_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4257i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_pi_xnl() {
    _check_set_test_info(
        b"test_pi_xnl\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4262,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?xnl nothing like data?><doc/>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut expected: *const XML_Char = b"xnl: nothing like data\n\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetProcessingInstructionHandler(
        g_parser,
        Some(
            accumulate_pi_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4272i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_pi_xmm() {
    _check_set_test_info(
        b"test_pi_xmm\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4277,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?xmm everything like data?><doc/>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut expected: *const XML_Char =
        b"xmm: everything like data\n\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetProcessingInstructionHandler(
        g_parser,
        Some(
            accumulate_pi_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4287i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_utf16_pi() {
    _check_set_test_info(
        b"test_utf16_pi\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4292,
    );
    let text: [::core::ffi::c_char; 21] = ::core::mem::transmute::<
        [u8; 21],
        [::core::ffi::c_char; 21],
    >(*b"<\0?\0\x04\x0E\x08\x0E?\0>\0<\0q\0/\0>\0\0");
    let mut expected: *const XML_Char =
        b"\xE0\xB8\x84\xE0\xB8\x88: \n\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetProcessingInstructionHandler(
        g_parser,
        Some(
            accumulate_pi_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        &raw const text as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 21]>() as ::core::ffi::c_int - 1,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4313i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_utf16_be_pi() {
    _check_set_test_info(
        b"test_utf16_be_pi\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4318,
    );
    let text: [::core::ffi::c_char; 21] = ::core::mem::transmute::<
        [u8; 21],
        [::core::ffi::c_char; 21],
    >(*b"\0<\0?\x0E\x04\x0E\x08\0?\0>\0<\0q\0/\0>\0");
    let mut expected: *const XML_Char =
        b"\xE0\xB8\x84\xE0\xB8\x88: \n\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetProcessingInstructionHandler(
        g_parser,
        Some(
            accumulate_pi_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        &raw const text as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 21]>() as ::core::ffi::c_int - 1,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4339i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_utf16_be_comment() {
    _check_set_test_info(
        b"test_utf16_be_comment\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4345,
    );
    let text: [::core::ffi::c_char; 51] =
        ::core::mem::transmute::<[u8; 51], [::core::ffi::c_char; 51]>(
            *b"\0<\0!\0-\0-\0 \0C\0o\0m\0m\0e\0n\0t\0 \0A\0 \0-\0-\0>\0\n\0<\0d\0o\0c\0/\0>\0",
        );
    let mut expected: *const XML_Char = b" Comment A \0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetCommentHandler(
        g_parser,
        Some(
            accumulate_comment
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        &raw const text as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 51]>() as ::core::ffi::c_int - 1,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4359i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_utf16_le_comment() {
    _check_set_test_info(
        b"test_utf16_le_comment\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4364,
    );
    let text: [::core::ffi::c_char; 51] =
        ::core::mem::transmute::<[u8; 51], [::core::ffi::c_char; 51]>(
            *b"<\0!\0-\0-\0 \0C\0o\0m\0m\0e\0n\0t\0 \0B\0 \0-\0-\0>\0\n\0<\0d\0o\0c\0/\0>\0\0",
        );
    let mut expected: *const XML_Char = b" Comment B \0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetCommentHandler(
        g_parser,
        Some(
            accumulate_comment
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        &raw const text as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 51]>() as ::core::ffi::c_int - 1,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4378i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_missing_encoding_conversion_fn() {
    _check_set_test_info(
        b"test_missing_encoding_conversion_fn\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4386,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='no-conv'?>\n<doc>\x81</doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        ::core::mem::transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Encoding,
                ) -> ::core::ffi::c_int,
        )),
        NULL,
    );
    _expect_failure(
        text,
        XML_ERROR_UNKNOWN_ENCODING,
        b"Encoding with missing convert() not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4398,
    );
}

unsafe extern "C" fn test_failing_encoding_conversion_fn() {
    _check_set_test_info(
        b"test_failing_encoding_conversion_fn\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4402,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='failing-conv'?>\n<doc>\x81</doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        ::core::mem::transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Encoding,
                ) -> ::core::ffi::c_int,
        )),
        NULL,
    );
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Encoding with failing convert() not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4413,
    );
}

unsafe extern "C" fn test_unknown_encoding_success() {
    _check_set_test_info(
        b"test_unknown_encoding_success\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4418,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='prefix-conv'?>\n<\x81d\x80oc>Hello, world</\x81d\x80oc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        ::core::mem::transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Encoding,
                ) -> ::core::ffi::c_int,
        )),
        NULL,
    );
    _run_character_check(
        text,
        b"Hello, world\0".as_ptr() as *const XML_Char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4424,
    );
}

unsafe extern "C" fn test_unknown_encoding_bad_name() {
    _check_set_test_info(
        b"test_unknown_encoding_bad_name\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4429,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='prefix-conv'?>\n<\xFFdoc>Hello, world</\xFFdoc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        ::core::mem::transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Encoding,
                ) -> ::core::ffi::c_int,
        )),
        NULL,
    );
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Bad name start in unknown encoding not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4435,
    );
}

unsafe extern "C" fn test_unknown_encoding_bad_name_2() {
    _check_set_test_info(
        b"test_unknown_encoding_bad_name_2\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4440,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='prefix-conv'?>\n<d\xFFoc>Hello, world</d\xFFoc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        ::core::mem::transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Encoding,
                ) -> ::core::ffi::c_int,
        )),
        NULL,
    );
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Bad name in unknown encoding not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4446,
    );
}

unsafe extern "C" fn test_unknown_encoding_long_name_1() {
    _check_set_test_info(
        b"test_unknown_encoding_long_name_1\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4453,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='prefix-conv'?>\n<abcdefghabcdefghabcdefghijkl\x80m\x80n\x80o\x80p>Hi</abcdefghabcdefghabcdefghijkl\x80m\x80n\x80o\x80p>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut expected: *const XML_Char =
        b"abcdefghabcdefghabcdefghijklmnop\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetUnknownEncodingHandler(
        g_parser,
        ::core::mem::transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Encoding,
                ) -> ::core::ffi::c_int,
        )),
        NULL,
    );
    XML_SetStartElementHandler(
        g_parser,
        Some(
            record_element_start_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4467i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_unknown_encoding_long_name_2() {
    _check_set_test_info(
        b"test_unknown_encoding_long_name_2\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4475,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='prefix-conv'?>\n<abcdefghabcdefghabcdefghijklmnop>Hi</abcdefghabcdefghabcdefghijklmnop>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut expected: *const XML_Char =
        b"abcdefghabcdefghabcdefghijklmnop\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetUnknownEncodingHandler(
        g_parser,
        ::core::mem::transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Encoding,
                ) -> ::core::ffi::c_int,
        )),
        NULL,
    );
    XML_SetStartElementHandler(
        g_parser,
        Some(
            record_element_start_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4489i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_invalid_unknown_encoding() {
    _check_set_test_info(
        b"test_invalid_unknown_encoding\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4494,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='invalid-9'?>\n<doc>Hello world</doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        ::core::mem::transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Encoding,
                ) -> ::core::ffi::c_int,
        )),
        NULL,
    );
    _expect_failure(
        text,
        XML_ERROR_UNKNOWN_ENCODING,
        b"Invalid unknown encoding not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4500,
    );
}

unsafe extern "C" fn test_unknown_ascii_encoding_ok() {
    _check_set_test_info(
        b"test_unknown_ascii_encoding_ok\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4504,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='ascii-like'?>\n<doc>Hello, world</doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        ::core::mem::transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Encoding,
                ) -> ::core::ffi::c_int,
        )),
        NULL,
    );
    _run_character_check(
        text,
        b"Hello, world\0".as_ptr() as *const XML_Char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4509,
    );
}

unsafe extern "C" fn test_unknown_ascii_encoding_fail() {
    _check_set_test_info(
        b"test_unknown_ascii_encoding_fail\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4513,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='ascii-like'?>\n<doc>Hello, \x80 world</doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        ::core::mem::transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Encoding,
                ) -> ::core::ffi::c_int,
        )),
        NULL,
    );
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Invalid character not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4519,
    );
}

unsafe extern "C" fn test_unknown_encoding_invalid_length() {
    _check_set_test_info(
        b"test_unknown_encoding_invalid_length\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4523,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='invalid-len'?>\n<doc>Hello, world</doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        ::core::mem::transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Encoding,
                ) -> ::core::ffi::c_int,
        )),
        NULL,
    );
    _expect_failure(
        text,
        XML_ERROR_UNKNOWN_ENCODING,
        b"Invalid unknown encoding not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4529,
    );
}

unsafe extern "C" fn test_unknown_encoding_invalid_topbit() {
    _check_set_test_info(
        b"test_unknown_encoding_invalid_topbit\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4533,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='invalid-a'?>\n<doc>Hello, world</doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        ::core::mem::transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Encoding,
                ) -> ::core::ffi::c_int,
        )),
        NULL,
    );
    _expect_failure(
        text,
        XML_ERROR_UNKNOWN_ENCODING,
        b"Invalid unknown encoding not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4539,
    );
}

unsafe extern "C" fn test_unknown_encoding_invalid_surrogate() {
    _check_set_test_info(
        b"test_unknown_encoding_invalid_surrogate\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4543,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='invalid-surrogate'?>\n<doc>Hello, \x82 world</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        ::core::mem::transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Encoding,
                ) -> ::core::ffi::c_int,
        )),
        NULL,
    );
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Invalid unknown encoding not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4549,
    );
}

unsafe extern "C" fn test_unknown_encoding_invalid_high() {
    _check_set_test_info(
        b"test_unknown_encoding_invalid_high\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4553,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='invalid-high'?>\n<doc>Hello, world</doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        ::core::mem::transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Encoding,
                ) -> ::core::ffi::c_int,
        )),
        NULL,
    );
    _expect_failure(
        text,
        XML_ERROR_UNKNOWN_ENCODING,
        b"Invalid unknown encoding not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4559,
    );
}

unsafe extern "C" fn test_unknown_encoding_invalid_attr_value() {
    _check_set_test_info(
        b"test_unknown_encoding_invalid_attr_value\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4563,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='prefix-conv'?>\n<doc attr='\xFF0'/>\0".as_ptr()
            as *const ::core::ffi::c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        ::core::mem::transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Encoding,
                ) -> ::core::ffi::c_int,
        )),
        NULL,
    );
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Invalid attribute valid not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4569,
    );
}

unsafe extern "C" fn test_unknown_encoding_user_data_primary() {
    _check_set_test_info(
        b"test_unknown_encoding_user_data_primary\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4573,
    );
    let text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='x-unk'?>\n<root />\n\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
    XML_SetUnknownEncodingHandler(
        parser,
        ::core::mem::transmute(Some(
            user_data_checking_unknown_encoding_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Encoding,
                ) -> ::core::ffi::c_int,
        )),
        0xc0ffeei32 as *mut ::core::ffi::c_void,
    );
    if !(_XML_Parse_SINGLE_BYTES(parser, text, strlen(text) as ::core::ffi::c_int, 1)
        == XML_STATUS_OK)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            4583i32,
            b"check failed: _XML_Parse_SINGLE_BYTES(parser, text, (int)strlen(text), XML_TRUE) == XML_STATUS_OK\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
    XML_ParserFree(parser);
}

unsafe extern "C" fn test_unknown_encoding_user_data_secondary() {
    _check_set_test_info(
        b"test_unknown_encoding_user_data_secondary\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4589,
    );
    let text_main: *const ::core::ffi::c_char =
        b"<!DOCTYPE r [\n  <!ENTITY ext SYSTEM 'ext.ent'>\n]>\n<r>&ext;</r>\n\0".as_ptr()
            as *const ::core::ffi::c_char;
    let text_external: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='x-unk'?>\n<e>data</e>\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut test_data: ExtTest2 = ExtTest2 {
        parse_text: text_external,
        parse_len: strlen(text_external) as ::core::ffi::c_int,
        encoding: ::core::ptr::null::<XML_Char>(),
        storage: ::core::ptr::null_mut::<CharData>(),
    };
    let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
    XML_SetExternalEntityRefHandler(
        parser,
        Some(
            external_entity_loader2
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetUnknownEncodingHandler(
        parser,
        ::core::mem::transmute(Some(
            user_data_checking_unknown_encoding_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Encoding,
                ) -> ::core::ffi::c_int,
        )),
        0xc0ffeei32 as *mut ::core::ffi::c_void,
    );
    XML_SetUserData(parser, &raw mut test_data as *mut ::core::ffi::c_void);
    if !(_XML_Parse_SINGLE_BYTES(
        parser,
        text_main,
        strlen(text_main) as ::core::ffi::c_int,
        1,
    ) == XML_STATUS_OK)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            4607i32,
            b"check failed: _XML_Parse_SINGLE_BYTES(parser, text_main, (int)strlen(text_main), XML_TRUE) == XML_STATUS_OK\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
    XML_ParserFree(parser);
}

unsafe extern "C" fn test_ext_entity_latin1_utf16le_bom() {
    _check_set_test_info(
        b"test_ext_entity_latin1_utf16le_bom\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4617,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut test_data: ExtTest2 = ExtTest2 {
        parse_text: b"\xFF\xFEL \0".as_ptr() as *const ::core::ffi::c_char,
        parse_len: 4,
        encoding: b"iso-8859-1\0".as_ptr() as *const XML_Char,
        storage: ::core::ptr::null_mut::<CharData>(),
    };
    let mut expected: *const XML_Char = b"\xC3\xBF\xC3\xBEL \0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    test_data.storage = &raw mut storage;
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader2
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            ext2_accumulate_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4643i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_ext_entity_latin1_utf16be_bom() {
    _check_set_test_info(
        b"test_ext_entity_latin1_utf16be_bom\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4648,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut test_data: ExtTest2 = ExtTest2 {
        parse_text: b"\xFE\xFF L\0".as_ptr() as *const ::core::ffi::c_char,
        parse_len: 4,
        encoding: b"iso-8859-1\0".as_ptr() as *const XML_Char,
        storage: ::core::ptr::null_mut::<CharData>(),
    };
    let mut expected: *const XML_Char = b"\xC3\xBE\xC3\xBF L\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    test_data.storage = &raw mut storage;
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader2
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            ext2_accumulate_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4674i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_ext_entity_latin1_utf16le_bom2() {
    _check_set_test_info(
        b"test_ext_entity_latin1_utf16le_bom2\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4683,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut test_data: ExtTest2 = ExtTest2 {
        parse_text: b"\xFF\xFEL \0".as_ptr() as *const ::core::ffi::c_char,
        parse_len: 4,
        encoding: b"iso-8859-1\0".as_ptr() as *const XML_Char,
        storage: ::core::ptr::null_mut::<CharData>(),
    };
    let mut expected: *const XML_Char = b"\xC3\xBF\xC3\xBEL \0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    test_data.storage = &raw mut storage;
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader2
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            ext2_accumulate_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4709i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_ext_entity_latin1_utf16be_bom2() {
    _check_set_test_info(
        b"test_ext_entity_latin1_utf16be_bom2\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4714,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut test_data: ExtTest2 = ExtTest2 {
        parse_text: b"\xFE\xFF L\0".as_ptr() as *const ::core::ffi::c_char,
        parse_len: 4,
        encoding: b"iso-8859-1\0".as_ptr() as *const XML_Char,
        storage: ::core::ptr::null_mut::<CharData>(),
    };
    let mut expected: *const XML_Char = b"\xC3\xBE\xC3\xBF L\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    test_data.storage = &raw mut storage;
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader2
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            ext2_accumulate_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4740i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_ext_entity_utf16_be() {
    _check_set_test_info(
        b"test_ext_entity_utf16_be\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4746,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut test_data: ExtTest2 = ExtTest2 {
        parse_text: b"<\0e\0/\0>\0\0".as_ptr() as *const ::core::ffi::c_char,
        parse_len: 8,
        encoding: b"utf-16be\0".as_ptr() as *const XML_Char,
        storage: ::core::ptr::null_mut::<CharData>(),
    };
    let mut expected: *const XML_Char =
        b"\xE3\xB0\x80\xE6\x94\x80\xE2\xBC\x80\xE3\xB8\x80\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    test_data.storage = &raw mut storage;
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader2
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            ext2_accumulate_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4769i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_ext_entity_utf16_le() {
    _check_set_test_info(
        b"test_ext_entity_utf16_le\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4775,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut test_data: ExtTest2 = ExtTest2 {
        parse_text: b"\0<\0e\0/\0>\0".as_ptr() as *const ::core::ffi::c_char,
        parse_len: 8,
        encoding: b"utf-16le\0".as_ptr() as *const XML_Char,
        storage: ::core::ptr::null_mut::<CharData>(),
    };
    let mut expected: *const XML_Char =
        b"\xE3\xB0\x80\xE6\x94\x80\xE2\xBC\x80\xE3\xB8\x80\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    test_data.storage = &raw mut storage;
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader2
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            ext2_accumulate_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4798i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_ext_entity_utf16_unknown() {
    _check_set_test_info(
        b"test_ext_entity_utf16_unknown\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4810,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut test_data: ExtFaults2 = ExtFaults2 {
        parse_text: b"a\0b\0c\0\0".as_ptr() as *const ::core::ffi::c_char,
        parse_len: 6,
        fail_text: b"Invalid character in entity not faulted\0".as_ptr()
            as *const ::core::ffi::c_char,
        encoding: ::core::ptr::null::<XML_Char>(),
        error: XML_ERROR_INVALID_TOKEN,
    };
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_faulter2
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
    _expect_failure(
        text,
        XML_ERROR_EXTERNAL_ENTITY_HANDLING,
        b"Invalid character should not have been accepted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4822,
    );
}

unsafe extern "C" fn test_ext_entity_utf8_non_bom() {
    _check_set_test_info(
        b"test_ext_entity_utf8_non_bom\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4827,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut test_data: ExtTest2 = ExtTest2 {
        parse_text: b"\xEF\xBB\x80\0".as_ptr() as *const ::core::ffi::c_char,
        parse_len: 3,
        encoding: ::core::ptr::null::<XML_Char>(),
        storage: ::core::ptr::null_mut::<CharData>(),
    };
    let mut expected: *const XML_Char = b"\xEF\xBB\x80\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    test_data.storage = &raw mut storage;
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader2
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            ext2_accumulate_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4849i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_utf8_in_cdata_section() {
    _check_set_test_info(
        b"test_utf8_in_cdata_section\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4855,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc><![CDATA[one \xC3\xA9 two]]></doc>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut expected: *const XML_Char = b"one \xC3\xA9 two\0".as_ptr() as *const XML_Char;
    _run_character_check(
        text,
        expected,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4863,
    );
}

unsafe extern "C" fn test_utf8_in_cdata_section_2() {
    _check_set_test_info(
        b"test_utf8_in_cdata_section_2\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4868,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc><![CDATA[\xC3\xA9]\xC3\xA9two]]></doc>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut expected: *const XML_Char = b"\xC3\xA9]\xC3\xA9two\0".as_ptr() as *const XML_Char;
    _run_character_check(
        text,
        expected,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4876,
    );
}

unsafe extern "C" fn test_utf8_in_start_tags() {
    _check_set_test_info(
        b"test_utf8_in_start_tags\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4880,
    );
    let mut cases: [test_case; 24] = [
        test_case {
            goodName: true_0 != 0,
            goodNameStart: true_0 != 0,
            tagName: b":\0".as_ptr() as *const ::core::ffi::c_char,
        },
        test_case {
            goodName: false_0 != 0,
            goodNameStart: false_0 != 0,
            tagName: b"\xBA\0".as_ptr() as *const ::core::ffi::c_char,
        },
        test_case {
            goodName: true_0 != 0,
            goodNameStart: false_0 != 0,
            tagName: b"9\0".as_ptr() as *const ::core::ffi::c_char,
        },
        test_case {
            goodName: false_0 != 0,
            goodNameStart: false_0 != 0,
            tagName: b"\xB9\0".as_ptr() as *const ::core::ffi::c_char,
        },
        test_case {
            goodName: true_0 != 0,
            goodNameStart: true_0 != 0,
            tagName: b"\xDB\xA5\0".as_ptr() as *const ::core::ffi::c_char,
        },
        test_case {
            goodName: false_0 != 0,
            goodNameStart: false_0 != 0,
            tagName: b"\x9B\xA5\0".as_ptr() as *const ::core::ffi::c_char,
        },
        test_case {
            goodName: false_0 != 0,
            goodNameStart: false_0 != 0,
            tagName: b"\xDB%\0".as_ptr() as *const ::core::ffi::c_char,
        },
        test_case {
            goodName: false_0 != 0,
            goodNameStart: false_0 != 0,
            tagName: b"\xDB\xE5\0".as_ptr() as *const ::core::ffi::c_char,
        },
        test_case {
            goodName: true_0 != 0,
            goodNameStart: false_0 != 0,
            tagName: b"\xCC\x81\0".as_ptr() as *const ::core::ffi::c_char,
        },
        test_case {
            goodName: false_0 != 0,
            goodNameStart: false_0 != 0,
            tagName: b"\x8C\x81\0".as_ptr() as *const ::core::ffi::c_char,
        },
        test_case {
            goodName: false_0 != 0,
            goodNameStart: false_0 != 0,
            tagName: b"\xCC\x01\0".as_ptr() as *const ::core::ffi::c_char,
        },
        test_case {
            goodName: false_0 != 0,
            goodNameStart: false_0 != 0,
            tagName: b"\xCC\xC1\0".as_ptr() as *const ::core::ffi::c_char,
        },
        test_case {
            goodName: true_0 != 0,
            goodNameStart: true_0 != 0,
            tagName: b"\xE0\xA4\x85\0".as_ptr() as *const ::core::ffi::c_char,
        },
        test_case {
            goodName: false_0 != 0,
            goodNameStart: false_0 != 0,
            tagName: b"\xA0\xA4\x85\0".as_ptr() as *const ::core::ffi::c_char,
        },
        test_case {
            goodName: false_0 != 0,
            goodNameStart: false_0 != 0,
            tagName: b"\xE0$\x85\0".as_ptr() as *const ::core::ffi::c_char,
        },
        test_case {
            goodName: false_0 != 0,
            goodNameStart: false_0 != 0,
            tagName: b"\xE0\xE4\x85\0".as_ptr() as *const ::core::ffi::c_char,
        },
        test_case {
            goodName: false_0 != 0,
            goodNameStart: false_0 != 0,
            tagName: b"\xE0\xA4\x05\0".as_ptr() as *const ::core::ffi::c_char,
        },
        test_case {
            goodName: false_0 != 0,
            goodNameStart: false_0 != 0,
            tagName: b"\xE0\xA4\xC5\0".as_ptr() as *const ::core::ffi::c_char,
        },
        test_case {
            goodName: true_0 != 0,
            goodNameStart: false_0 != 0,
            tagName: b"\xE0\xA4\x81\0".as_ptr() as *const ::core::ffi::c_char,
        },
        test_case {
            goodName: false_0 != 0,
            goodNameStart: false_0 != 0,
            tagName: b"\xA0\xA4\x81\0".as_ptr() as *const ::core::ffi::c_char,
        },
        test_case {
            goodName: false_0 != 0,
            goodNameStart: false_0 != 0,
            tagName: b"\xE0$\x81\0".as_ptr() as *const ::core::ffi::c_char,
        },
        test_case {
            goodName: false_0 != 0,
            goodNameStart: false_0 != 0,
            tagName: b"\xE0\xE4\x81\0".as_ptr() as *const ::core::ffi::c_char,
        },
        test_case {
            goodName: false_0 != 0,
            goodNameStart: false_0 != 0,
            tagName: b"\xE0\xA4\x01\0".as_ptr() as *const ::core::ffi::c_char,
        },
        test_case {
            goodName: false_0 != 0,
            goodNameStart: false_0 != 0,
            tagName: b"\xE0\xA4\xC1\0".as_ptr() as *const ::core::ffi::c_char,
        },
    ];
    let atNameStart: [bool; 2] = [true_0 != 0, false_0 != 0];
    let mut i: size_t = 0;
    let mut doc: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut failCount: size_t = 0;
    if g_reparseDeferralEnabledDefault != 0 {
        return;
    }
    while i
        < (::core::mem::size_of::<[test_case; 24]>())
            .wrapping_div(::core::mem::size_of::<test_case>())
    {
        let mut j: size_t = 0;
        while j < (::core::mem::size_of::<[bool; 2]>()).wrapping_div(::core::mem::size_of::<bool>())
        {
            let expectedSuccess: bool = if atNameStart[j] as ::core::ffi::c_int != 0 {
                cases[i].goodNameStart as ::core::ffi::c_int
            } else {
                cases[i].goodName as ::core::ffi::c_int
            } != 0;
            snprintf(
                &raw mut doc as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
                b"<%s%s><!--\0".as_ptr() as *const ::core::ffi::c_char,
                if atNameStart[j] as ::core::ffi::c_int != 0 {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"a\0".as_ptr() as *const ::core::ffi::c_char
                },
                cases[i].tagName,
            );
            let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
            let status: XML_Status = _XML_Parse_SINGLE_BYTES(
                parser,
                &raw mut doc as *mut ::core::ffi::c_char,
                strlen(&raw mut doc as *mut ::core::ffi::c_char) as ::core::ffi::c_int,
                XML_FALSE as ::core::ffi::c_int,
            );
            let mut success: bool = true_0 != 0;
            if (status == XML_STATUS_OK) as ::core::ffi::c_int
                != expectedSuccess as ::core::ffi::c_int
            {
                success = false_0 != 0;
            }
            if status == XML_STATUS_ERROR && XML_GetErrorCode(parser) != XML_ERROR_INVALID_TOKEN {
                success = false_0 != 0;
            }
            if !success {
                fprintf(
                    stderr,
                    b"FAIL case %2u (%sat name start, %u-byte sequence, error code %d)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (i as ::core::ffi::c_uint).wrapping_add(1u32),
                    if atNameStart[j] as ::core::ffi::c_int != 0 {
                        b"    \0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"not \0".as_ptr() as *const ::core::ffi::c_char
                    },
                    strlen(cases[i].tagName) as ::core::ffi::c_uint,
                    XML_GetErrorCode(parser),
                );
                failCount = failCount.wrapping_add(1);
            }
            XML_ParserFree(parser);
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    if failCount > 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4981i32,
            b"UTF-8 regression detected\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_trailing_spaces_in_elements() {
    _check_set_test_info(
        b"test_trailing_spaces_in_elements\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        4987,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc   >Hi</doc >\0".as_ptr() as *const ::core::ffi::c_char;
    let mut expected: *const XML_Char = b"doc/doc\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetElementHandler(
        g_parser,
        Some(
            record_element_start_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut *const XML_Char,
                ) -> (),
        ),
        Some(
            record_element_end_handler
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            4998i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_utf16_attribute() {
    _check_set_test_info(
        b"test_utf16_attribute\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5003,
    );
    let text: [::core::ffi::c_char; 23] = ::core::mem::transmute::<
        [u8; 23],
        [::core::ffi::c_char; 23],
    >(*b"<\0d\0 \0\x04\x0E\x08\x0E=\0'\0a\0'\0/\0>\0\0");
    let mut expected: *const XML_Char = b"a\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetStartElementHandler(
        g_parser,
        Some(
            accumulate_attribute
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        &raw const text as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 23]>() as ::core::ffi::c_int - 1,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5018i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_utf16_second_attr() {
    _check_set_test_info(
        b"test_utf16_second_attr\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5023,
    );
    let text: [::core::ffi::c_char; 35] =
        ::core::mem::transmute::<[u8; 35], [::core::ffi::c_char; 35]>(
            *b"<\0d\0 \0a\0=\0'\x001\0'\0 \0\x04\x0E\x08\x0E=\0'\x002\0'\0/\0>\0\0",
        );
    let mut expected: *const XML_Char = b"1\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetStartElementHandler(
        g_parser,
        Some(
            accumulate_attribute
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        &raw const text as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 35]>() as ::core::ffi::c_int - 1,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5038i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_attr_after_solidus() {
    _check_set_test_info(
        b"test_attr_after_solidus\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5043,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc attr1='a' / attr2='b'>\0".as_ptr() as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Misplaced / not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5046,
    );
}

unsafe extern "C" fn test_utf16_pe() {
    _check_set_test_info(
        b"test_utf16_pe\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5050,
    );
    let text: [::core::ffi::c_char; 155] = ::core::mem::transmute::<
        [u8; 155],
        [::core::ffi::c_char; 155],
    >(
        *b"\0<\0!\0D\0O\0C\0T\0Y\0P\0E\0 \0d\0o\0c\0 \0[\0\n\0<\0!\0E\0N\0T\0I\0T\0Y\0 \0%\0 \x0E\x04\x0E\x08\0 \0'\0<\0!\0E\0L\0E\0M\0E\0N\0T\0 \0d\0o\0c\0 \0(\0#\0P\0C\0D\0A\0T\0A\0)\0>\0'\0>\0\n\0%\x0E\x04\x0E\x08\0;\0\n\0]\0>\0\n\0<\0d\0o\0c\0>\0<\0/\0d\0o\0c\0>\0",
    );
    let mut expected: *const XML_Char =
        b"\xE0\xB8\x84\xE0\xB8\x88=<!ELEMENT doc (#PCDATA)>\n\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    XML_SetEntityDeclHandler(
        g_parser,
        Some(
            accumulate_entity_decl
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                    *const XML_Char,
                    ::core::ffi::c_int,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        &raw const text as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 155]>() as ::core::ffi::c_int - 1,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5080i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_bad_attr_desc_keyword() {
    _check_set_test_info(
        b"test_bad_attr_desc_keyword\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5086,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc [\n  <!ATTLIST doc attr CDATA #!IMPLIED>\n]>\n<doc />\0".as_ptr()
            as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Bad keyword !IMPLIED not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5093,
    );
}

unsafe extern "C" fn test_bad_attr_desc_keyword_utf16() {
    _check_set_test_info(
        b"test_bad_attr_desc_keyword_utf16\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5101,
    );
    let text: [::core::ffi::c_char; 91] = ::core::mem::transmute::<
        [u8; 91],
        [::core::ffi::c_char; 91],
    >(
        *b"\0<\0!\0D\0O\0C\0T\0Y\0P\0E\0 \0d\0 \0[\0\n\0<\0!\0A\0T\0T\0L\0I\0S\0T\0 \0d\0 \0a\0 \0C\0D\0A\0T\0A\0 \0#\x0E\x04\x0E\x08\0>\0\n\0]\0>\0<\0d\0/\0>\0",
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        &raw const text as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 91]>() as ::core::ffi::c_int - 1,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_ERROR
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5117i32,
            b"Invalid UTF16 attribute keyword not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_SYNTAX {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5119i32,
        );
    }
}

unsafe extern "C" fn test_bad_doctype() {
    _check_set_test_info(
        b"test_bad_doctype\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5126,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='prefix-conv'?>\n<!DOCTYPE doc [ \x80D ]><doc/>\0".as_ptr()
            as *const ::core::ffi::c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        ::core::mem::transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Encoding,
                ) -> ::core::ffi::c_int,
        )),
        NULL,
    );
    _expect_failure(
        text,
        XML_ERROR_SYNTAX,
        b"Invalid bytes in DOCTYPE not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5132,
    );
}

unsafe extern "C" fn test_bad_doctype_utf8() {
    _check_set_test_info(
        b"test_bad_doctype_utf8\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5136,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE \xDB%doc><doc/>\0".as_ptr() as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Invalid UTF-8 in DOCTYPE not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5140,
    );
}

unsafe extern "C" fn test_bad_doctype_utf16() {
    _check_set_test_info(
        b"test_bad_doctype_utf16\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5144,
    );
    let text: [::core::ffi::c_char; 53] = ::core::mem::transmute::<
        [u8; 53],
        [::core::ffi::c_char; 53],
    >(
        *b"\0<\0!\0D\0O\0C\0T\0Y\0P\0E\0 \0d\0o\0c\0 \0[\0 \x06\xF2\0 \0]\0>\0<\0d\0o\0c\0/\0>\0",
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        &raw const text as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 53]>() as ::core::ffi::c_int - 1,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_ERROR
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5157i32,
            b"Invalid bytes in DOCTYPE not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_GetErrorCode(g_parser) != XML_ERROR_SYNTAX {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5159i32,
        );
    }
}

unsafe extern "C" fn test_bad_doctype_plus() {
    _check_set_test_info(
        b"test_bad_doctype_plus\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5163,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE 1+ [ <!ENTITY foo 'bar'> ]>\n<1+>&foo;</1+>\0".as_ptr()
            as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"'+' in document name not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5168,
    );
}

unsafe extern "C" fn test_bad_doctype_star() {
    _check_set_test_info(
        b"test_bad_doctype_star\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5172,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE 1* [ <!ENTITY foo 'bar'> ]>\n<1*>&foo;</1*>\0".as_ptr()
            as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"'*' in document name not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5177,
    );
}

unsafe extern "C" fn test_bad_doctype_query() {
    _check_set_test_info(
        b"test_bad_doctype_query\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5181,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE 1? [ <!ENTITY foo 'bar'> ]>\n<1?>&foo;</1?>\0".as_ptr()
            as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"'?' in document name not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5186,
    );
}

unsafe extern "C" fn test_unknown_encoding_bad_ignore() {
    _check_set_test_info(
        b"test_unknown_encoding_bad_ignore\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5190,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='prefix-conv'?><!DOCTYPE doc SYSTEM 'foo'><doc><e>&entity;</e></doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut fault: ExtFaults = ext_faults {
        parse_text: b"<![IGNORE[<!ELEMENT \xFFG (#PCDATA)*>]]>\0".as_ptr()
            as *const ::core::ffi::c_char,
        fail_text: b"Invalid character not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        encoding: b"prefix-conv\0".as_ptr() as *const XML_Char,
        error: XML_ERROR_INVALID_TOKEN,
    };
    XML_SetUnknownEncodingHandler(
        g_parser,
        ::core::mem::transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Encoding,
                ) -> ::core::ffi::c_int,
        )),
        NULL,
    );
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_faulter
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetUserData(g_parser, &raw mut fault as *mut ::core::ffi::c_void);
    _expect_failure(
        text,
        XML_ERROR_EXTERNAL_ENTITY_HANDLING,
        b"Bad IGNORE section with unknown encoding not failed\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5203,
    );
}

unsafe extern "C" fn test_entity_in_utf16_be_attr() {
    _check_set_test_info(
        b"test_entity_in_utf16_be_attr\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5207,
    );
    let text: [::core::ffi::c_char; 55] = ::core::mem::transmute::<
        [u8; 55],
        [::core::ffi::c_char; 55],
    >(
        *b"\0<\0e\0 \0a\0=\0'\0&\0#\x002\x002\08\0;\0 \0&\0#\0x\x000\x000\0E\x004\0;\0'\0>\0<\0/\0e\0>\0",
    );
    let mut expected: *const XML_Char = b"\xC3\xA4 \xC3\xA4\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    XML_SetStartElementHandler(
        g_parser,
        Some(
            accumulate_attribute
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut *const XML_Char,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        &raw const text as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 55]>() as ::core::ffi::c_int - 1,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5224i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_entity_in_utf16_le_attr() {
    _check_set_test_info(
        b"test_entity_in_utf16_le_attr\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5229,
    );
    let text: [::core::ffi::c_char; 55] = ::core::mem::transmute::<
        [u8; 55],
        [::core::ffi::c_char; 55],
    >(
        *b"<\0e\0 \0a\0=\0'\0&\0#\x002\x002\08\0;\0 \0&\0#\0x\x000\x000\0E\x004\0;\0'\0>\0<\0/\0e\0>\0\0",
    );
    let mut expected: *const XML_Char = b"\xC3\xA4 \xC3\xA4\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    XML_SetStartElementHandler(
        g_parser,
        Some(
            accumulate_attribute
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut *const XML_Char,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        &raw const text as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 55]>() as ::core::ffi::c_int - 1,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5246i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_entity_public_utf16_be() {
    _check_set_test_info(
        b"test_entity_public_utf16_be\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5251,
    );
    let text: [::core::ffi::c_char; 137] = ::core::mem::transmute::<
        [u8; 137],
        [::core::ffi::c_char; 137],
    >(
        *b"\0<\0!\0D\0O\0C\0T\0Y\0P\0E\0 \0d\0 \0[\0\n\0<\0!\0E\0N\0T\0I\0T\0Y\0 \0%\0 \0e\0 \0P\0U\0B\0L\0I\0C\0 \0'\0f\0o\0o\0'\0 \0'\0b\0a\0r\0.\0e\0n\0t\0'\0>\0\n\0%\0e\0;\0\n\0]\0>\0\n\0<\0d\0>\0&\0j\0;\0<\0/\0d\0>\0",
    );
    let mut test_data: ExtTest2 = ExtTest2 {
        parse_text: b"\0<\0!\0E\0N\0T\0I\0T\0Y\0 \0j\0 \0'\0b\0a\0z\0'\0>\0".as_ptr()
            as *const ::core::ffi::c_char,
        parse_len: 34,
        encoding: ::core::ptr::null::<XML_Char>(),
        storage: ::core::ptr::null_mut::<CharData>(),
    };
    let mut expected: *const XML_Char = b"baz\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    test_data.storage = &raw mut storage;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader2
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            ext2_accumulate_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        &raw const text as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 137]>() as ::core::ffi::c_int - 1,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5278i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_entity_public_utf16_le() {
    _check_set_test_info(
        b"test_entity_public_utf16_le\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5283,
    );
    let text: [::core::ffi::c_char; 137] = ::core::mem::transmute::<
        [u8; 137],
        [::core::ffi::c_char; 137],
    >(
        *b"<\0!\0D\0O\0C\0T\0Y\0P\0E\0 \0d\0 \0[\0\n\0<\0!\0E\0N\0T\0I\0T\0Y\0 \0%\0 \0e\0 \0P\0U\0B\0L\0I\0C\0 \0'\0f\0o\0o\0'\0 \0'\0b\0a\0r\0.\0e\0n\0t\0'\0>\0\n\0%\0e\0;\0\n\0]\0>\0\n\0<\0d\0>\0&\0j\0;\0<\0/\0d\0>\0\0",
    );
    let mut test_data: ExtTest2 = ExtTest2 {
        parse_text: b"<\0!\0E\0N\0T\0I\0T\0Y\0 \0j\0 \0'\0b\0a\0z\0'\0>\0\0".as_ptr()
            as *const ::core::ffi::c_char,
        parse_len: 34,
        encoding: ::core::ptr::null::<XML_Char>(),
        storage: ::core::ptr::null_mut::<CharData>(),
    };
    let mut expected: *const XML_Char = b"baz\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    test_data.storage = &raw mut storage;
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_loader2
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
    XML_SetCharacterDataHandler(
        g_parser,
        Some(
            ext2_accumulate_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        &raw const text as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 137]>() as ::core::ffi::c_int - 1,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5310i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_short_doctype() {
    _check_set_test_info(
        b"test_short_doctype\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5318,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc></doc>\0".as_ptr() as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"DOCTYPE without subset not rejected\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5321,
    );
}

unsafe extern "C" fn test_short_doctype_2() {
    _check_set_test_info(
        b"test_short_doctype_2\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5325,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc PUBLIC></doc>\0".as_ptr() as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_SYNTAX,
        b"DOCTYPE without Public ID not rejected\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5328,
    );
}

unsafe extern "C" fn test_short_doctype_3() {
    _check_set_test_info(
        b"test_short_doctype_3\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5332,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc SYSTEM></doc>\0".as_ptr() as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_SYNTAX,
        b"DOCTYPE without System ID not rejected\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5335,
    );
}

unsafe extern "C" fn test_long_doctype() {
    _check_set_test_info(
        b"test_long_doctype\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5339,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc PUBLIC 'foo' 'bar' 'baz'></doc>\0".as_ptr() as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_SYNTAX,
        b"DOCTYPE with extra ID not rejected\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5341,
    );
}

unsafe extern "C" fn test_bad_entity() {
    _check_set_test_info(
        b"test_bad_entity\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5345,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc [\n  <!ENTITY foo PUBLIC>\n]>\n<doc/>\0".as_ptr()
            as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_SYNTAX,
        b"ENTITY without Public ID is not rejected\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5351,
    );
}

unsafe extern "C" fn test_bad_entity_2() {
    _check_set_test_info(
        b"test_bad_entity_2\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5356,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc [\n  <!ENTITY % foo bar>\n]>\n<doc/>\0".as_ptr()
            as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_SYNTAX,
        b"ENTITY without Public ID is not rejected\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5362,
    );
}

unsafe extern "C" fn test_bad_entity_3() {
    _check_set_test_info(
        b"test_bad_entity_3\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5366,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc [\n  <!ENTITY % foo PUBLIC>\n]>\n<doc/>\0".as_ptr()
            as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_SYNTAX,
        b"Parameter ENTITY without Public ID is not rejected\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5372,
    );
}

unsafe extern "C" fn test_bad_entity_4() {
    _check_set_test_info(
        b"test_bad_entity_4\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5376,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc [\n  <!ENTITY % foo SYSTEM>\n]>\n<doc/>\0".as_ptr()
            as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_SYNTAX,
        b"Parameter ENTITY without Public ID is not rejected\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5382,
    );
}

unsafe extern "C" fn test_bad_notation() {
    _check_set_test_info(
        b"test_bad_notation\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5386,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc [\n  <!NOTATION n SYSTEM>\n]>\n<doc/>\0".as_ptr()
            as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_SYNTAX,
        b"Notation without System ID is not rejected\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5392,
    );
}

unsafe extern "C" fn test_default_doctype_handler() {
    _check_set_test_info(
        b"test_default_doctype_handler\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5397,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc PUBLIC 'pubname' 'test.dtd' [\n  <!ENTITY foo 'bar'>\n]>\n<doc>&foo;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut test_data: [DefaultCheck; 3] = [
        default_check {
            expected: b"'pubname'\0".as_ptr() as *const XML_Char,
            expectedLen: 9,
            seen: XML_FALSE,
        },
        default_check {
            expected: b"'test.dtd'\0".as_ptr() as *const XML_Char,
            expectedLen: 10,
            seen: XML_FALSE,
        },
        default_check {
            expected: ::core::ptr::null::<XML_Char>(),
            expectedLen: 0,
            seen: XML_FALSE,
        },
    ];
    let mut i: ::core::ffi::c_int = 0;
    XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
    XML_SetDefaultHandler(
        g_parser,
        Some(
            checking_default_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetEntityDeclHandler(
        g_parser,
        Some(
            dummy_entity_decl_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                    *const XML_Char,
                    ::core::ffi::c_int,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5412i32,
        );
    }
    i = 0;
    while !test_data[i as usize].expected.is_null() {
        if test_data[i as usize].seen == 0 {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                5415i32,
                b"Default handler not run for public !DOCTYPE\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        i += 1;
    }
}

unsafe extern "C" fn test_empty_element_abort() {
    _check_set_test_info(
        b"test_empty_element_abort\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5419,
    );
    let mut text: *const ::core::ffi::c_char = b"<abort/>\0".as_ptr() as *const ::core::ffi::c_char;
    XML_SetStartElementHandler(
        g_parser,
        Some(
            start_element_suspender
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut *const XML_Char,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) != XML_STATUS_ERROR
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5425i32,
            b"Expected to error on abort\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_pool_integrity_with_unfinished_attr() {
    _check_set_test_info(
        b"test_pool_integrity_with_unfinished_attr\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5432,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='UTF-8'?>\n<!DOCTYPE foo [\n<!ELEMENT foo ANY>\n<!ENTITY % entp SYSTEM \"external.dtd\">\n%entp;\n]>\n<a></a>\n\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut expected: *const XML_Char = b"COMMENT\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_unfinished_attlist
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    XML_SetAttlistDeclHandler(
        g_parser,
        Some(
            dummy_attlist_decl_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetCommentHandler(
        g_parser,
        Some(
            accumulate_comment
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
        ),
    );
    XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5451i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
}

unsafe extern "C" fn test_entity_ref_no_elements() {
    _check_set_test_info(
        b"test_entity_ref_no_elements\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5457,
    );
    let text: *const ::core::ffi::c_char = b"<!DOCTYPE foo [\n<!ENTITY e1 \"test\">\n]> <foo>&e1;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
    if !(_XML_Parse_SINGLE_BYTES(parser, text, strlen(text) as ::core::ffi::c_int, 1)
        == XML_STATUS_ERROR)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            5464i32,
            b"check failed: _XML_Parse_SINGLE_BYTES(parser, text, (int)strlen(text), XML_TRUE) == XML_STATUS_ERROR\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !(XML_GetErrorCode(parser) == XML_ERROR_NO_ELEMENTS) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5465i32,
            b"check failed: XML_GetErrorCode(parser) == XML_ERROR_NO_ELEMENTS\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    XML_ParserFree(parser);
}

unsafe extern "C" fn test_deep_nested_entity() {
    _check_set_test_info(
        b"test_deep_nested_entity\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5471,
    );
    let N_LINES: size_t = 60000;
    let SIZE_PER_LINE: size_t = 50;
    let text: *mut ::core::ffi::c_char =
        malloc(N_LINES.wrapping_add(4usize).wrapping_mul(SIZE_PER_LINE))
            as *mut ::core::ffi::c_char;
    if text.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5477i32,
            b"malloc failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut textPtr: *mut ::core::ffi::c_char = text;
    textPtr = textPtr.offset(snprintf(
        textPtr,
        SIZE_PER_LINE,
        b"<!DOCTYPE foo [\n\t<!ENTITY s0 'deepText'>\n\0".as_ptr() as *const ::core::ffi::c_char,
    ) as isize);
    let mut i: size_t = 1;
    while i < N_LINES {
        textPtr = textPtr.offset(snprintf(
            textPtr,
            SIZE_PER_LINE,
            b"  <!ENTITY s%lu '&s%lu;'>\n\0".as_ptr() as *const ::core::ffi::c_char,
            i as ::core::ffi::c_ulong,
            i.wrapping_sub(1usize) as ::core::ffi::c_ulong,
        ) as isize);
        i = i.wrapping_add(1);
    }
    snprintf(
        textPtr,
        SIZE_PER_LINE,
        b"]> <foo>&s%lu;</foo>\n\0".as_ptr() as *const ::core::ffi::c_char,
        N_LINES.wrapping_sub(1usize) as ::core::ffi::c_ulong,
    );
    let expected: *const XML_Char = b"deepText\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
    XML_SetCharacterDataHandler(
        parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetUserData(parser, &raw mut storage as *mut ::core::ffi::c_void);
    if _XML_Parse_SINGLE_BYTES(
        parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5507i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
    XML_ParserFree(parser);
    free(text as *mut ::core::ffi::c_void);
}

unsafe extern "C" fn test_deep_nested_attribute_entity() {
    _check_set_test_info(
        b"test_deep_nested_attribute_entity\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5517,
    );
    let N_LINES: size_t = 60000;
    let SIZE_PER_LINE: size_t = 100;
    let text: *mut ::core::ffi::c_char =
        malloc(N_LINES.wrapping_add(4usize).wrapping_mul(SIZE_PER_LINE))
            as *mut ::core::ffi::c_char;
    if text.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5523i32,
            b"malloc failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut textPtr: *mut ::core::ffi::c_char = text;
    textPtr = textPtr.offset(snprintf(
        textPtr,
        SIZE_PER_LINE,
        b"<!DOCTYPE foo [\n\t<!ENTITY s0 'deepText'>\n\0".as_ptr() as *const ::core::ffi::c_char,
    ) as isize);
    let mut i: size_t = 1;
    while i < N_LINES {
        textPtr = textPtr.offset(snprintf(
            textPtr,
            SIZE_PER_LINE,
            b"  <!ENTITY s%lu '&s%lu;'>\n\0".as_ptr() as *const ::core::ffi::c_char,
            i as ::core::ffi::c_ulong,
            i.wrapping_sub(1usize) as ::core::ffi::c_ulong,
        ) as isize);
        i = i.wrapping_add(1);
    }
    snprintf(
        textPtr,
        SIZE_PER_LINE,
        b"]> <foo name='&s%lu;'>mainText</foo>\n\0".as_ptr() as *const ::core::ffi::c_char,
        N_LINES.wrapping_sub(1usize) as ::core::ffi::c_ulong,
    );
    let mut doc_info: [AttrInfo; 2] = [
        attrInfo {
            name: b"name\0".as_ptr() as *const XML_Char,
            value: b"deepText\0".as_ptr() as *const XML_Char,
        },
        attrInfo {
            name: ::core::ptr::null::<XML_Char>(),
            value: ::core::ptr::null::<XML_Char>(),
        },
    ];
    let mut info: [ElementInfo; 2] = [
        elementInfo {
            name: b"foo\0".as_ptr() as *const XML_Char,
            attr_count: 1,
            id_name: ::core::ptr::null::<XML_Char>(),
            attributes: ::core::ptr::null_mut::<AttrInfo>(),
        },
        elementInfo {
            name: ::core::ptr::null::<XML_Char>(),
            attr_count: 0,
            id_name: ::core::ptr::null::<XML_Char>(),
            attributes: ::core::ptr::null_mut::<AttrInfo>(),
        },
    ];
    info[0].attributes = &raw mut doc_info as *mut AttrInfo;
    let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
    let mut parserPlusElemenInfo: ParserAndElementInfo = StructParserAndElementInfo {
        parser: parser,
        info: &raw mut info as *mut ElementInfo,
    };
    XML_SetStartElementHandler(
        parser,
        Some(
            counting_start_element_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetUserData(
        parser,
        &raw mut parserPlusElemenInfo as *mut ::core::ffi::c_void,
    );
    if _XML_Parse_SINGLE_BYTES(
        parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5553i32,
        );
    }
    XML_ParserFree(parser);
    free(text as *mut ::core::ffi::c_void);
}

unsafe extern "C" fn test_deep_nested_entity_delayed_interpretation() {
    _check_set_test_info(
        b"test_deep_nested_entity_delayed_interpretation\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5560,
    );
    let N_LINES: size_t = 70000i32 as size_t;
    let SIZE_PER_LINE: size_t = 100;
    let text: *mut ::core::ffi::c_char =
        malloc(N_LINES.wrapping_add(4usize).wrapping_mul(SIZE_PER_LINE))
            as *mut ::core::ffi::c_char;
    if text.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5566i32,
            b"malloc failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut textPtr: *mut ::core::ffi::c_char = text;
    textPtr = textPtr.offset(snprintf(
        textPtr,
        SIZE_PER_LINE,
        b"<!DOCTYPE foo [\n\t<!ENTITY %% s0 'deepText'>\n\0".as_ptr() as *const ::core::ffi::c_char,
    ) as isize);
    let mut i: size_t = 1;
    while i < N_LINES {
        textPtr = textPtr.offset(snprintf(
            textPtr,
            SIZE_PER_LINE,
            b"  <!ENTITY %% s%lu '&#37;s%lu;'>\n\0".as_ptr() as *const ::core::ffi::c_char,
            i as ::core::ffi::c_ulong,
            i.wrapping_sub(1usize) as ::core::ffi::c_ulong,
        ) as isize);
        i = i.wrapping_add(1);
    }
    snprintf(
        textPtr,
        SIZE_PER_LINE,
        b"  <!ENTITY %% define_g \"<!ENTITY g '&#37;s%lu;'>\">\n  %%define_g;\n]>\n<foo/>\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        N_LINES.wrapping_sub(1usize) as ::core::ffi::c_ulong,
    );
    let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
    XML_SetParamEntityParsing(parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    if _XML_Parse_SINGLE_BYTES(
        parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) == XML_STATUS_ERROR
    {
        _xml_failure(
            parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5594i32,
        );
    }
    XML_ParserFree(parser);
    free(text as *mut ::core::ffi::c_void);
}

unsafe extern "C" fn test_nested_entity_suspend() {
    _check_set_test_info(
        b"test_nested_entity_suspend\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5601,
    );
    let text: *const ::core::ffi::c_char = b"<!DOCTYPE a [\n  <!ENTITY e1 '<!--e1-->'>\n  <!ENTITY e2 '<!--e2 head-->&e1;<!--e2 tail-->'>\n  <!ENTITY e3 '<!--e3 head-->&e2;<!--e3 tail-->'>\n]>\n<a><!--start-->&e3;<!--end--></a>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let expected: *const XML_Char =
        b"starte3 heade2 heade1e2 taile3 tailend\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
    let mut parserPlusStorage: ParserPlusStorage = ParserPlusStorage {
        parser: parser,
        storage: &raw mut storage,
    };
    XML_SetParamEntityParsing(parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    XML_SetCommentHandler(
        parser,
        Some(
            accumulate_and_suspend_comment_handler
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
        ),
    );
    XML_SetUserData(
        parser,
        &raw mut parserPlusStorage as *mut ::core::ffi::c_void,
    );
    let mut status: XML_Status = XML_Parse(
        parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    );
    while status == XML_STATUS_SUSPENDED {
        status = XML_ResumeParser(parser);
    }
    if status != XML_STATUS_OK {
        _xml_failure(
            parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5624i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
    XML_ParserFree(parser);
}

unsafe extern "C" fn test_nested_entity_suspend_2() {
    _check_set_test_info(
        b"test_nested_entity_suspend_2\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5631,
    );
    let text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY ge1 'head1Ztail1'>\n  <!ENTITY ge2 'head2&ge1;tail2'>\n  <!ENTITY ge3 'head3&ge2;tail3'>\n]>\n<doc>&ge3;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let expected: *const XML_Char =
        b"head3head2head1Ztail1tail2tail3\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
    let mut parserPlusStorage: ParserPlusStorage = ParserPlusStorage {
        parser: parser,
        storage: &raw mut storage,
    };
    XML_SetCharacterDataHandler(
        parser,
        Some(
            accumulate_char_data_and_suspend
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetUserData(
        parser,
        &raw mut parserPlusStorage as *mut ::core::ffi::c_void,
    );
    let mut status: XML_Status = XML_Parse(
        parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    );
    while status == XML_STATUS_SUSPENDED {
        status = XML_ResumeParser(parser);
    }
    if status != XML_STATUS_OK {
        _xml_failure(
            parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5653i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, expected);
    XML_ParserFree(parser);
}

unsafe extern "C" fn test_big_tokens_scale_linearly() {
    _check_set_test_info(
        b"test_big_tokens_scale_linearly\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5661,
    );
    let text: [C2Rust_Unnamed_10; 5] = [
        C2Rust_Unnamed_10 {
            pre: b"<a>\0".as_ptr() as *const ::core::ffi::c_char,
            post: b"</a>\0".as_ptr() as *const ::core::ffi::c_char,
        },
        C2Rust_Unnamed_10 {
            pre: b"<b><![CDATA[ value: \0".as_ptr() as *const ::core::ffi::c_char,
            post: b" ]]></b>\0".as_ptr() as *const ::core::ffi::c_char,
        },
        C2Rust_Unnamed_10 {
            pre: b"<c attr='\0".as_ptr() as *const ::core::ffi::c_char,
            post: b"'></c>\0".as_ptr() as *const ::core::ffi::c_char,
        },
        C2Rust_Unnamed_10 {
            pre: b"<d><!-- \0".as_ptr() as *const ::core::ffi::c_char,
            post: b" --></d>\0".as_ptr() as *const ::core::ffi::c_char,
        },
        C2Rust_Unnamed_10 {
            pre: b"<e><\0".as_ptr() as *const ::core::ffi::c_char,
            post: b"/></e>\0".as_ptr() as *const ::core::ffi::c_char,
        },
    ];
    let num_cases: ::core::ffi::c_int = (::core::mem::size_of::<[C2Rust_Unnamed_10; 5]>())
        .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_10>())
        as ::core::ffi::c_int;
    let mut aaaaaa: [::core::ffi::c_char; 4096] = [0; 4096];
    let fillsize: ::core::ffi::c_int =
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as ::core::ffi::c_int;
    let fillcount: ::core::ffi::c_int = 100;
    let approx_bytes: ::core::ffi::c_uint = (fillsize * fillcount) as ::core::ffi::c_uint;
    let max_factor: ::core::ffi::c_uint = 4;
    let max_scanned: ::core::ffi::c_uint = max_factor.wrapping_mul(approx_bytes);
    memset(
        &raw mut aaaaaa as *mut ::core::ffi::c_void,
        'a' as i32,
        fillsize as size_t,
    );
    if g_reparseDeferralEnabledDefault == 0 {
        return;
    }
    let mut i: ::core::ffi::c_int = 0;
    while i < num_cases {
        let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        if parser.is_null() {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                5688i32,
                b"check failed: parser != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut status: XML_Status = XML_STATUS_ERROR;
        set_subtest(
            b"text=\"%saaaaaa%s\"\0".as_ptr() as *const ::core::ffi::c_char,
            text[i as usize].pre,
            text[i as usize].post,
        );
        g_bytesScanned = 0;
        status = _XML_Parse_SINGLE_BYTES(
            parser,
            text[i as usize].pre,
            strlen(text[i as usize].pre) as ::core::ffi::c_int,
            XML_FALSE as ::core::ffi::c_int,
        );
        if status != XML_STATUS_OK {
            _xml_failure(
                parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                5697i32,
            );
        }
        let mut past_max_count: ::core::ffi::c_uint = 0;
        let mut f: ::core::ffi::c_int = 0;
        while f < fillcount {
            status = _XML_Parse_SINGLE_BYTES(
                parser,
                &raw mut aaaaaa as *mut ::core::ffi::c_char,
                fillsize,
                XML_FALSE as ::core::ffi::c_int,
            );
            if status != XML_STATUS_OK {
                _xml_failure(
                    parser,
                    b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    5705i32,
                );
            }
            if g_bytesScanned > max_scanned {
                let pushed: ::core::ffi::c_uint = (strlen(text[i as usize].pre)
                    as ::core::ffi::c_uint)
                    .wrapping_add(((f + 1) * fillsize) as ::core::ffi::c_uint);
                fprintf(
                    stderr,
                    b"after %d/%d loops: pushed=%u scanned=%u (factor ~%.2f) max_scanned: %u (factor ~%u)\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    f + 1i32,
                    fillcount,
                    pushed,
                    g_bytesScanned,
                    g_bytesScanned as ::core::ffi::c_double
                        / pushed as ::core::ffi::c_double,
                    max_scanned,
                    max_factor,
                );
                past_max_count = past_max_count.wrapping_add(1);
                if !(past_max_count < 5) {
                    _fail(
                        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        5720i32,
                        b"check failed: past_max_count < 5\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
            }
            f += 1;
        }
        status = _XML_Parse_SINGLE_BYTES(
            parser,
            text[i as usize].post,
            strlen(text[i as usize].post) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        );
        if status != XML_STATUS_OK {
            _xml_failure(
                parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                5728i32,
            );
        }
        if !(g_bytesScanned > approx_bytes) {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                5731i32,
                b"check failed: g_bytesScanned > approx_bytes\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if g_bytesScanned > max_scanned {
            fprintf(
                stderr,
                b"after all input: scanned=%u (factor ~%.2f) max_scanned: %u (factor ~%u)\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                g_bytesScanned,
                g_bytesScanned as ::core::ffi::c_double / approx_bytes as ::core::ffi::c_double,
                max_scanned,
                max_factor,
            );
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                5738i32,
                b"scanned too many bytes\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        XML_ParserFree(parser);
        i += 1;
    }
}

unsafe extern "C" fn test_set_reparse_deferral() {
    _check_set_test_info(
        b"test_set_reparse_deferral\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5746,
    );
    let pre: *const ::core::ffi::c_char = b"<d>\0".as_ptr() as *const ::core::ffi::c_char;
    let start: *const ::core::ffi::c_char = b"<x attr='\0".as_ptr() as *const ::core::ffi::c_char;
    let end: *const ::core::ffi::c_char = b"'></x>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut eeeeee: [::core::ffi::c_char; 100] = [0; 100];
    let fillsize: ::core::ffi::c_int =
        ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as ::core::ffi::c_int;
    memset(
        &raw mut eeeeee as *mut ::core::ffi::c_void,
        'e' as i32,
        fillsize as size_t,
    );
    let mut enabled: ::core::ffi::c_int = 0;
    while enabled <= 1 {
        set_subtest(
            b"deferral=%d\0".as_ptr() as *const ::core::ffi::c_char,
            enabled,
        );
        let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        if parser.is_null() {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                5758i32,
                b"check failed: parser != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if XML_SetReparseDeferralEnabled(parser, enabled as XML_Bool) == 0 {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                5759i32,
                b"check failed: XML_SetReparseDeferralEnabled(parser, enabled)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if XML_GetBuffer(parser, fillsize * 10103).is_null() {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                5761i32,
                b"check failed: XML_GetBuffer(parser, fillsize * 10103) != NULL\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        XML_SetUserData(parser, &raw mut storage as *mut ::core::ffi::c_void);
        XML_SetStartElementHandler(
            parser,
            Some(
                start_element_event_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut *const XML_Char,
                    ) -> (),
            ),
        );
        let mut status: XML_Status = XML_STATUS_ERROR;
        status = XML_Parse(
            parser,
            pre,
            strlen(pre) as ::core::ffi::c_int,
            XML_FALSE as ::core::ffi::c_int,
        );
        if status != XML_STATUS_OK {
            _xml_failure(
                parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                5772i32,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, b"d\0".as_ptr() as *const XML_Char);
        status = XML_Parse(
            parser,
            start,
            strlen(start) as ::core::ffi::c_int,
            XML_FALSE as ::core::ffi::c_int,
        );
        if status != XML_STATUS_OK {
            _xml_failure(
                parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                5779i32,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, b"d\0".as_ptr() as *const XML_Char);
        let mut c: ::core::ffi::c_int = 0;
        while c < 100 {
            status = XML_Parse(
                parser,
                &raw mut eeeeee as *mut ::core::ffi::c_char,
                fillsize,
                XML_FALSE as ::core::ffi::c_int,
            );
            if status != XML_STATUS_OK {
                _xml_failure(
                    parser,
                    b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    5787i32,
                );
            }
            c += 1;
        }
        CharData_CheckXMLChars(&raw mut storage, b"d\0".as_ptr() as *const XML_Char);
        status = XML_Parse(
            parser,
            end,
            strlen(end) as ::core::ffi::c_int,
            XML_FALSE as ::core::ffi::c_int,
        );
        if status != XML_STATUS_OK {
            _xml_failure(
                parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                5795i32,
            );
        }
        if enabled != 0 {
            CharData_CheckXMLChars(&raw mut storage, b"d\0".as_ptr() as *const XML_Char);
            let mut c_0: ::core::ffi::c_int = 0;
            while c_0 < 101 {
                status = XML_Parse(
                    parser,
                    &raw mut eeeeee as *mut ::core::ffi::c_char,
                    fillsize,
                    XML_FALSE as ::core::ffi::c_int,
                );
                if status != XML_STATUS_OK {
                    _xml_failure(
                        parser,
                        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        5806i32,
                    );
                }
                c_0 += 1;
            }
        }
        CharData_CheckXMLChars(&raw mut storage, b"dx\0".as_ptr() as *const XML_Char);
        XML_ParserFree(parser);
        enabled += 1;
    }
}

unsafe extern "C" fn element_decl_counter(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const XML_Char,
    mut model: *mut XML_Content,
) {
    let mut testdata: *mut element_decl_data = userData as *mut element_decl_data;
    (*testdata).count += 1;
    XML_FreeContentModel((*testdata).parser, model);
}

unsafe extern "C" fn external_inherited_parser(
    mut p: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let pre: *const ::core::ffi::c_char =
        b"<!ELEMENT document ANY>\n\0".as_ptr() as *const ::core::ffi::c_char;
    let start: *const ::core::ffi::c_char = b"<!ELEMENT \0".as_ptr() as *const ::core::ffi::c_char;
    let end: *const ::core::ffi::c_char = b" ANY>\n\0".as_ptr() as *const ::core::ffi::c_char;
    let post: *const ::core::ffi::c_char =
        b"<!ELEMENT xyz ANY>\n\0".as_ptr() as *const ::core::ffi::c_char;
    let enabled: ::core::ffi::c_int =
        *(*(p as *mut *mut ::core::ffi::c_void) as *mut ::core::ffi::c_int);
    let mut eeeeee: [::core::ffi::c_char; 100] = [0; 100];
    let mut spaces: [::core::ffi::c_char; 100] = [0; 100];
    let fillsize: ::core::ffi::c_int =
        ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as ::core::ffi::c_int;
    if !(fillsize == ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as ::core::ffi::c_int) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5845i32,
            b"check failed: fillsize == (int)sizeof(spaces)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    memset(
        &raw mut eeeeee as *mut ::core::ffi::c_void,
        'e' as i32,
        fillsize as size_t,
    );
    memset(
        &raw mut spaces as *mut ::core::ffi::c_void,
        ' ' as i32,
        fillsize as size_t,
    );
    let mut parser: XML_Parser =
        XML_ExternalEntityParserCreate(p, context, ::core::ptr::null::<XML_Char>());
    if parser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5850i32,
            b"check failed: parser != NULL\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_GetBuffer(parser, fillsize * 10103).is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5852i32,
            b"check failed: XML_GetBuffer(parser, fillsize * 10103) != NULL\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    let mut testdata: element_decl_data = element_decl_data {
        parser: ::core::ptr::null_mut::<XML_ParserStruct>(),
        count: 0,
    };
    testdata.parser = parser;
    testdata.count = 0;
    XML_SetUserData(parser, &raw mut testdata as *mut ::core::ffi::c_void);
    XML_SetElementDeclHandler(
        parser,
        ::core::mem::transmute(Some(
            element_decl_counter
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Content,
                ) -> (),
        )),
    );
    let mut status: XML_Status = XML_STATUS_ERROR;
    status = XML_Parse(
        parser,
        pre,
        strlen(pre) as ::core::ffi::c_int,
        XML_FALSE as ::core::ffi::c_int,
    );
    if status != XML_STATUS_OK {
        _xml_failure(
            parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5864i32,
        );
    }
    if !(testdata.count == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5866i32,
            b"check failed: testdata.count == 1\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    status = XML_Parse(
        parser,
        start,
        strlen(start) as ::core::ffi::c_int,
        XML_FALSE as ::core::ffi::c_int,
    );
    if status != XML_STATUS_OK {
        _xml_failure(
            parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5871i32,
        );
    }
    if !(testdata.count == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5873i32,
            b"check failed: testdata.count == 1\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut c: ::core::ffi::c_int = 0;
    while c < 100 {
        status = XML_Parse(
            parser,
            &raw mut eeeeee as *mut ::core::ffi::c_char,
            fillsize,
            XML_FALSE as ::core::ffi::c_int,
        );
        if status != XML_STATUS_OK {
            _xml_failure(
                parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                5879i32,
            );
        }
        c += 1;
    }
    if !(testdata.count == 1) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5882i32,
            b"check failed: testdata.count == 1\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    status = XML_Parse(
        parser,
        end,
        strlen(end) as ::core::ffi::c_int,
        XML_FALSE as ::core::ffi::c_int,
    );
    if status != XML_STATUS_OK {
        _xml_failure(
            parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5887i32,
        );
    }
    if enabled != 0 {
        if !(testdata.count == 1) {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                5893i32,
                b"check failed: testdata.count == 1\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut c_0: ::core::ffi::c_int = 0;
        while c_0 < 101 {
            status = XML_Parse(
                parser,
                &raw mut spaces as *mut ::core::ffi::c_char,
                fillsize,
                XML_FALSE as ::core::ffi::c_int,
            );
            if status != XML_STATUS_OK {
                _xml_failure(
                    parser,
                    b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    5898i32,
                );
            }
            c_0 += 1;
        }
    }
    if !(testdata.count == 2) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5902i32,
            b"check failed: testdata.count == 2\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    status = XML_Parse(
        parser,
        post,
        strlen(post) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    );
    if status != XML_STATUS_OK {
        _xml_failure(
            parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5907i32,
        );
    }
    if !(testdata.count == 3) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5909i32,
            b"check failed: testdata.count == 3\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    XML_ParserFree(parser);
    return XML_STATUS_OK as ::core::ffi::c_int;
}

unsafe extern "C" fn test_reparse_deferral_is_inherited() {
    _check_set_test_info(
        b"test_reparse_deferral_is_inherited\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5915,
    );
    let text: *const ::core::ffi::c_char =
        b"<!DOCTYPE document SYSTEM 'something.ext'><document/>\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut enabled: ::core::ffi::c_int = 0;
    while enabled <= 1 {
        set_subtest(
            b"deferral=%d\0".as_ptr() as *const ::core::ffi::c_char,
            enabled,
        );
        let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        if parser.is_null() {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                5922i32,
                b"check failed: parser != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        XML_SetUserData(parser, &raw mut enabled as *mut ::core::ffi::c_void);
        XML_SetParamEntityParsing(parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            parser,
            Some(
                external_inherited_parser
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        if XML_SetReparseDeferralEnabled(parser, enabled as XML_Bool) == 0 {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                5928i32,
                b"check failed: XML_SetReparseDeferralEnabled(parser, enabled)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if XML_Parse(
            parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) != XML_STATUS_OK
        {
            _xml_failure(
                parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                5930i32,
            );
        }
        XML_ParserFree(parser);
        enabled += 1;
    }
}

unsafe extern "C" fn test_set_reparse_deferral_on_null_parser() {
    _check_set_test_info(
        b"test_set_reparse_deferral_on_null_parser\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5937,
    );
    if !(XML_SetReparseDeferralEnabled(::core::ptr::null_mut::<XML_ParserStruct>(), 0)
        as ::core::ffi::c_int
        == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5938i32,
            b"check failed: XML_SetReparseDeferralEnabled(NULL, 0) == XML_FALSE\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !(XML_SetReparseDeferralEnabled(::core::ptr::null_mut::<XML_ParserStruct>(), 1)
        as ::core::ffi::c_int
        == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5939i32,
            b"check failed: XML_SetReparseDeferralEnabled(NULL, 1) == XML_FALSE\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !(XML_SetReparseDeferralEnabled(::core::ptr::null_mut::<XML_ParserStruct>(), 10)
        as ::core::ffi::c_int
        == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5940i32,
            b"check failed: XML_SetReparseDeferralEnabled(NULL, 10) == XML_FALSE\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !(XML_SetReparseDeferralEnabled(::core::ptr::null_mut::<XML_ParserStruct>(), 100)
        as ::core::ffi::c_int
        == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5941i32,
            b"check failed: XML_SetReparseDeferralEnabled(NULL, 100) == XML_FALSE\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !(XML_SetReparseDeferralEnabled(
        ::core::ptr::null_mut::<XML_ParserStruct>(),
        (-2147483647i32 - 1) as XML_Bool,
    ) as ::core::ffi::c_int
        == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5943i32,
            b"check failed: XML_SetReparseDeferralEnabled(NULL, (XML_Bool)INT_MIN) == XML_FALSE\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !(XML_SetReparseDeferralEnabled(
        ::core::ptr::null_mut::<XML_ParserStruct>(),
        2147483647i32 as XML_Bool,
    ) as ::core::ffi::c_int
        == 0)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5945i32,
            b"check failed: XML_SetReparseDeferralEnabled(NULL, (XML_Bool)INT_MAX) == XML_FALSE\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_set_reparse_deferral_on_the_fly() {
    _check_set_test_info(
        b"test_set_reparse_deferral_on_the_fly\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        5949,
    );
    let pre: *const ::core::ffi::c_char = b"<d><x attr='\0".as_ptr() as *const ::core::ffi::c_char;
    let end: *const ::core::ffi::c_char = b"'></x>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut iiiiii: [::core::ffi::c_char; 100] = [0; 100];
    let fillsize: ::core::ffi::c_int =
        ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as ::core::ffi::c_int;
    memset(
        &raw mut iiiiii as *mut ::core::ffi::c_void,
        'i' as i32,
        fillsize as size_t,
    );
    let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
    if parser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5957i32,
            b"check failed: parser != NULL\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_SetReparseDeferralEnabled(parser, 1) == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5958i32,
            b"check failed: XML_SetReparseDeferralEnabled(parser, XML_TRUE)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    CharData_Init(&raw mut storage);
    XML_SetUserData(parser, &raw mut storage as *mut ::core::ffi::c_void);
    XML_SetStartElementHandler(
        parser,
        Some(
            start_element_event_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut *const XML_Char,
                ) -> (),
        ),
    );
    let mut status: XML_Status = XML_STATUS_ERROR;
    status = XML_Parse(
        parser,
        pre,
        strlen(pre) as ::core::ffi::c_int,
        XML_FALSE as ::core::ffi::c_int,
    );
    if status != XML_STATUS_OK {
        _xml_failure(
            parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5969i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, b"d\0".as_ptr() as *const XML_Char);
    status = XML_Parse(
        parser,
        &raw mut iiiiii as *mut ::core::ffi::c_char,
        fillsize,
        XML_FALSE as ::core::ffi::c_int,
    );
    if status != XML_STATUS_OK {
        _xml_failure(
            parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5976i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, b"d\0".as_ptr() as *const XML_Char);
    status = XML_Parse(
        parser,
        end,
        strlen(end) as ::core::ffi::c_int,
        XML_FALSE as ::core::ffi::c_int,
    );
    if status != XML_STATUS_OK {
        _xml_failure(
            parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5983i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, b"d\0".as_ptr() as *const XML_Char);
    if XML_SetReparseDeferralEnabled(parser, 0) == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5988i32,
            b"check failed: XML_SetReparseDeferralEnabled(parser, XML_FALSE)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    status = XML_Parse(
        parser,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        0,
        XML_FALSE as ::core::ffi::c_int,
    );
    if status != XML_STATUS_OK {
        _xml_failure(
            parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            5992i32,
        );
    }
    CharData_CheckXMLChars(&raw mut storage, b"dx\0".as_ptr() as *const XML_Char);
    XML_ParserFree(parser);
}

unsafe extern "C" fn test_set_bad_reparse_option() {
    _check_set_test_info(
        b"test_set_bad_reparse_option\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        6000,
    );
    let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
    if !(0 == XML_SetReparseDeferralEnabled(parser, 2) as ::core::ffi::c_int) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            6002i32,
            b"check failed: XML_FALSE == XML_SetReparseDeferralEnabled(parser, 2)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !(0 == XML_SetReparseDeferralEnabled(parser, 3) as ::core::ffi::c_int) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            6003i32,
            b"check failed: XML_FALSE == XML_SetReparseDeferralEnabled(parser, 3)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !(0 == XML_SetReparseDeferralEnabled(parser, 99) as ::core::ffi::c_int) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            6004i32,
            b"check failed: XML_FALSE == XML_SetReparseDeferralEnabled(parser, 99)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !(0 == XML_SetReparseDeferralEnabled(parser, 127) as ::core::ffi::c_int) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            6005i32,
            b"check failed: XML_FALSE == XML_SetReparseDeferralEnabled(parser, 127)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !(0 == XML_SetReparseDeferralEnabled(parser, 128) as ::core::ffi::c_int) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            6006i32,
            b"check failed: XML_FALSE == XML_SetReparseDeferralEnabled(parser, 128)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !(0 == XML_SetReparseDeferralEnabled(parser, 129) as ::core::ffi::c_int) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            6007i32,
            b"check failed: XML_FALSE == XML_SetReparseDeferralEnabled(parser, 129)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !(0 == XML_SetReparseDeferralEnabled(parser, 255) as ::core::ffi::c_int) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            6008i32,
            b"check failed: XML_FALSE == XML_SetReparseDeferralEnabled(parser, 255)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !(1 == XML_SetReparseDeferralEnabled(parser, 0) as ::core::ffi::c_int) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            6009i32,
            b"check failed: XML_TRUE == XML_SetReparseDeferralEnabled(parser, 0)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !(1 == XML_SetReparseDeferralEnabled(parser, 1) as ::core::ffi::c_int) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            6010i32,
            b"check failed: XML_TRUE == XML_SetReparseDeferralEnabled(parser, 1)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    XML_ParserFree(parser);
}

static mut g_totalAlloc: size_t = 0;

static mut g_biggestAlloc: size_t = 0;

unsafe extern "C" fn counting_realloc(
    mut ptr: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    g_totalAlloc = g_totalAlloc.wrapping_add(size);
    if size > g_biggestAlloc {
        g_biggestAlloc = size;
    }
    return realloc(ptr, size);
}

unsafe extern "C" fn counting_malloc(mut size: size_t) -> *mut ::core::ffi::c_void {
    return counting_realloc(NULL, size);
}

unsafe extern "C" fn test_bypass_heuristic_when_close_to_bufsize() {
    _check_set_test_info(
        b"test_bypass_heuristic_when_close_to_bufsize\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        6032,
    );
    if g_chunkSize != 0 {
        return;
    }
    if g_reparseDeferralEnabledDefault == 0 {
        return;
    }
    let document_length: ::core::ffi::c_int = 65536;
    let document: *mut ::core::ffi::c_char =
        malloc(document_length as size_t) as *mut ::core::ffi::c_char;
    let memfuncs: XML_Memory_Handling_Suite = XML_Memory_Handling_Suite {
        malloc_fcn: Some(
            counting_malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void,
        ),
        realloc_fcn: Some(
            counting_realloc
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                ) -> *mut ::core::ffi::c_void,
        ),
        free_fcn: Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
    };
    let leading_list: [::core::ffi::c_int; 10] = [0, 3, 61, 96, 400, 401, 4000, 4010, 4099, -1];
    let bigtoken_list: [::core::ffi::c_int; 8] = [3000, 4000, 4001, 4096, 4099, 5000, 20000, -1];
    let fillsize_list: [::core::ffi::c_int; 9] = [131, 256, 399, 400, 401, 1025, 4099, 4321, -1];
    let mut leading: *const ::core::ffi::c_int =
        &raw const leading_list as *const ::core::ffi::c_int;
    while *leading >= 0 {
        let mut bigtoken: *const ::core::ffi::c_int =
            &raw const bigtoken_list as *const ::core::ffi::c_int;
        while *bigtoken >= 0 {
            let mut fillsize: *const ::core::ffi::c_int =
                &raw const fillsize_list as *const ::core::ffi::c_int;
            while *fillsize >= 0 {
                set_subtest(
                    b"leading=%d bigtoken=%d fillsize=%d\0".as_ptr() as *const ::core::ffi::c_char,
                    *leading,
                    *bigtoken,
                    *fillsize,
                );
                if !(*leading + *bigtoken <= document_length) {
                    _fail(
                        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        6061i32,
                        b"check failed: *leading + *bigtoken <= document_length\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
                memset(
                    document as *mut ::core::ffi::c_void,
                    'x' as i32,
                    document_length as size_t,
                );
                if *leading != 0 {
                    if !(*leading >= 3) {
                        _fail(
                            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            6067i32,
                            b"check failed: *leading >= 3\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                    memcpy(
                        document as *mut ::core::ffi::c_void,
                        b"<a>\0".as_ptr() as *const ::core::ffi::c_void,
                        3usize,
                    );
                }
                *document.offset((*leading + 0) as isize) = '<' as ::core::ffi::c_char;
                *document.offset((*leading + 1) as isize) = 'b' as ::core::ffi::c_char;
                memset(
                    document.offset((*leading + 2) as isize) as *mut ::core::ffi::c_void,
                    ' ' as i32,
                    (*bigtoken - 2) as size_t,
                );
                *document.offset((*leading + *bigtoken - 1) as isize) = '>' as ::core::ffi::c_char;
                let expected_elem_total: ::core::ffi::c_int =
                    1 + (if *leading != 0 { 1 } else { 0 });
                let mut parser: XML_Parser = XML_ParserCreate_MM(
                    ::core::ptr::null::<XML_Char>(),
                    &raw const memfuncs,
                    ::core::ptr::null::<XML_Char>(),
                );
                if parser.is_null() {
                    _fail(
                        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        6080i32,
                        b"check failed: parser != NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
                let mut storage: CharData = CharData {
                    count: 0,
                    data: [0; 2048],
                };
                CharData_Init(&raw mut storage);
                XML_SetUserData(parser, &raw mut storage as *mut ::core::ffi::c_void);
                XML_SetStartElementHandler(
                    parser,
                    Some(
                        start_element_event_handler
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *const XML_Char,
                                *mut *const XML_Char,
                            ) -> (),
                    ),
                );
                g_biggestAlloc = 0;
                g_totalAlloc = 0;
                let mut offset: ::core::ffi::c_int = 0;
                while offset < *leading + *bigtoken {
                    if !(offset + *fillsize <= document_length) {
                        _fail(
                            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            6092i32,
                            b"check failed: offset + *fillsize <= document_length\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                    let status: XML_Status = XML_Parse(
                        parser,
                        document.offset(offset as isize),
                        *fillsize,
                        XML_FALSE as ::core::ffi::c_int,
                    );
                    if status != XML_STATUS_OK {
                        _xml_failure(
                            parser,
                            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            6096i32,
                        );
                    }
                    offset += *fillsize;
                }
                let bigtok_first_chunk_bytes: ::core::ffi::c_int = *fillsize - *leading % *fillsize;
                if !(bigtok_first_chunk_bytes >= *bigtoken && XML_CONTEXT_BYTES == 0) {
                    if *leading < XML_CONTEXT_BYTES {
                        if !(g_biggestAlloc
                            >= (*leading as size_t).wrapping_add(*bigtoken as size_t))
                        {
                            _fail(
                                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                6110i32,
                                b"check failed: g_biggestAlloc >= *leading + (size_t)*bigtoken\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                            );
                        }
                    } else if !(g_biggestAlloc >= (1024usize).wrapping_add(*bigtoken as size_t)) {
                        _fail(
                            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            6112i32,
                            b"check failed: g_biggestAlloc >= XML_CONTEXT_BYTES + (size_t)*bigtoken\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                }
                while storage.count < expected_elem_total {
                    let alloc_before: size_t = g_totalAlloc;
                    if !(offset + *fillsize <= document_length) {
                        _fail(
                            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            6117i32,
                            b"check failed: offset + *fillsize <= document_length\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                    let status_0: XML_Status = XML_Parse(
                        parser,
                        document.offset(offset as isize),
                        *fillsize,
                        XML_FALSE as ::core::ffi::c_int,
                    );
                    if status_0 != XML_STATUS_OK {
                        _xml_failure(
                            parser,
                            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            6121i32,
                        );
                    }
                    offset += *fillsize;
                    if !(g_totalAlloc.wrapping_sub(alloc_before) < 4096usize) {
                        _fail(
                            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            6128i32,
                            b"check failed: g_totalAlloc - alloc_before < 4096\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                }
                if !(g_totalAlloc > 0) {
                    _fail(
                        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        6131i32,
                        b"check failed: g_totalAlloc > 0\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
                if !(storage.count == expected_elem_total) {
                    _fail(
                        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        6133i32,
                        b"check failed: storage.count == expected_elem_total\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
                XML_ParserFree(parser);
                fillsize = fillsize.offset(1);
            }
            bigtoken = bigtoken.offset(1);
        }
        leading = leading.offset(1);
    }
    free(document as *mut ::core::ffi::c_void);
}

unsafe extern "C" fn test_varying_buffer_fills() {
    _check_set_test_info(
        b"test_varying_buffer_fills\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        6143,
    );
    let KiB: ::core::ffi::c_int = 1024;
    let MiB: ::core::ffi::c_int = 1024 * KiB;
    let document_length: ::core::ffi::c_int = 16 * MiB;
    let big: ::core::ffi::c_int = 7654321;
    if g_chunkSize != 0 {
        return;
    }
    let document: *mut ::core::ffi::c_char =
        malloc(document_length as size_t) as *mut ::core::ffi::c_char;
    if document.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            6154i32,
            b"check failed: document != NULL\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    memset(
        document as *mut ::core::ffi::c_void,
        'x' as i32,
        document_length as size_t,
    );
    *document.offset(0) = '<' as ::core::ffi::c_char;
    *document.offset(1) = 't' as ::core::ffi::c_char;
    memset(
        document.offset(2) as *mut ::core::ffi::c_void,
        ' ' as i32,
        (big - 2) as size_t,
    );
    *document.offset((big - 1) as isize) = '>' as ::core::ffi::c_char;
    let testcases: [[::core::ffi::c_int; 30]; 11] = [
        [
            8 * MiB,
            -8 * MiB,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
        [
            4 * MiB,
            4 * MiB,
            -12 * MiB,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
        [
            4 * MiB,
            0,
            4 * MiB,
            -12 * MiB,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
        [
            4 * MiB,
            0,
            0,
            4 * MiB,
            -12 * MiB,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
        [
            4 * MiB,
            0,
            1 * MiB,
            0,
            3 * MiB,
            -12 * MiB,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
        [
            4 * MiB,
            2 * MiB,
            1 * MiB,
            512 * KiB,
            256 * KiB,
            256 * KiB,
            -12 * MiB,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
        [
            4 * MiB + 1,
            2 * MiB,
            1 * MiB,
            512 * KiB,
            -25 * MiB,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
        [
            1 * KiB,
            2 * KiB,
            4 * KiB,
            8 * KiB,
            16 * KiB,
            32 * KiB,
            64 * KiB,
            128 * KiB,
            256 * KiB,
            512 * KiB,
            1 * MiB,
            2 * MiB,
            4 * MiB,
            -16 * MiB,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
        [
            2 * KiB + 1,
            2 * KiB,
            4 * KiB,
            8 * KiB,
            16 * KiB,
            32 * KiB,
            64 * KiB,
            128 * KiB,
            256 * KiB,
            512 * KiB,
            1 * MiB,
            2 * MiB,
            4 * MiB,
            -(10 * MiB + 682 * KiB + 7),
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
        [
            2 * KiB + 1,
            2 * KiB,
            4 * KiB,
            8 * KiB,
            16 * KiB,
            32 * KiB,
            64 * KiB,
            128 * KiB,
            256 * KiB,
            512 * KiB,
            1 * MiB,
            2 * MiB,
            4 * MiB - 1,
            -(10 * MiB + 682 * KiB + 6),
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
        [
            512 * KiB + 1,
            256 * KiB,
            128 * KiB,
            128 * KiB - 1,
            512 * KiB + 1,
            256 * KiB,
            128 * KiB,
            128 * KiB - 1,
            1 * MiB + 1,
            512 * KiB,
            256 * KiB,
            256 * KiB - 1,
            2 * MiB + 1,
            1 * MiB,
            512 * KiB,
            -(45 * MiB + 12),
            0,
            0,
            0,
            0,
            0,
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
    ];
    let testcount: ::core::ffi::c_int = (::core::mem::size_of::<[[::core::ffi::c_int; 30]; 11]>())
        .wrapping_div(::core::mem::size_of::<[::core::ffi::c_int; 30]>())
        as ::core::ffi::c_int;
    let mut test_i: ::core::ffi::c_int = 0;
    while test_i < testcount {
        let mut fillsize: *const ::core::ffi::c_int =
            &raw const *(&raw const testcases as *const [::core::ffi::c_int; 30])
                .offset(test_i as isize) as *const ::core::ffi::c_int;
        set_subtest(
            b"#%d {%d %d %d %d ...}\0".as_ptr() as *const ::core::ffi::c_char,
            test_i,
            *fillsize.offset(0isize),
            *fillsize.offset(1isize),
            *fillsize.offset(2isize),
            *fillsize.offset(3isize),
        );
        let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        if parser.is_null() {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                6214i32,
                b"check failed: parser != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        XML_SetUserData(parser, &raw mut storage as *mut ::core::ffi::c_void);
        XML_SetStartElementHandler(
            parser,
            Some(
                start_element_event_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut *const XML_Char,
                    ) -> (),
            ),
        );
        g_bytesScanned = 0;
        let mut worstcase_bytes: ::core::ffi::c_int = 0;
        let mut offset: ::core::ffi::c_int = 0;
        while *fillsize >= 0 {
            if !(offset + *fillsize <= document_length) {
                _fail(
                    b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    6225i32,
                    b"check failed: offset + *fillsize <= document_length\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            let status: XML_Status = XML_Parse(
                parser,
                document.offset(offset as isize),
                *fillsize,
                XML_FALSE as ::core::ffi::c_int,
            );
            if status != XML_STATUS_OK {
                _xml_failure(
                    parser,
                    b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    6229i32,
                );
            }
            offset += *fillsize;
            fillsize = fillsize.offset(1);
            if !(offset <= 2147483647 - worstcase_bytes) {
                _fail(
                    b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    6233i32,
                    b"check failed: offset <= INT_MAX - worstcase_bytes\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            worstcase_bytes += offset;
        }
        if !(storage.count == 1) {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                6236i32,
                b"check failed: storage.count == 1\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if !(g_bytesScanned > 0) {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                6237i32,
                b"check failed: g_bytesScanned > 0\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if g_reparseDeferralEnabledDefault != 0 {
            let max_bytes_scanned: ::core::ffi::c_uint = -*fillsize as ::core::ffi::c_uint;
            if g_bytesScanned > max_bytes_scanned {
                fprintf(
                    stderr,
                    b"bytes scanned in parse attempts: actual=%u limit=%u \n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    g_bytesScanned,
                    max_bytes_scanned,
                );
                _fail(
                    b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    6245i32,
                    b"too many bytes scanned in parse attempts\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        }
        if !(g_bytesScanned <= worstcase_bytes as ::core::ffi::c_uint) {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/basic_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                6248i32,
                b"check failed: g_bytesScanned <= (unsigned)worstcase_bytes\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        XML_ParserFree(parser);
        test_i += 1;
    }
    free(document as *mut ::core::ffi::c_void);
}
#[no_mangle]

pub unsafe extern "C" fn make_basic_test_case(mut s: *mut Suite) {
    let mut tc_basic: *mut TCase =
        tcase_create(b"basic tests\0".as_ptr() as *const ::core::ffi::c_char);
    suite_add_tcase(s, tc_basic);
    tcase_add_checked_fixture(
        tc_basic,
        Some(basic_setup as unsafe extern "C" fn() -> ()),
        Some(basic_teardown as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_nul_byte as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_u0000_char as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_siphash_self as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_siphash_spec as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_bom_utf8 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_bom_utf16_be as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_bom_utf16_le as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_nobom_utf16_le as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_hash_collision as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_illegal_utf8 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_utf8_auto_align as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(tc_basic, Some(test_utf16 as unsafe extern "C" fn() -> ()));
    tcase_add_test(
        tc_basic,
        Some(test_utf16_le_epilog_newline as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_not_utf16 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_bad_encoding as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_latin1_umlauts as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_long_utf8_character as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_long_latin1_attribute as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_long_ascii_attribute as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_danish_latin1 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_french_charref_hexidecimal as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_french_charref_decimal as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_french_latin1 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_french_utf8 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_utf8_false_rejection as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_line_number_after_parse as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_column_number_after_parse as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_line_and_column_numbers_inside_handlers as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_line_number_after_error as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_column_number_after_error as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_really_long_lines as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_really_long_encoded_lines as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_end_element_events as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_helper_is_whitespace_normalized as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_attr_whitespace_normalization as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_xmldecl_misplaced as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_xmldecl_invalid as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_xmldecl_missing_attr as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_xmldecl_missing_value as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        tc_basic,
        Some(test_unknown_encoding_internal_entity as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_unrecognised_encoding_internal_entity as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_ext_entity_set_encoding as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_ext_entity_no_handler as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_ext_entity_set_bom as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_ext_entity_bad_encoding as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_ext_entity_bad_encoding_2 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_wfc_undeclared_entity_unread_external_subset as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_wfc_undeclared_entity_no_external_subset as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_wfc_undeclared_entity_standalone as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(
            test_wfc_undeclared_entity_with_external_subset_standalone
                as unsafe extern "C" fn() -> (),
        ),
    );
    tcase_add_test(
        tc_basic,
        Some(test_entity_with_external_subset_unless_standalone as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_wfc_undeclared_entity_with_external_subset as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_not_standalone_handler_reject as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_not_standalone_handler_accept as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_entity_start_tag_level_greater_than_one as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        tc_basic,
        Some(test_wfc_no_recursive_entity_refs as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_no_indirectly_recursive_entity_refs as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_ext_entity_invalid_parse as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        tc_basic,
        Some(test_dtd_default_handling as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_dtd_attr_handling as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_empty_ns_without_namespaces as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_ns_in_attribute_default_without_namespaces as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_stop_parser_between_char_data_calls as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_suspend_parser_between_char_data_calls as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_repeated_stop_parser_between_char_data_calls as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_good_cdata_ascii as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_good_cdata_utf16 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_good_cdata_utf16_le as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_long_cdata_utf16 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_multichar_cdata_utf16 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_utf16_bad_surrogate_pair as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_bad_cdata as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_bad_cdata_utf16 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_stop_parser_between_cdata_calls as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_suspend_parser_between_cdata_calls as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_memory_allocation as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        tc_basic,
        Some(test_default_current as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_dtd_elements as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_dtd_elements_nesting as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_set_foreign_dtd as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_foreign_dtd_not_standalone as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_invalid_foreign_dtd as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_foreign_dtd_with_doctype as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_foreign_dtd_without_external_subset as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_empty_foreign_dtd as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_set_base as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_attributes as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        tc_basic,
        Some(test_reset_in_entity as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_resume_invalid_parse as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_resume_resuspended as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_cdata_default as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_subordinate_reset as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_subordinate_suspend as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        tc_basic,
        Some(test_subordinate_xdecl_suspend as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        tc_basic,
        Some(test_subordinate_xdecl_abort as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_ext_entity_invalid_suspended_parse as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_explicit_encoding as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_trailing_cr as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        tc_basic,
        Some(test_ext_entity_trailing_cr as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_trailing_rsqb as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        tc_basic,
        Some(test_ext_entity_trailing_rsqb as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        tc_basic,
        Some(test_ext_entity_good_cdata as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_user_parameters as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_ext_entity_ref_parameter as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_empty_parse as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_negative_len_parse as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_negative_len_parse_buffer as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_get_buffer_1 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_get_buffer_2 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_get_buffer_3_overflow as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_buffer_can_grow_to_max as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_getbuffer_allocates_on_zero_len as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_byte_info_at_end as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_byte_info_at_error as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_byte_info_at_cdata as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_predefined_entities as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_invalid_tag_in_dtd as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_not_predefined_entities as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_ignore_section as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_ignore_section_utf16 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_ignore_section_utf16_be as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_bad_ignore_section as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_external_bom_consumed as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_external_entity_values as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_ext_entity_not_standalone as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_ext_entity_value_abort as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_bad_public_doctype as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_attribute_enum_value as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_predefined_entity_redefinition as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_dtd_stop_processing as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_public_notation_no_sysid as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_nested_groups as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_group_choice as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_standalone_parameter_entity as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_skipped_parameter_entity as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_recursive_external_parameter_entity as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_recursive_external_parameter_entity_2 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_undefined_ext_entity_in_external_dtd as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_suspend_xdecl as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_abort_epilog as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_abort_epilog_2 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_suspend_epilog as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_suspend_in_sole_empty_tag as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_unfinished_epilog as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_partial_char_in_epilog as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_suspend_resume_internal_entity as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_suspend_resume_internal_entity_issue_629 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_resume_entity_with_syntax_error as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_suspend_resume_parameter_entity as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_restart_on_error as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_reject_lt_in_attribute_value as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_reject_unfinished_param_in_att_value as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_trailing_cr_in_att_value as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_standalone_internal_entity as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_skipped_external_entity as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_skipped_null_loaded_ext_entity as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_skipped_unloaded_ext_entity as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_param_entity_with_trailing_cr as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        tc_basic,
        Some(test_invalid_character_entity as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        tc_basic,
        Some(test_invalid_character_entity_2 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        tc_basic,
        Some(test_invalid_character_entity_3 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        tc_basic,
        Some(test_invalid_character_entity_4 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_pi_handled_in_default as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_comment_handled_in_default as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(tc_basic, Some(test_pi_yml as unsafe extern "C" fn() -> ()));
    tcase_add_test(tc_basic, Some(test_pi_xnl as unsafe extern "C" fn() -> ()));
    tcase_add_test(tc_basic, Some(test_pi_xmm as unsafe extern "C" fn() -> ()));
    tcase_add_test(
        tc_basic,
        Some(test_utf16_pi as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_utf16_be_pi as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_utf16_be_comment as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_utf16_le_comment as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_missing_encoding_conversion_fn as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_failing_encoding_conversion_fn as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_unknown_encoding_success as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_unknown_encoding_bad_name as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_unknown_encoding_bad_name_2 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_unknown_encoding_long_name_1 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_unknown_encoding_long_name_2 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_invalid_unknown_encoding as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_unknown_ascii_encoding_ok as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_unknown_ascii_encoding_fail as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_unknown_encoding_invalid_length as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_unknown_encoding_invalid_topbit as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_unknown_encoding_invalid_surrogate as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_unknown_encoding_invalid_high as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_unknown_encoding_invalid_attr_value as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_unknown_encoding_user_data_primary as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_unknown_encoding_user_data_secondary as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        tc_basic,
        Some(test_ext_entity_latin1_utf16le_bom as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        tc_basic,
        Some(test_ext_entity_latin1_utf16be_bom as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        tc_basic,
        Some(test_ext_entity_latin1_utf16le_bom2 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        tc_basic,
        Some(test_ext_entity_latin1_utf16be_bom2 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        tc_basic,
        Some(test_ext_entity_utf16_be as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        tc_basic,
        Some(test_ext_entity_utf16_le as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        tc_basic,
        Some(test_ext_entity_utf16_unknown as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        tc_basic,
        Some(test_ext_entity_utf8_non_bom as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_utf8_in_cdata_section as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_utf8_in_cdata_section_2 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_utf8_in_start_tags as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_trailing_spaces_in_elements as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_utf16_attribute as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_utf16_second_attr as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_attr_after_solidus as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_utf16_pe as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_bad_attr_desc_keyword as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_bad_attr_desc_keyword_utf16 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_bad_doctype as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_bad_doctype_utf8 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_bad_doctype_utf16 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_bad_doctype_plus as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_bad_doctype_star as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_bad_doctype_query as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_unknown_encoding_bad_ignore as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_entity_in_utf16_be_attr as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_entity_in_utf16_le_attr as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_entity_public_utf16_be as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_entity_public_utf16_le as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_short_doctype as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_short_doctype_2 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_short_doctype_3 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_long_doctype as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_bad_entity as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_bad_entity_2 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_bad_entity_3 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_bad_entity_4 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_bad_notation as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_default_doctype_handler as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_empty_element_abort as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_basic,
        Some(test_pool_integrity_with_unfinished_attr as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        tc_basic,
        Some(test_entity_ref_no_elements as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        tc_basic,
        Some(test_deep_nested_entity as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        tc_basic,
        Some(test_deep_nested_attribute_entity as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        tc_basic,
        Some(test_deep_nested_entity_delayed_interpretation as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        tc_basic,
        Some(test_nested_entity_suspend as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        tc_basic,
        Some(test_nested_entity_suspend_2 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_big_tokens_scale_linearly as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_set_reparse_deferral as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_reparse_deferral_is_inherited as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_set_reparse_deferral_on_null_parser as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_set_reparse_deferral_on_the_fly as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_set_bad_reparse_option as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_bypass_heuristic_when_close_to_bufsize as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_basic,
        Some(test_varying_buffer_fills as unsafe extern "C" fn() -> ()),
    );
}
