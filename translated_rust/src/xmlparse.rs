use ::c2rust_bitfields;
use ::std::os::unix::ffi::OsStrExt;
use ::std::sync::atomic::{AtomicU32, AtomicU8, Ordering};

pub mod siphash_h {

    fn load_u64_le(bytes: &[::core::ffi::c_uchar]) -> crate::stdlib::uint64_t {
        (bytes[0] as crate::stdlib::uint64_t)
            | (bytes[1] as crate::stdlib::uint64_t) << 8
            | (bytes[2] as crate::stdlib::uint64_t) << 16
            | (bytes[3] as crate::stdlib::uint64_t) << 24
            | (bytes[4] as crate::stdlib::uint64_t) << 32
            | (bytes[5] as crate::stdlib::uint64_t) << 40
            | (bytes[6] as crate::stdlib::uint64_t) << 48
            | (bytes[7] as crate::stdlib::uint64_t) << 56
    }

    fn new_state() -> crate::siphash_h::siphash {
        crate::siphash_h::siphash {
            v0: 0,
            v1: 0,
            v2: 0,
            v3: 0,
            buf: [0; 8],
            p: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
            c: 0,
        }
    }

    fn buffered_len(H: &crate::siphash_h::siphash) -> usize {
        let start = H.buf.as_ptr() as usize;
        (H.p as usize).wrapping_sub(start).min(H.buf.len())
    }

    fn set_buffered_len(H: &mut crate::siphash_h::siphash, len: usize) {
        H.p = H.buf.as_mut_ptr().wrapping_add(len);
    }

    pub fn sip_tokey<'a>(
        key: &'a mut crate::siphash_h::sipkey,
        src: &[::core::ffi::c_uchar],
    ) -> &'a mut crate::siphash_h::sipkey {
        key.k[0] = load_u64_le(&src[0..8]);
        key.k[1] = load_u64_le(&src[8..16]);
        key
    }

    pub fn sip_round(H: &mut crate::siphash_h::siphash, rounds: ::core::ffi::c_int) {
        for _ in 0..rounds {
            H.v0 = H.v0.wrapping_add(H.v1);
            H.v1 = H.v1.rotate_left(13);
            H.v1 ^= H.v0;
            H.v0 = H.v0.rotate_left(32);
            H.v2 = H.v2.wrapping_add(H.v3);
            H.v3 = H.v3.rotate_left(16);
            H.v3 ^= H.v2;
            H.v0 = H.v0.wrapping_add(H.v3);
            H.v3 = H.v3.rotate_left(21);
            H.v3 ^= H.v0;
            H.v2 = H.v2.wrapping_add(H.v1);
            H.v1 = H.v1.rotate_left(17);
            H.v1 ^= H.v2;
            H.v2 = H.v2.rotate_left(32);
        }
    }

    pub fn sip24_init<'a>(
        H: &'a mut crate::siphash_h::siphash,
        key: &crate::siphash_h::sipkey,
    ) -> &'a mut crate::siphash_h::siphash {
        H.v0 = 0x736f6d6570736575 as crate::stdlib::uint64_t ^ key.k[0];
        H.v1 = 0x646f72616e646f6d as crate::stdlib::uint64_t ^ key.k[1];
        H.v2 = 0x6c7967656e657261 as crate::stdlib::uint64_t ^ key.k[0];
        H.v3 = 0x7465646279746573 as crate::stdlib::uint64_t ^ key.k[1];
        set_buffered_len(H, 0);
        H.c = 0;
        H
    }

    pub fn sip24_update<'a>(
        H: &'a mut crate::siphash_h::siphash,
        src: &[::core::ffi::c_uchar],
    ) -> &'a mut crate::siphash_h::siphash {
        let mut input_pos = 0;
        loop {
            while input_pos < src.len() && buffered_len(H) < H.buf.len() {
                let buffered = buffered_len(H);
                H.buf[buffered] = src[input_pos];
                set_buffered_len(H, buffered + 1);
                input_pos += 1;
            }
            if buffered_len(H) < H.buf.len() {
                break;
            }
            let m = load_u64_le(&H.buf);
            H.v3 ^= m;
            sip_round(H, 2);
            H.v0 ^= m;
            set_buffered_len(H, 0);
            H.c = H.c.wrapping_add(8 as crate::stdlib::uint64_t);
            if input_pos >= src.len() {
                break;
            }
        }
        H
    }

    pub fn sip24_final(H: &mut crate::siphash_h::siphash) -> crate::stdlib::uint64_t {
        let left = buffered_len(H);
        let mut b = H.c.wrapping_add(left as crate::stdlib::uint64_t) << 56;
        for (i, byte) in H.buf[..left].iter().enumerate() {
            b |= (*byte as crate::stdlib::uint64_t) << (8 * i);
        }
        H.v3 ^= b;
        sip_round(H, 2);
        H.v0 ^= b;
        H.v2 ^= 0xff as crate::stdlib::uint64_t;
        sip_round(H, 4);
        H.v0 ^ H.v1 ^ H.v2 ^ H.v3
    }

    pub fn siphash24(
        src: &[::core::ffi::c_uchar],
        key: &crate::siphash_h::sipkey,
    ) -> crate::stdlib::uint64_t {
        let mut state = new_state();
        sip24_init(&mut state, key);
        sip24_update(&mut state, src);
        sip24_final(&mut state)
    }

    pub fn sip24_valid() -> ::core::ffi::c_int {
        pub static vectors: [[::core::ffi::c_uchar; 8]; 64] = [
            [
                0x31 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xe as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xe as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xdd as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x47 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xdb as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x6f as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x72 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0xfd as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x67 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xdc as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x93 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xc5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x39 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xf8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x74 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x5a as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x4f as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xa9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xd9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x80 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x6c as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xd as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x2d as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x7e as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xfb as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xd7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x96 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x66 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x67 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x85 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0xb7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x87 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x71 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x94 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xcf as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x8d as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xa6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x99 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xcd as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x64 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x55 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x76 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x18 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0xce as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xe3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x58 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x6e as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x46 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xc9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xcb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x37 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xd1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x8b as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xf5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xab as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x62 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x93 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x9a as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x79 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xf5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xf5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x93 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0xb0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xe4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xa9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xb as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x82 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x9e as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0xf3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xb9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xdd as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x94 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xc5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xbb as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x5d as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x7a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0xa7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xad as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x6b as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x46 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x2f as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xb3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xf4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0xfb as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xe5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xe as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x86 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xbc as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x8f as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x1e as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x75 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x90 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x3d as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x84 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x56 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xea as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0xee as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xf2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x7a as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x8e as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x90 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xca as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x23 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xf7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0xe5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x45 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xbe as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x49 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x61 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xca as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x29 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0xdb as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x9b as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xc2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x57 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xcc as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x2a as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x94 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x47 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xbe as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x2c as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xf5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xe9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x9a as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x69 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x9c as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xd3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x8d as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x96 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xf0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xb3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x4b as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0xbd as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x61 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x79 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xa7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x1d as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xc9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x6d as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xbb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x98 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xee as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xa2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x1a as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xf2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x5c as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xd6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xbe as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0xc7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x67 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x3b as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x2e as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xb0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xcb as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xf2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xd0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x88 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x3e as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xa3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xe3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x95 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x67 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x53 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x93 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0xc8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xce as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x5c as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xcd as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x8c as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xa8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x94 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xaf as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x49 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xf6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xc6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x50 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xad as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xb8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0xea as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xb8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x85 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x8a as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xde as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x92 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xe1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xbc as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0xf3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xbb as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x5b as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xb8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x35 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xd8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x17 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0xad as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xcf as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x6b as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x63 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x61 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x2e as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x2f as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0xa5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xc9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x1d as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xa7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xac as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xaa as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x4d as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xde as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x71 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x65 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x95 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x87 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x66 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x50 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xa2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xa6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xef as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x49 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x5c as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x53 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xa3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x87 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xad as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x42 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xc3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x41 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xd8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xfa as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x92 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xd8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x32 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0xce as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x7c as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xf2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x72 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x2f as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x51 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x71 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0xe3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x78 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x59 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xf9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x46 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x23 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xf3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xa7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x38 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xbb as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x1a as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xb0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0xae as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x97 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xf as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xd4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x34 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xe0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0xb4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xa3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xbe as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x4d as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x31 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x81 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x39 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x62 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x29 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xf0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x90 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x79 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x4d as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xf4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x9e as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xe5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xd4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xdc as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xca as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x5c as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x33 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x6a as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x76 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xd8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xbf as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x9a as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0xd0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xa7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x53 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x6b as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xa9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x3e as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xe as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x92 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x59 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x58 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xfc as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xd6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x42 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xad as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0xa9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xc2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x9b as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xc8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x18 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x95 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x2b as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x79 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xf3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xbc as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xa as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xa6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xd4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0xf2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x1d as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xf2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xe4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x1d as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x45 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x35 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xf9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x87 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x57 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x75 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x19 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x8f as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x53 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xa9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xa5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x6c as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xf5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xdf as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xcd as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x9a as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xdb as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0xeb as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x75 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x5c as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xcd as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x98 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x6c as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xd0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x51 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xa9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xcb as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x9e as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xcb as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xa3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xe6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x96 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xaf as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xad as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xfc as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x2c as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xe6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x66 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xc7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x72 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x52 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x97 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x5a as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x43 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x64 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xee as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x5a as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x16 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x45 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xb2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x76 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xd5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x92 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0xb2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x74 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xcb as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x8e as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xbf as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x87 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x87 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xa as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x6f as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x9b as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xb4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x20 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x3d as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xe7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xb3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x81 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0xea as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xec as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xb2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xa3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xb as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xa8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x99 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xa4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x3c as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x31 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x57 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0xbd as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x83 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x8d as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x3a as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xaf as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xbf as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x8d as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xb7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0xb as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x1a as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x2a as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x32 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x65 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xd5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x1a as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xea as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x50 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x79 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xa3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x23 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x1c as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xe6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x60 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x93 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x2b as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x46 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xe4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xd7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x66 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0xe1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x91 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x5f as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x5c as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xb1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xec as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xa4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x6c as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0xf3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x25 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x96 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x5c as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x6d as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x62 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x9f as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x57 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x5f as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xf2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x8e as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x60 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x38 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x1b as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xe5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
            [
                0x72 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x45 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0xeb as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x4c as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x32 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x8a as ::core::ffi::c_int as ::core::ffi::c_uchar,
                0x95 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            ],
        ];
        let mut in_0: [::core::ffi::c_uchar; 64] = [0; 64];
        let mut k: crate::siphash_h::sipkey = crate::siphash_h::sipkey { k: [0; 2] };
        let mut i: crate::__stddef_size_t_h::size_t = 0;
        sip_tokey(
            &mut k,
            b"\0\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0B\x0C\r\x0E\x0F",
        );
        i = 0 as crate::__stddef_size_t_h::size_t;
        while i < ::core::mem::size_of::<[::core::ffi::c_uchar; 64]>() as usize {
            in_0[i as usize] = i as ::core::ffi::c_uchar;
            let expected = load_u64_le(&vectors[i as usize]);
            if siphash24(&in_0[..i], &k) != expected {
                return 0 as ::core::ffi::c_int;
            }
            i = i.wrapping_add(1);
        }
        return 1 as ::core::ffi::c_int;
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
use crate::stdlib::__assert_fail;
use crate::stdlib::__errno_location;
pub use crate::stdlib::XML_CONTEXT_BYTES;

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
pub use crate::src::xmlparse::siphash_h::sip24_final;
pub use crate::src::xmlparse::siphash_h::sip24_init;
pub use crate::src::xmlparse::siphash_h::sip24_update;
pub use crate::src::xmlparse::siphash_h::sip24_valid;
pub use crate::src::xmlparse::siphash_h::sip_round;
pub use crate::src::xmlparse::siphash_h::sip_tokey;
pub use crate::src::xmlparse::siphash_h::siphash24;
pub use crate::stdbool_h::false_0;
pub use crate::stdbool_h::true_0;
pub use crate::stdlib::uint64_t;
pub use crate::stdlib::SIZE_MAX;

pub use crate::src::xmlrole::prolog_state;
pub use crate::src::xmlrole::C2Rust_Unnamed_0;
pub use crate::src::xmlrole::XmlPrologStateInit;
pub use crate::src::xmlrole::XmlPrologStateInitExternalEntity;
pub use crate::src::xmlrole::PROLOG_STATE;
pub use crate::src::xmlrole::XML_ROLE_ATTLIST_ELEMENT_NAME;
pub use crate::src::xmlrole::XML_ROLE_ATTLIST_NONE;
pub use crate::src::xmlrole::XML_ROLE_ATTRIBUTE_ENUM_VALUE;
pub use crate::src::xmlrole::XML_ROLE_ATTRIBUTE_NAME;
pub use crate::src::xmlrole::XML_ROLE_ATTRIBUTE_NOTATION_VALUE;
pub use crate::src::xmlrole::XML_ROLE_ATTRIBUTE_TYPE_CDATA;
pub use crate::src::xmlrole::XML_ROLE_ATTRIBUTE_TYPE_ENTITIES;
pub use crate::src::xmlrole::XML_ROLE_ATTRIBUTE_TYPE_ENTITY;
pub use crate::src::xmlrole::XML_ROLE_ATTRIBUTE_TYPE_ID;
pub use crate::src::xmlrole::XML_ROLE_ATTRIBUTE_TYPE_IDREF;
pub use crate::src::xmlrole::XML_ROLE_ATTRIBUTE_TYPE_IDREFS;
pub use crate::src::xmlrole::XML_ROLE_ATTRIBUTE_TYPE_NMTOKEN;
pub use crate::src::xmlrole::XML_ROLE_ATTRIBUTE_TYPE_NMTOKENS;
pub use crate::src::xmlrole::XML_ROLE_COMMENT;
pub use crate::src::xmlrole::XML_ROLE_CONTENT_ANY;
pub use crate::src::xmlrole::XML_ROLE_CONTENT_ELEMENT;
pub use crate::src::xmlrole::XML_ROLE_CONTENT_ELEMENT_OPT;
pub use crate::src::xmlrole::XML_ROLE_CONTENT_ELEMENT_PLUS;
pub use crate::src::xmlrole::XML_ROLE_CONTENT_ELEMENT_REP;
pub use crate::src::xmlrole::XML_ROLE_CONTENT_EMPTY;
pub use crate::src::xmlrole::XML_ROLE_CONTENT_PCDATA;
pub use crate::src::xmlrole::XML_ROLE_DEFAULT_ATTRIBUTE_VALUE;
pub use crate::src::xmlrole::XML_ROLE_DOCTYPE_CLOSE;
pub use crate::src::xmlrole::XML_ROLE_DOCTYPE_INTERNAL_SUBSET;
pub use crate::src::xmlrole::XML_ROLE_DOCTYPE_NAME;
pub use crate::src::xmlrole::XML_ROLE_DOCTYPE_NONE;
pub use crate::src::xmlrole::XML_ROLE_DOCTYPE_PUBLIC_ID;
pub use crate::src::xmlrole::XML_ROLE_DOCTYPE_SYSTEM_ID;
pub use crate::src::xmlrole::XML_ROLE_ELEMENT_NAME;
pub use crate::src::xmlrole::XML_ROLE_ELEMENT_NONE;
pub use crate::src::xmlrole::XML_ROLE_ENTITY_COMPLETE;
pub use crate::src::xmlrole::XML_ROLE_ENTITY_NONE;
pub use crate::src::xmlrole::XML_ROLE_ENTITY_NOTATION_NAME;
pub use crate::src::xmlrole::XML_ROLE_ENTITY_PUBLIC_ID;
pub use crate::src::xmlrole::XML_ROLE_ENTITY_SYSTEM_ID;
pub use crate::src::xmlrole::XML_ROLE_ENTITY_VALUE;
pub use crate::src::xmlrole::XML_ROLE_ERROR;
pub use crate::src::xmlrole::XML_ROLE_FIXED_ATTRIBUTE_VALUE;
pub use crate::src::xmlrole::XML_ROLE_GENERAL_ENTITY_NAME;
pub use crate::src::xmlrole::XML_ROLE_GROUP_CHOICE;
pub use crate::src::xmlrole::XML_ROLE_GROUP_CLOSE;
pub use crate::src::xmlrole::XML_ROLE_GROUP_CLOSE_OPT;
pub use crate::src::xmlrole::XML_ROLE_GROUP_CLOSE_PLUS;
pub use crate::src::xmlrole::XML_ROLE_GROUP_CLOSE_REP;
pub use crate::src::xmlrole::XML_ROLE_GROUP_OPEN;
pub use crate::src::xmlrole::XML_ROLE_GROUP_SEQUENCE;
pub use crate::src::xmlrole::XML_ROLE_IGNORE_SECT;
pub use crate::src::xmlrole::XML_ROLE_IMPLIED_ATTRIBUTE_VALUE;
pub use crate::src::xmlrole::XML_ROLE_INNER_PARAM_ENTITY_REF;
pub use crate::src::xmlrole::XML_ROLE_INSTANCE_START;
pub use crate::src::xmlrole::XML_ROLE_NONE;
pub use crate::src::xmlrole::XML_ROLE_NOTATION_NAME;
pub use crate::src::xmlrole::XML_ROLE_NOTATION_NONE;
pub use crate::src::xmlrole::XML_ROLE_NOTATION_NO_SYSTEM_ID;
pub use crate::src::xmlrole::XML_ROLE_NOTATION_PUBLIC_ID;
pub use crate::src::xmlrole::XML_ROLE_NOTATION_SYSTEM_ID;
pub use crate::src::xmlrole::XML_ROLE_PARAM_ENTITY_NAME;
pub use crate::src::xmlrole::XML_ROLE_PARAM_ENTITY_REF;
pub use crate::src::xmlrole::XML_ROLE_PI;
pub use crate::src::xmlrole::XML_ROLE_REQUIRED_ATTRIBUTE_VALUE;
pub use crate::src::xmlrole::XML_ROLE_TEXT_DECL;
pub use crate::src::xmlrole::XML_ROLE_XML_DECL;
pub use crate::src::xmltok::encoding;
pub use crate::src::xmltok::position;
pub use crate::src::xmltok::xmltok_ns_c::XmlGetUtf8InternalEncoding;
pub use crate::src::xmltok::xmltok_ns_c::XmlGetUtf8InternalEncodingNS;
pub use crate::src::xmltok::xmltok_ns_c::XmlInitEncoding;
pub use crate::src::xmltok::xmltok_ns_c::XmlInitEncodingNS;
pub use crate::src::xmltok::xmltok_ns_c::XmlParseXmlDecl;
pub use crate::src::xmltok::xmltok_ns_c::XmlParseXmlDeclNS;
pub use crate::src::xmltok::XML_Convert_Result;
pub use crate::src::xmltok::XmlInitUnknownEncoding;
pub use crate::src::xmltok::XmlInitUnknownEncodingNS;
pub use crate::src::xmltok::XmlSizeOfUnknownEncoding;
pub use crate::src::xmltok::XmlUtf8Encode;
pub use crate::src::xmltok::ATTRIBUTE;
pub use crate::src::xmltok::CONVERTER;
pub use crate::src::xmltok::ENCODING;
pub use crate::src::xmltok::INIT_ENCODING;
pub use crate::src::xmltok::POSITION;
pub use crate::src::xmltok::SCANNER;
pub use crate::src::xmltok::XML_CONVERT_COMPLETED;
pub use crate::src::xmltok::XML_CONVERT_INPUT_INCOMPLETE;
pub use crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
pub use crate::src::xmltok::XML_TOK_ATTRIBUTE_VALUE_S;
pub use crate::src::xmltok::XML_TOK_BOM;
pub use crate::src::xmltok::XML_TOK_CDATA_SECT_CLOSE;
pub use crate::src::xmltok::XML_TOK_CDATA_SECT_OPEN;
pub use crate::src::xmltok::XML_TOK_CHAR_REF;
pub use crate::src::xmltok::XML_TOK_COMMENT;
pub use crate::src::xmltok::XML_TOK_DATA_CHARS;
pub use crate::src::xmltok::XML_TOK_DATA_NEWLINE;
pub use crate::src::xmltok::XML_TOK_EMPTY_ELEMENT_NO_ATTS;
pub use crate::src::xmltok::XML_TOK_EMPTY_ELEMENT_WITH_ATTS;
pub use crate::src::xmltok::XML_TOK_END_TAG;
pub use crate::src::xmltok::XML_TOK_ENTITY_REF;
pub use crate::src::xmltok::XML_TOK_IGNORE_SECT;
pub use crate::src::xmltok::XML_TOK_INSTANCE_START;
pub use crate::src::xmltok::XML_TOK_INVALID;
pub use crate::src::xmltok::XML_TOK_NONE;
pub use crate::src::xmltok::XML_TOK_PARAM_ENTITY_REF;
pub use crate::src::xmltok::XML_TOK_PARTIAL;
pub use crate::src::xmltok::XML_TOK_PARTIAL_CHAR;
pub use crate::src::xmltok::XML_TOK_PI;
pub use crate::src::xmltok::XML_TOK_PROLOG_S;
pub use crate::src::xmltok::XML_TOK_START_TAG_NO_ATTS;
pub use crate::src::xmltok::XML_TOK_START_TAG_WITH_ATTS;
pub use crate::src::xmltok::XML_TOK_TRAILING_CR;
pub use crate::src::xmltok::XML_TOK_TRAILING_RSQB;
pub use crate::src::xmltok::XML_TOK_XML_DECL;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::__uint64_t;
use crate::stdlib::arc4random_buf;
use crate::stdlib::free;
use crate::stdlib::getenv;
use crate::stdlib::malloc;
use crate::stdlib::memcmp;
use crate::stdlib::memcpy;
use crate::stdlib::memmove;
use crate::stdlib::memset;
use crate::stdlib::realloc;
use crate::stdlib::strtoul;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct XML_ParserStruct {
    pub m_userData: *mut ::core::ffi::c_void,
    pub m_handlerArg: *mut ::core::ffi::c_void,
    pub m_buffer: *mut ::core::ffi::c_char,
    pub m_mem: crate::expat_h::XML_Memory_Handling_Suite,
    pub m_bufferPtr: *const ::core::ffi::c_char,
    pub m_bufferEnd: *mut ::core::ffi::c_char,
    pub m_bufferLim: *const ::core::ffi::c_char,
    pub m_parseEndByteIndex: crate::expat_external_h::XML_Index,
    pub m_parseEndPtr: *const ::core::ffi::c_char,
    pub m_partialTokenBytesBefore: crate::__stddef_size_t_h::size_t,
    pub m_reparseDeferralEnabled: crate::expat_h::XML_Bool,
    pub m_lastBufferRequestSize: ::core::ffi::c_int,
    pub m_dataBuf: *mut crate::expat_external_h::XML_Char,
    pub m_dataBufEnd: *mut crate::expat_external_h::XML_Char,
    pub m_startElementHandler: crate::expat_h::XML_StartElementHandler,
    pub m_endElementHandler: crate::expat_h::XML_EndElementHandler,
    pub m_characterDataHandler: crate::expat_h::XML_CharacterDataHandler,
    pub m_processingInstructionHandler: crate::expat_h::XML_ProcessingInstructionHandler,
    pub m_commentHandler: crate::expat_h::XML_CommentHandler,
    pub m_startCdataSectionHandler: crate::expat_h::XML_StartCdataSectionHandler,
    pub m_endCdataSectionHandler: crate::expat_h::XML_EndCdataSectionHandler,
    pub m_defaultHandler: crate::expat_h::XML_DefaultHandler,
    pub m_startDoctypeDeclHandler: crate::expat_h::XML_StartDoctypeDeclHandler,
    pub m_endDoctypeDeclHandler: crate::expat_h::XML_EndDoctypeDeclHandler,
    pub m_unparsedEntityDeclHandler: crate::expat_h::XML_UnparsedEntityDeclHandler,
    pub m_notationDeclHandler: crate::expat_h::XML_NotationDeclHandler,
    pub m_startNamespaceDeclHandler: crate::expat_h::XML_StartNamespaceDeclHandler,
    pub m_endNamespaceDeclHandler: crate::expat_h::XML_EndNamespaceDeclHandler,
    pub m_notStandaloneHandler: crate::expat_h::XML_NotStandaloneHandler,
    pub m_externalEntityRefHandler: crate::expat_h::XML_ExternalEntityRefHandler,
    pub m_externalEntityRefHandlerArg: crate::expat_h::XML_Parser,
    pub m_skippedEntityHandler: crate::expat_h::XML_SkippedEntityHandler,
    pub m_unknownEncodingHandler: crate::expat_h::XML_UnknownEncodingHandler,
    pub m_elementDeclHandler: crate::expat_h::XML_ElementDeclHandler,
    pub m_attlistDeclHandler: crate::expat_h::XML_AttlistDeclHandler,
    pub m_entityDeclHandler: crate::expat_h::XML_EntityDeclHandler,
    pub m_xmlDeclHandler: crate::expat_h::XML_XmlDeclHandler,
    pub m_encoding: *const crate::src::xmltok::ENCODING,
    pub m_initEncoding: crate::src::xmltok::INIT_ENCODING,
    pub m_internalEncoding: *const crate::src::xmltok::ENCODING,
    pub m_protocolEncodingName: *const crate::expat_external_h::XML_Char,
    pub m_ns: crate::expat_h::XML_Bool,
    pub m_ns_triplets: crate::expat_h::XML_Bool,
    pub m_unknownEncodingMem: *mut ::core::ffi::c_void,
    pub m_unknownEncodingData: *mut ::core::ffi::c_void,
    pub m_unknownEncodingHandlerData: *mut ::core::ffi::c_void,
    pub m_unknownEncodingRelease: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub m_prologState: crate::src::xmlrole::PROLOG_STATE,
    pub m_processor: Option<Processor>,
    pub m_errorCode: crate::expat_h::XML_Error,
    pub m_eventPtr: *const ::core::ffi::c_char,
    pub m_eventEndPtr: *const ::core::ffi::c_char,
    pub m_positionPtr: *const ::core::ffi::c_char,
    pub m_openInternalEntities: *mut OPEN_INTERNAL_ENTITY,
    pub m_freeInternalEntities: *mut OPEN_INTERNAL_ENTITY,
    pub m_openAttributeEntities: *mut OPEN_INTERNAL_ENTITY,
    pub m_freeAttributeEntities: *mut OPEN_INTERNAL_ENTITY,
    pub m_openValueEntities: *mut OPEN_INTERNAL_ENTITY,
    pub m_freeValueEntities: *mut OPEN_INTERNAL_ENTITY,
    pub m_defaultExpandInternalEntities: crate::expat_h::XML_Bool,
    pub m_tagLevel: ::core::ffi::c_int,
    pub m_declEntity: *mut ENTITY,
    pub m_doctypeName: *const crate::expat_external_h::XML_Char,
    pub m_doctypeSysid: *const crate::expat_external_h::XML_Char,
    pub m_doctypePubid: *const crate::expat_external_h::XML_Char,
    pub m_declAttributeType: *const crate::expat_external_h::XML_Char,
    pub m_declNotationName: *const crate::expat_external_h::XML_Char,
    pub m_declNotationPublicId: *const crate::expat_external_h::XML_Char,
    pub m_declElementType: *mut ELEMENT_TYPE,
    pub m_declAttributeId: *mut ATTRIBUTE_ID,
    pub m_declAttributeIsCdata: crate::expat_h::XML_Bool,
    pub m_declAttributeIsId: crate::expat_h::XML_Bool,
    pub m_dtd: *mut DTD,
    pub m_curBase: *const crate::expat_external_h::XML_Char,
    pub m_tagStack: *mut TAG,
    pub m_freeTagList: *mut TAG,
    pub m_inheritedBindings: *mut BINDING,
    pub m_freeBindingList: *mut BINDING,
    pub m_attsSize: ::core::ffi::c_int,
    pub m_nSpecifiedAtts: ::core::ffi::c_int,
    pub m_idAttIndex: ::core::ffi::c_int,
    pub m_atts: *mut crate::src::xmltok::ATTRIBUTE,
    pub m_nsAtts: *mut NS_ATT,
    pub m_nsAttsVersion: ::core::ffi::c_ulong,
    pub m_nsAttsPower: ::core::ffi::c_uchar,
    pub m_position: crate::src::xmltok::POSITION,
    pub m_tempPool: STRING_POOL,
    pub m_temp2Pool: STRING_POOL,
    pub m_groupConnector: *mut ::core::ffi::c_char,
    pub m_groupSize: ::core::ffi::c_uint,
    pub m_namespaceSeparator: crate::expat_external_h::XML_Char,
    pub m_parentParser: crate::expat_h::XML_Parser,
    pub m_parsingStatus: crate::expat_h::XML_ParsingStatus,
    pub m_isParamEntity: crate::expat_h::XML_Bool,
    pub m_useForeignDTD: crate::expat_h::XML_Bool,
    pub m_paramEntityParsing: crate::expat_h::XML_ParamEntityParsing,
    pub m_hash_secret_salt: ::core::ffi::c_ulong,
    pub m_accounting: ACCOUNTING,
    pub m_alloc_tracker: MALLOC_TRACKER,
    pub m_entity_stats: ENTITY_STATS,
    pub m_reenter: crate::expat_h::XML_Bool,
}

pub type ENTITY_STATS = entity_stats;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct entity_stats {
    pub countEverOpened: ::core::ffi::c_uint,
    pub currentDepth: ::core::ffi::c_uint,
    pub maximumDepthSeen: ::core::ffi::c_uint,
    pub debugLevel: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct MALLOC_TRACKER {
    pub bytesAllocated: XmlBigCount,
    pub peakBytesAllocated: XmlBigCount,
    pub debugLevel: ::core::ffi::c_ulong,
    pub maximumAmplificationFactor: ::core::ffi::c_float,
    pub activationThresholdBytes: XmlBigCount,
}

pub type XmlBigCount = ::core::ffi::c_ulonglong;

pub type ACCOUNTING = accounting;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct accounting {
    pub countBytesDirect: XmlBigCount,
    pub countBytesIndirect: XmlBigCount,
    pub debugLevel: ::core::ffi::c_ulong,
    pub maximumAmplificationFactor: ::core::ffi::c_float,
    pub activationThresholdBytes: ::core::ffi::c_ulonglong,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct STRING_POOL {
    pub blocks: *mut BLOCK,
    pub freeBlocks: *mut BLOCK,
    pub end: *const crate::expat_external_h::XML_Char,
    pub ptr: *mut crate::expat_external_h::XML_Char,
    pub start: *mut crate::expat_external_h::XML_Char,
    pub parser: crate::expat_h::XML_Parser,
}

pub type BLOCK = block;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct block {
    pub next: *mut block,
    pub size: ::core::ffi::c_int,
    pub s: [crate::expat_external_h::XML_Char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct NS_ATT {
    pub version: ::core::ffi::c_ulong,
    pub hash: ::core::ffi::c_ulong,
    pub uriName: *const crate::expat_external_h::XML_Char,
}

pub type BINDING = binding;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct binding {
    pub prefix: *mut prefix,
    pub nextTagBinding: *mut binding,
    pub prevPrefixBinding: *mut binding,
    pub attId: *const attribute_id,
    pub uri: *mut crate::expat_external_h::XML_Char,
    pub uriLen: ::core::ffi::c_int,
    pub uriAlloc: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct attribute_id {
    pub name: *mut crate::expat_external_h::XML_Char,
    pub prefix: *mut PREFIX,
    pub maybeTokenized: crate::expat_h::XML_Bool,
    pub xmlns: crate::expat_h::XML_Bool,
}

pub type PREFIX = prefix;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct prefix {
    pub name: *const crate::expat_external_h::XML_Char,
    pub binding: *mut BINDING,
}

pub type TAG = tag;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct tag {
    pub parent: *mut tag,
    pub rawName: *const ::core::ffi::c_char,
    pub rawNameLength: ::core::ffi::c_int,
    pub name: TAG_NAME,
    pub buf: C2Rust_Unnamed_1,
    pub bufEnd: *mut ::core::ffi::c_char,
    pub bindings: *mut BINDING,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub union C2Rust_Unnamed_1 {
    pub raw: *mut ::core::ffi::c_char,
    pub str: *mut crate::expat_external_h::XML_Char,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct TAG_NAME {
    pub str: *const crate::expat_external_h::XML_Char,
    pub localPart: *const crate::expat_external_h::XML_Char,
    pub prefix: *const crate::expat_external_h::XML_Char,
    pub strLen: ::core::ffi::c_int,
    pub uriLen: ::core::ffi::c_int,
    pub prefixLen: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct DTD {
    pub generalEntities: HASH_TABLE,
    pub elementTypes: HASH_TABLE,
    pub attributeIds: HASH_TABLE,
    pub prefixes: HASH_TABLE,
    pub pool: STRING_POOL,
    pub entityValuePool: STRING_POOL,
    pub keepProcessing: crate::expat_h::XML_Bool,
    pub hasParamEntityRefs: crate::expat_h::XML_Bool,
    pub standalone: crate::expat_h::XML_Bool,
    pub paramEntityRead: crate::expat_h::XML_Bool,
    pub paramEntities: HASH_TABLE,
    pub defaultPrefix: PREFIX,
    pub in_eldecl: crate::expat_h::XML_Bool,
    pub scaffold: *mut CONTENT_SCAFFOLD,
    pub contentStringLen: ::core::ffi::c_uint,
    pub scaffSize: ::core::ffi::c_uint,
    pub scaffCount: ::core::ffi::c_uint,
    pub scaffLevel: ::core::ffi::c_int,
    pub scaffIndex: *mut ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct CONTENT_SCAFFOLD {
    pub type_0: crate::expat_h::XML_Content_Type,
    pub quant: crate::expat_h::XML_Content_Quant,
    pub name: *const crate::expat_external_h::XML_Char,
    pub firstchild: ::core::ffi::c_int,
    pub lastchild: ::core::ffi::c_int,
    pub childcnt: ::core::ffi::c_int,
    pub nextsib: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct HASH_TABLE {
    pub v: *mut *mut NAMED,
    pub power: ::core::ffi::c_uchar,
    pub size: crate::__stddef_size_t_h::size_t,
    pub used: crate::__stddef_size_t_h::size_t,
    pub parser: crate::expat_h::XML_Parser,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct NAMED {
    pub name: KEY,
}

pub type KEY = *const crate::expat_external_h::XML_Char;

pub type ATTRIBUTE_ID = attribute_id;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct ELEMENT_TYPE {
    pub name: *const crate::expat_external_h::XML_Char,
    pub prefix: *mut PREFIX,
    pub idAtt: *const ATTRIBUTE_ID,
    pub nDefaultAtts: ::core::ffi::c_int,
    pub allocDefaultAtts: ::core::ffi::c_int,
    pub defaultAtts: *mut DEFAULT_ATTRIBUTE,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct DEFAULT_ATTRIBUTE {
    pub id: *const ATTRIBUTE_ID,
    pub isCdata: crate::expat_h::XML_Bool,
    pub value: *const crate::expat_external_h::XML_Char,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct ENTITY {
    pub name: *const crate::expat_external_h::XML_Char,
    pub textPtr: *const crate::expat_external_h::XML_Char,
    pub textLen: ::core::ffi::c_int,
    pub processed: ::core::ffi::c_int,
    pub systemId: *const crate::expat_external_h::XML_Char,
    pub base: *const crate::expat_external_h::XML_Char,
    pub publicId: *const crate::expat_external_h::XML_Char,
    pub notation: *const crate::expat_external_h::XML_Char,
    pub open: crate::expat_h::XML_Bool,
    pub hasMore: crate::expat_h::XML_Bool,
    pub is_param: crate::expat_h::XML_Bool,
    pub is_internal: crate::expat_h::XML_Bool,
}

pub type OPEN_INTERNAL_ENTITY = open_internal_entity;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct open_internal_entity {
    pub internalEventPtr: *const ::core::ffi::c_char,
    pub internalEventEndPtr: *const ::core::ffi::c_char,
    pub next: *mut open_internal_entity,
    pub entity: *mut ENTITY,
    pub startTagLevel: ::core::ffi::c_int,
    pub betweenDecl: crate::expat_h::XML_Bool,
    pub type_0: EntityType,
}

pub type EntityType = ::core::ffi::c_uint;

pub const ENTITY_VALUE: EntityType = 2;

pub const ENTITY_ATTRIBUTE: EntityType = 1;

pub const ENTITY_INTERNAL: EntityType = 0;

pub type Processor = unsafe extern "C" fn(
    crate::expat_h::XML_Parser,
    *const ::core::ffi::c_char,
    *const ::core::ffi::c_char,
    *mut *const ::core::ffi::c_char,
) -> crate::expat_h::XML_Error;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct HASH_TABLE_ITER {
    pub p: *mut *mut NAMED,
    pub end: *mut *mut NAMED,
}

pub type XML_Account = ::core::ffi::c_uint;

pub const XML_ACCOUNT_NONE: XML_Account = 2;

pub const XML_ACCOUNT_ENTITY_EXPANSION: XML_Account = 1;

pub const XML_ACCOUNT_DIRECT: XML_Account = 0;

const ACCOUNTING_ABORTING_EPILOG: &str = " ABORTING\n";

pub type ICHAR = ::core::ffi::c_char;

pub const INIT_TAG_BUF_SIZE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;

pub const INIT_DATA_BUF_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;

pub const INIT_ATTS_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;

pub const INIT_ATTS_VERSION: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;

pub const INIT_BLOCK_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;

pub const INIT_BUFFER_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;

pub const EXPAND_SPARE: ::core::ffi::c_int = 24 as ::core::ffi::c_int;

pub const INIT_SCAFFOLD_ELEMENTS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
#[no_mangle]

pub static g_reparseDeferralEnabledDefault: AtomicU8 = AtomicU8::new(crate::expat_h::XML_TRUE);
#[no_mangle]

pub static g_bytesScanned: AtomicU32 = AtomicU32::new(0 as ::core::ffi::c_uint);

fn expat_heap_stat(
    root_parser: &XML_ParserStruct,
    operator: ::core::ffi::c_char,
    abs_diff: XmlBigCount,
    new_total: XmlBigCount,
    peak_total: XmlBigCount,
    source_line: ::core::ffi::c_int,
) {
    let amplification: ::core::ffi::c_float =
        new_total as ::core::ffi::c_float / root_parser.m_accounting.countBytesDirect as f32;
    use std::io::Write as _;

    let _ = writeln!(
        std::io::stderr(),
        "expat: Allocations({:p}): Direct {:10}, allocated {}{:10} to {:10} ({:10} peak), amplification {:8.2} (xmlparse.c:{})",
        root_parser,
        root_parser.m_accounting.countBytesDirect,
        operator as u8 as char,
        abs_diff,
        new_total,
        peak_total,
        amplification,
        source_line,
    );
}

fn expat_heap_increase_tolerable(
    root_parser: &XML_ParserStruct,
    increase: XmlBigCount,
    source_line: ::core::ffi::c_int,
) -> bool {
    if increase <= 0 as XmlBigCount {
        std::process::abort();
    }
    let mut new_total: XmlBigCount = 0 as XmlBigCount;
    let mut tolerable: bool = crate::stdbool_h::true_0 != 0;
    if (-1 as ::core::ffi::c_int as XmlBigCount)
        .wrapping_sub(root_parser.m_alloc_tracker.bytesAllocated)
        < increase
    {
        tolerable = crate::stdbool_h::false_0 != 0;
    } else {
        new_total = root_parser
            .m_alloc_tracker
            .bytesAllocated
            .wrapping_add(increase);
        if new_total >= root_parser.m_alloc_tracker.activationThresholdBytes {
            if new_total <= 0 as XmlBigCount {
                std::process::abort();
            }
            let amplification: ::core::ffi::c_float = new_total as ::core::ffi::c_float
                / root_parser.m_accounting.countBytesDirect as ::core::ffi::c_float;
            if amplification > root_parser.m_alloc_tracker.maximumAmplificationFactor {
                tolerable = crate::stdbool_h::false_0 != 0;
            }
        }
    }
    if !tolerable && root_parser.m_alloc_tracker.debugLevel >= 1 as ::core::ffi::c_ulong {
        expat_heap_stat(
            root_parser,
            '+' as i32 as ::core::ffi::c_char,
            increase,
            new_total,
            new_total,
            source_line,
        );
    }
    return tolerable;
}
#[export_name = "expat_malloc"]
pub unsafe extern "C" fn expat_malloc(
    mut parser: crate::expat_h::XML_Parser,
    mut size: crate::__stddef_size_t_h::size_t,
    mut sourceLine: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    if (crate::stdlib::SIZE_MAX as crate::__stddef_size_t_h::size_t).wrapping_sub(size)
        < (::core::mem::size_of::<crate::__stddef_size_t_h::size_t>() as usize)
            .wrapping_add(crate::internal_h::EXPAT_MALLOC_PADDING)
    {
        return crate::__stddef_null_h::NULL;
    }
    let rootParser: crate::expat_h::XML_Parser =
        getRootParserOf(parser, ::core::ptr::null_mut::<::core::ffi::c_uint>())
            as crate::expat_h::XML_Parser;
    '_c2rust_label: {
        if (*rootParser).m_parentParser.is_null() {
        } else {
            crate::stdlib::__assert_fail(
                b"rootParser->m_parentParser == NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../../expat/lib/xmlparse.c\0".as_ptr() as *const ::core::ffi::c_char,
                858 as ::core::ffi::c_uint,
                b"void *expat_malloc(XML_Parser, size_t, int)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    let bytesToAllocate: crate::__stddef_size_t_h::size_t =
        (::core::mem::size_of::<crate::__stddef_size_t_h::size_t>()
            as crate::__stddef_size_t_h::size_t)
            .wrapping_add(crate::internal_h::EXPAT_MALLOC_PADDING)
            .wrapping_add(size);
    if (-1 as ::core::ffi::c_int as XmlBigCount)
        .wrapping_sub((*rootParser).m_alloc_tracker.bytesAllocated)
        < bytesToAllocate as XmlBigCount
    {
        return crate::__stddef_null_h::NULL;
    }
    if !expat_heap_increase_tolerable(&*rootParser, bytesToAllocate as XmlBigCount, sourceLine) {
        return crate::__stddef_null_h::NULL;
    }
    let mallocedPtr: *mut ::core::ffi::c_void = (*parser)
        .m_mem
        .malloc_fcn
        .expect("non-null function pointer")(
        bytesToAllocate
    ) as *mut ::core::ffi::c_void;
    if mallocedPtr.is_null() {
        return crate::__stddef_null_h::NULL;
    }
    *(mallocedPtr as *mut crate::__stddef_size_t_h::size_t) = size;
    (*rootParser).m_alloc_tracker.bytesAllocated = (*rootParser)
        .m_alloc_tracker
        .bytesAllocated
        .wrapping_add(bytesToAllocate as XmlBigCount);
    if (*rootParser).m_alloc_tracker.debugLevel >= 2 as ::core::ffi::c_ulong {
        if (*rootParser).m_alloc_tracker.bytesAllocated
            > (*rootParser).m_alloc_tracker.peakBytesAllocated
        {
            (*rootParser).m_alloc_tracker.peakBytesAllocated =
                (*rootParser).m_alloc_tracker.bytesAllocated;
        }
        expat_heap_stat(
            &*rootParser,
            '+' as i32 as ::core::ffi::c_char,
            bytesToAllocate as XmlBigCount,
            (*rootParser).m_alloc_tracker.bytesAllocated,
            (*rootParser).m_alloc_tracker.peakBytesAllocated,
            sourceLine,
        );
    }
    return (mallocedPtr as *mut ::core::ffi::c_char)
        .offset(::core::mem::size_of::<crate::__stddef_size_t_h::size_t>() as usize as isize)
        .offset(crate::internal_h::EXPAT_MALLOC_PADDING as isize)
        as *mut ::core::ffi::c_void;
}
#[export_name = "expat_free"]
pub unsafe extern "C" fn expat_free(
    mut parser: crate::expat_h::XML_Parser,
    mut ptr: *mut ::core::ffi::c_void,
    mut sourceLine: ::core::ffi::c_int,
) {
    '_c2rust_label: {
        if !parser.is_null() {
        } else {
            crate::stdlib::__assert_fail(
                b"parser != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../../expat/lib/xmlparse.c\0".as_ptr() as *const ::core::ffi::c_char,
                906 as ::core::ffi::c_uint,
                b"void expat_free(XML_Parser, void *, int)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if ptr.is_null() {
        return;
    }
    let rootParser: crate::expat_h::XML_Parser =
        getRootParserOf(parser, ::core::ptr::null_mut::<::core::ffi::c_uint>())
            as crate::expat_h::XML_Parser;
    '_c2rust_label_0: {
        if (*rootParser).m_parentParser.is_null() {
        } else {
            crate::stdlib::__assert_fail(
                b"rootParser->m_parentParser == NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../../expat/lib/xmlparse.c\0".as_ptr() as *const ::core::ffi::c_char,
                913 as ::core::ffi::c_uint,
                b"void expat_free(XML_Parser, void *, int)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    let mallocedPtr: *mut ::core::ffi::c_void = (ptr as *mut ::core::ffi::c_char)
        .offset(-(crate::internal_h::EXPAT_MALLOC_PADDING as isize))
        .offset(-(::core::mem::size_of::<crate::__stddef_size_t_h::size_t>() as usize as isize))
        as *mut ::core::ffi::c_void;
    let bytesAllocated: crate::__stddef_size_t_h::size_t =
        (::core::mem::size_of::<crate::__stddef_size_t_h::size_t>()
            as crate::__stddef_size_t_h::size_t)
            .wrapping_add(crate::internal_h::EXPAT_MALLOC_PADDING)
            .wrapping_add(*(mallocedPtr as *mut crate::__stddef_size_t_h::size_t));
    '_c2rust_label_1: {
        if (*rootParser).m_alloc_tracker.bytesAllocated >= bytesAllocated as XmlBigCount {
        } else {
            crate::stdlib::__assert_fail(
                b"rootParser->m_alloc_tracker.bytesAllocated >= bytesAllocated\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../../expat/lib/xmlparse.c\0".as_ptr() as *const ::core::ffi::c_char,
                922 as ::core::ffi::c_uint,
                b"void expat_free(XML_Parser, void *, int)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*rootParser).m_alloc_tracker.bytesAllocated = (*rootParser)
        .m_alloc_tracker
        .bytesAllocated
        .wrapping_sub(bytesAllocated as XmlBigCount);
    if (*rootParser).m_alloc_tracker.debugLevel >= 2 as ::core::ffi::c_ulong {
        expat_heap_stat(
            &*rootParser,
            '-' as i32 as ::core::ffi::c_char,
            bytesAllocated as XmlBigCount,
            (*rootParser).m_alloc_tracker.bytesAllocated,
            (*rootParser).m_alloc_tracker.peakBytesAllocated,
            sourceLine,
        );
    }
    (*parser).m_mem.free_fcn.expect("non-null function pointer")(mallocedPtr);
}
#[export_name = "expat_realloc"]
pub unsafe extern "C" fn expat_realloc(
    mut parser: crate::expat_h::XML_Parser,
    mut ptr: *mut ::core::ffi::c_void,
    mut size: crate::__stddef_size_t_h::size_t,
    mut sourceLine: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    '_c2rust_label: {
        if !parser.is_null() {
        } else {
            crate::stdlib::__assert_fail(
                b"parser != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../../expat/lib/xmlparse.c\0".as_ptr() as *const ::core::ffi::c_char,
                942 as ::core::ffi::c_uint,
                b"void *expat_realloc(XML_Parser, void *, size_t, int)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if ptr.is_null() {
        return expat_malloc(parser, size, sourceLine);
    }
    if size == 0 as crate::__stddef_size_t_h::size_t {
        expat_free(parser, ptr, sourceLine);
        return crate::__stddef_null_h::NULL;
    }
    let rootParser: crate::expat_h::XML_Parser =
        getRootParserOf(parser, ::core::ptr::null_mut::<::core::ffi::c_uint>())
            as crate::expat_h::XML_Parser;
    '_c2rust_label_0: {
        if (*rootParser).m_parentParser.is_null() {
        } else {
            crate::stdlib::__assert_fail(
                b"rootParser->m_parentParser == NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../../expat/lib/xmlparse.c\0".as_ptr() as *const ::core::ffi::c_char,
                954 as ::core::ffi::c_uint,
                b"void *expat_realloc(XML_Parser, void *, size_t, int)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    let mut mallocedPtr: *mut ::core::ffi::c_void = (ptr as *mut ::core::ffi::c_char)
        .offset(-(crate::internal_h::EXPAT_MALLOC_PADDING as isize))
        .offset(-(::core::mem::size_of::<crate::__stddef_size_t_h::size_t>() as usize as isize))
        as *mut ::core::ffi::c_void;
    let prevSize: crate::__stddef_size_t_h::size_t =
        *(mallocedPtr as *mut crate::__stddef_size_t_h::size_t);
    let isIncrease: bool = size > prevSize;
    let absDiff: crate::__stddef_size_t_h::size_t = if size > prevSize {
        size.wrapping_sub(prevSize)
    } else {
        prevSize.wrapping_sub(size)
    };
    if isIncrease {
        if !expat_heap_increase_tolerable(&*rootParser, absDiff as XmlBigCount, sourceLine) {
            return crate::__stddef_null_h::NULL;
        }
    }
    '_c2rust_label_1: {
        if (18446744073709551615 as usize)
            .wrapping_sub(::core::mem::size_of::<crate::__stddef_size_t_h::size_t>() as usize)
            .wrapping_sub(
                (::core::mem::size_of::<::core::ffi::c_longlong>() as usize).wrapping_sub(
                    ::core::mem::size_of::<crate::__stddef_size_t_h::size_t>() as usize,
                ),
            )
            >= size
        {
        } else {
            crate::stdlib::__assert_fail(
                b"SIZE_MAX - sizeof(size_t) - EXPAT_MALLOC_PADDING >= size\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../../expat/lib/xmlparse.c\0".as_ptr() as *const ::core::ffi::c_char,
                975 as ::core::ffi::c_uint,
                b"void *expat_realloc(XML_Parser, void *, size_t, int)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    mallocedPtr = (*parser)
        .m_mem
        .realloc_fcn
        .expect("non-null function pointer")(
        mallocedPtr,
        (::core::mem::size_of::<crate::__stddef_size_t_h::size_t>()
            as crate::__stddef_size_t_h::size_t)
            .wrapping_add(crate::internal_h::EXPAT_MALLOC_PADDING)
            .wrapping_add(size),
    );
    if mallocedPtr.is_null() {
        return crate::__stddef_null_h::NULL;
    }
    if isIncrease {
        '_c2rust_label_2: {
            if (-1 as ::core::ffi::c_int as XmlBigCount)
                .wrapping_sub((*rootParser).m_alloc_tracker.bytesAllocated)
                >= absDiff as XmlBigCount
            {
            } else {
                crate::stdlib::__assert_fail(
                    b"(XmlBigCount)-1 - rootParser->m_alloc_tracker.bytesAllocated >= absDiff\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../../expat/lib/xmlparse.c\0".as_ptr() as *const ::core::ffi::c_char,
                    988 as ::core::ffi::c_uint,
                    b"void *expat_realloc(XML_Parser, void *, size_t, int)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        };
        (*rootParser).m_alloc_tracker.bytesAllocated = (*rootParser)
            .m_alloc_tracker
            .bytesAllocated
            .wrapping_add(absDiff as XmlBigCount);
    } else {
        '_c2rust_label_3: {
            if (*rootParser).m_alloc_tracker.bytesAllocated >= absDiff as XmlBigCount {
            } else {
                crate::stdlib::__assert_fail(
                    b"rootParser->m_alloc_tracker.bytesAllocated >= absDiff\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../../expat/lib/xmlparse.c\0".as_ptr() as *const ::core::ffi::c_char,
                    991 as ::core::ffi::c_uint,
                    b"void *expat_realloc(XML_Parser, void *, size_t, int)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        };
        (*rootParser).m_alloc_tracker.bytesAllocated = (*rootParser)
            .m_alloc_tracker
            .bytesAllocated
            .wrapping_sub(absDiff as XmlBigCount);
    }
    if (*rootParser).m_alloc_tracker.debugLevel >= 2 as ::core::ffi::c_ulong {
        if (*rootParser).m_alloc_tracker.bytesAllocated
            > (*rootParser).m_alloc_tracker.peakBytesAllocated
        {
            (*rootParser).m_alloc_tracker.peakBytesAllocated =
                (*rootParser).m_alloc_tracker.bytesAllocated;
        }
        expat_heap_stat(
            &*rootParser,
            (if isIncrease as ::core::ffi::c_int != 0 {
                '+' as i32
            } else {
                '-' as i32
            }) as ::core::ffi::c_char,
            absDiff as XmlBigCount,
            (*rootParser).m_alloc_tracker.bytesAllocated,
            (*rootParser).m_alloc_tracker.peakBytesAllocated,
            sourceLine,
        );
    }
    *(mallocedPtr as *mut crate::__stddef_size_t_h::size_t) = size;
    return (mallocedPtr as *mut ::core::ffi::c_char)
        .offset(::core::mem::size_of::<crate::__stddef_size_t_h::size_t>() as usize as isize)
        .offset(crate::internal_h::EXPAT_MALLOC_PADDING as isize)
        as *mut ::core::ffi::c_void;
}
#[export_name = "XML_ParserCreate"]

pub unsafe extern "C" fn XML_ParserCreate_ffi(
    mut encodingName: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Parser {
    parserCreate(
        encodingName,
        ::core::ptr::null::<crate::expat_h::XML_Memory_Handling_Suite>(),
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
        ::core::ptr::null_mut::<DTD>(),
        ::core::ptr::null_mut::<XML_ParserStruct>(),
    )
}
#[export_name = "XML_ParserCreateNS"]

pub unsafe extern "C" fn XML_ParserCreateNS_ffi(
    mut encodingName: *const crate::expat_external_h::XML_Char,
    mut nsSep: crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Parser {
    let mut tmp: [crate::expat_external_h::XML_Char; 2] = [
        nsSep,
        0 as ::core::ffi::c_int as crate::expat_external_h::XML_Char,
    ];
    parserCreate(
        encodingName,
        ::core::ptr::null::<crate::expat_h::XML_Memory_Handling_Suite>(),
        &raw mut tmp as *mut crate::expat_external_h::XML_Char,
        ::core::ptr::null_mut::<DTD>(),
        ::core::ptr::null_mut::<XML_ParserStruct>(),
    )
}
static implicitContext: [crate::expat_external_h::XML_Char; 41] = [
    crate::ascii_h::ASCII_x as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_m as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_l as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_EQUALS as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_h as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_t as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_t as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_p as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_COLON as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_SLASH as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_SLASH as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_w as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_w as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_w as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_PERIOD as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_w as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_3 as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_PERIOD as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_o as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_r as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_g as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_SLASH as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_X as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_M as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_L as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_SLASH as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_1 as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_9 as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_9 as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_8 as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_SLASH as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_n as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_a as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_m as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_e as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_s as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_p as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_a as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_c as crate::expat_external_h::XML_Char,
    crate::ascii_h::ASCII_e as crate::expat_external_h::XML_Char,
    '\0' as i32 as crate::expat_external_h::XML_Char,
];

fn ENTROPY_DEBUG(label: &str, entropy: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    if getDebugLevel("EXPAT_ENTROPY_DEBUG", 0 as ::core::ffi::c_ulong) >= 1 as ::core::ffi::c_ulong
    {
        let size = ::core::mem::size_of::<::core::ffi::c_ulong>();
        eprintln!(
            "expat: Entropy: {} --> 0x{:0width$x} ({} bytes)",
            label,
            entropy,
            size,
            width = size * 2
        );
    }
    entropy
}

unsafe extern "C" fn callProcessor(
    mut parser: crate::expat_h::XML_Parser,
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut endPtr: *mut *const ::core::ffi::c_char,
) -> crate::expat_h::XML_Error {
    let have_now: crate::__stddef_size_t_h::size_t = (if !end.is_null() && !start.is_null() {
        end.offset_from(start) as ::core::ffi::c_long
    } else {
        0 as ::core::ffi::c_long
    }) as crate::__stddef_size_t_h::size_t;
    if (*parser).m_reparseDeferralEnabled as ::core::ffi::c_int != 0
        && (*parser).m_parsingStatus.finalBuffer == 0
    {
        let had_before: crate::__stddef_size_t_h::size_t = (*parser).m_partialTokenBytesBefore;
        let mut available_buffer: crate::__stddef_size_t_h::size_t =
            (if !(*parser).m_bufferPtr.is_null() && !(*parser).m_buffer.is_null() {
                (*parser).m_bufferPtr.offset_from((*parser).m_buffer) as ::core::ffi::c_long
            } else {
                0 as ::core::ffi::c_long
            }) as crate::__stddef_size_t_h::size_t;
        available_buffer = available_buffer.wrapping_sub(
            if available_buffer < 1024 as crate::__stddef_size_t_h::size_t {
                available_buffer
            } else {
                1024 as crate::__stddef_size_t_h::size_t
            },
        );
        available_buffer = available_buffer.wrapping_add(
            (if !(*parser).m_bufferLim.is_null() && !(*parser).m_bufferEnd.is_null() {
                (*parser).m_bufferLim.offset_from((*parser).m_bufferEnd) as ::core::ffi::c_long
            } else {
                0 as ::core::ffi::c_long
            }) as crate::__stddef_size_t_h::size_t,
        );
        let enough: bool = have_now
            >= (2 as crate::__stddef_size_t_h::size_t).wrapping_mul(had_before)
            || (*parser).m_lastBufferRequestSize as crate::__stddef_size_t_h::size_t
                > available_buffer;
        if !enough {
            *endPtr = start;
            return crate::expat_h::XML_ERROR_NONE;
        }
    }
    g_bytesScanned.fetch_add(have_now as ::core::ffi::c_uint, Ordering::Relaxed);
    let mut ret: crate::expat_h::XML_Error = crate::expat_h::XML_ERROR_NONE;
    *endPtr = start;
    loop {
        ret =
            (*parser).m_processor.expect("non-null function pointer")(parser, *endPtr, end, endPtr);
        if (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint
            != crate::expat_h::XML_PARSING as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*parser).m_reenter = crate::expat_h::XML_FALSE;
        }
        if (*parser).m_reenter == 0 {
            break;
        }
        (*parser).m_reenter = crate::expat_h::XML_FALSE;
        if ret as ::core::ffi::c_uint
            != crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return ret;
        }
    }
    if ret as ::core::ffi::c_uint
        == crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if *endPtr == start {
            (*parser).m_partialTokenBytesBefore = have_now;
        } else {
            (*parser).m_partialTokenBytesBefore = 0 as crate::__stddef_size_t_h::size_t;
        }
    }
    return ret;
}

unsafe extern "C" fn startParsing(
    mut parser: crate::expat_h::XML_Parser,
) -> crate::expat_h::XML_Bool {
    if (*parser).m_hash_secret_salt == 0 as ::core::ffi::c_ulong {
        let mut entropy: ::core::ffi::c_ulong = 0;
        arc4random_buf(
            &raw mut entropy as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_ulong>() as crate::__stddef_size_t_h::size_t,
        );
        (*parser).m_hash_secret_salt = ENTROPY_DEBUG("arc4random_buf", entropy);
    }
    if (*parser).m_ns != 0 {
        return setContext(parser, implicitContext.as_ptr());
    }
    return crate::expat_h::XML_TRUE;
}
#[export_name = "XML_ParserCreate_MM"]

pub unsafe extern "C" fn XML_ParserCreate_MM_ffi(
    mut encodingName: *const crate::expat_external_h::XML_Char,
    mut memsuite: *const crate::expat_h::XML_Memory_Handling_Suite,
    mut nameSep: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Parser {
    parserCreate(
        encodingName,
        memsuite,
        nameSep,
        ::core::ptr::null_mut::<DTD>(),
        ::core::ptr::null_mut::<XML_ParserStruct>(),
    )
}
unsafe extern "C" fn parserCreate(
    mut encodingName: *const crate::expat_external_h::XML_Char,
    mut memsuite: *const crate::expat_h::XML_Memory_Handling_Suite,
    mut nameSep: *const crate::expat_external_h::XML_Char,
    mut dtd: *mut DTD,
    mut parentParser: crate::expat_h::XML_Parser,
) -> crate::expat_h::XML_Parser {
    let mut parser: crate::expat_h::XML_Parser = ::core::ptr::null_mut::<XML_ParserStruct>();
    let increase: crate::__stddef_size_t_h::size_t = (::core::mem::size_of::<
        crate::__stddef_size_t_h::size_t,
    >() as crate::__stddef_size_t_h::size_t)
        .wrapping_add(crate::internal_h::EXPAT_MALLOC_PADDING)
        .wrapping_add(
            ::core::mem::size_of::<XML_ParserStruct>() as crate::__stddef_size_t_h::size_t
        );
    if !parentParser.is_null() {
        let rootParser: crate::expat_h::XML_Parser =
            getRootParserOf(parentParser, ::core::ptr::null_mut::<::core::ffi::c_uint>())
                as crate::expat_h::XML_Parser;
        if !expat_heap_increase_tolerable(
            &*rootParser,
            increase as XmlBigCount,
            1354 as ::core::ffi::c_int,
        ) {
            return ::core::ptr::null_mut::<XML_ParserStruct>();
        }
    }
    if !memsuite.is_null() {
        let mut mtemp: *mut crate::expat_h::XML_Memory_Handling_Suite =
            ::core::ptr::null_mut::<crate::expat_h::XML_Memory_Handling_Suite>();
        let sizeAndParser: *mut ::core::ffi::c_void =
            (*memsuite).malloc_fcn.expect("non-null function pointer")(
                (::core::mem::size_of::<crate::__stddef_size_t_h::size_t>()
                    as crate::__stddef_size_t_h::size_t)
                    .wrapping_add(crate::internal_h::EXPAT_MALLOC_PADDING)
                    .wrapping_add(::core::mem::size_of::<XML_ParserStruct>()
                        as crate::__stddef_size_t_h::size_t),
            ) as *mut ::core::ffi::c_void;
        if !sizeAndParser.is_null() {
            *(sizeAndParser as *mut crate::__stddef_size_t_h::size_t) =
                ::core::mem::size_of::<XML_ParserStruct>() as usize
                    as crate::__stddef_size_t_h::size_t;
            parser = (sizeAndParser as *mut ::core::ffi::c_char)
                .offset(
                    ::core::mem::size_of::<crate::__stddef_size_t_h::size_t>() as usize as isize,
                )
                .offset(crate::internal_h::EXPAT_MALLOC_PADDING as isize)
                as crate::expat_h::XML_Parser;
            mtemp = &raw const (*parser).m_mem as *mut crate::expat_h::XML_Memory_Handling_Suite;
            (*mtemp).malloc_fcn = (*memsuite).malloc_fcn;
            (*mtemp).realloc_fcn = (*memsuite).realloc_fcn;
            (*mtemp).free_fcn = (*memsuite).free_fcn;
        }
    } else {
        let mut mtemp_0: *mut crate::expat_h::XML_Memory_Handling_Suite =
            ::core::ptr::null_mut::<crate::expat_h::XML_Memory_Handling_Suite>();
        let sizeAndParser_0: *mut ::core::ffi::c_void = crate::stdlib::malloc(
            (::core::mem::size_of::<crate::__stddef_size_t_h::size_t>()
                as crate::__stddef_size_t_h::size_t)
                .wrapping_add(crate::internal_h::EXPAT_MALLOC_PADDING)
                .wrapping_add(
                    ::core::mem::size_of::<XML_ParserStruct>() as crate::__stddef_size_t_h::size_t
                ),
        ) as *mut ::core::ffi::c_void;
        if !sizeAndParser_0.is_null() {
            *(sizeAndParser_0 as *mut crate::__stddef_size_t_h::size_t) =
                ::core::mem::size_of::<XML_ParserStruct>() as usize
                    as crate::__stddef_size_t_h::size_t;
            parser = (sizeAndParser_0 as *mut ::core::ffi::c_char)
                .offset(
                    ::core::mem::size_of::<crate::__stddef_size_t_h::size_t>() as usize as isize,
                )
                .offset(crate::internal_h::EXPAT_MALLOC_PADDING as isize)
                as crate::expat_h::XML_Parser;
            mtemp_0 = &raw const (*parser).m_mem as *mut crate::expat_h::XML_Memory_Handling_Suite;
            (*mtemp_0).malloc_fcn = Some(
                crate::stdlib::malloc
                    as unsafe extern "C" fn(
                        crate::__stddef_size_t_h::size_t,
                    ) -> *mut ::core::ffi::c_void,
            )
                as Option<
                    unsafe extern "C" fn(
                        crate::__stddef_size_t_h::size_t,
                    ) -> *mut ::core::ffi::c_void,
                >;
            (*mtemp_0).realloc_fcn = Some(
                crate::stdlib::realloc
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        crate::__stddef_size_t_h::size_t,
                    ) -> *mut ::core::ffi::c_void,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        crate::__stddef_size_t_h::size_t,
                    ) -> *mut ::core::ffi::c_void,
                >;
            (*mtemp_0).free_fcn =
                Some(crate::stdlib::free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ())
                    as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
        }
    }
    if parser.is_null() {
        return parser;
    }
    crate::stdlib::memset(
        &raw mut (*parser).m_alloc_tracker as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<MALLOC_TRACKER>() as crate::__stddef_size_t_h::size_t,
    );
    if parentParser.is_null() {
        (*parser).m_alloc_tracker.debugLevel =
            getDebugLevel("EXPAT_MALLOC_DEBUG", 0 as ::core::ffi::c_ulong);
        (*parser).m_alloc_tracker.maximumAmplificationFactor =
            crate::internal_h::EXPAT_ALLOC_TRACKER_MAXIMUM_AMPLIFICATION_DEFAULT;
        (*parser).m_alloc_tracker.activationThresholdBytes =
            crate::internal_h::EXPAT_ALLOC_TRACKER_ACTIVATION_THRESHOLD_DEFAULT as XmlBigCount;
        (*parser).m_parentParser = ::core::ptr::null_mut::<XML_ParserStruct>();
        (*parser).m_accounting.countBytesDirect = 0 as XmlBigCount;
    } else {
        (*parser).m_parentParser = parentParser;
    }
    let rootParser_0: crate::expat_h::XML_Parser =
        getRootParserOf(parser, ::core::ptr::null_mut::<::core::ffi::c_uint>())
            as crate::expat_h::XML_Parser;
    '_c2rust_label: {
        if (*rootParser_0).m_parentParser.is_null() {
        } else {
            crate::stdlib::__assert_fail(
                b"rootParser->m_parentParser == NULL\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../../expat/lib/xmlparse.c\0".as_ptr() as *const ::core::ffi::c_char,
                1425 as ::core::ffi::c_uint,
                b"XML_Parser parserCreate(const XML_Char *, const XML_Memory_Handling_Suite *, const XML_Char *, DTD *, XML_Parser)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (18446744073709551615 as XmlBigCount)
            .wrapping_sub((*rootParser_0).m_alloc_tracker.bytesAllocated)
            >= increase as XmlBigCount
        {
        } else {
            crate::stdlib::__assert_fail(
                b"SIZE_MAX - rootParser->m_alloc_tracker.bytesAllocated >= increase\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../../expat/lib/xmlparse.c\0".as_ptr() as *const ::core::ffi::c_char,
                1426 as ::core::ffi::c_uint,
                b"XML_Parser parserCreate(const XML_Char *, const XML_Memory_Handling_Suite *, const XML_Char *, DTD *, XML_Parser)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    (*rootParser_0).m_alloc_tracker.bytesAllocated = (*rootParser_0)
        .m_alloc_tracker
        .bytesAllocated
        .wrapping_add(increase as XmlBigCount);
    if (*rootParser_0).m_alloc_tracker.debugLevel >= 2 as ::core::ffi::c_ulong {
        if (*rootParser_0).m_alloc_tracker.bytesAllocated
            > (*rootParser_0).m_alloc_tracker.peakBytesAllocated
        {
            (*rootParser_0).m_alloc_tracker.peakBytesAllocated =
                (*rootParser_0).m_alloc_tracker.bytesAllocated;
        }
        expat_heap_stat(
            &*rootParser_0,
            '+' as i32 as ::core::ffi::c_char,
            increase as XmlBigCount,
            (*rootParser_0).m_alloc_tracker.bytesAllocated,
            (*rootParser_0).m_alloc_tracker.peakBytesAllocated,
            1439 as ::core::ffi::c_int,
        );
    }
    (*parser).m_buffer = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*parser).m_bufferLim = ::core::ptr::null::<::core::ffi::c_char>();
    (*parser).m_attsSize = INIT_ATTS_SIZE;
    (*parser).m_atts = expat_malloc(
        parser,
        ((*parser).m_attsSize as crate::__stddef_size_t_h::size_t)
            .wrapping_mul(::core::mem::size_of::<crate::src::xmltok::ATTRIBUTE>()
                as crate::__stddef_size_t_h::size_t),
        1449 as ::core::ffi::c_int,
    ) as *mut crate::src::xmltok::ATTRIBUTE;
    if (*parser).m_atts.is_null() {
        expat_free(
            parser,
            parser as *mut ::core::ffi::c_void,
            1451 as ::core::ffi::c_int,
        );
        return ::core::ptr::null_mut::<XML_ParserStruct>();
    }
    (*parser).m_dataBuf = expat_malloc(
        parser,
        (1024 as crate::__stddef_size_t_h::size_t)
            .wrapping_mul(::core::mem::size_of::<crate::expat_external_h::XML_Char>()
                as crate::__stddef_size_t_h::size_t),
        1462 as ::core::ffi::c_int,
    ) as *mut crate::expat_external_h::XML_Char;
    if (*parser).m_dataBuf.is_null() {
        expat_free(
            parser,
            (*parser).m_atts as *mut ::core::ffi::c_void,
            1464 as ::core::ffi::c_int,
        );
        expat_free(
            parser,
            parser as *mut ::core::ffi::c_void,
            1468 as ::core::ffi::c_int,
        );
        return ::core::ptr::null_mut::<XML_ParserStruct>();
    }
    (*parser).m_dataBufEnd = (*parser).m_dataBuf.offset(INIT_DATA_BUF_SIZE as isize);
    if !dtd.is_null() {
        (*parser).m_dtd = dtd;
    } else {
        (*parser).m_dtd = dtdCreate(parser);
        if (*parser).m_dtd.is_null() {
            expat_free(
                parser,
                (*parser).m_dataBuf as *mut ::core::ffi::c_void,
                1478 as ::core::ffi::c_int,
            );
            expat_free(
                parser,
                (*parser).m_atts as *mut ::core::ffi::c_void,
                1479 as ::core::ffi::c_int,
            );
            expat_free(
                parser,
                parser as *mut ::core::ffi::c_void,
                1483 as ::core::ffi::c_int,
            );
            return ::core::ptr::null_mut::<XML_ParserStruct>();
        }
    }
    (*parser).m_freeBindingList = ::core::ptr::null_mut::<BINDING>();
    (*parser).m_freeTagList = ::core::ptr::null_mut::<TAG>();
    (*parser).m_freeInternalEntities = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
    (*parser).m_freeAttributeEntities = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
    (*parser).m_freeValueEntities = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
    (*parser).m_groupSize = 0 as ::core::ffi::c_uint;
    (*parser).m_groupConnector = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*parser).m_unknownEncodingHandler = None;
    (*parser).m_unknownEncodingHandlerData = crate::__stddef_null_h::NULL;
    (*parser).m_namespaceSeparator =
        crate::ascii_h::ASCII_EXCL as crate::expat_external_h::XML_Char;
    (*parser).m_ns = crate::expat_h::XML_FALSE;
    (*parser).m_ns_triplets = crate::expat_h::XML_FALSE;
    (*parser).m_nsAtts = ::core::ptr::null_mut::<NS_ATT>();
    (*parser).m_nsAttsVersion = 0 as ::core::ffi::c_ulong;
    (*parser).m_nsAttsPower = 0 as ::core::ffi::c_uchar;
    (*parser).m_protocolEncodingName = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    poolInit(&mut (*parser).m_tempPool, parser);
    poolInit(&mut (*parser).m_temp2Pool, parser);
    parserInit(parser, encodingName);
    if !encodingName.is_null() && (*parser).m_protocolEncodingName.is_null() {
        if !dtd.is_null() {
            (*parser).m_dtd = ::core::ptr::null_mut::<DTD>();
        }
        XML_ParserFree_ffi(parser);
        return ::core::ptr::null_mut::<XML_ParserStruct>();
    }
    if !nameSep.is_null() {
        (*parser).m_ns = crate::expat_h::XML_TRUE;
        (*parser).m_internalEncoding =
            crate::src::xmltok::xmltok_ns_c::XmlGetUtf8InternalEncodingNS()
                as *const crate::src::xmltok::encoding;
        (*parser).m_namespaceSeparator = *nameSep;
    } else {
        (*parser).m_internalEncoding = crate::src::xmltok::xmltok_ns_c::XmlGetUtf8InternalEncoding()
            as *const crate::src::xmltok::encoding;
    }
    return parser;
}

unsafe extern "C" fn parserInit(
    mut parser: crate::expat_h::XML_Parser,
    mut encodingName: *const crate::expat_external_h::XML_Char,
) {
    (*parser).m_processor = Some(
        prologInitProcessor
            as unsafe extern "C" fn(
                crate::expat_h::XML_Parser,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut *const ::core::ffi::c_char,
            ) -> crate::expat_h::XML_Error,
    );
    crate::src::xmlrole::XmlPrologStateInit(&mut (*parser).m_prologState);
    if !encodingName.is_null() {
        let mut chars_required: crate::__stddef_size_t_h::size_t =
            0 as crate::__stddef_size_t_h::size_t;
        while *encodingName.offset(chars_required as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            chars_required = chars_required.wrapping_add(1);
        }
        chars_required = chars_required.wrapping_add(1);
        (*parser).m_protocolEncodingName = expat_malloc(
            parser,
            chars_required
                .wrapping_mul(::core::mem::size_of::<crate::expat_external_h::XML_Char>()
                    as crate::__stddef_size_t_h::size_t),
            8456 as ::core::ffi::c_int,
        ) as *mut crate::expat_external_h::XML_Char;
        if !(*parser).m_protocolEncodingName.is_null() {
            crate::stdlib::memcpy(
                (*parser).m_protocolEncodingName as *mut ::core::ffi::c_void,
                encodingName as *const ::core::ffi::c_void,
                chars_required
                    .wrapping_mul(::core::mem::size_of::<crate::expat_external_h::XML_Char>()
                        as crate::__stddef_size_t_h::size_t),
            );
        }
    }
    (*parser).m_curBase = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    crate::src::xmltok::xmltok_ns_c::XmlInitEncoding(
        &mut (*parser).m_initEncoding,
        &mut (*parser).m_encoding,
        None,
    );
    (*parser).m_userData = crate::__stddef_null_h::NULL;
    (*parser).m_handlerArg = crate::__stddef_null_h::NULL;
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
    (*parser).m_parseEndByteIndex = 0 as crate::expat_external_h::XML_Index;
    (*parser).m_parseEndPtr = ::core::ptr::null::<::core::ffi::c_char>();
    (*parser).m_partialTokenBytesBefore = 0 as crate::__stddef_size_t_h::size_t;
    (*parser).m_reparseDeferralEnabled = g_reparseDeferralEnabledDefault.load(Ordering::Relaxed);
    (*parser).m_lastBufferRequestSize = 0 as ::core::ffi::c_int;
    (*parser).m_declElementType = ::core::ptr::null_mut::<ELEMENT_TYPE>();
    (*parser).m_declAttributeId = ::core::ptr::null_mut::<ATTRIBUTE_ID>();
    (*parser).m_declEntity = ::core::ptr::null_mut::<ENTITY>();
    (*parser).m_doctypeName = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    (*parser).m_doctypeSysid = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    (*parser).m_doctypePubid = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    (*parser).m_declAttributeType = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    (*parser).m_declNotationName = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    (*parser).m_declNotationPublicId = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    (*parser).m_declAttributeIsCdata = crate::expat_h::XML_FALSE;
    (*parser).m_declAttributeIsId = crate::expat_h::XML_FALSE;
    crate::stdlib::memset(
        &raw mut (*parser).m_position as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<crate::src::xmltok::POSITION>() as crate::__stddef_size_t_h::size_t,
    );
    (*parser).m_errorCode = crate::expat_h::XML_ERROR_NONE;
    (*parser).m_eventPtr = ::core::ptr::null::<::core::ffi::c_char>();
    (*parser).m_eventEndPtr = ::core::ptr::null::<::core::ffi::c_char>();
    (*parser).m_positionPtr = ::core::ptr::null::<::core::ffi::c_char>();
    (*parser).m_openInternalEntities = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
    (*parser).m_openAttributeEntities = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
    (*parser).m_openValueEntities = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
    (*parser).m_defaultExpandInternalEntities = crate::expat_h::XML_TRUE;
    (*parser).m_tagLevel = 0 as ::core::ffi::c_int;
    (*parser).m_tagStack = ::core::ptr::null_mut::<TAG>();
    (*parser).m_inheritedBindings = ::core::ptr::null_mut::<BINDING>();
    (*parser).m_nSpecifiedAtts = 0 as ::core::ffi::c_int;
    (*parser).m_unknownEncodingMem = crate::__stddef_null_h::NULL;
    (*parser).m_unknownEncodingRelease = None;
    (*parser).m_unknownEncodingData = crate::__stddef_null_h::NULL;
    (*parser).m_parsingStatus.parsing = crate::expat_h::XML_INITIALIZED;
    (*parser).m_reenter = crate::expat_h::XML_FALSE;
    (*parser).m_isParamEntity = crate::expat_h::XML_FALSE;
    (*parser).m_useForeignDTD = crate::expat_h::XML_FALSE;
    (*parser).m_paramEntityParsing = crate::expat_h::XML_PARAM_ENTITY_PARSING_NEVER;
    (*parser).m_hash_secret_salt = 0 as ::core::ffi::c_ulong;
    crate::stdlib::memset(
        &raw mut (*parser).m_accounting as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<ACCOUNTING>() as crate::__stddef_size_t_h::size_t,
    );
    (*parser).m_accounting.debugLevel =
        getDebugLevel("EXPAT_ACCOUNTING_DEBUG", 0 as ::core::ffi::c_ulong);
    (*parser).m_accounting.maximumAmplificationFactor =
        crate::internal_h::EXPAT_BILLION_LAUGHS_ATTACK_PROTECTION_MAXIMUM_AMPLIFICATION_DEFAULT;
    (*parser).m_accounting.activationThresholdBytes =
        crate::internal_h::EXPAT_BILLION_LAUGHS_ATTACK_PROTECTION_ACTIVATION_THRESHOLD_DEFAULT
            as ::core::ffi::c_ulonglong;
    crate::stdlib::memset(
        &raw mut (*parser).m_entity_stats as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<ENTITY_STATS>() as crate::__stddef_size_t_h::size_t,
    );
    (*parser).m_entity_stats.debugLevel =
        getDebugLevel("EXPAT_ENTITY_DEBUG", 0 as ::core::ffi::c_ulong);
}

unsafe extern "C" fn moveToFreeBindingList(
    mut parser: crate::expat_h::XML_Parser,
    mut bindings: *mut BINDING,
) {
    while !bindings.is_null() {
        let mut b: *mut BINDING = bindings;
        bindings = (*bindings).nextTagBinding as *mut BINDING;
        (*b).nextTagBinding = (*parser).m_freeBindingList as *mut binding;
        (*parser).m_freeBindingList = b;
    }
}
#[export_name = "XML_ParserReset"]
pub unsafe extern "C" fn XML_ParserReset_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut encodingName: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Bool {
    let mut tStk: *mut TAG = ::core::ptr::null_mut::<TAG>();
    let mut openEntityList: *mut OPEN_INTERNAL_ENTITY =
        ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
    if parser.is_null() {
        return crate::expat_h::XML_FALSE;
    }
    if !(*parser).m_parentParser.is_null() {
        return crate::expat_h::XML_FALSE;
    }
    tStk = (*parser).m_tagStack;
    while !tStk.is_null() {
        let mut tag: *mut TAG = tStk;
        tStk = (*tStk).parent as *mut TAG;
        (*tag).parent = (*parser).m_freeTagList as *mut tag;
        moveToFreeBindingList(parser, (*tag).bindings);
        (*tag).bindings = ::core::ptr::null_mut::<BINDING>();
        (*parser).m_freeTagList = tag;
    }
    openEntityList = (*parser).m_openInternalEntities;
    while !openEntityList.is_null() {
        let mut openEntity: *mut OPEN_INTERNAL_ENTITY = openEntityList;
        openEntityList = (*openEntity).next as *mut OPEN_INTERNAL_ENTITY;
        (*openEntity).next = (*parser).m_freeInternalEntities as *mut open_internal_entity;
        (*parser).m_freeInternalEntities = openEntity;
    }
    openEntityList = (*parser).m_openAttributeEntities;
    while !openEntityList.is_null() {
        let mut openEntity_0: *mut OPEN_INTERNAL_ENTITY = openEntityList;
        openEntityList = (*openEntity_0).next as *mut OPEN_INTERNAL_ENTITY;
        (*openEntity_0).next = (*parser).m_freeAttributeEntities as *mut open_internal_entity;
        (*parser).m_freeAttributeEntities = openEntity_0;
    }
    openEntityList = (*parser).m_openValueEntities;
    while !openEntityList.is_null() {
        let mut openEntity_1: *mut OPEN_INTERNAL_ENTITY = openEntityList;
        openEntityList = (*openEntity_1).next as *mut OPEN_INTERNAL_ENTITY;
        (*openEntity_1).next = (*parser).m_freeValueEntities as *mut open_internal_entity;
        (*parser).m_freeValueEntities = openEntity_1;
    }
    moveToFreeBindingList(parser, (*parser).m_inheritedBindings);
    expat_free(
        parser,
        (*parser).m_unknownEncodingMem,
        1686 as ::core::ffi::c_int,
    );
    if (*parser).m_unknownEncodingRelease.is_some() {
        (*parser)
            .m_unknownEncodingRelease
            .expect("non-null function pointer")((*parser).m_unknownEncodingData);
    }
    poolClear(&raw mut (*parser).m_tempPool);
    poolClear(&raw mut (*parser).m_temp2Pool);
    expat_free(
        parser,
        (*parser).m_protocolEncodingName as *mut ::core::ffi::c_void,
        1691 as ::core::ffi::c_int,
    );
    (*parser).m_protocolEncodingName = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    parserInit(parser, encodingName);
    dtdReset((*parser).m_dtd, parser);
    return crate::expat_h::XML_TRUE;
}
fn parserBusy(parser: &XML_ParserStruct) -> crate::expat_h::XML_Bool {
    match parser.m_parsingStatus.parsing as ::core::ffi::c_uint {
        1 | 3 => return crate::expat_h::XML_TRUE,
        0 | 2 | _ => return crate::expat_h::XML_FALSE,
    };
}
pub fn XML_SetEncoding(
    parser: Option<&mut XML_ParserStruct>,
    encodingName: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Status {
    let Some(parser) = parser else {
        return crate::expat_h::XML_STATUS_ERROR;
    };
    if parserBusy(parser) != 0 {
        return crate::expat_h::XML_STATUS_ERROR;
    }
    parser.m_protocolEncodingName = encodingName;
    return crate::expat_h::XML_STATUS_OK;
}
#[export_name = "XML_SetEncoding"]

pub unsafe extern "C" fn XML_SetEncoding_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut encodingName: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Status {
    if parser.is_null() {
        return crate::expat_h::XML_STATUS_ERROR;
    }
    if parserBusy(&*parser) != 0 {
        return crate::expat_h::XML_STATUS_ERROR;
    }
    expat_free(
        parser,
        (*parser).m_protocolEncodingName as *mut ::core::ffi::c_void,
        1723 as ::core::ffi::c_int,
    );
    (*parser).m_protocolEncodingName = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    let copied_encoding_name = if encodingName.is_null() {
        ::core::ptr::null::<crate::expat_external_h::XML_Char>()
    } else {
        let mut chars_required: crate::__stddef_size_t_h::size_t =
            0 as crate::__stddef_size_t_h::size_t;
        while *encodingName.offset(chars_required as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            chars_required = chars_required.wrapping_add(1);
        }
        chars_required = chars_required.wrapping_add(1);
        let copied = expat_malloc(
            parser,
            chars_required
                .wrapping_mul(::core::mem::size_of::<crate::expat_external_h::XML_Char>()
                    as crate::__stddef_size_t_h::size_t),
            8456 as ::core::ffi::c_int,
        ) as *mut crate::expat_external_h::XML_Char;
        if copied.is_null() {
            return crate::expat_h::XML_STATUS_ERROR;
        }
        crate::stdlib::memcpy(
            copied as *mut ::core::ffi::c_void,
            encodingName as *const ::core::ffi::c_void,
            chars_required
                .wrapping_mul(::core::mem::size_of::<crate::expat_external_h::XML_Char>()
                    as crate::__stddef_size_t_h::size_t),
        );
        copied
    };
    XML_SetEncoding(parser.as_mut(), copied_encoding_name)
}
#[export_name = "XML_ExternalEntityParserCreate"]
pub unsafe extern "C" fn XML_ExternalEntityParserCreate_ffi(
    mut oldParser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut encodingName: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Parser {
    let mut parser: crate::expat_h::XML_Parser = oldParser;
    let mut newDtd: *mut DTD = ::core::ptr::null_mut::<DTD>();
    let mut oldDtd: *mut DTD = ::core::ptr::null_mut::<DTD>();
    let mut oldStartElementHandler: crate::expat_h::XML_StartElementHandler = None;
    let mut oldEndElementHandler: crate::expat_h::XML_EndElementHandler = None;
    let mut oldCharacterDataHandler: crate::expat_h::XML_CharacterDataHandler = None;
    let mut oldProcessingInstructionHandler: crate::expat_h::XML_ProcessingInstructionHandler =
        None;
    let mut oldCommentHandler: crate::expat_h::XML_CommentHandler = None;
    let mut oldStartCdataSectionHandler: crate::expat_h::XML_StartCdataSectionHandler = None;
    let mut oldEndCdataSectionHandler: crate::expat_h::XML_EndCdataSectionHandler = None;
    let mut oldDefaultHandler: crate::expat_h::XML_DefaultHandler = None;
    let mut oldUnparsedEntityDeclHandler: crate::expat_h::XML_UnparsedEntityDeclHandler = None;
    let mut oldNotationDeclHandler: crate::expat_h::XML_NotationDeclHandler = None;
    let mut oldStartNamespaceDeclHandler: crate::expat_h::XML_StartNamespaceDeclHandler = None;
    let mut oldEndNamespaceDeclHandler: crate::expat_h::XML_EndNamespaceDeclHandler = None;
    let mut oldNotStandaloneHandler: crate::expat_h::XML_NotStandaloneHandler = None;
    let mut oldExternalEntityRefHandler: crate::expat_h::XML_ExternalEntityRefHandler = None;
    let mut oldSkippedEntityHandler: crate::expat_h::XML_SkippedEntityHandler = None;
    let mut oldUnknownEncodingHandler: crate::expat_h::XML_UnknownEncodingHandler = None;
    let mut oldUnknownEncodingHandlerData: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut oldElementDeclHandler: crate::expat_h::XML_ElementDeclHandler = None;
    let mut oldAttlistDeclHandler: crate::expat_h::XML_AttlistDeclHandler = None;
    let mut oldEntityDeclHandler: crate::expat_h::XML_EntityDeclHandler = None;
    let mut oldXmlDeclHandler: crate::expat_h::XML_XmlDeclHandler = None;
    let mut oldDeclElementType: *mut ELEMENT_TYPE = ::core::ptr::null_mut::<ELEMENT_TYPE>();
    let mut oldUserData: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut oldHandlerArg: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut oldDefaultExpandInternalEntities: crate::expat_h::XML_Bool = 0;
    let mut oldExternalEntityRefHandlerArg: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    let mut oldParamEntityParsing: crate::expat_h::XML_ParamEntityParsing =
        crate::expat_h::XML_PARAM_ENTITY_PARSING_NEVER;
    let mut oldInEntityValue: ::core::ffi::c_int = 0;
    let mut oldns_triplets: crate::expat_h::XML_Bool = 0;
    let mut oldhash_secret_salt: ::core::ffi::c_ulong = 0;
    let mut oldReparseDeferralEnabled: crate::expat_h::XML_Bool = 0;
    if oldParser.is_null() {
        return ::core::ptr::null_mut::<XML_ParserStruct>();
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
        let mut tmp: [crate::expat_external_h::XML_Char; 2] = [
            (*parser).m_namespaceSeparator,
            0 as ::core::ffi::c_int as crate::expat_external_h::XML_Char,
        ];
        parser = parserCreate(
            encodingName,
            &raw const (*parser).m_mem,
            &raw mut tmp as *mut crate::expat_external_h::XML_Char,
            newDtd,
            oldParser,
        );
    } else {
        parser = parserCreate(
            encodingName,
            &raw const (*parser).m_mem,
            ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
            newDtd,
            oldParser,
        );
    }
    if parser.is_null() {
        return ::core::ptr::null_mut::<XML_ParserStruct>();
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
        (*parser).m_handlerArg = parser as *mut ::core::ffi::c_void;
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
            XML_ParserFree_ffi(parser);
            return ::core::ptr::null_mut::<XML_ParserStruct>();
        }
        (*parser).m_processor = Some(
            externalEntityInitProcessor
                as unsafe extern "C" fn(
                    crate::expat_h::XML_Parser,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> crate::expat_h::XML_Error,
        );
    } else {
        (*parser).m_isParamEntity = crate::expat_h::XML_TRUE;
        crate::src::xmlrole::XmlPrologStateInitExternalEntity(&mut (*parser).m_prologState);
        (*parser).m_processor = Some(
            externalParEntInitProcessor
                as unsafe extern "C" fn(
                    crate::expat_h::XML_Parser,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> crate::expat_h::XML_Error,
        );
    }
    return parser;
}
unsafe extern "C" fn destroyBindings(
    mut bindings: *mut BINDING,
    mut parser: crate::expat_h::XML_Parser,
) {
    loop {
        let mut b: *mut BINDING = bindings;
        if b.is_null() {
            break;
        }
        bindings = (*b).nextTagBinding as *mut BINDING;
        expat_free(
            parser,
            (*b).uri as *mut ::core::ffi::c_void,
            1919 as ::core::ffi::c_int,
        );
        expat_free(
            parser,
            b as *mut ::core::ffi::c_void,
            1920 as ::core::ffi::c_int,
        );
    }
}
#[export_name = "XML_ParserFree"]
pub unsafe extern "C" fn XML_ParserFree_ffi(mut parser: crate::expat_h::XML_Parser) {
    let mut tagList: *mut TAG = ::core::ptr::null_mut::<TAG>();
    let mut entityList: *mut OPEN_INTERNAL_ENTITY = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
    if parser.is_null() {
        return;
    }
    tagList = (*parser).m_tagStack;
    loop {
        let mut p: *mut TAG = ::core::ptr::null_mut::<TAG>();
        if tagList.is_null() {
            if (*parser).m_freeTagList.is_null() {
                break;
            }
            tagList = (*parser).m_freeTagList;
            (*parser).m_freeTagList = ::core::ptr::null_mut::<TAG>();
        }
        p = tagList;
        tagList = (*tagList).parent as *mut TAG;
        expat_free(
            parser,
            (*p).buf.raw as *mut ::core::ffi::c_void,
            1942 as ::core::ffi::c_int,
        );
        destroyBindings((*p).bindings, parser);
        expat_free(
            parser,
            p as *mut ::core::ffi::c_void,
            1944 as ::core::ffi::c_int,
        );
    }
    entityList = (*parser).m_openInternalEntities;
    loop {
        let mut openEntity: *mut OPEN_INTERNAL_ENTITY =
            ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
        if entityList.is_null() {
            if (*parser).m_freeInternalEntities.is_null() {
                break;
            }
            entityList = (*parser).m_freeInternalEntities;
            (*parser).m_freeInternalEntities = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
        }
        openEntity = entityList;
        entityList = (*entityList).next as *mut OPEN_INTERNAL_ENTITY;
        expat_free(
            parser,
            openEntity as *mut ::core::ffi::c_void,
            1958 as ::core::ffi::c_int,
        );
    }
    entityList = (*parser).m_openAttributeEntities;
    loop {
        let mut openEntity_0: *mut OPEN_INTERNAL_ENTITY =
            ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
        if entityList.is_null() {
            if (*parser).m_freeAttributeEntities.is_null() {
                break;
            }
            entityList = (*parser).m_freeAttributeEntities;
            (*parser).m_freeAttributeEntities = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
        }
        openEntity_0 = entityList;
        entityList = (*entityList).next as *mut OPEN_INTERNAL_ENTITY;
        expat_free(
            parser,
            openEntity_0 as *mut ::core::ffi::c_void,
            1972 as ::core::ffi::c_int,
        );
    }
    entityList = (*parser).m_openValueEntities;
    loop {
        let mut openEntity_1: *mut OPEN_INTERNAL_ENTITY =
            ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
        if entityList.is_null() {
            if (*parser).m_freeValueEntities.is_null() {
                break;
            }
            entityList = (*parser).m_freeValueEntities;
            (*parser).m_freeValueEntities = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
        }
        openEntity_1 = entityList;
        entityList = (*entityList).next as *mut OPEN_INTERNAL_ENTITY;
        expat_free(
            parser,
            openEntity_1 as *mut ::core::ffi::c_void,
            1986 as ::core::ffi::c_int,
        );
    }
    destroyBindings((*parser).m_freeBindingList, parser);
    destroyBindings((*parser).m_inheritedBindings, parser);
    poolDestroy(&raw mut (*parser).m_tempPool);
    poolDestroy(&raw mut (*parser).m_temp2Pool);
    expat_free(
        parser,
        (*parser).m_protocolEncodingName as *mut ::core::ffi::c_void,
        1992 as ::core::ffi::c_int,
    );
    if (*parser).m_isParamEntity == 0 && !(*parser).m_dtd.is_null() {
        dtdDestroy(
            (*parser).m_dtd,
            (*parser).m_parentParser.is_null() as ::core::ffi::c_int as crate::expat_h::XML_Bool,
            parser,
        );
    }
    expat_free(
        parser,
        (*parser).m_atts as *mut ::core::ffi::c_void,
        2002 as ::core::ffi::c_int,
    );
    expat_free(
        parser,
        (*parser).m_groupConnector as *mut ::core::ffi::c_void,
        2006 as ::core::ffi::c_int,
    );
    (*parser).m_mem.free_fcn.expect("non-null function pointer")(
        (*parser).m_buffer as *mut ::core::ffi::c_void,
    );
    expat_free(
        parser,
        (*parser).m_dataBuf as *mut ::core::ffi::c_void,
        2011 as ::core::ffi::c_int,
    );
    expat_free(
        parser,
        (*parser).m_nsAtts as *mut ::core::ffi::c_void,
        2012 as ::core::ffi::c_int,
    );
    expat_free(
        parser,
        (*parser).m_unknownEncodingMem,
        2013 as ::core::ffi::c_int,
    );
    if (*parser).m_unknownEncodingRelease.is_some() {
        (*parser)
            .m_unknownEncodingRelease
            .expect("non-null function pointer")((*parser).m_unknownEncodingData);
    }
    expat_free(
        parser,
        parser as *mut ::core::ffi::c_void,
        2016 as ::core::ffi::c_int,
    );
}
pub fn XML_UseParserAsHandlerArg(parser: Option<&mut XML_ParserStruct>) {
    if let Some(parser) = parser {
        parser.m_handlerArg = parser as *mut XML_ParserStruct as *mut ::core::ffi::c_void;
    }
}
#[export_name = "XML_UseParserAsHandlerArg"]

pub unsafe extern "C" fn XML_UseParserAsHandlerArg_ffi(mut parser: crate::expat_h::XML_Parser) {
    XML_UseParserAsHandlerArg(parser.as_mut())
}
pub fn XML_UseForeignDTD(
    parser: Option<&mut XML_ParserStruct>,
    useDTD: crate::expat_h::XML_Bool,
) -> crate::expat_h::XML_Error {
    let Some(parser) = parser else {
        return crate::expat_h::XML_ERROR_INVALID_ARGUMENT;
    };
    if parserBusy(parser) != 0 {
        return crate::expat_h::XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING;
    }
    parser.m_useForeignDTD = useDTD;
    return crate::expat_h::XML_ERROR_NONE;
}
#[export_name = "XML_UseForeignDTD"]

pub unsafe extern "C" fn XML_UseForeignDTD_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut useDTD: crate::expat_h::XML_Bool,
) -> crate::expat_h::XML_Error {
    XML_UseForeignDTD(parser.as_mut(), useDTD)
}
pub fn XML_SetReturnNSTriplet(parser: Option<&mut XML_ParserStruct>, do_nst: ::core::ffi::c_int) {
    let Some(parser) = parser else {
        return;
    };
    if parserBusy(parser) != 0 {
        return;
    }
    parser.m_ns_triplets = (if do_nst != 0 {
        crate::expat_h::XML_TRUE as ::core::ffi::c_int
    } else {
        crate::expat_h::XML_FALSE as ::core::ffi::c_int
    }) as crate::expat_h::XML_Bool;
}
#[export_name = "XML_SetReturnNSTriplet"]

pub unsafe extern "C" fn XML_SetReturnNSTriplet_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut do_nst: ::core::ffi::c_int,
) {
    XML_SetReturnNSTriplet(parser.as_mut(), do_nst)
}
pub fn XML_SetUserData(parser: Option<&mut XML_ParserStruct>, p: *mut ::core::ffi::c_void) {
    let Some(parser) = parser else {
        return;
    };
    if parser.m_handlerArg == parser.m_userData {
        parser.m_userData = p;
        parser.m_handlerArg = parser.m_userData;
    } else {
        parser.m_userData = p;
    };
}
#[export_name = "XML_SetUserData"]

pub unsafe extern "C" fn XML_SetUserData_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut p: *mut ::core::ffi::c_void,
) {
    XML_SetUserData(parser.as_mut(), p)
}
pub fn XML_SetBase(
    parser: Option<&mut XML_ParserStruct>,
    p: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Status {
    let Some(parser) = parser else {
        return crate::expat_h::XML_STATUS_ERROR;
    };
    parser.m_curBase = p;
    return crate::expat_h::XML_STATUS_OK;
}
#[export_name = "XML_SetBase"]

pub unsafe extern "C" fn XML_SetBase_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut p: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Status {
    if parser.is_null() {
        return crate::expat_h::XML_STATUS_ERROR;
    }
    let copied_base = if p.is_null() {
        ::core::ptr::null::<crate::expat_external_h::XML_Char>()
    } else {
        let copied = poolCopyString(&raw mut (*(*parser).m_dtd).pool, p);
        if copied.is_null() {
            return crate::expat_h::XML_STATUS_ERROR;
        }
        copied
    };
    XML_SetBase(parser.as_mut(), copied_base)
}
pub fn XML_GetBase(parser: Option<&XML_ParserStruct>) -> *const crate::expat_external_h::XML_Char {
    match parser {
        Some(parser) => parser.m_curBase,
        None => ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    }
}
#[export_name = "XML_GetBase"]

pub unsafe extern "C" fn XML_GetBase_ffi(
    mut parser: crate::expat_h::XML_Parser,
) -> *const crate::expat_external_h::XML_Char {
    XML_GetBase(parser.as_ref())
}
pub fn XML_GetSpecifiedAttributeCount(parser: Option<&XML_ParserStruct>) -> ::core::ffi::c_int {
    match parser {
        Some(parser) => parser.m_nSpecifiedAtts,
        None => -1 as ::core::ffi::c_int,
    }
}
#[export_name = "XML_GetSpecifiedAttributeCount"]

pub unsafe extern "C" fn XML_GetSpecifiedAttributeCount_ffi(
    mut parser: crate::expat_h::XML_Parser,
) -> ::core::ffi::c_int {
    XML_GetSpecifiedAttributeCount(parser.as_ref())
}
pub fn XML_GetIdAttributeIndex(parser: Option<&XML_ParserStruct>) -> ::core::ffi::c_int {
    match parser {
        Some(parser) => parser.m_idAttIndex,
        None => -1 as ::core::ffi::c_int,
    }
}
#[export_name = "XML_GetIdAttributeIndex"]

pub unsafe extern "C" fn XML_GetIdAttributeIndex_ffi(
    mut parser: crate::expat_h::XML_Parser,
) -> ::core::ffi::c_int {
    XML_GetIdAttributeIndex(parser.as_ref())
}
pub fn XML_SetElementHandler(
    parser: Option<&mut XML_ParserStruct>,
    start: crate::expat_h::XML_StartElementHandler,
    end: crate::expat_h::XML_EndElementHandler,
) {
    if let Some(parser) = parser {
        parser.m_startElementHandler = start;
        parser.m_endElementHandler = end;
    }
}
#[export_name = "XML_SetElementHandler"]

pub unsafe extern "C" fn XML_SetElementHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartElementHandler,
    mut end: crate::expat_h::XML_EndElementHandler,
) {
    XML_SetElementHandler(parser.as_mut(), start, end)
}
pub fn XML_SetStartElementHandler(
    parser: Option<&mut XML_ParserStruct>,
    start: crate::expat_h::XML_StartElementHandler,
) {
    if let Some(parser) = parser {
        parser.m_startElementHandler = start;
    }
}
#[export_name = "XML_SetStartElementHandler"]

pub unsafe extern "C" fn XML_SetStartElementHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartElementHandler,
) {
    XML_SetStartElementHandler(parser.as_mut(), start)
}
pub fn XML_SetEndElementHandler(
    parser: Option<&mut XML_ParserStruct>,
    end: crate::expat_h::XML_EndElementHandler,
) {
    if let Some(parser) = parser {
        parser.m_endElementHandler = end;
    }
}
#[export_name = "XML_SetEndElementHandler"]

pub unsafe extern "C" fn XML_SetEndElementHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut end: crate::expat_h::XML_EndElementHandler,
) {
    XML_SetEndElementHandler(parser.as_mut(), end)
}
pub fn XML_SetCharacterDataHandler(
    parser: Option<&mut XML_ParserStruct>,
    handler: crate::expat_h::XML_CharacterDataHandler,
) {
    if let Some(parser) = parser {
        parser.m_characterDataHandler = handler;
    }
}
#[export_name = "XML_SetCharacterDataHandler"]

pub unsafe extern "C" fn XML_SetCharacterDataHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_CharacterDataHandler,
) {
    XML_SetCharacterDataHandler(parser.as_mut(), handler)
}
pub fn XML_SetProcessingInstructionHandler(
    parser: Option<&mut XML_ParserStruct>,
    handler: crate::expat_h::XML_ProcessingInstructionHandler,
) {
    if let Some(parser) = parser {
        parser.m_processingInstructionHandler = handler;
    }
}
#[export_name = "XML_SetProcessingInstructionHandler"]

pub unsafe extern "C" fn XML_SetProcessingInstructionHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_ProcessingInstructionHandler,
) {
    XML_SetProcessingInstructionHandler(parser.as_mut(), handler)
}
pub fn XML_SetCommentHandler(
    parser: Option<&mut XML_ParserStruct>,
    handler: crate::expat_h::XML_CommentHandler,
) {
    if let Some(parser) = parser {
        parser.m_commentHandler = handler;
    }
}
#[export_name = "XML_SetCommentHandler"]

pub unsafe extern "C" fn XML_SetCommentHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_CommentHandler,
) {
    XML_SetCommentHandler(parser.as_mut(), handler)
}
pub fn XML_SetCdataSectionHandler(
    parser: Option<&mut XML_ParserStruct>,
    start: crate::expat_h::XML_StartCdataSectionHandler,
    end: crate::expat_h::XML_EndCdataSectionHandler,
) {
    if let Some(parser) = parser {
        parser.m_startCdataSectionHandler = start;
        parser.m_endCdataSectionHandler = end;
    }
}
#[export_name = "XML_SetCdataSectionHandler"]

pub unsafe extern "C" fn XML_SetCdataSectionHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartCdataSectionHandler,
    mut end: crate::expat_h::XML_EndCdataSectionHandler,
) {
    XML_SetCdataSectionHandler(parser.as_mut(), start, end)
}
pub fn XML_SetStartCdataSectionHandler(
    parser: Option<&mut XML_ParserStruct>,
    start: crate::expat_h::XML_StartCdataSectionHandler,
) {
    if let Some(parser) = parser {
        parser.m_startCdataSectionHandler = start;
    }
}
#[export_name = "XML_SetStartCdataSectionHandler"]

pub unsafe extern "C" fn XML_SetStartCdataSectionHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartCdataSectionHandler,
) {
    XML_SetStartCdataSectionHandler(parser.as_mut(), start)
}
pub fn XML_SetEndCdataSectionHandler(
    parser: Option<&mut XML_ParserStruct>,
    end: crate::expat_h::XML_EndCdataSectionHandler,
) {
    if let Some(parser) = parser {
        parser.m_endCdataSectionHandler = end;
    }
}
#[export_name = "XML_SetEndCdataSectionHandler"]

pub unsafe extern "C" fn XML_SetEndCdataSectionHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut end: crate::expat_h::XML_EndCdataSectionHandler,
) {
    XML_SetEndCdataSectionHandler(parser.as_mut(), end)
}
pub fn XML_SetDefaultHandler(
    parser: Option<&mut XML_ParserStruct>,
    handler: crate::expat_h::XML_DefaultHandler,
) {
    if let Some(parser) = parser {
        parser.m_defaultHandler = handler;
        parser.m_defaultExpandInternalEntities = crate::expat_h::XML_FALSE;
    }
}
#[export_name = "XML_SetDefaultHandler"]

pub unsafe extern "C" fn XML_SetDefaultHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_DefaultHandler,
) {
    XML_SetDefaultHandler(parser.as_mut(), handler)
}
pub fn XML_SetDefaultHandlerExpand(
    parser: Option<&mut XML_ParserStruct>,
    handler: crate::expat_h::XML_DefaultHandler,
) {
    if let Some(parser) = parser {
        parser.m_defaultHandler = handler;
        parser.m_defaultExpandInternalEntities = crate::expat_h::XML_TRUE;
    }
}
#[export_name = "XML_SetDefaultHandlerExpand"]

pub unsafe extern "C" fn XML_SetDefaultHandlerExpand_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_DefaultHandler,
) {
    XML_SetDefaultHandlerExpand(parser.as_mut(), handler)
}
pub fn XML_SetDoctypeDeclHandler(
    parser: Option<&mut XML_ParserStruct>,
    start: crate::expat_h::XML_StartDoctypeDeclHandler,
    end: crate::expat_h::XML_EndDoctypeDeclHandler,
) {
    if let Some(parser) = parser {
        parser.m_startDoctypeDeclHandler = start;
        parser.m_endDoctypeDeclHandler = end;
    }
}
#[export_name = "XML_SetDoctypeDeclHandler"]

pub unsafe extern "C" fn XML_SetDoctypeDeclHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartDoctypeDeclHandler,
    mut end: crate::expat_h::XML_EndDoctypeDeclHandler,
) {
    XML_SetDoctypeDeclHandler(parser.as_mut(), start, end)
}
pub fn XML_SetStartDoctypeDeclHandler(
    parser: Option<&mut XML_ParserStruct>,
    start: crate::expat_h::XML_StartDoctypeDeclHandler,
) {
    if let Some(parser) = parser {
        parser.m_startDoctypeDeclHandler = start;
    }
}
#[export_name = "XML_SetStartDoctypeDeclHandler"]

pub unsafe extern "C" fn XML_SetStartDoctypeDeclHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartDoctypeDeclHandler,
) {
    XML_SetStartDoctypeDeclHandler(parser.as_mut(), start)
}
pub fn XML_SetEndDoctypeDeclHandler(
    parser: Option<&mut XML_ParserStruct>,
    end: crate::expat_h::XML_EndDoctypeDeclHandler,
) {
    if let Some(parser) = parser {
        parser.m_endDoctypeDeclHandler = end;
    }
}
#[export_name = "XML_SetEndDoctypeDeclHandler"]

pub unsafe extern "C" fn XML_SetEndDoctypeDeclHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut end: crate::expat_h::XML_EndDoctypeDeclHandler,
) {
    XML_SetEndDoctypeDeclHandler(parser.as_mut(), end)
}
pub fn XML_SetUnparsedEntityDeclHandler(
    parser: Option<&mut XML_ParserStruct>,
    handler: crate::expat_h::XML_UnparsedEntityDeclHandler,
) {
    if let Some(parser) = parser {
        parser.m_unparsedEntityDeclHandler = handler;
    }
}
#[export_name = "XML_SetUnparsedEntityDeclHandler"]

pub unsafe extern "C" fn XML_SetUnparsedEntityDeclHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_UnparsedEntityDeclHandler,
) {
    XML_SetUnparsedEntityDeclHandler(parser.as_mut(), handler)
}
pub fn XML_SetNotationDeclHandler(
    parser: Option<&mut XML_ParserStruct>,
    handler: crate::expat_h::XML_NotationDeclHandler,
) {
    if let Some(parser) = parser {
        parser.m_notationDeclHandler = handler;
    }
}
#[export_name = "XML_SetNotationDeclHandler"]

pub unsafe extern "C" fn XML_SetNotationDeclHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_NotationDeclHandler,
) {
    XML_SetNotationDeclHandler(parser.as_mut(), handler)
}
pub fn XML_SetNamespaceDeclHandler(
    parser: Option<&mut XML_ParserStruct>,
    start: crate::expat_h::XML_StartNamespaceDeclHandler,
    end: crate::expat_h::XML_EndNamespaceDeclHandler,
) {
    if let Some(parser) = parser {
        parser.m_startNamespaceDeclHandler = start;
        parser.m_endNamespaceDeclHandler = end;
    }
}
#[export_name = "XML_SetNamespaceDeclHandler"]

pub unsafe extern "C" fn XML_SetNamespaceDeclHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartNamespaceDeclHandler,
    mut end: crate::expat_h::XML_EndNamespaceDeclHandler,
) {
    XML_SetNamespaceDeclHandler(parser.as_mut(), start, end)
}
pub fn XML_SetStartNamespaceDeclHandler(
    parser: Option<&mut XML_ParserStruct>,
    start: crate::expat_h::XML_StartNamespaceDeclHandler,
) {
    if let Some(parser) = parser {
        parser.m_startNamespaceDeclHandler = start;
    }
}
#[export_name = "XML_SetStartNamespaceDeclHandler"]

pub unsafe extern "C" fn XML_SetStartNamespaceDeclHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartNamespaceDeclHandler,
) {
    XML_SetStartNamespaceDeclHandler(parser.as_mut(), start)
}
pub fn XML_SetEndNamespaceDeclHandler(
    parser: Option<&mut XML_ParserStruct>,
    end: crate::expat_h::XML_EndNamespaceDeclHandler,
) {
    if let Some(parser) = parser {
        parser.m_endNamespaceDeclHandler = end;
    }
}
#[export_name = "XML_SetEndNamespaceDeclHandler"]

pub unsafe extern "C" fn XML_SetEndNamespaceDeclHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut end: crate::expat_h::XML_EndNamespaceDeclHandler,
) {
    XML_SetEndNamespaceDeclHandler(parser.as_mut(), end)
}
pub fn XML_SetNotStandaloneHandler(
    parser: Option<&mut XML_ParserStruct>,
    handler: crate::expat_h::XML_NotStandaloneHandler,
) {
    if let Some(parser) = parser {
        parser.m_notStandaloneHandler = handler;
    }
}
#[export_name = "XML_SetNotStandaloneHandler"]

pub unsafe extern "C" fn XML_SetNotStandaloneHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_NotStandaloneHandler,
) {
    XML_SetNotStandaloneHandler(parser.as_mut(), handler)
}
pub fn XML_SetExternalEntityRefHandler(
    parser: Option<&mut XML_ParserStruct>,
    handler: crate::expat_h::XML_ExternalEntityRefHandler,
) {
    if let Some(parser) = parser {
        parser.m_externalEntityRefHandler = handler;
    }
}
#[export_name = "XML_SetExternalEntityRefHandler"]

pub unsafe extern "C" fn XML_SetExternalEntityRefHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_ExternalEntityRefHandler,
) {
    XML_SetExternalEntityRefHandler(parser.as_mut(), handler)
}
pub fn XML_SetExternalEntityRefHandlerArg(
    parser: Option<&mut XML_ParserStruct>,
    arg: *mut ::core::ffi::c_void,
) {
    if let Some(parser) = parser {
        if !arg.is_null() {
            parser.m_externalEntityRefHandlerArg = arg as crate::expat_h::XML_Parser;
        } else {
            parser.m_externalEntityRefHandlerArg = parser as *mut XML_ParserStruct;
        }
    };
}
#[export_name = "XML_SetExternalEntityRefHandlerArg"]

pub unsafe extern "C" fn XML_SetExternalEntityRefHandlerArg_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut arg: *mut ::core::ffi::c_void,
) {
    XML_SetExternalEntityRefHandlerArg(parser.as_mut(), arg)
}
pub fn XML_SetSkippedEntityHandler(
    parser: Option<&mut XML_ParserStruct>,
    handler: crate::expat_h::XML_SkippedEntityHandler,
) {
    if let Some(parser) = parser {
        parser.m_skippedEntityHandler = handler;
    }
}
#[export_name = "XML_SetSkippedEntityHandler"]

pub unsafe extern "C" fn XML_SetSkippedEntityHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_SkippedEntityHandler,
) {
    XML_SetSkippedEntityHandler(parser.as_mut(), handler)
}
pub fn XML_SetUnknownEncodingHandler(
    parser: Option<&mut XML_ParserStruct>,
    handler: crate::expat_h::XML_UnknownEncodingHandler,
    data: *mut ::core::ffi::c_void,
) {
    if let Some(parser) = parser {
        parser.m_unknownEncodingHandler = handler;
        parser.m_unknownEncodingHandlerData = data;
    }
}
#[export_name = "XML_SetUnknownEncodingHandler"]

pub unsafe extern "C" fn XML_SetUnknownEncodingHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_UnknownEncodingHandler,
    mut data: *mut ::core::ffi::c_void,
) {
    XML_SetUnknownEncodingHandler(parser.as_mut(), handler, data)
}
pub fn XML_SetElementDeclHandler(
    parser: Option<&mut XML_ParserStruct>,
    eldecl: crate::expat_h::XML_ElementDeclHandler,
) {
    if let Some(parser) = parser {
        parser.m_elementDeclHandler = eldecl;
    }
}
#[export_name = "XML_SetElementDeclHandler"]

pub unsafe extern "C" fn XML_SetElementDeclHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut eldecl: crate::expat_h::XML_ElementDeclHandler,
) {
    XML_SetElementDeclHandler(parser.as_mut(), eldecl)
}
pub fn XML_SetAttlistDeclHandler(
    parser: Option<&mut XML_ParserStruct>,
    attdecl: crate::expat_h::XML_AttlistDeclHandler,
) {
    if let Some(parser) = parser {
        parser.m_attlistDeclHandler = attdecl;
    }
}
#[export_name = "XML_SetAttlistDeclHandler"]

pub unsafe extern "C" fn XML_SetAttlistDeclHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut attdecl: crate::expat_h::XML_AttlistDeclHandler,
) {
    XML_SetAttlistDeclHandler(parser.as_mut(), attdecl)
}
pub fn XML_SetEntityDeclHandler(
    parser: Option<&mut XML_ParserStruct>,
    handler: crate::expat_h::XML_EntityDeclHandler,
) {
    if let Some(parser) = parser {
        parser.m_entityDeclHandler = handler;
    }
}
#[export_name = "XML_SetEntityDeclHandler"]

pub unsafe extern "C" fn XML_SetEntityDeclHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_EntityDeclHandler,
) {
    XML_SetEntityDeclHandler(parser.as_mut(), handler)
}
pub fn XML_SetXmlDeclHandler(
    parser: Option<&mut XML_ParserStruct>,
    handler: crate::expat_h::XML_XmlDeclHandler,
) {
    if let Some(parser) = parser {
        parser.m_xmlDeclHandler = handler;
    }
}
#[export_name = "XML_SetXmlDeclHandler"]

pub unsafe extern "C" fn XML_SetXmlDeclHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_XmlDeclHandler,
) {
    XML_SetXmlDeclHandler(parser.as_mut(), handler)
}
pub fn XML_SetParamEntityParsing(
    parser: Option<&mut XML_ParserStruct>,
    peParsing: crate::expat_h::XML_ParamEntityParsing,
) -> ::core::ffi::c_int {
    let Some(parser) = parser else {
        return 0 as ::core::ffi::c_int;
    };
    if parserBusy(parser) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    parser.m_paramEntityParsing = peParsing;
    return 1 as ::core::ffi::c_int;
}
#[export_name = "XML_SetParamEntityParsing"]

pub unsafe extern "C" fn XML_SetParamEntityParsing_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut peParsing: crate::expat_h::XML_ParamEntityParsing,
) -> ::core::ffi::c_int {
    XML_SetParamEntityParsing(parser.as_mut(), peParsing)
}
pub fn XML_SetHashSalt(
    parser: Option<&mut XML_ParserStruct>,
    hash_salt: ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    let Some(parser) = parser else {
        return 0 as ::core::ffi::c_int;
    };
    if !parser.m_parentParser.is_null() {
        std::process::abort();
    }
    if parserBusy(parser) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    parser.m_hash_secret_salt = hash_salt;
    return 1 as ::core::ffi::c_int;
}
#[export_name = "XML_SetHashSalt"]

pub unsafe extern "C" fn XML_SetHashSalt_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut hash_salt: ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    if parser.is_null() {
        return XML_SetHashSalt(None, hash_salt);
    }
    let rootParser: crate::expat_h::XML_Parser =
        getRootParserOf(parser, ::core::ptr::null_mut::<::core::ffi::c_uint>())
            as crate::expat_h::XML_Parser;
    XML_SetHashSalt(rootParser.as_mut(), hash_salt)
}
#[export_name = "XML_Parse"]

pub unsafe extern "C" fn XML_Parse_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut s: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
    mut isFinal: ::core::ffi::c_int,
) -> crate::expat_h::XML_Status {
    if parser.is_null()
        || len < 0 as ::core::ffi::c_int
        || s.is_null() && len != 0 as ::core::ffi::c_int
    {
        if !parser.is_null() {
            (*parser).m_errorCode = crate::expat_h::XML_ERROR_INVALID_ARGUMENT;
        }
        return crate::expat_h::XML_STATUS_ERROR;
    }
    match (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint {
        3 => {
            (*parser).m_errorCode = crate::expat_h::XML_ERROR_SUSPENDED;
            return crate::expat_h::XML_STATUS_ERROR;
        }
        2 => {
            (*parser).m_errorCode = crate::expat_h::XML_ERROR_FINISHED;
            return crate::expat_h::XML_STATUS_ERROR;
        }
        0 => {
            if (*parser).m_parentParser.is_null() && startParsing(parser) == 0 {
                (*parser).m_errorCode = crate::expat_h::XML_ERROR_NO_MEMORY;
                return crate::expat_h::XML_STATUS_ERROR;
            }
        }
        _ => {}
    }
    (*parser).m_parsingStatus.parsing = crate::expat_h::XML_PARSING;
    let mut buff: *mut ::core::ffi::c_void = XML_GetBuffer_ffi(parser, len);
    if buff.is_null() {
        return crate::expat_h::XML_STATUS_ERROR;
    }
    if len > 0 as ::core::ffi::c_int {
        '_c2rust_label: {
            if !s.is_null() {
            } else {
                crate::stdlib::__assert_fail(
                    b"s != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../../expat/lib/xmlparse.c\0".as_ptr() as *const ::core::ffi::c_char,
                    2445 as ::core::ffi::c_uint,
                    b"enum XML_Status XML_Parse(XML_Parser, const char *, int, int)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        };
        crate::stdlib::memcpy(
            buff,
            s as *const ::core::ffi::c_void,
            len as crate::__stddef_size_t_h::size_t,
        );
    }
    return XML_ParseBuffer_ffi(parser, len, isFinal);
}
#[export_name = "XML_ParseBuffer"]

pub unsafe extern "C" fn XML_ParseBuffer_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut len: ::core::ffi::c_int,
    mut isFinal: ::core::ffi::c_int,
) -> crate::expat_h::XML_Status {
    let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut result: crate::expat_h::XML_Status = crate::expat_h::XML_STATUS_OK;
    if parser.is_null() {
        return crate::expat_h::XML_STATUS_ERROR;
    }
    if len < 0 as ::core::ffi::c_int {
        (*parser).m_errorCode = crate::expat_h::XML_ERROR_INVALID_ARGUMENT;
        return crate::expat_h::XML_STATUS_ERROR;
    }
    match (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint {
        3 => {
            (*parser).m_errorCode = crate::expat_h::XML_ERROR_SUSPENDED;
            return crate::expat_h::XML_STATUS_ERROR;
        }
        2 => {
            (*parser).m_errorCode = crate::expat_h::XML_ERROR_FINISHED;
            return crate::expat_h::XML_STATUS_ERROR;
        }
        0 => {
            if (*parser).m_bufferPtr.is_null() {
                (*parser).m_errorCode = crate::expat_h::XML_ERROR_NO_BUFFER;
                return crate::expat_h::XML_STATUS_ERROR;
            }
            if (*parser).m_parentParser.is_null() && startParsing(parser) == 0 {
                (*parser).m_errorCode = crate::expat_h::XML_ERROR_NO_MEMORY;
                return crate::expat_h::XML_STATUS_ERROR;
            }
        }
        _ => {}
    }
    (*parser).m_parsingStatus.parsing = crate::expat_h::XML_PARSING;
    start = (*parser).m_bufferPtr;
    (*parser).m_positionPtr = start;
    (*parser).m_bufferEnd = (*parser).m_bufferEnd.offset(len as isize);
    (*parser).m_parseEndPtr = (*parser).m_bufferEnd;
    (*parser).m_parseEndByteIndex += len as crate::expat_external_h::XML_Index;
    (*parser).m_parsingStatus.finalBuffer = isFinal as crate::expat_h::XML_Bool;
    (*parser).m_errorCode = callProcessor(
        parser,
        start,
        (*parser).m_parseEndPtr,
        &raw mut (*parser).m_bufferPtr,
    );
    if (*parser).m_errorCode as ::core::ffi::c_uint
        != crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*parser).m_eventEndPtr = (*parser).m_eventPtr;
        (*parser).m_processor = Some(
            errorProcessor
                as unsafe extern "C" fn(
                    crate::expat_h::XML_Parser,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> crate::expat_h::XML_Error,
        );
        return crate::expat_h::XML_STATUS_ERROR;
    } else {
        match (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint {
            3 => {
                result = crate::expat_h::XML_STATUS_SUSPENDED;
            }
            0 | 1 => {
                if isFinal != 0 {
                    (*parser).m_parsingStatus.parsing = crate::expat_h::XML_FINISHED;
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
        &mut (*parser).m_position,
    );
    (*parser).m_positionPtr = (*parser).m_bufferPtr;
    return result;
}
#[export_name = "XML_GetBuffer"]

pub unsafe extern "C" fn XML_GetBuffer_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut len: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    if parser.is_null() {
        return crate::__stddef_null_h::NULL;
    }
    if len < 0 as ::core::ffi::c_int {
        (*parser).m_errorCode = crate::expat_h::XML_ERROR_NO_MEMORY;
        return crate::__stddef_null_h::NULL;
    }
    match (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint {
        3 => {
            (*parser).m_errorCode = crate::expat_h::XML_ERROR_SUSPENDED;
            return crate::__stddef_null_h::NULL;
        }
        2 => {
            (*parser).m_errorCode = crate::expat_h::XML_ERROR_FINISHED;
            return crate::__stddef_null_h::NULL;
        }
        _ => {}
    }
    (*parser).m_lastBufferRequestSize = len;
    if len as ::core::ffi::c_long
        > (if !(*parser).m_bufferLim.is_null() && !(*parser).m_bufferEnd.is_null() {
            (*parser).m_bufferLim.offset_from((*parser).m_bufferEnd) as ::core::ffi::c_long
        } else {
            0 as ::core::ffi::c_long
        })
        || (*parser).m_buffer.is_null()
    {
        let mut keep: ::core::ffi::c_int = 0;
        let mut neededSize: ::core::ffi::c_int = (len as ::core::ffi::c_uint).wrapping_add(
            (if !(*parser).m_bufferEnd.is_null() && !(*parser).m_bufferPtr.is_null() {
                (*parser).m_bufferEnd.offset_from((*parser).m_bufferPtr) as ::core::ffi::c_long
            } else {
                0 as ::core::ffi::c_long
            }) as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int;
        if neededSize < 0 as ::core::ffi::c_int {
            (*parser).m_errorCode = crate::expat_h::XML_ERROR_NO_MEMORY;
            return crate::__stddef_null_h::NULL;
        }
        keep = (if !(*parser).m_bufferPtr.is_null() && !(*parser).m_buffer.is_null() {
            (*parser).m_bufferPtr.offset_from((*parser).m_buffer) as ::core::ffi::c_long
        } else {
            0 as ::core::ffi::c_long
        }) as ::core::ffi::c_int;
        if keep > crate::stdlib::XML_CONTEXT_BYTES {
            keep = crate::stdlib::XML_CONTEXT_BYTES;
        }
        if keep > crate::limits_h::INT_MAX - neededSize {
            (*parser).m_errorCode = crate::expat_h::XML_ERROR_NO_MEMORY;
            return crate::__stddef_null_h::NULL;
        }
        neededSize += keep;
        if !(*parser).m_buffer.is_null()
            && !(*parser).m_bufferPtr.is_null()
            && neededSize as ::core::ffi::c_long
                <= (if !(*parser).m_bufferLim.is_null() && !(*parser).m_buffer.is_null() {
                    (*parser).m_bufferLim.offset_from((*parser).m_buffer) as ::core::ffi::c_long
                } else {
                    0 as ::core::ffi::c_long
                })
        {
            if (keep as ::core::ffi::c_long)
                < (if !(*parser).m_bufferPtr.is_null() && !(*parser).m_buffer.is_null() {
                    (*parser).m_bufferPtr.offset_from((*parser).m_buffer) as ::core::ffi::c_long
                } else {
                    0 as ::core::ffi::c_long
                })
            {
                let mut offset: ::core::ffi::c_int =
                    (if !(*parser).m_bufferPtr.is_null() && !(*parser).m_buffer.is_null() {
                        (*parser).m_bufferPtr.offset_from((*parser).m_buffer) as ::core::ffi::c_long
                    } else {
                        0 as ::core::ffi::c_long
                    }) as ::core::ffi::c_int
                        - keep;
                crate::stdlib::memmove(
                    (*parser).m_buffer as *mut ::core::ffi::c_void,
                    (*parser).m_buffer.offset(offset as isize) as *mut ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    ((*parser).m_bufferEnd.offset_from((*parser).m_bufferPtr)
                        as ::core::ffi::c_long
                        + keep as ::core::ffi::c_long)
                        as crate::__stddef_size_t_h::size_t,
                );
                (*parser).m_bufferEnd = (*parser).m_bufferEnd.offset(-(offset as isize));
                (*parser).m_bufferPtr = (*parser).m_bufferPtr.offset(-(offset as isize));
            }
        } else {
            let mut newBuf: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut bufferSize: ::core::ffi::c_int =
                (if !(*parser).m_bufferLim.is_null() && !(*parser).m_buffer.is_null() {
                    (*parser).m_bufferLim.offset_from((*parser).m_buffer) as ::core::ffi::c_long
                } else {
                    0 as ::core::ffi::c_long
                }) as ::core::ffi::c_int;
            if bufferSize == 0 as ::core::ffi::c_int {
                bufferSize = INIT_BUFFER_SIZE;
            }
            loop {
                bufferSize = (2 as ::core::ffi::c_uint)
                    .wrapping_mul(bufferSize as ::core::ffi::c_uint)
                    as ::core::ffi::c_int;
                if !(bufferSize < neededSize && bufferSize > 0 as ::core::ffi::c_int) {
                    break;
                }
            }
            if bufferSize <= 0 as ::core::ffi::c_int {
                (*parser).m_errorCode = crate::expat_h::XML_ERROR_NO_MEMORY;
                return crate::__stddef_null_h::NULL;
            }
            newBuf = (*parser)
                .m_mem
                .malloc_fcn
                .expect("non-null function pointer")(
                bufferSize as crate::__stddef_size_t_h::size_t
            ) as *mut ::core::ffi::c_char;
            if newBuf.is_null() {
                (*parser).m_errorCode = crate::expat_h::XML_ERROR_NO_MEMORY;
                return crate::__stddef_null_h::NULL;
            }
            (*parser).m_bufferLim = newBuf.offset(bufferSize as isize);
            if !(*parser).m_bufferPtr.is_null() {
                crate::stdlib::memcpy(
                    newBuf as *mut ::core::ffi::c_void,
                    (*parser).m_bufferPtr.offset(-keep as isize) as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    ((if !(*parser).m_bufferEnd.is_null() && !(*parser).m_bufferPtr.is_null() {
                        (*parser).m_bufferEnd.offset_from((*parser).m_bufferPtr)
                            as ::core::ffi::c_long
                    } else {
                        0 as ::core::ffi::c_long
                    }) + keep as ::core::ffi::c_long)
                        as crate::__stddef_size_t_h::size_t,
                );
                (*parser).m_mem.free_fcn.expect("non-null function pointer")(
                    (*parser).m_buffer as *mut ::core::ffi::c_void,
                );
                (*parser).m_buffer = newBuf;
                (*parser).m_bufferEnd = (*parser)
                    .m_buffer
                    .offset(
                        (if !(*parser).m_bufferEnd.is_null() && !(*parser).m_bufferPtr.is_null() {
                            (*parser).m_bufferEnd.offset_from((*parser).m_bufferPtr)
                                as ::core::ffi::c_long
                        } else {
                            0 as ::core::ffi::c_long
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
        (*parser).m_eventEndPtr = ::core::ptr::null::<::core::ffi::c_char>();
        (*parser).m_eventPtr = (*parser).m_eventEndPtr;
        (*parser).m_positionPtr = ::core::ptr::null::<::core::ffi::c_char>();
    }
    return (*parser).m_bufferEnd as *mut ::core::ffi::c_void;
}
fn triggerReenter(parser: &mut XML_ParserStruct) {
    parser.m_reenter = crate::expat_h::XML_TRUE;
}
pub fn XML_StopParser(
    parser: Option<&mut XML_ParserStruct>,
    mut resumable: crate::expat_h::XML_Bool,
) -> crate::expat_h::XML_Status {
    let Some(parser) = parser else {
        return crate::expat_h::XML_STATUS_ERROR;
    };
    match parser.m_parsingStatus.parsing as ::core::ffi::c_uint {
        0 => {
            parser.m_errorCode = crate::expat_h::XML_ERROR_NOT_STARTED;
            return crate::expat_h::XML_STATUS_ERROR;
        }
        3 => {
            if resumable != 0 {
                parser.m_errorCode = crate::expat_h::XML_ERROR_SUSPENDED;
                return crate::expat_h::XML_STATUS_ERROR;
            }
            parser.m_parsingStatus.parsing = crate::expat_h::XML_FINISHED;
        }
        2 => {
            parser.m_errorCode = crate::expat_h::XML_ERROR_FINISHED;
            return crate::expat_h::XML_STATUS_ERROR;
        }
        1 => {
            if resumable != 0 {
                if parser.m_isParamEntity != 0 {
                    parser.m_errorCode = crate::expat_h::XML_ERROR_SUSPEND_PE;
                    return crate::expat_h::XML_STATUS_ERROR;
                }
                parser.m_parsingStatus.parsing = crate::expat_h::XML_SUSPENDED;
            } else {
                parser.m_parsingStatus.parsing = crate::expat_h::XML_FINISHED;
            }
        }
        _ => {
            ::std::process::abort();
        }
    }
    return crate::expat_h::XML_STATUS_OK;
}
#[export_name = "XML_StopParser"]

pub unsafe extern "C" fn XML_StopParser_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut resumable: crate::expat_h::XML_Bool,
) -> crate::expat_h::XML_Status {
    XML_StopParser(parser.as_mut(), resumable)
}
#[export_name = "XML_ResumeParser"]
pub unsafe extern "C" fn XML_ResumeParser_ffi(
    mut parser: crate::expat_h::XML_Parser,
) -> crate::expat_h::XML_Status {
    let mut result: crate::expat_h::XML_Status = crate::expat_h::XML_STATUS_OK;
    if parser.is_null() {
        return crate::expat_h::XML_STATUS_ERROR;
    }
    if (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint
        != crate::expat_h::XML_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*parser).m_errorCode = crate::expat_h::XML_ERROR_NOT_SUSPENDED;
        return crate::expat_h::XML_STATUS_ERROR;
    }
    (*parser).m_parsingStatus.parsing = crate::expat_h::XML_PARSING;
    (*parser).m_errorCode = callProcessor(
        parser,
        (*parser).m_bufferPtr,
        (*parser).m_parseEndPtr,
        &raw mut (*parser).m_bufferPtr,
    );
    if (*parser).m_errorCode as ::core::ffi::c_uint
        != crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*parser).m_eventEndPtr = (*parser).m_eventPtr;
        (*parser).m_processor = Some(
            errorProcessor
                as unsafe extern "C" fn(
                    crate::expat_h::XML_Parser,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> crate::expat_h::XML_Error,
        );
        return crate::expat_h::XML_STATUS_ERROR;
    } else {
        match (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint {
            3 => {
                result = crate::expat_h::XML_STATUS_SUSPENDED;
            }
            0 | 1 => {
                if (*parser).m_parsingStatus.finalBuffer != 0 {
                    (*parser).m_parsingStatus.parsing = crate::expat_h::XML_FINISHED;
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
        &mut (*parser).m_position,
    );
    (*parser).m_positionPtr = (*parser).m_bufferPtr;
    return result;
}
pub fn XML_GetParsingStatus(
    parser: Option<&XML_ParserStruct>,
    status: &mut crate::expat_h::XML_ParsingStatus,
) {
    let Some(parser) = parser else {
        return;
    };
    *status = parser.m_parsingStatus;
}
#[export_name = "XML_GetParsingStatus"]

pub unsafe extern "C" fn XML_GetParsingStatus_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut status: *mut crate::expat_h::XML_ParsingStatus,
) {
    if parser.is_null() {
        return;
    }
    '_c2rust_label: {
        if !status.is_null() {
        } else {
            crate::stdlib::__assert_fail(
                b"status != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../../expat/lib/xmlparse.c\0".as_ptr() as *const ::core::ffi::c_char,
                2741 as ::core::ffi::c_uint,
                b"void XML_GetParsingStatus(XML_Parser, XML_ParsingStatus *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    XML_GetParsingStatus(
        parser.as_ref(),
        status.as_mut().expect("status was checked"),
    )
}
pub fn XML_GetErrorCode(parser: Option<&XML_ParserStruct>) -> crate::expat_h::XML_Error {
    match parser {
        Some(parser) => parser.m_errorCode,
        None => crate::expat_h::XML_ERROR_INVALID_ARGUMENT,
    }
}
#[export_name = "XML_GetErrorCode"]

pub unsafe extern "C" fn XML_GetErrorCode_ffi(
    mut parser: crate::expat_h::XML_Parser,
) -> crate::expat_h::XML_Error {
    XML_GetErrorCode(parser.as_ref())
}
fn byte_offset(
    end: *const ::core::ffi::c_char,
    start: *const ::core::ffi::c_char,
) -> ::core::ffi::c_long {
    (end as isize).wrapping_sub(start as isize) as ::core::ffi::c_long
}

pub fn XML_GetCurrentByteIndex(
    parser: Option<&XML_ParserStruct>,
) -> crate::expat_external_h::XML_Index {
    let Some(parser) = parser else {
        return -1 as ::core::ffi::c_int as crate::expat_external_h::XML_Index;
    };
    if !parser.m_eventPtr.is_null() {
        return parser.m_parseEndByteIndex as ::core::ffi::c_long
            - byte_offset(parser.m_parseEndPtr, parser.m_eventPtr);
    }
    return -1 as ::core::ffi::c_int as crate::expat_external_h::XML_Index;
}
#[export_name = "XML_GetCurrentByteIndex"]

pub unsafe extern "C" fn XML_GetCurrentByteIndex_ffi(
    mut parser: crate::expat_h::XML_Parser,
) -> crate::expat_external_h::XML_Index {
    XML_GetCurrentByteIndex(parser.as_ref())
}
pub fn XML_GetCurrentByteCount(parser: Option<&XML_ParserStruct>) -> ::core::ffi::c_int {
    let Some(parser) = parser else {
        return 0 as ::core::ffi::c_int;
    };
    if !parser.m_eventEndPtr.is_null() && !parser.m_eventPtr.is_null() {
        return byte_offset(parser.m_eventEndPtr, parser.m_eventPtr) as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[export_name = "XML_GetCurrentByteCount"]

pub unsafe extern "C" fn XML_GetCurrentByteCount_ffi(
    mut parser: crate::expat_h::XML_Parser,
) -> ::core::ffi::c_int {
    XML_GetCurrentByteCount(parser.as_ref())
}
struct XmlInputContext {
    buffer: *const ::core::ffi::c_char,
    offset: ::core::ffi::c_int,
    size: ::core::ffi::c_int,
}

fn XML_GetInputContext(parser: Option<&XML_ParserStruct>) -> Option<XmlInputContext> {
    let Some(parser) = parser else {
        return None;
    };
    if !parser.m_eventPtr.is_null() && !parser.m_buffer.is_null() {
        return Some(XmlInputContext {
            buffer: parser.m_buffer,
            offset: byte_offset(parser.m_eventPtr, parser.m_buffer) as ::core::ffi::c_int,
            size: byte_offset(parser.m_bufferEnd, parser.m_buffer) as ::core::ffi::c_int,
        });
    }
    None
}
#[export_name = "XML_GetInputContext"]

pub unsafe extern "C" fn XML_GetInputContext_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut offset: *mut ::core::ffi::c_int,
    mut size: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    match XML_GetInputContext(parser.as_ref()) {
        Some(context) => {
            if !offset.is_null() {
                *offset = context.offset;
            }
            if !size.is_null() {
                *size = context.size;
            }
            context.buffer
        }
        None => ::core::ptr::null::<::core::ffi::c_char>(),
    }
}
fn update_position_to_event(parser: &mut XML_ParserStruct) {
    if !parser.m_eventPtr.is_null() && parser.m_eventPtr >= parser.m_positionPtr {
        crate::src::xmltok::encoding_update_position(
            parser.m_encoding,
            parser.m_positionPtr,
            parser.m_eventPtr,
            &mut parser.m_position,
        );
        parser.m_positionPtr = parser.m_eventPtr;
    }
}

pub fn XML_GetCurrentLineNumber(
    parser: Option<&mut XML_ParserStruct>,
) -> crate::expat_external_h::XML_Size {
    let Some(parser) = parser else {
        return 0 as crate::expat_external_h::XML_Size;
    };
    update_position_to_event(parser);
    return parser
        .m_position
        .lineNumber
        .wrapping_add(1 as crate::expat_external_h::XML_Size);
}
#[export_name = "XML_GetCurrentLineNumber"]

pub unsafe extern "C" fn XML_GetCurrentLineNumber_ffi(
    mut parser: crate::expat_h::XML_Parser,
) -> crate::expat_external_h::XML_Size {
    XML_GetCurrentLineNumber(parser.as_mut())
}
pub fn XML_GetCurrentColumnNumber(
    parser: Option<&mut XML_ParserStruct>,
) -> crate::expat_external_h::XML_Size {
    let Some(parser) = parser else {
        return 0 as crate::expat_external_h::XML_Size;
    };
    update_position_to_event(parser);
    return parser.m_position.columnNumber;
}
#[export_name = "XML_GetCurrentColumnNumber"]

pub unsafe extern "C" fn XML_GetCurrentColumnNumber_ffi(
    mut parser: crate::expat_h::XML_Parser,
) -> crate::expat_external_h::XML_Size {
    XML_GetCurrentColumnNumber(parser.as_mut())
}
#[export_name = "XML_FreeContentModel"]

pub unsafe extern "C" fn XML_FreeContentModel_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut model: *mut crate::expat_h::XML_Content,
) {
    if !parser.is_null() {
        (*parser).m_mem.free_fcn.expect("non-null function pointer")(
            model as *mut ::core::ffi::c_void,
        );
    }
}
#[export_name = "XML_MemMalloc"]

pub unsafe extern "C" fn XML_MemMalloc_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut size: crate::__stddef_size_t_h::size_t,
) -> *mut ::core::ffi::c_void {
    if !parser.is_null() {
        return (*parser)
            .m_mem
            .malloc_fcn
            .expect("non-null function pointer")(size);
    }
    return crate::__stddef_null_h::NULL;
}
#[export_name = "XML_MemRealloc"]

pub unsafe extern "C" fn XML_MemRealloc_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut ptr: *mut ::core::ffi::c_void,
    mut size: crate::__stddef_size_t_h::size_t,
) -> *mut ::core::ffi::c_void {
    if !parser.is_null() {
        return (*parser)
            .m_mem
            .realloc_fcn
            .expect("non-null function pointer")(ptr, size);
    }
    return crate::__stddef_null_h::NULL;
}
#[export_name = "XML_MemFree"]

pub unsafe extern "C" fn XML_MemFree_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut ptr: *mut ::core::ffi::c_void,
) {
    if !parser.is_null() {
        (*parser).m_mem.free_fcn.expect("non-null function pointer")(ptr);
    }
}
#[export_name = "XML_DefaultCurrent"]

pub unsafe extern "C" fn XML_DefaultCurrent_ffi(mut parser: crate::expat_h::XML_Parser) {
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
pub extern "C" fn XML_ErrorString(
    mut code: crate::expat_h::XML_Error,
) -> *const crate::expat_external_h::XML_LChar {
    match code as ::core::ffi::c_uint {
        0 => return ::core::ptr::null::<crate::expat_external_h::XML_LChar>(),
        1 => return b"out of memory\0".as_ptr() as *const crate::expat_external_h::XML_LChar,
        2 => return b"syntax error\0".as_ptr() as *const crate::expat_external_h::XML_LChar,
        3 => return b"no element found\0".as_ptr() as *const crate::expat_external_h::XML_LChar,
        4 => {
            return b"not well-formed (invalid token)\0".as_ptr()
                as *const crate::expat_external_h::XML_LChar
        }
        5 => return b"unclosed token\0".as_ptr() as *const crate::expat_external_h::XML_LChar,
        6 => return b"partial character\0".as_ptr() as *const crate::expat_external_h::XML_LChar,
        7 => return b"mismatched tag\0".as_ptr() as *const crate::expat_external_h::XML_LChar,
        8 => return b"duplicate attribute\0".as_ptr() as *const crate::expat_external_h::XML_LChar,
        9 => {
            return b"junk after document element\0".as_ptr()
                as *const crate::expat_external_h::XML_LChar
        }
        10 => {
            return b"illegal parameter entity reference\0".as_ptr()
                as *const crate::expat_external_h::XML_LChar
        }
        11 => return b"undefined entity\0".as_ptr() as *const crate::expat_external_h::XML_LChar,
        12 => {
            return b"recursive entity reference\0".as_ptr()
                as *const crate::expat_external_h::XML_LChar
        }
        13 => {
            return b"asynchronous entity\0".as_ptr() as *const crate::expat_external_h::XML_LChar
        }
        14 => {
            return b"reference to invalid character number\0".as_ptr()
                as *const crate::expat_external_h::XML_LChar;
        }
        15 => {
            return b"reference to binary entity\0".as_ptr()
                as *const crate::expat_external_h::XML_LChar
        }
        16 => {
            return b"reference to external entity in attribute\0".as_ptr()
                as *const crate::expat_external_h::XML_LChar;
        }
        17 => {
            return b"XML or text declaration not at start of entity\0".as_ptr()
                as *const crate::expat_external_h::XML_LChar;
        }
        18 => return b"unknown encoding\0".as_ptr() as *const crate::expat_external_h::XML_LChar,
        19 => {
            return b"encoding specified in XML declaration is incorrect\0".as_ptr()
                as *const crate::expat_external_h::XML_LChar;
        }
        20 => {
            return b"unclosed CDATA section\0".as_ptr()
                as *const crate::expat_external_h::XML_LChar
        }
        21 => {
            return b"error in processing external entity reference\0".as_ptr()
                as *const crate::expat_external_h::XML_LChar;
        }
        22 => {
            return b"document is not standalone\0".as_ptr()
                as *const crate::expat_external_h::XML_LChar
        }
        23 => {
            return b"unexpected parser state - please send a bug report\0".as_ptr()
                as *const crate::expat_external_h::XML_LChar;
        }
        24 => {
            return b"entity declared in parameter entity\0".as_ptr()
                as *const crate::expat_external_h::XML_LChar;
        }
        25 => {
            return b"requested feature requires XML_DTD support in Expat\0".as_ptr()
                as *const crate::expat_external_h::XML_LChar;
        }
        26 => {
            return b"cannot change setting once parsing has begun\0".as_ptr()
                as *const crate::expat_external_h::XML_LChar;
        }
        27 => return b"unbound prefix\0".as_ptr() as *const crate::expat_external_h::XML_LChar,
        28 => {
            return b"must not undeclare prefix\0".as_ptr()
                as *const crate::expat_external_h::XML_LChar
        }
        29 => {
            return b"incomplete markup in parameter entity\0".as_ptr()
                as *const crate::expat_external_h::XML_LChar;
        }
        30 => {
            return b"XML declaration not well-formed\0".as_ptr()
                as *const crate::expat_external_h::XML_LChar
        }
        31 => {
            return b"text declaration not well-formed\0".as_ptr()
                as *const crate::expat_external_h::XML_LChar
        }
        32 => {
            return b"illegal character(s) in public id\0".as_ptr()
                as *const crate::expat_external_h::XML_LChar
        }
        33 => return b"parser suspended\0".as_ptr() as *const crate::expat_external_h::XML_LChar,
        34 => {
            return b"parser not suspended\0".as_ptr() as *const crate::expat_external_h::XML_LChar
        }
        35 => return b"parsing aborted\0".as_ptr() as *const crate::expat_external_h::XML_LChar,
        36 => return b"parsing finished\0".as_ptr() as *const crate::expat_external_h::XML_LChar,
        37 => {
            return b"cannot suspend in external parameter entity\0".as_ptr()
                as *const crate::expat_external_h::XML_LChar;
        }
        38 => {
            return b"reserved prefix (xml) must not be undeclared or bound to another namespace name\0"
                .as_ptr() as *const crate::expat_external_h::XML_LChar;
        }
        39 => {
            return b"reserved prefix (xmlns) must not be declared or undeclared\0".as_ptr()
                as *const crate::expat_external_h::XML_LChar;
        }
        40 => {
            return b"prefix must not be bound to one of the reserved namespace names\0".as_ptr()
                as *const crate::expat_external_h::XML_LChar;
        }
        41 => return b"invalid argument\0".as_ptr() as *const crate::expat_external_h::XML_LChar,
        42 => {
            return b"a successful prior call to function XML_GetBuffer is required\0".as_ptr()
                as *const crate::expat_external_h::XML_LChar;
        }
        43 => {
            return b"limit on input amplification factor (from DTD and entities) breached\0"
                .as_ptr() as *const crate::expat_external_h::XML_LChar;
        }
        44 => return b"parser not started\0".as_ptr() as *const crate::expat_external_h::XML_LChar,
        _ => {}
    }
    return ::core::ptr::null::<crate::expat_external_h::XML_LChar>();
}
#[export_name = "XML_ErrorString"]

pub unsafe extern "C" fn XML_ErrorString_ffi(
    mut code: crate::expat_h::XML_Error,
) -> *const crate::expat_external_h::XML_LChar {
    XML_ErrorString(code)
}
pub extern "C" fn XML_ExpatVersion() -> *const crate::expat_external_h::XML_LChar {
    return b"expat_2.7.4\0".as_ptr() as *const crate::expat_external_h::XML_LChar;
}
#[export_name = "XML_ExpatVersion"]

pub unsafe extern "C" fn XML_ExpatVersion_ffi() -> *const crate::expat_external_h::XML_LChar {
    XML_ExpatVersion()
}
pub extern "C" fn XML_ExpatVersionInfo() -> crate::expat_h::XML_Expat_Version {
    let mut version: crate::expat_h::XML_Expat_Version = crate::expat_h::XML_Expat_Version {
        major: 0,
        minor: 0,
        micro: 0,
    };
    version.major = crate::expat_h::XML_MAJOR_VERSION;
    version.minor = crate::expat_h::XML_MINOR_VERSION;
    version.micro = crate::expat_h::XML_MICRO_VERSION;
    return version;
}
#[export_name = "XML_ExpatVersionInfo"]

pub unsafe extern "C" fn XML_ExpatVersionInfo_ffi() -> crate::expat_h::XML_Expat_Version {
    XML_ExpatVersionInfo()
}
pub extern "C" fn XML_GetFeatureList() -> *const crate::expat_h::XML_Feature {
    static FEATURES: std::sync::OnceLock<
        std::sync::atomic::AtomicPtr<crate::expat_h::XML_Feature>,
    > = std::sync::OnceLock::new();

    FEATURES
        .get_or_init(|| {
            std::sync::atomic::AtomicPtr::new(Box::leak(Box::new([
        crate::expat_h::XML_Feature {
    feature:  crate::expat_h::XML_FEATURE_SIZEOF_XML_CHAR,
    name:  b"sizeof(XML_Char)\0".as_ptr() as *const crate::expat_external_h::XML_LChar,
    value:  ::core::mem::size_of:: <crate::expat_external_h::XML_Char>() as ::core::ffi::c_long,
},
        crate::expat_h::XML_Feature {
    feature:  crate::expat_h::XML_FEATURE_SIZEOF_XML_LCHAR,
    name:  b"sizeof(XML_LChar)\0".as_ptr() as *const crate::expat_external_h::XML_LChar,
    value:  ::core::mem::size_of:: <crate::expat_external_h::XML_LChar>() as ::core::ffi::c_long,
},
        crate::expat_h::XML_Feature {
    feature:  crate::expat_h::XML_FEATURE_DTD,
    name:  b"XML_DTD\0".as_ptr() as *const crate::expat_external_h::XML_LChar,
    value:  0 as ::core::ffi::c_long,
},
        crate::expat_h::XML_Feature {
    feature:  crate::expat_h::XML_FEATURE_CONTEXT_BYTES,
    name:  b"XML_CONTEXT_BYTES\0".as_ptr() as *const crate::expat_external_h::XML_LChar,
    value:  crate::stdlib::XML_CONTEXT_BYTES as ::core::ffi::c_long,
},
        crate::expat_h::XML_Feature {
    feature:  crate::expat_h::XML_FEATURE_NS,
    name:  b"XML_NS\0".as_ptr() as *const crate::expat_external_h::XML_LChar,
    value:  0 as ::core::ffi::c_long,
},
        crate::expat_h::XML_Feature {
    feature:  crate::expat_h::XML_FEATURE_BILLION_LAUGHS_ATTACK_PROTECTION_MAXIMUM_AMPLIFICATION_DEFAULT,
    name:  b"XML_BLAP_MAX_AMP\0".as_ptr() as *const crate::expat_external_h::XML_LChar,
    value:  crate::internal_h::EXPAT_BILLION_LAUGHS_ATTACK_PROTECTION_MAXIMUM_AMPLIFICATION_DEFAULT
                as ::core::ffi::c_long,
},
        crate::expat_h::XML_Feature {
    feature:  crate::expat_h::XML_FEATURE_BILLION_LAUGHS_ATTACK_PROTECTION_ACTIVATION_THRESHOLD_DEFAULT,
    name:  b"XML_BLAP_ACT_THRES\0".as_ptr() as *const crate::expat_external_h::XML_LChar,
    value:  crate::internal_h::EXPAT_BILLION_LAUGHS_ATTACK_PROTECTION_ACTIVATION_THRESHOLD_DEFAULT
                as ::core::ffi::c_long,
},
        crate::expat_h::XML_Feature {
    feature:  crate::expat_h::XML_FEATURE_GE,
    name:  b"XML_GE\0".as_ptr() as *const crate::expat_external_h::XML_LChar,
    value:  0 as ::core::ffi::c_long,
},
        crate::expat_h::XML_Feature {
    feature:  crate::expat_h::XML_FEATURE_ALLOC_TRACKER_MAXIMUM_AMPLIFICATION_DEFAULT,
    name:  b"XML_AT_MAX_AMP\0".as_ptr() as *const crate::expat_external_h::XML_LChar,
    value:  crate::internal_h::EXPAT_ALLOC_TRACKER_MAXIMUM_AMPLIFICATION_DEFAULT as ::core::ffi::c_long,
},
        crate::expat_h::XML_Feature {
    feature:  crate::expat_h::XML_FEATURE_ALLOC_TRACKER_ACTIVATION_THRESHOLD_DEFAULT,
    name:  b"XML_AT_ACT_THRES\0".as_ptr() as *const crate::expat_external_h::XML_LChar,
    value:  crate::internal_h::EXPAT_ALLOC_TRACKER_ACTIVATION_THRESHOLD_DEFAULT as ::core::ffi::c_long,
},
        crate::expat_h::XML_Feature {
    feature:  crate::expat_h::XML_FEATURE_END,
    name:  ::core::ptr::null:: <crate::expat_external_h::XML_LChar>(),
    value:  0 as ::core::ffi::c_long,
},
    ]))
            .as_mut_ptr())
        })
        .load(std::sync::atomic::Ordering::Relaxed)
        as *const crate::expat_h::XML_Feature
}
#[export_name = "XML_GetFeatureList"]

pub unsafe extern "C" fn XML_GetFeatureList_ffi() -> *const crate::expat_h::XML_Feature {
    XML_GetFeatureList()
}
pub fn XML_SetBillionLaughsAttackProtectionMaximumAmplification(
    parser: Option<&mut XML_ParserStruct>,
    maximumAmplificationFactor: ::core::ffi::c_float,
) -> crate::expat_h::XML_Bool {
    let Some(parser) = parser else {
        return crate::expat_h::XML_FALSE;
    };
    if !parser.m_parentParser.is_null()
        || maximumAmplificationFactor.is_nan() as i32 != 0
        || maximumAmplificationFactor < 1.0f32
    {
        return crate::expat_h::XML_FALSE;
    }
    parser.m_accounting.maximumAmplificationFactor = maximumAmplificationFactor;
    return crate::expat_h::XML_TRUE;
}
#[export_name = "XML_SetBillionLaughsAttackProtectionMaximumAmplification"]

pub unsafe extern "C" fn XML_SetBillionLaughsAttackProtectionMaximumAmplification_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut maximumAmplificationFactor: ::core::ffi::c_float,
) -> crate::expat_h::XML_Bool {
    XML_SetBillionLaughsAttackProtectionMaximumAmplification(
        parser.as_mut(),
        maximumAmplificationFactor,
    )
}
pub fn XML_SetBillionLaughsAttackProtectionActivationThreshold(
    parser: Option<&mut XML_ParserStruct>,
    activationThresholdBytes: ::core::ffi::c_ulonglong,
) -> crate::expat_h::XML_Bool {
    let Some(parser) = parser else {
        return crate::expat_h::XML_FALSE;
    };
    if !parser.m_parentParser.is_null() {
        return crate::expat_h::XML_FALSE;
    }
    parser.m_accounting.activationThresholdBytes = activationThresholdBytes;
    return crate::expat_h::XML_TRUE;
}
#[export_name = "XML_SetBillionLaughsAttackProtectionActivationThreshold"]

pub unsafe extern "C" fn XML_SetBillionLaughsAttackProtectionActivationThreshold_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut activationThresholdBytes: ::core::ffi::c_ulonglong,
) -> crate::expat_h::XML_Bool {
    XML_SetBillionLaughsAttackProtectionActivationThreshold(
        parser.as_mut(),
        activationThresholdBytes,
    )
}
pub fn XML_SetAllocTrackerMaximumAmplification(
    parser: Option<&mut XML_ParserStruct>,
    maximumAmplificationFactor: ::core::ffi::c_float,
) -> crate::expat_h::XML_Bool {
    let Some(parser) = parser else {
        return crate::expat_h::XML_FALSE;
    };
    if !parser.m_parentParser.is_null()
        || maximumAmplificationFactor.is_nan() as i32 != 0
        || maximumAmplificationFactor < 1.0f32
    {
        return crate::expat_h::XML_FALSE;
    }
    parser.m_alloc_tracker.maximumAmplificationFactor = maximumAmplificationFactor;
    return crate::expat_h::XML_TRUE;
}
#[export_name = "XML_SetAllocTrackerMaximumAmplification"]

pub unsafe extern "C" fn XML_SetAllocTrackerMaximumAmplification_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut maximumAmplificationFactor: ::core::ffi::c_float,
) -> crate::expat_h::XML_Bool {
    XML_SetAllocTrackerMaximumAmplification(parser.as_mut(), maximumAmplificationFactor)
}
pub fn XML_SetAllocTrackerActivationThreshold(
    parser: Option<&mut XML_ParserStruct>,
    activationThresholdBytes: ::core::ffi::c_ulonglong,
) -> crate::expat_h::XML_Bool {
    let Some(parser) = parser else {
        return crate::expat_h::XML_FALSE;
    };
    if !parser.m_parentParser.is_null() {
        return crate::expat_h::XML_FALSE;
    }
    parser.m_alloc_tracker.activationThresholdBytes = activationThresholdBytes as XmlBigCount;
    return crate::expat_h::XML_TRUE;
}
#[export_name = "XML_SetAllocTrackerActivationThreshold"]

pub unsafe extern "C" fn XML_SetAllocTrackerActivationThreshold_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut activationThresholdBytes: ::core::ffi::c_ulonglong,
) -> crate::expat_h::XML_Bool {
    XML_SetAllocTrackerActivationThreshold(parser.as_mut(), activationThresholdBytes)
}
pub fn XML_SetReparseDeferralEnabled(
    parser: Option<&mut XML_ParserStruct>,
    enabled: crate::expat_h::XML_Bool,
) -> crate::expat_h::XML_Bool {
    if let Some(parser) = parser {
        if enabled as ::core::ffi::c_int == crate::expat_h::XML_TRUE as ::core::ffi::c_int
            || enabled as ::core::ffi::c_int == crate::expat_h::XML_FALSE as ::core::ffi::c_int
        {
            parser.m_reparseDeferralEnabled = enabled;
            return crate::expat_h::XML_TRUE;
        }
    }
    return crate::expat_h::XML_FALSE;
}
#[export_name = "XML_SetReparseDeferralEnabled"]

pub unsafe extern "C" fn XML_SetReparseDeferralEnabled_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut enabled: crate::expat_h::XML_Bool,
) -> crate::expat_h::XML_Bool {
    XML_SetReparseDeferralEnabled(parser.as_mut(), enabled)
}
unsafe extern "C" fn storeRawNames(
    mut parser: crate::expat_h::XML_Parser,
) -> crate::expat_h::XML_Bool {
    let mut tag: *mut TAG = (*parser).m_tagStack;
    while !tag.is_null() {
        let mut bufSize: crate::__stddef_size_t_h::size_t = 0;
        let mut nameLen: crate::__stddef_size_t_h::size_t = (::core::mem::size_of::<
            crate::expat_external_h::XML_Char,
        >()
            as crate::__stddef_size_t_h::size_t)
            .wrapping_mul(
                ((*tag).name.strLen + 1 as ::core::ffi::c_int) as crate::__stddef_size_t_h::size_t,
            );
        let mut rawNameLen: crate::__stddef_size_t_h::size_t = 0;
        let mut rawNameBuf: *mut ::core::ffi::c_char = (*tag).buf.raw.offset(nameLen as isize);
        if (*tag).rawName == rawNameBuf as *const ::core::ffi::c_char {
            break;
        }
        rawNameLen = (((*tag).rawNameLength as usize).wrapping_add(
            (::core::mem::size_of::<crate::expat_external_h::XML_Char>() as usize)
                .wrapping_sub(1 as usize),
        ) & !(::core::mem::size_of::<crate::expat_external_h::XML_Char>() as usize)
            .wrapping_sub(1 as usize)) as crate::__stddef_size_t_h::size_t;
        if rawNameLen
            > (crate::limits_h::INT_MAX as crate::__stddef_size_t_h::size_t).wrapping_sub(nameLen)
        {
            return crate::expat_h::XML_FALSE;
        }
        bufSize = nameLen.wrapping_add(rawNameLen);
        if bufSize
            > (*tag).bufEnd.offset_from((*tag).buf.raw) as ::core::ffi::c_long
                as crate::__stddef_size_t_h::size_t
        {
            let mut temp: *mut ::core::ffi::c_char = expat_realloc(
                parser,
                (*tag).buf.raw as *mut ::core::ffi::c_void,
                bufSize,
                3151 as ::core::ffi::c_int,
            ) as *mut ::core::ffi::c_char;
            if temp.is_null() {
                return crate::expat_h::XML_FALSE;
            }
            if (*tag).name.str == (*tag).buf.str as *const crate::expat_external_h::XML_Char {
                (*tag).name.str = temp as *mut crate::expat_external_h::XML_Char;
            }
            if !(*tag).name.localPart.is_null() {
                (*tag).name.localPart =
                    (temp as *mut crate::expat_external_h::XML_Char)
                        .offset((*tag).name.localPart.offset_from((*tag).buf.str)
                            as ::core::ffi::c_long as isize);
            }
            (*tag).buf.raw = temp;
            (*tag).bufEnd = temp.offset(bufSize as isize);
            rawNameBuf = temp.offset(nameLen as isize);
        }
        crate::stdlib::memcpy(
            rawNameBuf as *mut ::core::ffi::c_void,
            (*tag).rawName as *const ::core::ffi::c_void,
            (*tag).rawNameLength as crate::__stddef_size_t_h::size_t,
        );
        (*tag).rawName = rawNameBuf;
        tag = (*tag).parent as *mut TAG;
    }
    return crate::expat_h::XML_TRUE;
}

unsafe extern "C" fn contentProcessor(
    mut parser: crate::expat_h::XML_Parser,
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut endPtr: *mut *const ::core::ffi::c_char,
) -> crate::expat_h::XML_Error {
    let mut result: crate::expat_h::XML_Error = doContent(
        parser,
        if !(*parser).m_parentParser.is_null() {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
        (*parser).m_encoding,
        start,
        end,
        endPtr,
        ((*parser).m_parsingStatus.finalBuffer == 0) as ::core::ffi::c_int
            as crate::expat_h::XML_Bool,
        XML_ACCOUNT_DIRECT,
    );
    if result as ::core::ffi::c_uint
        == crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if storeRawNames(parser) == 0 {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
    }
    return result;
}

unsafe extern "C" fn externalEntityInitProcessor(
    mut parser: crate::expat_h::XML_Parser,
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut endPtr: *mut *const ::core::ffi::c_char,
) -> crate::expat_h::XML_Error {
    let mut result: crate::expat_h::XML_Error = initializeEncoding(parser);
    if result as ::core::ffi::c_uint
        != crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return result;
    }
    (*parser).m_processor = Some(
        externalEntityInitProcessor2
            as unsafe extern "C" fn(
                crate::expat_h::XML_Parser,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut *const ::core::ffi::c_char,
            ) -> crate::expat_h::XML_Error,
    );
    return externalEntityInitProcessor2(parser, start, end, endPtr);
}

unsafe extern "C" fn externalEntityInitProcessor2(
    mut parser: crate::expat_h::XML_Parser,
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut endPtr: *mut *const ::core::ffi::c_char,
) -> crate::expat_h::XML_Error {
    let mut next: *const ::core::ffi::c_char = start;
    let mut tok: ::core::ffi::c_int = (*(*parser).m_encoding).scanners
        [1 as ::core::ffi::c_int as usize]
        .expect("non-null function pointer")(
        (*parser).m_encoding, start, end, &raw mut next
    );
    match tok {
        crate::src::xmltok::XML_TOK_BOM => {
            let mut accounting_levels: ::core::ffi::c_uint = 0;
            let accounting_root =
                getRootParserOf(parser, &raw mut accounting_levels) as crate::expat_h::XML_Parser;
            if accountingDiffTolerated(
                &mut *accounting_root,
                accounting_levels,
                parser == accounting_root,
                tok,
                || {
                    ::core::slice::from_raw_parts(
                        start as *const ::core::ffi::c_uchar,
                        byte_offset(next, start) as usize,
                    )
                },
                3208 as ::core::ffi::c_int,
                XML_ACCOUNT_DIRECT,
            ) == 0
            {
                accountingReportStats(&*accounting_root, ACCOUNTING_ABORTING_EPILOG);
                return crate::expat_h::XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
            }
            if next == end && (*parser).m_parsingStatus.finalBuffer == 0 {
                *endPtr = next;
                return crate::expat_h::XML_ERROR_NONE;
            }
            start = next;
        }
        crate::src::xmltok::XML_TOK_PARTIAL => {
            if (*parser).m_parsingStatus.finalBuffer == 0 {
                *endPtr = start;
                return crate::expat_h::XML_ERROR_NONE;
            }
            (*parser).m_eventPtr = start;
            return crate::expat_h::XML_ERROR_UNCLOSED_TOKEN;
        }
        crate::src::xmltok::XML_TOK_PARTIAL_CHAR => {
            if (*parser).m_parsingStatus.finalBuffer == 0 {
                *endPtr = start;
                return crate::expat_h::XML_ERROR_NONE;
            }
            (*parser).m_eventPtr = start;
            return crate::expat_h::XML_ERROR_PARTIAL_CHAR;
        }
        _ => {}
    }
    (*parser).m_processor = Some(
        externalEntityInitProcessor3
            as unsafe extern "C" fn(
                crate::expat_h::XML_Parser,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut *const ::core::ffi::c_char,
            ) -> crate::expat_h::XML_Error,
    );
    return externalEntityInitProcessor3(parser, start, end, endPtr);
}

unsafe extern "C" fn externalEntityInitProcessor3(
    mut parser: crate::expat_h::XML_Parser,
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut endPtr: *mut *const ::core::ffi::c_char,
) -> crate::expat_h::XML_Error {
    let mut tok: ::core::ffi::c_int = 0;
    let mut next: *const ::core::ffi::c_char = start;
    (*parser).m_eventPtr = start;
    tok = (*(*parser).m_encoding).scanners[1 as ::core::ffi::c_int as usize]
        .expect("non-null function pointer")(
        (*parser).m_encoding, start, end, &raw mut next
    );
    (*parser).m_eventEndPtr = next;
    match tok {
        crate::src::xmltok::XML_TOK_XML_DECL => {
            let mut result: crate::expat_h::XML_Error = crate::expat_h::XML_ERROR_NONE;
            result = processXmlDecl(parser, 1 as ::core::ffi::c_int, start, next);
            if result as ::core::ffi::c_uint
                != crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return result;
            }
            match (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint {
                3 => {
                    *endPtr = next;
                    return crate::expat_h::XML_ERROR_NONE;
                }
                2 => return crate::expat_h::XML_ERROR_ABORTED,
                1 => {
                    if (*parser).m_reenter != 0 {
                        return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
                    }
                }
                _ => {}
            }
            start = next;
        }
        crate::src::xmltok::XML_TOK_PARTIAL => {
            if (*parser).m_parsingStatus.finalBuffer == 0 {
                *endPtr = start;
                return crate::expat_h::XML_ERROR_NONE;
            }
            return crate::expat_h::XML_ERROR_UNCLOSED_TOKEN;
        }
        crate::src::xmltok::XML_TOK_PARTIAL_CHAR => {
            if (*parser).m_parsingStatus.finalBuffer == 0 {
                *endPtr = start;
                return crate::expat_h::XML_ERROR_NONE;
            }
            return crate::expat_h::XML_ERROR_PARTIAL_CHAR;
        }
        _ => {}
    }
    (*parser).m_processor = Some(
        externalEntityContentProcessor
            as unsafe extern "C" fn(
                crate::expat_h::XML_Parser,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut *const ::core::ffi::c_char,
            ) -> crate::expat_h::XML_Error,
    );
    (*parser).m_tagLevel = 1 as ::core::ffi::c_int;
    return externalEntityContentProcessor(parser, start, end, endPtr);
}

unsafe extern "C" fn externalEntityContentProcessor(
    mut parser: crate::expat_h::XML_Parser,
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut endPtr: *mut *const ::core::ffi::c_char,
) -> crate::expat_h::XML_Error {
    let mut result: crate::expat_h::XML_Error = doContent(
        parser,
        1 as ::core::ffi::c_int,
        (*parser).m_encoding,
        start,
        end,
        endPtr,
        ((*parser).m_parsingStatus.finalBuffer == 0) as ::core::ffi::c_int
            as crate::expat_h::XML_Bool,
        XML_ACCOUNT_ENTITY_EXPANSION,
    );
    if result as ::core::ffi::c_uint
        == crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if storeRawNames(parser) == 0 {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
    }
    return result;
}

unsafe extern "C" fn doContent(
    mut parser: crate::expat_h::XML_Parser,
    mut startTagLevel: ::core::ffi::c_int,
    mut enc: *const crate::src::xmltok::ENCODING,
    mut s: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextPtr: *mut *const ::core::ffi::c_char,
    mut haveMore: crate::expat_h::XML_Bool,
    mut account: XML_Account,
) -> crate::expat_h::XML_Error {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut eventPP: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    let mut eventEndPP: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    if enc == (*parser).m_encoding {
        eventPP = &raw mut (*parser).m_eventPtr;
        eventEndPP = &raw mut (*parser).m_eventEndPtr;
    } else {
        eventPP = &raw mut (*(*parser).m_openInternalEntities).internalEventPtr;
        eventEndPP = &raw mut (*(*parser).m_openInternalEntities).internalEventEndPtr;
    }
    *eventPP = s;
    loop {
        let mut next: *const ::core::ffi::c_char = s;
        let mut tok: ::core::ffi::c_int = (*enc).scanners[1 as ::core::ffi::c_int as usize]
            .expect("non-null function pointer")(
            enc, s, end, &raw mut next
        );
        let mut accountAfter: *const ::core::ffi::c_char = if tok
            == crate::src::xmltok::XML_TOK_TRAILING_RSQB
            || tok == crate::src::xmltok::XML_TOK_TRAILING_CR
        {
            if haveMore as ::core::ffi::c_int != 0 {
                s
            } else {
                end
            }
        } else {
            next
        };
        let mut accounting_levels: ::core::ffi::c_uint = 0;
        let accounting_root =
            getRootParserOf(parser, &raw mut accounting_levels) as crate::expat_h::XML_Parser;
        if accountingDiffTolerated(
            &mut *accounting_root,
            accounting_levels,
            parser == accounting_root,
            tok,
            || {
                ::core::slice::from_raw_parts(
                    s as *const ::core::ffi::c_uchar,
                    byte_offset(accountAfter, s) as usize,
                )
            },
            3337 as ::core::ffi::c_int,
            account,
        ) == 0
        {
            accountingReportStats(&*accounting_root, ACCOUNTING_ABORTING_EPILOG);
            return crate::expat_h::XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
        }
        *eventEndPP = next;
        let mut c2rust_current_block_281: u64;
        match tok {
            crate::src::xmltok::XML_TOK_TRAILING_CR => {
                if haveMore != 0 {
                    *nextPtr = s;
                    return crate::expat_h::XML_ERROR_NONE;
                }
                *eventEndPP = end;
                if (*parser).m_characterDataHandler.is_some() {
                    let mut c: crate::expat_external_h::XML_Char =
                        0xa as crate::expat_external_h::XML_Char;
                    (*parser)
                        .m_characterDataHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        &raw mut c,
                        1 as ::core::ffi::c_int,
                    );
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, end);
                }
                if startTagLevel == 0 as ::core::ffi::c_int {
                    return crate::expat_h::XML_ERROR_NO_ELEMENTS;
                }
                if (*parser).m_tagLevel != startTagLevel {
                    return crate::expat_h::XML_ERROR_ASYNC_ENTITY;
                }
                *nextPtr = end;
                return crate::expat_h::XML_ERROR_NONE;
            }
            crate::src::xmltok::XML_TOK_NONE => {
                if haveMore != 0 {
                    *nextPtr = s;
                    return crate::expat_h::XML_ERROR_NONE;
                }
                if startTagLevel > 0 as ::core::ffi::c_int {
                    if (*parser).m_tagLevel != startTagLevel {
                        return crate::expat_h::XML_ERROR_ASYNC_ENTITY;
                    }
                    *nextPtr = s;
                    return crate::expat_h::XML_ERROR_NONE;
                }
                return crate::expat_h::XML_ERROR_NO_ELEMENTS;
            }
            crate::src::xmltok::XML_TOK_INVALID => {
                *eventPP = next;
                return crate::expat_h::XML_ERROR_INVALID_TOKEN;
            }
            crate::src::xmltok::XML_TOK_PARTIAL => {
                if haveMore != 0 {
                    *nextPtr = s;
                    return crate::expat_h::XML_ERROR_NONE;
                }
                return crate::expat_h::XML_ERROR_UNCLOSED_TOKEN;
            }
            crate::src::xmltok::XML_TOK_PARTIAL_CHAR => {
                if haveMore != 0 {
                    *nextPtr = s;
                    return crate::expat_h::XML_ERROR_NONE;
                }
                return crate::expat_h::XML_ERROR_PARTIAL_CHAR;
            }
            crate::src::xmltok::XML_TOK_ENTITY_REF => {
                let mut name: *const crate::expat_external_h::XML_Char =
                    ::core::ptr::null::<crate::expat_external_h::XML_Char>();
                let mut entity: *mut ENTITY = ::core::ptr::null_mut::<ENTITY>();
                let mut ch: crate::expat_external_h::XML_Char = (*enc)
                    .predefinedEntityName
                    .expect("non-null function pointer")(
                    enc,
                    s.offset((*enc).minBytesPerChar as isize),
                    next.offset(-((*enc).minBytesPerChar as isize)),
                )
                    as crate::expat_external_h::XML_Char;
                if ch != 0 {
                    let mut accounting_levels: ::core::ffi::c_uint = 0;
                    let accounting_root = getRootParserOf(parser, &raw mut accounting_levels)
                        as crate::expat_h::XML_Parser;
                    let ch_start = &raw mut ch as *mut ::core::ffi::c_char;
                    let ch_end = ch_start.offset(::core::mem::size_of::<
                        crate::expat_external_h::XML_Char,
                    >() as usize as isize);
                    accountingDiffTolerated(
                        &mut *accounting_root,
                        accounting_levels,
                        parser == accounting_root,
                        tok,
                        || {
                            ::core::slice::from_raw_parts(
                                ch_start as *const ::core::ffi::c_uchar,
                                byte_offset(ch_end, ch_start) as usize,
                            )
                        },
                        3403 as ::core::ffi::c_int,
                        XML_ACCOUNT_ENTITY_EXPANSION,
                    );
                    if (*parser).m_characterDataHandler.is_some() {
                        (*parser)
                            .m_characterDataHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            &raw mut ch,
                            1 as ::core::ffi::c_int,
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
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    entity = lookup(
                        parser,
                        &raw mut (*dtd).generalEntities,
                        name as KEY,
                        0 as crate::__stddef_size_t_h::size_t,
                    ) as *mut ENTITY;
                    (*dtd).pool.ptr = (*dtd).pool.start;
                    if (*dtd).hasParamEntityRefs == 0
                        || (*dtd).standalone as ::core::ffi::c_int != 0
                    {
                        if entity.is_null() {
                            return crate::expat_h::XML_ERROR_UNDEFINED_ENTITY;
                        } else if (*entity).is_internal == 0 {
                            return crate::expat_h::XML_ERROR_ENTITY_DECLARED_IN_PE;
                        }
                        c2rust_current_block_281 = 3546145585875536353;
                    } else if entity.is_null() {
                        if (*parser).m_skippedEntityHandler.is_some() {
                            (*parser)
                                .m_skippedEntityHandler
                                .expect("non-null function pointer")(
                                (*parser).m_handlerArg,
                                name,
                                0 as ::core::ffi::c_int,
                            );
                        } else if (*parser).m_defaultHandler.is_some() {
                            reportDefault(parser, enc, s, next);
                        }
                        c2rust_current_block_281 = 1957216233951053322;
                    } else {
                        c2rust_current_block_281 = 3546145585875536353;
                    }
                    match c2rust_current_block_281 {
                        1957216233951053322 => {}
                        _ => {
                            if (*entity).open != 0 {
                                return crate::expat_h::XML_ERROR_RECURSIVE_ENTITY_REF;
                            }
                            if !(*entity).notation.is_null() {
                                return crate::expat_h::XML_ERROR_BINARY_ENTITY_REF;
                            }
                            if !(*entity).textPtr.is_null() {
                                let mut result: crate::expat_h::XML_Error =
                                    crate::expat_h::XML_ERROR_NONE;
                                if (*parser).m_defaultExpandInternalEntities == 0 {
                                    if (*parser).m_skippedEntityHandler.is_some() {
                                        (*parser)
                                            .m_skippedEntityHandler
                                            .expect("non-null function pointer")(
                                            (*parser).m_handlerArg,
                                            (*entity).name,
                                            0 as ::core::ffi::c_int,
                                        );
                                    } else if (*parser).m_defaultHandler.is_some() {
                                        reportDefault(parser, enc, s, next);
                                    }
                                } else {
                                    result = processEntity(
                                        parser,
                                        entity,
                                        crate::expat_h::XML_FALSE,
                                        ENTITY_INTERNAL,
                                    );
                                    if result as ::core::ffi::c_uint
                                        != crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                    {
                                        return result;
                                    }
                                }
                            } else if (*parser).m_externalEntityRefHandler.is_some() {
                                let mut context: *const crate::expat_external_h::XML_Char =
                                    ::core::ptr::null::<crate::expat_external_h::XML_Char>();
                                (*entity).open = crate::expat_h::XML_TRUE;
                                context = getContext(parser);
                                (*entity).open = crate::expat_h::XML_FALSE;
                                if context.is_null() {
                                    return crate::expat_h::XML_ERROR_NO_MEMORY;
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
                                    return crate::expat_h::XML_ERROR_EXTERNAL_ENTITY_HANDLING;
                                }
                                (*parser).m_tempPool.ptr = (*parser).m_tempPool.start;
                            } else if (*parser).m_defaultHandler.is_some() {
                                reportDefault(parser, enc, s, next);
                            }
                        }
                    }
                }
            }
            crate::src::xmltok::XML_TOK_START_TAG_NO_ATTS
            | crate::src::xmltok::XML_TOK_START_TAG_WITH_ATTS => {
                let mut tag: *mut TAG = ::core::ptr::null_mut::<TAG>();
                let mut result_0: crate::expat_h::XML_Error = crate::expat_h::XML_ERROR_NONE;
                let mut toPtr: *mut crate::expat_external_h::XML_Char =
                    ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
                if !(*parser).m_freeTagList.is_null() {
                    tag = (*parser).m_freeTagList;
                    (*parser).m_freeTagList = (*(*parser).m_freeTagList).parent as *mut TAG;
                } else {
                    tag = expat_malloc(
                        parser,
                        ::core::mem::size_of::<TAG>() as crate::__stddef_size_t_h::size_t,
                        3477 as ::core::ffi::c_int,
                    ) as *mut TAG;
                    if tag.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    (*tag).buf.raw = expat_malloc(
                        parser,
                        32 as crate::__stddef_size_t_h::size_t,
                        3480 as ::core::ffi::c_int,
                    ) as *mut ::core::ffi::c_char;
                    if (*tag).buf.raw.is_null() {
                        expat_free(
                            parser,
                            tag as *mut ::core::ffi::c_void,
                            3482 as ::core::ffi::c_int,
                        );
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    (*tag).bufEnd = (*tag).buf.raw.offset(INIT_TAG_BUF_SIZE as isize);
                }
                (*tag).bindings = ::core::ptr::null_mut::<BINDING>();
                (*tag).parent = (*parser).m_tagStack as *mut tag;
                (*parser).m_tagStack = tag;
                (*tag).name.localPart = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
                (*tag).name.prefix = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
                (*tag).rawName = s.offset((*enc).minBytesPerChar as isize);
                (*tag).rawNameLength =
                    (*enc).nameLength.expect("non-null function pointer")(enc, (*tag).rawName);
                (*parser).m_tagLevel += 1;
                let mut rawNameEnd: *const ::core::ffi::c_char =
                    (*tag).rawName.offset((*tag).rawNameLength as isize);
                let mut fromPtr: *const ::core::ffi::c_char = (*tag).rawName;
                toPtr = (*tag).buf.str;
                loop {
                    let mut convLen: ::core::ffi::c_int = 0;
                    let convert_res: crate::src::xmltok::XML_Convert_Result =
                        (*enc).utf8Convert.expect("non-null function pointer")(
                            enc,
                            &raw mut fromPtr,
                            rawNameEnd,
                            &raw mut toPtr as *mut *mut ::core::ffi::c_char,
                            ((*tag).bufEnd as *mut ICHAR)
                                .offset(-(1 as ::core::ffi::c_int as isize)),
                        ) as crate::src::xmltok::XML_Convert_Result;
                    convLen = toPtr.offset_from((*tag).buf.str) as ::core::ffi::c_long
                        as ::core::ffi::c_int;
                    if fromPtr >= rawNameEnd
                        || convert_res as ::core::ffi::c_uint
                            == crate::src::xmltok::XML_CONVERT_INPUT_INCOMPLETE
                                as ::core::ffi::c_int
                                as ::core::ffi::c_uint
                    {
                        (*tag).name.strLen = convLen;
                        break;
                    } else {
                        if (crate::stdlib::SIZE_MAX as crate::__stddef_size_t_h::size_t)
                            .wrapping_div(2 as crate::__stddef_size_t_h::size_t)
                            < (*tag).bufEnd.offset_from((*tag).buf.raw) as ::core::ffi::c_long
                                as crate::__stddef_size_t_h::size_t
                        {
                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                        }
                        let bufSize: crate::__stddef_size_t_h::size_t =
                            ((*tag).bufEnd.offset_from((*tag).buf.raw) as ::core::ffi::c_long
                                as crate::__stddef_size_t_h::size_t)
                                .wrapping_mul(2 as crate::__stddef_size_t_h::size_t);
                        let mut temp: *mut ::core::ffi::c_char = expat_realloc(
                            parser,
                            (*tag).buf.raw as *mut ::core::ffi::c_void,
                            bufSize,
                            3514 as ::core::ffi::c_int,
                        )
                            as *mut ::core::ffi::c_char;
                        if temp.is_null() {
                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                        }
                        (*tag).buf.raw = temp;
                        (*tag).bufEnd = temp.offset(bufSize as isize);
                        toPtr = (temp as *mut crate::expat_external_h::XML_Char)
                            .offset(convLen as isize);
                    }
                }
                (*tag).name.str = (*tag).buf.str;
                *toPtr = '\0' as i32 as crate::expat_external_h::XML_Char;
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
                        (*tag).name.str,
                        (*parser).m_atts as *mut *const crate::expat_external_h::XML_Char,
                    );
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
                poolClear(&raw mut (*parser).m_tempPool);
            }
            crate::src::xmltok::XML_TOK_EMPTY_ELEMENT_NO_ATTS
            | crate::src::xmltok::XML_TOK_EMPTY_ELEMENT_WITH_ATTS => {
                let mut rawName: *const ::core::ffi::c_char =
                    s.offset((*enc).minBytesPerChar as isize);
                let mut result_1: crate::expat_h::XML_Error = crate::expat_h::XML_ERROR_NONE;
                let mut bindings: *mut BINDING = ::core::ptr::null_mut::<BINDING>();
                let mut noElmHandlers: crate::expat_h::XML_Bool = crate::expat_h::XML_TRUE;
                let mut name_0: TAG_NAME = TAG_NAME {
                    str: ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
                    localPart: ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
                    prefix: ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
                    strLen: 0,
                    uriLen: 0,
                    prefixLen: 0,
                };
                name_0.str = poolStoreString(
                    &raw mut (*parser).m_tempPool,
                    enc,
                    rawName,
                    rawName.offset((*enc).nameLength.expect("non-null function pointer")(
                        enc, rawName,
                    ) as isize),
                );
                if name_0.str.is_null() {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
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
                if result_1 as ::core::ffi::c_uint
                    != crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    freeBindings(parser, bindings);
                    return result_1;
                }
                (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                if (*parser).m_startElementHandler.is_some() {
                    (*parser)
                        .m_startElementHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        name_0.str,
                        (*parser).m_atts as *mut *const crate::expat_external_h::XML_Char,
                    );
                    noElmHandlers = crate::expat_h::XML_FALSE;
                }
                if (*parser).m_endElementHandler.is_some() {
                    if (*parser).m_startElementHandler.is_some() {
                        *eventPP = *eventEndPP;
                    }
                    (*parser)
                        .m_endElementHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg, name_0.str
                    );
                    noElmHandlers = crate::expat_h::XML_FALSE;
                }
                if noElmHandlers as ::core::ffi::c_int != 0 && (*parser).m_defaultHandler.is_some()
                {
                    reportDefault(parser, enc, s, next);
                }
                poolClear(&raw mut (*parser).m_tempPool);
                freeBindings(parser, bindings);
                if (*parser).m_tagLevel == 0 as ::core::ffi::c_int
                    && (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint
                        != crate::expat_h::XML_FINISHED as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    if (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint
                        == crate::expat_h::XML_SUSPENDED as ::core::ffi::c_int
                            as ::core::ffi::c_uint
                        || (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint
                            == crate::expat_h::XML_PARSING as ::core::ffi::c_int
                                as ::core::ffi::c_uint
                            && (*parser).m_reenter as ::core::ffi::c_int != 0
                    {
                        (*parser).m_processor = Some(
                            epilogProcessor
                                as unsafe extern "C" fn(
                                    crate::expat_h::XML_Parser,
                                    *const ::core::ffi::c_char,
                                    *const ::core::ffi::c_char,
                                    *mut *const ::core::ffi::c_char,
                                )
                                    -> crate::expat_h::XML_Error,
                        );
                    } else {
                        return epilogProcessor(parser, next, end, nextPtr);
                    }
                }
            }
            crate::src::xmltok::XML_TOK_END_TAG => {
                if (*parser).m_tagLevel == startTagLevel {
                    return crate::expat_h::XML_ERROR_ASYNC_ENTITY;
                } else {
                    let mut len: ::core::ffi::c_int = 0;
                    let mut rawName_0: *const ::core::ffi::c_char =
                        ::core::ptr::null::<::core::ffi::c_char>();
                    let mut tag_0: *mut TAG = (*parser).m_tagStack;
                    rawName_0 =
                        s.offset(((*enc).minBytesPerChar * 2 as ::core::ffi::c_int) as isize);
                    len = (*enc).nameLength.expect("non-null function pointer")(enc, rawName_0);
                    if len != (*tag_0).rawNameLength
                        || crate::stdlib::memcmp(
                            (*tag_0).rawName as *const ::core::ffi::c_void,
                            rawName_0 as *const ::core::ffi::c_void,
                            len as crate::__stddef_size_t_h::size_t,
                        ) != 0 as ::core::ffi::c_int
                    {
                        *eventPP = rawName_0;
                        return crate::expat_h::XML_ERROR_TAG_MISMATCH;
                    }
                    (*parser).m_tagStack = (*tag_0).parent as *mut TAG;
                    (*tag_0).parent = (*parser).m_freeTagList as *mut tag;
                    (*parser).m_freeTagList = tag_0;
                    (*parser).m_tagLevel -= 1;
                    if (*parser).m_endElementHandler.is_some() {
                        let mut localPart: *const crate::expat_external_h::XML_Char =
                            ::core::ptr::null::<crate::expat_external_h::XML_Char>();
                        let mut prefix: *const crate::expat_external_h::XML_Char =
                            ::core::ptr::null::<crate::expat_external_h::XML_Char>();
                        let mut uri: *mut crate::expat_external_h::XML_Char =
                            ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
                        localPart = (*tag_0).name.localPart;
                        if (*parser).m_ns as ::core::ffi::c_int != 0 && !localPart.is_null() {
                            uri = ((*tag_0).name.str as *mut crate::expat_external_h::XML_Char)
                                .offset((*tag_0).name.uriLen as isize);
                            while *localPart != 0 {
                                let c2rust_fresh22 = localPart;
                                localPart = localPart.offset(1);
                                let c2rust_fresh23 = uri;
                                uri = uri.offset(1);
                                *c2rust_fresh23 = *c2rust_fresh22;
                            }
                            prefix = (*tag_0).name.prefix;
                            if (*parser).m_ns_triplets as ::core::ffi::c_int != 0
                                && !prefix.is_null()
                            {
                                let c2rust_fresh24 = uri;
                                uri = uri.offset(1);
                                *c2rust_fresh24 = (*parser).m_namespaceSeparator;
                                while *prefix != 0 {
                                    let c2rust_fresh25 = prefix;
                                    prefix = prefix.offset(1);
                                    let c2rust_fresh26 = uri;
                                    uri = uri.offset(1);
                                    *c2rust_fresh26 = *c2rust_fresh25;
                                }
                            }
                            *uri = '\0' as i32 as crate::expat_external_h::XML_Char;
                        }
                        (*parser)
                            .m_endElementHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            (*tag_0).name.str,
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
                        (*tag_0).bindings = (*(*tag_0).bindings).nextTagBinding as *mut BINDING;
                        (*b).nextTagBinding = (*parser).m_freeBindingList as *mut binding;
                        (*parser).m_freeBindingList = b;
                        (*(*b).prefix).binding = (*b).prevPrefixBinding as *mut BINDING;
                    }
                    if (*parser).m_tagLevel == 0 as ::core::ffi::c_int
                        && (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint
                            != crate::expat_h::XML_FINISHED as ::core::ffi::c_int
                                as ::core::ffi::c_uint
                    {
                        if (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint
                            == crate::expat_h::XML_SUSPENDED as ::core::ffi::c_int
                                as ::core::ffi::c_uint
                            || (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint
                                == crate::expat_h::XML_PARSING as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                                && (*parser).m_reenter as ::core::ffi::c_int != 0
                        {
                            (*parser).m_processor = Some(
                                epilogProcessor
                                    as unsafe extern "C" fn(
                                        crate::expat_h::XML_Parser,
                                        *const ::core::ffi::c_char,
                                        *const ::core::ffi::c_char,
                                        *mut *const ::core::ffi::c_char,
                                    )
                                        -> crate::expat_h::XML_Error,
                            );
                        } else {
                            return epilogProcessor(parser, next, end, nextPtr);
                        }
                    }
                }
            }
            crate::src::xmltok::XML_TOK_CHAR_REF => {
                let mut n: ::core::ffi::c_int =
                    (*enc).charRefNumber.expect("non-null function pointer")(enc, s);
                if n < 0 as ::core::ffi::c_int {
                    return crate::expat_h::XML_ERROR_BAD_CHAR_REF;
                }
                if (*parser).m_characterDataHandler.is_some() {
                    let mut buf: [crate::expat_external_h::XML_Char; 4] = [0; 4];
                    (*parser)
                        .m_characterDataHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        &raw mut buf as *mut crate::expat_external_h::XML_Char,
                        crate::src::xmltok::XmlUtf8Encode(n, &mut buf),
                    );
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
            }
            crate::src::xmltok::XML_TOK_XML_DECL => {
                return crate::expat_h::XML_ERROR_MISPLACED_XML_PI
            }
            crate::src::xmltok::XML_TOK_DATA_NEWLINE => {
                if (*parser).m_characterDataHandler.is_some() {
                    let mut c_0: crate::expat_external_h::XML_Char =
                        0xa as crate::expat_external_h::XML_Char;
                    (*parser)
                        .m_characterDataHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        &raw mut c_0,
                        1 as ::core::ffi::c_int,
                    );
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
            }
            crate::src::xmltok::XML_TOK_CDATA_SECT_OPEN => {
                let mut result_2: crate::expat_h::XML_Error = crate::expat_h::XML_ERROR_NONE;
                if (*parser).m_startCdataSectionHandler.is_some() {
                    (*parser)
                        .m_startCdataSectionHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg
                    );
                } else if false && (*parser).m_characterDataHandler.is_some() {
                    (*parser)
                        .m_characterDataHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        (*parser).m_dataBuf,
                        0 as ::core::ffi::c_int,
                    );
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
                result_2 =
                    doCdataSection(parser, enc, &raw mut next, end, nextPtr, haveMore, account);
                if result_2 as ::core::ffi::c_uint
                    != crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    return result_2;
                } else if next.is_null() {
                    (*parser).m_processor = Some(
                        cdataSectionProcessor
                            as unsafe extern "C" fn(
                                crate::expat_h::XML_Parser,
                                *const ::core::ffi::c_char,
                                *const ::core::ffi::c_char,
                                *mut *const ::core::ffi::c_char,
                            )
                                -> crate::expat_h::XML_Error,
                    );
                    return result_2;
                }
            }
            crate::src::xmltok::XML_TOK_TRAILING_RSQB => {
                if haveMore != 0 {
                    *nextPtr = s;
                    return crate::expat_h::XML_ERROR_NONE;
                }
                if (*parser).m_characterDataHandler.is_some() {
                    if (*enc).isUtf8 == 0 {
                        let mut dataPtr: *mut ICHAR = (*parser).m_dataBuf as *mut ICHAR;
                        (*enc).utf8Convert.expect("non-null function pointer")(
                            enc,
                            &raw mut s,
                            end,
                            &raw mut dataPtr,
                            (*parser).m_dataBufEnd as *mut ICHAR,
                        );
                        (*parser)
                            .m_characterDataHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            (*parser).m_dataBuf,
                            dataPtr.offset_from((*parser).m_dataBuf as *mut ICHAR)
                                as ::core::ffi::c_long
                                as ::core::ffi::c_int,
                        );
                    } else {
                        (*parser)
                            .m_characterDataHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            s as *const crate::expat_external_h::XML_Char,
                            (end as *const crate::expat_external_h::XML_Char)
                                .offset_from(s as *const crate::expat_external_h::XML_Char)
                                as ::core::ffi::c_long
                                as ::core::ffi::c_int,
                        );
                    }
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, end);
                }
                if startTagLevel == 0 as ::core::ffi::c_int {
                    *eventPP = end;
                    return crate::expat_h::XML_ERROR_NO_ELEMENTS;
                }
                if (*parser).m_tagLevel != startTagLevel {
                    *eventPP = end;
                    return crate::expat_h::XML_ERROR_ASYNC_ENTITY;
                }
                *nextPtr = end;
                return crate::expat_h::XML_ERROR_NONE;
            }
            crate::src::xmltok::XML_TOK_DATA_CHARS => {
                let mut charDataHandler: crate::expat_h::XML_CharacterDataHandler =
                    (*parser).m_characterDataHandler;
                if charDataHandler.is_some() {
                    if (*enc).isUtf8 == 0 {
                        loop {
                            let mut dataPtr_0: *mut ICHAR = (*parser).m_dataBuf as *mut ICHAR;
                            let convert_res_0: crate::src::xmltok::XML_Convert_Result =
                                (*enc).utf8Convert.expect("non-null function pointer")(
                                    enc,
                                    &raw mut s,
                                    next,
                                    &raw mut dataPtr_0,
                                    (*parser).m_dataBufEnd as *mut ICHAR,
                                )
                                    as crate::src::xmltok::XML_Convert_Result;
                            *eventEndPP = s;
                            charDataHandler.expect("non-null function pointer")(
                                (*parser).m_handlerArg,
                                (*parser).m_dataBuf,
                                dataPtr_0.offset_from((*parser).m_dataBuf as *mut ICHAR)
                                    as ::core::ffi::c_long
                                    as ::core::ffi::c_int,
                            );
                            if convert_res_0 as ::core::ffi::c_uint
                                == crate::src::xmltok::XML_CONVERT_COMPLETED as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                                || convert_res_0 as ::core::ffi::c_uint
                                    == crate::src::xmltok::XML_CONVERT_INPUT_INCOMPLETE
                                        as ::core::ffi::c_int
                                        as ::core::ffi::c_uint
                            {
                                break;
                            }
                            *eventPP = s;
                        }
                    } else {
                        charDataHandler.expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            s as *const crate::expat_external_h::XML_Char,
                            (next as *const crate::expat_external_h::XML_Char)
                                .offset_from(s as *const crate::expat_external_h::XML_Char)
                                as ::core::ffi::c_long
                                as ::core::ffi::c_int,
                        );
                    }
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
            }
            crate::src::xmltok::XML_TOK_PI => {
                if reportProcessingInstruction(parser, enc, s, next) == 0 {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
            }
            crate::src::xmltok::XML_TOK_COMMENT => {
                if reportComment(parser, enc, s, next) == 0 {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
            }
            _ => {
                if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
            }
        }
        match (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint {
            3 => {
                *eventPP = next;
                *nextPtr = next;
                return crate::expat_h::XML_ERROR_NONE;
            }
            2 => {
                *eventPP = next;
                return crate::expat_h::XML_ERROR_ABORTED;
            }
            1 => {
                if (*parser).m_reenter != 0 {
                    *nextPtr = next;
                    return crate::expat_h::XML_ERROR_NONE;
                }
            }
            _ => {}
        }
        s = next;
        *eventPP = s;
    }
}

unsafe extern "C" fn freeBindings(
    mut parser: crate::expat_h::XML_Parser,
    mut bindings: *mut BINDING,
) {
    while !bindings.is_null() {
        let mut b: *mut BINDING = bindings;
        if (*parser).m_endNamespaceDeclHandler.is_some() {
            (*parser)
                .m_endNamespaceDeclHandler
                .expect("non-null function pointer")(
                (*parser).m_handlerArg, (*(*b).prefix).name
            );
        }
        bindings = (*bindings).nextTagBinding as *mut BINDING;
        (*b).nextTagBinding = (*parser).m_freeBindingList as *mut binding;
        (*parser).m_freeBindingList = b;
        (*(*b).prefix).binding = (*b).prevPrefixBinding as *mut BINDING;
    }
}

unsafe extern "C" fn storeAtts(
    mut parser: crate::expat_h::XML_Parser,
    mut enc: *const crate::src::xmltok::ENCODING,
    mut attStr: *const ::core::ffi::c_char,
    mut tagNamePtr: *mut TAG_NAME,
    mut bindingsPtr: *mut *mut BINDING,
    mut account: XML_Account,
) -> crate::expat_h::XML_Error {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut elementType: *mut ELEMENT_TYPE = ::core::ptr::null_mut::<ELEMENT_TYPE>();
    let mut nDefaultAtts: ::core::ffi::c_int = 0;
    let mut appAtts: *mut *const crate::expat_external_h::XML_Char =
        ::core::ptr::null_mut::<*const crate::expat_external_h::XML_Char>();
    let mut attIndex: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut prefixLen: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut uri: *mut crate::expat_external_h::XML_Char =
        ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
    let mut nPrefixes: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut binding: *mut BINDING = ::core::ptr::null_mut::<BINDING>();
    let mut localPart: *const crate::expat_external_h::XML_Char =
        ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    elementType = lookup(
        parser,
        &raw mut (*dtd).elementTypes,
        (*tagNamePtr).str as KEY,
        0 as crate::__stddef_size_t_h::size_t,
    ) as *mut ELEMENT_TYPE;
    if elementType.is_null() {
        let mut name: *const crate::expat_external_h::XML_Char =
            poolCopyString(&raw mut (*dtd).pool, (*tagNamePtr).str);
        if name.is_null() {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
        elementType = lookup(
            parser,
            &raw mut (*dtd).elementTypes,
            name as KEY,
            ::core::mem::size_of::<ELEMENT_TYPE>() as crate::__stddef_size_t_h::size_t,
        ) as *mut ELEMENT_TYPE;
        if elementType.is_null() {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
        if (*parser).m_ns as ::core::ffi::c_int != 0
            && setElementTypePrefix(parser, elementType) == 0
        {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
    }
    nDefaultAtts = (*elementType).nDefaultAtts;
    n = (*enc).getAtts.expect("non-null function pointer")(
        enc,
        attStr,
        (*parser).m_attsSize,
        (*parser).m_atts,
    );
    if n > crate::limits_h::INT_MAX - nDefaultAtts {
        return crate::expat_h::XML_ERROR_NO_MEMORY;
    }
    if n + nDefaultAtts > (*parser).m_attsSize {
        let mut oldAttsSize: ::core::ffi::c_int = (*parser).m_attsSize;
        let mut temp: *mut crate::src::xmltok::ATTRIBUTE =
            ::core::ptr::null_mut::<crate::src::xmltok::ATTRIBUTE>();
        if nDefaultAtts > crate::limits_h::INT_MAX - INIT_ATTS_SIZE
            || n > crate::limits_h::INT_MAX - (nDefaultAtts + INIT_ATTS_SIZE)
        {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
        (*parser).m_attsSize = n + nDefaultAtts + INIT_ATTS_SIZE;
        temp = expat_realloc(
            parser,
            (*parser).m_atts as *mut ::core::ffi::c_void,
            ((*parser).m_attsSize as crate::__stddef_size_t_h::size_t)
                .wrapping_mul(::core::mem::size_of::<crate::src::xmltok::ATTRIBUTE>()
                    as crate::__stddef_size_t_h::size_t),
            3894 as ::core::ffi::c_int,
        ) as *mut crate::src::xmltok::ATTRIBUTE;
        if temp.is_null() {
            (*parser).m_attsSize = oldAttsSize;
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
        (*parser).m_atts = temp;
        if n > oldAttsSize {
            (*enc).getAtts.expect("non-null function pointer")(enc, attStr, n, (*parser).m_atts);
        }
    }
    appAtts = (*parser).m_atts as *mut *const crate::expat_external_h::XML_Char;
    i = 0 as ::core::ffi::c_int;
    while i < n {
        let mut currAtt: *mut crate::src::xmltok::ATTRIBUTE =
            (*parser).m_atts.offset(i as isize) as *mut crate::src::xmltok::ATTRIBUTE;
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
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
        if *(*attId).name.offset(-1 as ::core::ffi::c_int as isize) != 0 {
            if enc == (*parser).m_encoding {
                (*parser).m_eventPtr = (*(*parser).m_atts.offset(i as isize)).name;
            }
            return crate::expat_h::XML_ERROR_DUPLICATE_ATTRIBUTE;
        }
        *(*attId).name.offset(-1 as ::core::ffi::c_int as isize) =
            1 as crate::expat_external_h::XML_Char;
        let c2rust_fresh27 = attIndex;
        attIndex = attIndex + 1;
        let ref mut c2rust_fresh28 = *appAtts.offset(c2rust_fresh27 as isize);
        *c2rust_fresh28 = (*attId).name;
        if (*(*parser).m_atts.offset(i as isize)).normalized == 0 {
            let mut result: crate::expat_h::XML_Error = crate::expat_h::XML_ERROR_NONE;
            let mut isCdata: crate::expat_h::XML_Bool = crate::expat_h::XML_TRUE;
            if (*attId).maybeTokenized != 0 {
                let mut j: ::core::ffi::c_int = 0;
                j = 0 as ::core::ffi::c_int;
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
            let ref mut c2rust_fresh29 = *appAtts.offset(attIndex as isize);
            *c2rust_fresh29 = (*parser).m_tempPool.start;
            (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
        } else {
            let ref mut c2rust_fresh30 = *appAtts.offset(attIndex as isize);
            *c2rust_fresh30 = poolStoreString(
                &raw mut (*parser).m_tempPool,
                enc,
                (*(*parser).m_atts.offset(i as isize)).valuePtr,
                (*(*parser).m_atts.offset(i as isize)).valueEnd,
            );
            if (*appAtts.offset(attIndex as isize)).is_null() {
                return crate::expat_h::XML_ERROR_NO_MEMORY;
            }
            (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
        }
        if !(*attId).prefix.is_null() {
            if (*attId).xmlns != 0 {
                let mut result_0: crate::expat_h::XML_Error = addBinding(
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
                *(*attId).name.offset(-1 as ::core::ffi::c_int as isize) =
                    2 as crate::expat_external_h::XML_Char;
            }
        } else {
            attIndex += 1;
        }
        i += 1;
    }
    (*parser).m_nSpecifiedAtts = attIndex;
    if !(*elementType).idAtt.is_null()
        && *(*(*elementType).idAtt)
            .name
            .offset(-1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0
    {
        i = 0 as ::core::ffi::c_int;
        while i < attIndex {
            if *appAtts.offset(i as isize)
                == (*(*elementType).idAtt).name as *const crate::expat_external_h::XML_Char
            {
                (*parser).m_idAttIndex = i;
                break;
            } else {
                i += 2 as ::core::ffi::c_int;
            }
        }
    } else {
        (*parser).m_idAttIndex = -1 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < nDefaultAtts {
        let mut da: *const DEFAULT_ATTRIBUTE = (*elementType).defaultAtts.offset(i as isize);
        if *(*(*da).id).name.offset(-1 as ::core::ffi::c_int as isize) == 0
            && !(*da).value.is_null()
        {
            if !(*(*da).id).prefix.is_null() {
                if (*(*da).id).xmlns != 0 {
                    let mut result_1: crate::expat_h::XML_Error = addBinding(
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
                    *(*(*da).id).name.offset(-1 as ::core::ffi::c_int as isize) =
                        2 as crate::expat_external_h::XML_Char;
                    nPrefixes += 1;
                    let c2rust_fresh31 = attIndex;
                    attIndex = attIndex + 1;
                    let ref mut c2rust_fresh32 = *appAtts.offset(c2rust_fresh31 as isize);
                    *c2rust_fresh32 = (*(*da).id).name;
                    let c2rust_fresh33 = attIndex;
                    attIndex = attIndex + 1;
                    let ref mut c2rust_fresh34 = *appAtts.offset(c2rust_fresh33 as isize);
                    *c2rust_fresh34 = (*da).value;
                }
            } else {
                *(*(*da).id).name.offset(-1 as ::core::ffi::c_int as isize) =
                    1 as crate::expat_external_h::XML_Char;
                let c2rust_fresh35 = attIndex;
                attIndex = attIndex + 1;
                let ref mut c2rust_fresh36 = *appAtts.offset(c2rust_fresh35 as isize);
                *c2rust_fresh36 = (*(*da).id).name;
                let c2rust_fresh37 = attIndex;
                attIndex = attIndex + 1;
                let ref mut c2rust_fresh38 = *appAtts.offset(c2rust_fresh37 as isize);
                *c2rust_fresh38 = (*da).value;
            }
        }
        i += 1;
    }
    let ref mut c2rust_fresh39 = *appAtts.offset(attIndex as isize);
    *c2rust_fresh39 = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    i = 0 as ::core::ffi::c_int;
    if nPrefixes != 0 {
        let mut j_0: ::core::ffi::c_uint = 0;
        let mut version: ::core::ffi::c_ulong = (*parser).m_nsAttsVersion;
        if (*parser).m_nsAttsPower as usize
            >= (::core::mem::size_of::<::core::ffi::c_uint>() as usize).wrapping_mul(8 as usize)
        {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
        let mut nsAttsSize: ::core::ffi::c_uint =
            (1 as ::core::ffi::c_uint) << (*parser).m_nsAttsPower as ::core::ffi::c_int;
        let mut oldNsAttsPower: ::core::ffi::c_uchar = (*parser).m_nsAttsPower;
        if nPrefixes << 1 as ::core::ffi::c_int >> (*parser).m_nsAttsPower as ::core::ffi::c_int
            != 0
        {
            let mut temp_0: *mut NS_ATT = ::core::ptr::null_mut::<NS_ATT>();
            loop {
                let c2rust_fresh40 = (*parser).m_nsAttsPower;
                (*parser).m_nsAttsPower = (*parser).m_nsAttsPower.wrapping_add(1);
                if !(nPrefixes >> c2rust_fresh40 as ::core::ffi::c_int != 0) {
                    break;
                }
            }
            if ((*parser).m_nsAttsPower as ::core::ffi::c_int) < 3 as ::core::ffi::c_int {
                (*parser).m_nsAttsPower = 3 as ::core::ffi::c_uchar;
            }
            if (*parser).m_nsAttsPower as usize
                >= (::core::mem::size_of::<::core::ffi::c_uint>() as usize).wrapping_mul(8 as usize)
            {
                (*parser).m_nsAttsPower = oldNsAttsPower;
                return crate::expat_h::XML_ERROR_NO_MEMORY;
            }
            nsAttsSize =
                (1 as ::core::ffi::c_uint) << (*parser).m_nsAttsPower as ::core::ffi::c_int;
            temp_0 = expat_realloc(
                parser,
                (*parser).m_nsAtts as *mut ::core::ffi::c_void,
                (nsAttsSize as crate::__stddef_size_t_h::size_t).wrapping_mul(
                    ::core::mem::size_of::<NS_ATT>() as crate::__stddef_size_t_h::size_t,
                ),
                4089 as ::core::ffi::c_int,
            ) as *mut NS_ATT;
            if temp_0.is_null() {
                (*parser).m_nsAttsPower = oldNsAttsPower;
                return crate::expat_h::XML_ERROR_NO_MEMORY;
            }
            (*parser).m_nsAtts = temp_0;
            version = 0 as ::core::ffi::c_ulong;
        }
        if version == 0 {
            version = INIT_ATTS_VERSION as ::core::ffi::c_ulong;
            j_0 = nsAttsSize;
            while j_0 != 0 as ::core::ffi::c_uint {
                j_0 = j_0.wrapping_sub(1);
                (*(*parser).m_nsAtts.offset(j_0 as isize)).version = version;
            }
        }
        version = version.wrapping_sub(1);
        (*parser).m_nsAttsVersion = version;
        while i < attIndex {
            let mut s: *const crate::expat_external_h::XML_Char = *appAtts.offset(i as isize);
            if *s.offset(-1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 2 as ::core::ffi::c_int
            {
                let mut id: *mut ATTRIBUTE_ID = ::core::ptr::null_mut::<ATTRIBUTE_ID>();
                let mut b: *const BINDING = ::core::ptr::null::<BINDING>();
                let mut uriHash: ::core::ffi::c_ulong = 0;
                let mut sip_state: crate::siphash_h::siphash = crate::siphash_h::siphash {
                    v0: 0,
                    v1: 0,
                    v2: 0,
                    v3: 0,
                    buf: [0; 8],
                    p: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                    c: 0,
                };
                let mut sip_key: crate::siphash_h::sipkey = crate::siphash_h::sipkey { k: [0; 2] };
                let root_parser: crate::expat_h::XML_Parser =
                    getRootParserOf(parser, ::core::ptr::null_mut::<::core::ffi::c_uint>())
                        as crate::expat_h::XML_Parser;
                '_c2rust_label: {
                    if (*root_parser).m_parentParser.is_null() {
                    } else {
                        crate::stdlib::__assert_fail(
                            b"! rootParser->m_parentParser\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"../../expat/lib/xmlparse.c\0".as_ptr() as *const ::core::ffi::c_char,
                            1251 as ::core::ffi::c_uint,
                            b"unsigned long get_hash_secret_salt(XML_Parser)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                };
                sip_key.k[0 as ::core::ffi::c_int as usize] = 0 as crate::stdlib::uint64_t;
                sip_key.k[1 as ::core::ffi::c_int as usize] =
                    (*root_parser).m_hash_secret_salt as crate::stdlib::uint64_t;
                sip24_init(&mut sip_state, &sip_key);
                *(s as *mut crate::expat_external_h::XML_Char)
                    .offset(-1 as ::core::ffi::c_int as isize) =
                    0 as crate::expat_external_h::XML_Char;
                id = lookup(
                    parser,
                    &raw mut (*dtd).attributeIds,
                    s as KEY,
                    0 as crate::__stddef_size_t_h::size_t,
                ) as *mut ATTRIBUTE_ID;
                if id.is_null() || (*id).prefix.is_null() {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
                b = (*(*id).prefix).binding;
                if b.is_null() {
                    return crate::expat_h::XML_ERROR_UNBOUND_PREFIX;
                }
                j_0 = 0 as ::core::ffi::c_uint;
                while j_0 < (*b).uriLen as ::core::ffi::c_uint {
                    let c: crate::expat_external_h::XML_Char = *(*b).uri.offset(j_0 as isize);
                    if if (*parser).m_tempPool.ptr
                        == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
                        && poolGrow(&raw mut (*parser).m_tempPool) == 0
                    {
                        0 as ::core::ffi::c_int
                    } else {
                        let c2rust_fresh41 = (*parser).m_tempPool.ptr;
                        (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                        *c2rust_fresh41 = c;
                        1 as ::core::ffi::c_int
                    } == 0
                    {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    j_0 = j_0.wrapping_add(1);
                }
                let uri_bytes_len = ((*b).uriLen as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(::core::mem::size_of::<crate::expat_external_h::XML_Char>()
                        as crate::__stddef_size_t_h::size_t);
                sip24_update(
                    &mut sip_state,
                    ::core::slice::from_raw_parts(
                        (*b).uri as *const ::core::ffi::c_uchar,
                        uri_bytes_len,
                    ),
                );
                loop {
                    let c2rust_fresh42 = s;
                    s = s.offset(1);
                    if !(*c2rust_fresh42 as ::core::ffi::c_int != 0x3a as ::core::ffi::c_int) {
                        break;
                    }
                }
                let local_name = std::ffi::CStr::from_ptr(s);
                sip24_update(
                    &mut sip_state,
                    ::core::slice::from_raw_parts(
                        local_name.as_ptr() as *const ::core::ffi::c_uchar,
                        local_name.to_bytes().len(),
                    ),
                );
                loop {
                    if if (*parser).m_tempPool.ptr
                        == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
                        && poolGrow(&raw mut (*parser).m_tempPool) == 0
                    {
                        0 as ::core::ffi::c_int
                    } else {
                        let c2rust_fresh43 = (*parser).m_tempPool.ptr;
                        (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                        *c2rust_fresh43 = *s;
                        1 as ::core::ffi::c_int
                    } == 0
                    {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    let c2rust_fresh44 = s;
                    s = s.offset(1);
                    if !(*c2rust_fresh44 != 0) {
                        break;
                    }
                }
                uriHash = sip24_final(&mut sip_state) as ::core::ffi::c_ulong;
                let mut step: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                let mut mask: ::core::ffi::c_ulong =
                    nsAttsSize.wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_ulong;
                j_0 = (uriHash & mask) as ::core::ffi::c_uint;
                while (*(*parser).m_nsAtts.offset(j_0 as isize)).version == version {
                    if uriHash == (*(*parser).m_nsAtts.offset(j_0 as isize)).hash {
                        let mut s1: *const crate::expat_external_h::XML_Char =
                            (*parser).m_tempPool.start;
                        let mut s2: *const crate::expat_external_h::XML_Char =
                            (*(*parser).m_nsAtts.offset(j_0 as isize)).uriName;
                        while *s1 as ::core::ffi::c_int == *s2 as ::core::ffi::c_int
                            && *s1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                        {
                            s1 = s1.offset(1);
                            s2 = s2.offset(1);
                        }
                        if *s1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            return crate::expat_h::XML_ERROR_DUPLICATE_ATTRIBUTE;
                        }
                    }
                    if step == 0 {
                        step = ((uriHash & !mask)
                            >> (*parser).m_nsAttsPower as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                            & mask >> 2 as ::core::ffi::c_int
                            | 1 as ::core::ffi::c_ulong)
                            as ::core::ffi::c_uchar;
                    }
                    if j_0 < step as ::core::ffi::c_uint {
                        j_0 =
                            j_0.wrapping_add(nsAttsSize.wrapping_sub(step as ::core::ffi::c_uint));
                    } else {
                        j_0 = j_0.wrapping_sub(step as ::core::ffi::c_uint);
                    };
                }
                if (*parser).m_ns_triplets != 0 {
                    *(*parser)
                        .m_tempPool
                        .ptr
                        .offset(-1 as ::core::ffi::c_int as isize) = (*parser).m_namespaceSeparator;
                    s = (*(*b).prefix).name;
                    loop {
                        if if (*parser).m_tempPool.ptr
                            == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
                            && poolGrow(&raw mut (*parser).m_tempPool) == 0
                        {
                            0 as ::core::ffi::c_int
                        } else {
                            let c2rust_fresh45 = (*parser).m_tempPool.ptr;
                            (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                            *c2rust_fresh45 = *s;
                            1 as ::core::ffi::c_int
                        } == 0
                        {
                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                        }
                        let c2rust_fresh46 = s;
                        s = s.offset(1);
                        if !(*c2rust_fresh46 != 0) {
                            break;
                        }
                    }
                }
                s = (*parser).m_tempPool.start;
                (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                let ref mut c2rust_fresh47 = *appAtts.offset(i as isize);
                *c2rust_fresh47 = s;
                (*(*parser).m_nsAtts.offset(j_0 as isize)).version = version;
                (*(*parser).m_nsAtts.offset(j_0 as isize)).hash = uriHash;
                let ref mut c2rust_fresh48 = (*(*parser).m_nsAtts.offset(j_0 as isize)).uriName;
                *c2rust_fresh48 = s;
                nPrefixes -= 1;
                if nPrefixes == 0 {
                    i += 2 as ::core::ffi::c_int;
                    break;
                }
            } else {
                *(s as *mut crate::expat_external_h::XML_Char)
                    .offset(-1 as ::core::ffi::c_int as isize) =
                    0 as crate::expat_external_h::XML_Char;
            }
            i += 2 as ::core::ffi::c_int;
        }
    }
    while i < attIndex {
        *(*appAtts.offset(i as isize) as *mut crate::expat_external_h::XML_Char)
            .offset(-1 as ::core::ffi::c_int as isize) = 0 as crate::expat_external_h::XML_Char;
        i += 2 as ::core::ffi::c_int;
    }
    binding = *bindingsPtr;
    while !binding.is_null() {
        *(*(*binding).attId)
            .name
            .offset(-1 as ::core::ffi::c_int as isize) = 0 as crate::expat_external_h::XML_Char;
        binding = (*binding).nextTagBinding as *mut BINDING;
    }
    if (*parser).m_ns == 0 {
        return crate::expat_h::XML_ERROR_NONE;
    }
    if !(*elementType).prefix.is_null() {
        binding = (*(*elementType).prefix).binding;
        if binding.is_null() {
            return crate::expat_h::XML_ERROR_UNBOUND_PREFIX;
        }
        localPart = (*tagNamePtr).str;
        loop {
            let c2rust_fresh49 = localPart;
            localPart = localPart.offset(1);
            if !(*c2rust_fresh49 as ::core::ffi::c_int != 0x3a as ::core::ffi::c_int) {
                break;
            }
        }
    } else if !(*dtd).defaultPrefix.binding.is_null() {
        binding = (*dtd).defaultPrefix.binding;
        localPart = (*tagNamePtr).str;
    } else {
        return crate::expat_h::XML_ERROR_NONE;
    }
    prefixLen = 0 as ::core::ffi::c_int;
    if (*parser).m_ns_triplets as ::core::ffi::c_int != 0 && !(*(*binding).prefix).name.is_null() {
        loop {
            let c2rust_fresh50 = prefixLen;
            prefixLen = prefixLen + 1;
            if !(*(*(*binding).prefix).name.offset(c2rust_fresh50 as isize) != 0) {
                break;
            }
        }
    }
    (*tagNamePtr).localPart = localPart;
    (*tagNamePtr).uriLen = (*binding).uriLen;
    (*tagNamePtr).prefix = (*(*binding).prefix).name;
    (*tagNamePtr).prefixLen = prefixLen;
    i = 0 as ::core::ffi::c_int;
    loop {
        let c2rust_fresh51 = i;
        i = i + 1;
        if !(*localPart.offset(c2rust_fresh51 as isize) != 0) {
            break;
        }
    }
    if (*binding).uriLen > crate::limits_h::INT_MAX - prefixLen
        || i > crate::limits_h::INT_MAX - ((*binding).uriLen + prefixLen)
    {
        return crate::expat_h::XML_ERROR_NO_MEMORY;
    }
    n = i + (*binding).uriLen + prefixLen;
    if n > (*binding).uriAlloc {
        let mut p: *mut TAG = ::core::ptr::null_mut::<TAG>();
        if n > crate::limits_h::INT_MAX - EXPAND_SPARE {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
        uri = expat_malloc(
            parser,
            ((n + 24 as ::core::ffi::c_int) as crate::__stddef_size_t_h::size_t)
                .wrapping_mul(::core::mem::size_of::<crate::expat_external_h::XML_Char>()
                    as crate::__stddef_size_t_h::size_t),
            4270 as ::core::ffi::c_int,
        ) as *mut crate::expat_external_h::XML_Char;
        if uri.is_null() {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
        (*binding).uriAlloc = n + EXPAND_SPARE;
        crate::stdlib::memcpy(
            uri as *mut ::core::ffi::c_void,
            (*binding).uri as *const ::core::ffi::c_void,
            ((*binding).uriLen as crate::__stddef_size_t_h::size_t)
                .wrapping_mul(::core::mem::size_of::<crate::expat_external_h::XML_Char>()
                    as crate::__stddef_size_t_h::size_t),
        );
        p = (*parser).m_tagStack;
        while !p.is_null() {
            if (*p).name.str == (*binding).uri as *const crate::expat_external_h::XML_Char {
                (*p).name.str = uri;
            }
            p = (*p).parent as *mut TAG;
        }
        expat_free(
            parser,
            (*binding).uri as *mut ::core::ffi::c_void,
            4278 as ::core::ffi::c_int,
        );
        (*binding).uri = uri;
    }
    uri = (*binding).uri.offset((*binding).uriLen as isize);
    crate::stdlib::memcpy(
        uri as *mut ::core::ffi::c_void,
        localPart as *const ::core::ffi::c_void,
        (i as crate::__stddef_size_t_h::size_t)
            .wrapping_mul(::core::mem::size_of::<crate::expat_external_h::XML_Char>()
                as crate::__stddef_size_t_h::size_t),
    );
    if prefixLen != 0 {
        uri = uri.offset((i - 1 as ::core::ffi::c_int) as isize);
        *uri = (*parser).m_namespaceSeparator;
        crate::stdlib::memcpy(
            uri.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            (*(*binding).prefix).name as *const ::core::ffi::c_void,
            (prefixLen as crate::__stddef_size_t_h::size_t)
                .wrapping_mul(::core::mem::size_of::<crate::expat_external_h::XML_Char>()
                    as crate::__stddef_size_t_h::size_t),
        );
    }
    (*tagNamePtr).str = (*binding).uri;
    return crate::expat_h::XML_ERROR_NONE;
}

extern "C" fn is_rfc3986_uri_char(
    mut candidate: crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Bool {
    match candidate as ::core::ffi::c_int {
        65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82
        | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 | 97 | 98 | 99 | 100 | 101 | 102 | 103 | 104
        | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118
        | 119 | 120 | 121 | 122 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 37 | 45
        | 46 | 95 | 126 | 58 | 47 | 63 | 35 | 91 | 93 | 64 | 33 | 36 | 38 | 39 | 40 | 41 | 42
        | 43 | 44 | 59 | 61 => return crate::expat_h::XML_TRUE,
        _ => return crate::expat_h::XML_FALSE,
    };
}

unsafe extern "C" fn addBinding(
    mut parser: crate::expat_h::XML_Parser,
    mut prefix: *mut PREFIX,
    mut attId: *const ATTRIBUTE_ID,
    mut uri: *const crate::expat_external_h::XML_Char,
    mut bindingsPtr: *mut *mut BINDING,
) -> crate::expat_h::XML_Error {
    static xmlNamespace: [crate::expat_external_h::XML_Char; 37] = [
        crate::ascii_h::ASCII_h as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_t as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_t as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_p as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_COLON as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_SLASH as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_SLASH as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_w as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_w as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_w as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_PERIOD as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_w as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_3 as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_PERIOD as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_o as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_r as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_g as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_SLASH as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_X as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_M as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_L as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_SLASH as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_1 as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_9 as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_9 as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_8 as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_SLASH as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_n as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_a as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_m as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_e as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_s as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_p as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_a as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_c as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_e as crate::expat_external_h::XML_Char,
        '\0' as i32 as crate::expat_external_h::XML_Char,
    ];
    static xmlnsNamespace: [crate::expat_external_h::XML_Char; 30] = [
        crate::ascii_h::ASCII_h as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_t as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_t as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_p as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_COLON as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_SLASH as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_SLASH as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_w as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_w as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_w as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_PERIOD as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_w as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_3 as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_PERIOD as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_o as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_r as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_g as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_SLASH as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_2 as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_0 as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_0 as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_0 as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_SLASH as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_x as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_m as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_l as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_n as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_s as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_SLASH as crate::expat_external_h::XML_Char,
        '\0' as i32 as crate::expat_external_h::XML_Char,
    ];
    let mut mustBeXML: crate::expat_h::XML_Bool = crate::expat_h::XML_FALSE;
    let mut isXML: crate::expat_h::XML_Bool = crate::expat_h::XML_TRUE;
    let mut isXMLNS: crate::expat_h::XML_Bool = crate::expat_h::XML_TRUE;
    let mut b: *mut BINDING = ::core::ptr::null_mut::<BINDING>();
    let mut len: ::core::ffi::c_int = 0;
    let xml_len = (xmlNamespace.len() - 1) as ::core::ffi::c_int;
    let xmlns_len = (xmlnsNamespace.len() - 1) as ::core::ffi::c_int;
    if *uri as ::core::ffi::c_int == '\0' as i32 && !(*prefix).name.is_null() {
        return crate::expat_h::XML_ERROR_UNDECLARING_PREFIX;
    }
    if !(*prefix).name.is_null()
        && *(*prefix).name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x78 as ::core::ffi::c_int
        && *(*prefix).name.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x6d as ::core::ffi::c_int
        && *(*prefix).name.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x6c as ::core::ffi::c_int
    {
        if *(*prefix).name.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x6e as ::core::ffi::c_int
            && *(*prefix).name.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0x73 as ::core::ffi::c_int
            && *(*prefix).name.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\0' as i32
        {
            return crate::expat_h::XML_ERROR_RESERVED_PREFIX_XMLNS;
        }
        if *(*prefix).name.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '\0' as i32
        {
            mustBeXML = crate::expat_h::XML_TRUE;
        }
    }
    len = 0 as ::core::ffi::c_int;
    while *uri.offset(len as isize) != 0 {
        if isXML as ::core::ffi::c_int != 0
            && (len > xml_len
                || *uri.offset(len as isize) as ::core::ffi::c_int
                    != xmlNamespace[len as usize] as ::core::ffi::c_int)
        {
            isXML = crate::expat_h::XML_FALSE;
        }
        if mustBeXML == 0
            && isXMLNS as ::core::ffi::c_int != 0
            && (len > xmlns_len
                || *uri.offset(len as isize) as ::core::ffi::c_int
                    != xmlnsNamespace[len as usize] as ::core::ffi::c_int)
        {
            isXMLNS = crate::expat_h::XML_FALSE;
        }
        if (*parser).m_ns as ::core::ffi::c_int != 0
            && *uri.offset(len as isize) as ::core::ffi::c_int
                == (*parser).m_namespaceSeparator as ::core::ffi::c_int
            && is_rfc3986_uri_char(*uri.offset(len as isize)) == 0
        {
            return crate::expat_h::XML_ERROR_SYNTAX;
        }
        len += 1;
    }
    isXML = (isXML as ::core::ffi::c_int != 0 && len == xml_len) as ::core::ffi::c_int
        as crate::expat_h::XML_Bool;
    isXMLNS = (isXMLNS as ::core::ffi::c_int != 0 && len == xmlns_len) as ::core::ffi::c_int
        as crate::expat_h::XML_Bool;
    if mustBeXML as ::core::ffi::c_int != isXML as ::core::ffi::c_int {
        return (if mustBeXML as ::core::ffi::c_int != 0 {
            crate::expat_h::XML_ERROR_RESERVED_PREFIX_XML as ::core::ffi::c_int
        } else {
            crate::expat_h::XML_ERROR_RESERVED_NAMESPACE_URI as ::core::ffi::c_int
        }) as crate::expat_h::XML_Error;
    }
    if isXMLNS != 0 {
        return crate::expat_h::XML_ERROR_RESERVED_NAMESPACE_URI;
    }
    if (*parser).m_namespaceSeparator != 0 {
        len += 1;
    }
    if !(*parser).m_freeBindingList.is_null() {
        b = (*parser).m_freeBindingList;
        if len > (*b).uriAlloc {
            if len > crate::limits_h::INT_MAX - EXPAND_SPARE {
                return crate::expat_h::XML_ERROR_NO_MEMORY;
            }
            let mut temp: *mut crate::expat_external_h::XML_Char = expat_realloc(
                parser,
                (*b).uri as *mut ::core::ffi::c_void,
                (::core::mem::size_of::<crate::expat_external_h::XML_Char>()
                    as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(
                        (len + 24 as ::core::ffi::c_int) as crate::__stddef_size_t_h::size_t,
                    ),
                4517 as ::core::ffi::c_int,
            )
                as *mut crate::expat_external_h::XML_Char;
            if temp.is_null() {
                return crate::expat_h::XML_ERROR_NO_MEMORY;
            }
            (*b).uri = temp;
            (*b).uriAlloc = len + EXPAND_SPARE;
        }
        (*parser).m_freeBindingList = (*b).nextTagBinding as *mut BINDING;
    } else {
        b = expat_malloc(
            parser,
            ::core::mem::size_of::<BINDING>() as crate::__stddef_size_t_h::size_t,
            4525 as ::core::ffi::c_int,
        ) as *mut BINDING;
        if b.is_null() {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
        if len > crate::limits_h::INT_MAX - EXPAND_SPARE {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
        (*b).uri = expat_malloc(
            parser,
            (::core::mem::size_of::<crate::expat_external_h::XML_Char>()
                as crate::__stddef_size_t_h::size_t)
                .wrapping_mul((len + 24 as ::core::ffi::c_int) as crate::__stddef_size_t_h::size_t),
            4543 as ::core::ffi::c_int,
        ) as *mut crate::expat_external_h::XML_Char;
        if (*b).uri.is_null() {
            expat_free(
                parser,
                b as *mut ::core::ffi::c_void,
                4545 as ::core::ffi::c_int,
            );
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
        (*b).uriAlloc = len + EXPAND_SPARE;
    }
    (*b).uriLen = len;
    crate::stdlib::memcpy(
        (*b).uri as *mut ::core::ffi::c_void,
        uri as *const ::core::ffi::c_void,
        (len as crate::__stddef_size_t_h::size_t)
            .wrapping_mul(::core::mem::size_of::<crate::expat_external_h::XML_Char>()
                as crate::__stddef_size_t_h::size_t),
    );
    if (*parser).m_namespaceSeparator != 0 {
        *(*b).uri.offset((len - 1 as ::core::ffi::c_int) as isize) = (*parser).m_namespaceSeparator;
    }
    (*b).prefix = prefix as *mut prefix;
    (*b).attId = attId as *const attribute_id;
    (*b).prevPrefixBinding = (*prefix).binding as *mut binding;
    if *uri as ::core::ffi::c_int == '\0' as i32
        && prefix == &raw mut (*(*parser).m_dtd).defaultPrefix
    {
        (*prefix).binding = ::core::ptr::null_mut::<BINDING>();
    } else {
        (*prefix).binding = b;
    }
    (*b).nextTagBinding = *bindingsPtr as *mut binding;
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
                ::core::ptr::null::<crate::expat_external_h::XML_Char>()
            },
        );
    }
    return crate::expat_h::XML_ERROR_NONE;
}

unsafe extern "C" fn cdataSectionProcessor(
    mut parser: crate::expat_h::XML_Parser,
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut endPtr: *mut *const ::core::ffi::c_char,
) -> crate::expat_h::XML_Error {
    let mut result: crate::expat_h::XML_Error = doCdataSection(
        parser,
        (*parser).m_encoding,
        &raw mut start,
        end,
        endPtr,
        ((*parser).m_parsingStatus.finalBuffer == 0) as ::core::ffi::c_int
            as crate::expat_h::XML_Bool,
        XML_ACCOUNT_DIRECT,
    );
    if result as ::core::ffi::c_uint
        != crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return result;
    }
    if !start.is_null() {
        if !(*parser).m_parentParser.is_null() {
            (*parser).m_processor = Some(
                externalEntityContentProcessor
                    as unsafe extern "C" fn(
                        crate::expat_h::XML_Parser,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> crate::expat_h::XML_Error,
            );
            return externalEntityContentProcessor(parser, start, end, endPtr);
        } else {
            (*parser).m_processor = Some(
                contentProcessor
                    as unsafe extern "C" fn(
                        crate::expat_h::XML_Parser,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> crate::expat_h::XML_Error,
            );
            return contentProcessor(parser, start, end, endPtr);
        }
    }
    return result;
}

unsafe extern "C" fn doCdataSection(
    mut parser: crate::expat_h::XML_Parser,
    mut enc: *const crate::src::xmltok::ENCODING,
    mut startPtr: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextPtr: *mut *const ::core::ffi::c_char,
    mut haveMore: crate::expat_h::XML_Bool,
    mut account: XML_Account,
) -> crate::expat_h::XML_Error {
    let mut s: *const ::core::ffi::c_char = *startPtr;
    let mut eventPP: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    let mut eventEndPP: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    if enc == (*parser).m_encoding {
        eventPP = &raw mut (*parser).m_eventPtr;
        *eventPP = s;
        eventEndPP = &raw mut (*parser).m_eventEndPtr;
    } else {
        eventPP = &raw mut (*(*parser).m_openInternalEntities).internalEventPtr;
        eventEndPP = &raw mut (*(*parser).m_openInternalEntities).internalEventEndPtr;
    }
    *eventPP = s;
    *startPtr = ::core::ptr::null::<::core::ffi::c_char>();
    loop {
        let mut next: *const ::core::ffi::c_char = s;
        let mut tok: ::core::ffi::c_int = (*enc).scanners[2 as ::core::ffi::c_int as usize]
            .expect("non-null function pointer")(
            enc, s, end, &raw mut next
        );
        let mut accounting_levels: ::core::ffi::c_uint = 0;
        let accounting_root =
            getRootParserOf(parser, &raw mut accounting_levels) as crate::expat_h::XML_Parser;
        if accountingDiffTolerated(
            &mut *accounting_root,
            accounting_levels,
            parser == accounting_root,
            tok,
            || {
                ::core::slice::from_raw_parts(
                    s as *const ::core::ffi::c_uchar,
                    byte_offset(next, s) as usize,
                )
            },
            4619 as ::core::ffi::c_int,
            account,
        ) == 0
        {
            accountingReportStats(&*accounting_root, ACCOUNTING_ABORTING_EPILOG);
            return crate::expat_h::XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
        }
        *eventEndPP = next;
        match tok {
            crate::src::xmltok::XML_TOK_CDATA_SECT_CLOSE => {
                if (*parser).m_endCdataSectionHandler.is_some() {
                    (*parser)
                        .m_endCdataSectionHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg
                    );
                } else if false && (*parser).m_characterDataHandler.is_some() {
                    (*parser)
                        .m_characterDataHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        (*parser).m_dataBuf,
                        0 as ::core::ffi::c_int,
                    );
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
                *startPtr = next;
                *nextPtr = next;
                if (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint
                    == crate::expat_h::XML_FINISHED as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    return crate::expat_h::XML_ERROR_ABORTED;
                } else {
                    return crate::expat_h::XML_ERROR_NONE;
                }
            }
            crate::src::xmltok::XML_TOK_DATA_NEWLINE => {
                if (*parser).m_characterDataHandler.is_some() {
                    let mut c: crate::expat_external_h::XML_Char =
                        0xa as crate::expat_external_h::XML_Char;
                    (*parser)
                        .m_characterDataHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        &raw mut c,
                        1 as ::core::ffi::c_int,
                    );
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
            }
            crate::src::xmltok::XML_TOK_DATA_CHARS => {
                let mut charDataHandler: crate::expat_h::XML_CharacterDataHandler =
                    (*parser).m_characterDataHandler;
                if charDataHandler.is_some() {
                    if (*enc).isUtf8 == 0 {
                        loop {
                            let mut dataPtr: *mut ICHAR = (*parser).m_dataBuf as *mut ICHAR;
                            let convert_res: crate::src::xmltok::XML_Convert_Result =
                                (*enc).utf8Convert.expect("non-null function pointer")(
                                    enc,
                                    &raw mut s,
                                    next,
                                    &raw mut dataPtr,
                                    (*parser).m_dataBufEnd as *mut ICHAR,
                                )
                                    as crate::src::xmltok::XML_Convert_Result;
                            *eventEndPP = next;
                            charDataHandler.expect("non-null function pointer")(
                                (*parser).m_handlerArg,
                                (*parser).m_dataBuf,
                                dataPtr.offset_from((*parser).m_dataBuf as *mut ICHAR)
                                    as ::core::ffi::c_long
                                    as ::core::ffi::c_int,
                            );
                            if convert_res as ::core::ffi::c_uint
                                == crate::src::xmltok::XML_CONVERT_COMPLETED as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                                || convert_res as ::core::ffi::c_uint
                                    == crate::src::xmltok::XML_CONVERT_INPUT_INCOMPLETE
                                        as ::core::ffi::c_int
                                        as ::core::ffi::c_uint
                            {
                                break;
                            }
                            *eventPP = s;
                        }
                    } else {
                        charDataHandler.expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            s as *const crate::expat_external_h::XML_Char,
                            (next as *const crate::expat_external_h::XML_Char)
                                .offset_from(s as *const crate::expat_external_h::XML_Char)
                                as ::core::ffi::c_long
                                as ::core::ffi::c_int,
                        );
                    }
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
            }
            crate::src::xmltok::XML_TOK_INVALID => {
                *eventPP = next;
                return crate::expat_h::XML_ERROR_INVALID_TOKEN;
            }
            crate::src::xmltok::XML_TOK_PARTIAL_CHAR => {
                if haveMore != 0 {
                    *nextPtr = s;
                    return crate::expat_h::XML_ERROR_NONE;
                }
                return crate::expat_h::XML_ERROR_PARTIAL_CHAR;
            }
            crate::src::xmltok::XML_TOK_PARTIAL | crate::src::xmltok::XML_TOK_NONE => {
                if haveMore != 0 {
                    *nextPtr = s;
                    return crate::expat_h::XML_ERROR_NONE;
                }
                return crate::expat_h::XML_ERROR_UNCLOSED_CDATA_SECTION;
            }
            _ => {
                *eventPP = next;
                return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
            }
        }
        match (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint {
            3 => {
                *eventPP = next;
                *nextPtr = next;
                return crate::expat_h::XML_ERROR_NONE;
            }
            2 => {
                *eventPP = next;
                return crate::expat_h::XML_ERROR_ABORTED;
            }
            1 => {
                if (*parser).m_reenter != 0 {
                    return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
                }
            }
            _ => {}
        }
        s = next;
        *eventPP = s;
    }
}

unsafe extern "C" fn ignoreSectionProcessor(
    mut parser: crate::expat_h::XML_Parser,
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut endPtr: *mut *const ::core::ffi::c_char,
) -> crate::expat_h::XML_Error {
    let mut result: crate::expat_h::XML_Error = doIgnoreSection(
        parser,
        (*parser).m_encoding,
        &raw mut start,
        end,
        endPtr,
        ((*parser).m_parsingStatus.finalBuffer == 0) as ::core::ffi::c_int
            as crate::expat_h::XML_Bool,
    );
    if result as ::core::ffi::c_uint
        != crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return result;
    }
    if !start.is_null() {
        (*parser).m_processor = Some(
            prologProcessor
                as unsafe extern "C" fn(
                    crate::expat_h::XML_Parser,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> crate::expat_h::XML_Error,
        );
        return prologProcessor(parser, start, end, endPtr);
    }
    return result;
}

unsafe extern "C" fn doIgnoreSection(
    mut parser: crate::expat_h::XML_Parser,
    mut enc: *const crate::src::xmltok::ENCODING,
    mut startPtr: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextPtr: *mut *const ::core::ffi::c_char,
    mut haveMore: crate::expat_h::XML_Bool,
) -> crate::expat_h::XML_Error {
    let mut next: *const ::core::ffi::c_char = *startPtr;
    let mut tok: ::core::ffi::c_int = 0;
    let mut s: *const ::core::ffi::c_char = *startPtr;
    let mut eventPP: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    let mut eventEndPP: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    if enc == (*parser).m_encoding {
        eventPP = &raw mut (*parser).m_eventPtr;
        *eventPP = s;
        eventEndPP = &raw mut (*parser).m_eventEndPtr;
    } else {
        eventPP = &raw mut (*(*parser).m_openInternalEntities).internalEventPtr;
        eventEndPP = &raw mut (*(*parser).m_openInternalEntities).internalEventEndPtr;
    }
    *eventPP = s;
    *startPtr = ::core::ptr::null::<::core::ffi::c_char>();
    tok = (*enc).scanners[3 as ::core::ffi::c_int as usize].expect("non-null function pointer")(
        enc,
        s,
        end,
        &raw mut next,
    );
    let mut accounting_levels: ::core::ffi::c_uint = 0;
    let accounting_root =
        getRootParserOf(parser, &raw mut accounting_levels) as crate::expat_h::XML_Parser;
    if accountingDiffTolerated(
        &mut *accounting_root,
        accounting_levels,
        parser == accounting_root,
        tok,
        || {
            ::core::slice::from_raw_parts(
                s as *const ::core::ffi::c_uchar,
                byte_offset(next, s) as usize,
            )
        },
        4778 as ::core::ffi::c_int,
        XML_ACCOUNT_DIRECT,
    ) == 0
    {
        accountingReportStats(&*accounting_root, ACCOUNTING_ABORTING_EPILOG);
        return crate::expat_h::XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
    }
    *eventEndPP = next;
    match tok {
        crate::src::xmltok::XML_TOK_IGNORE_SECT => {
            if (*parser).m_defaultHandler.is_some() {
                reportDefault(parser, enc, s, next);
            }
            *startPtr = next;
            *nextPtr = next;
            if (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint
                == crate::expat_h::XML_FINISHED as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return crate::expat_h::XML_ERROR_ABORTED;
            } else {
                return crate::expat_h::XML_ERROR_NONE;
            }
        }
        crate::src::xmltok::XML_TOK_INVALID => {
            *eventPP = next;
            return crate::expat_h::XML_ERROR_INVALID_TOKEN;
        }
        crate::src::xmltok::XML_TOK_PARTIAL_CHAR => {
            if haveMore != 0 {
                *nextPtr = s;
                return crate::expat_h::XML_ERROR_NONE;
            }
            return crate::expat_h::XML_ERROR_PARTIAL_CHAR;
        }
        crate::src::xmltok::XML_TOK_PARTIAL | crate::src::xmltok::XML_TOK_NONE => {
            if haveMore != 0 {
                *nextPtr = s;
                return crate::expat_h::XML_ERROR_NONE;
            }
            return crate::expat_h::XML_ERROR_SYNTAX;
        }
        _ => {
            *eventPP = next;
            return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
        }
    };
}

unsafe extern "C" fn initializeEncoding(
    mut parser: crate::expat_h::XML_Parser,
) -> crate::expat_h::XML_Error {
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    s = (*parser).m_protocolEncodingName as *const ::core::ffi::c_char;
    let name = if s.is_null() {
        None
    } else {
        let len = std::ffi::CStr::from_ptr(s).to_bytes_with_nul().len();
        Some(::core::slice::from_raw_parts(s, len))
    };
    if if (*parser).m_ns as ::core::ffi::c_int != 0 {
        crate::src::xmltok::xmltok_ns_c::XmlInitEncodingNS(
            &mut (*parser).m_initEncoding,
            &mut (*parser).m_encoding,
            name,
        )
    } else {
        crate::src::xmltok::xmltok_ns_c::XmlInitEncoding(
            &mut (*parser).m_initEncoding,
            &mut (*parser).m_encoding,
            name,
        )
    } != 0
    {
        return crate::expat_h::XML_ERROR_NONE;
    }
    return handleUnknownEncoding(parser, (*parser).m_protocolEncodingName);
}

unsafe extern "C" fn processXmlDecl(
    mut parser: crate::expat_h::XML_Parser,
    mut isGeneralTextEntity: ::core::ffi::c_int,
    mut s: *const ::core::ffi::c_char,
    mut next: *const ::core::ffi::c_char,
) -> crate::expat_h::XML_Error {
    let mut encodingName: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut storedEncName: *const crate::expat_external_h::XML_Char =
        ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    let mut newEncoding: *const crate::src::xmltok::ENCODING =
        ::core::ptr::null::<crate::src::xmltok::ENCODING>();
    let mut version: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut versionend: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut storedversion: *const crate::expat_external_h::XML_Char =
        ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    let mut standalone: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut accounting_levels: ::core::ffi::c_uint = 0;
    let accounting_root =
        getRootParserOf(parser, &raw mut accounting_levels) as crate::expat_h::XML_Parser;
    if accountingDiffTolerated(
        &mut *accounting_root,
        accounting_levels,
        parser == accounting_root,
        crate::src::xmltok::XML_TOK_XML_DECL,
        || {
            ::core::slice::from_raw_parts(
                s as *const ::core::ffi::c_uchar,
                byte_offset(next, s) as usize,
            )
        },
        4870 as ::core::ffi::c_int,
        XML_ACCOUNT_DIRECT,
    ) == 0
    {
        accountingReportStats(&*accounting_root, ACCOUNTING_ABORTING_EPILOG);
        return crate::expat_h::XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
    }
    if if (*parser).m_ns as ::core::ffi::c_int != 0 {
        crate::src::xmltok::xmltok_ns_c::XmlParseXmlDeclNS(
            isGeneralTextEntity,
            (*parser).m_encoding,
            s,
            next,
            &mut (*parser).m_eventPtr,
            Some(&mut version),
            Some(&mut versionend),
            Some(&mut encodingName),
            Some(&mut newEncoding),
            Some(&mut standalone),
        )
    } else {
        crate::src::xmltok::xmltok_ns_c::XmlParseXmlDecl(
            isGeneralTextEntity,
            (*parser).m_encoding,
            s,
            next,
            &mut (*parser).m_eventPtr,
            Some(&mut version),
            Some(&mut versionend),
            Some(&mut encodingName),
            Some(&mut newEncoding),
            Some(&mut standalone),
        )
    } == 0
    {
        if isGeneralTextEntity != 0 {
            return crate::expat_h::XML_ERROR_TEXT_DECL;
        } else {
            return crate::expat_h::XML_ERROR_XML_DECL;
        }
    }
    if isGeneralTextEntity == 0 && standalone == 1 as ::core::ffi::c_int {
        (*(*parser).m_dtd).standalone = crate::expat_h::XML_TRUE;
        if (*parser).m_paramEntityParsing as ::core::ffi::c_uint
            == crate::expat_h::XML_PARAM_ENTITY_PARSING_UNLESS_STANDALONE as ::core::ffi::c_int
                as ::core::ffi::c_uint
        {
            (*parser).m_paramEntityParsing = crate::expat_h::XML_PARAM_ENTITY_PARSING_NEVER;
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
                return crate::expat_h::XML_ERROR_NO_MEMORY;
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
                return crate::expat_h::XML_ERROR_NO_MEMORY;
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
                || (*newEncoding).minBytesPerChar == 2 as ::core::ffi::c_int
                    && newEncoding != (*parser).m_encoding
            {
                (*parser).m_eventPtr = encodingName;
                return crate::expat_h::XML_ERROR_INCORRECT_ENCODING;
            }
            (*parser).m_encoding = newEncoding;
        } else if !encodingName.is_null() {
            let mut result: crate::expat_h::XML_Error = crate::expat_h::XML_ERROR_NONE;
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
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
            }
            result = handleUnknownEncoding(parser, storedEncName);
            poolClear(&raw mut (*parser).m_temp2Pool);
            if result as ::core::ffi::c_uint
                == crate::expat_h::XML_ERROR_UNKNOWN_ENCODING as ::core::ffi::c_int
                    as ::core::ffi::c_uint
            {
                (*parser).m_eventPtr = encodingName;
            }
            return result;
        }
    }
    if !storedEncName.is_null() || !storedversion.is_null() {
        poolClear(&raw mut (*parser).m_temp2Pool);
    }
    return crate::expat_h::XML_ERROR_NONE;
}

unsafe extern "C" fn handleUnknownEncoding(
    mut parser: crate::expat_h::XML_Parser,
    mut encodingName: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Error {
    if (*parser).m_unknownEncodingHandler.is_some() {
        let mut info: crate::expat_h::XML_Encoding = crate::expat_h::XML_Encoding {
            map: [0; 256],
            data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            convert: None,
            release: None,
        };
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < 256 as ::core::ffi::c_int {
            info.map[i as usize] = -1 as ::core::ffi::c_int;
            i += 1;
        }
        info.convert = None;
        info.data = crate::__stddef_null_h::NULL;
        info.release = None;
        if (*parser)
            .m_unknownEncodingHandler
            .expect("non-null function pointer")(
            (*parser).m_unknownEncodingHandlerData,
            encodingName,
            &raw mut info,
        ) != 0
        {
            let mut enc: *mut crate::src::xmltok::ENCODING =
                ::core::ptr::null_mut::<crate::src::xmltok::ENCODING>();
            (*parser).m_unknownEncodingMem = expat_malloc(
                parser,
                crate::src::xmltok::XmlSizeOfUnknownEncoding() as crate::__stddef_size_t_h::size_t,
                4963 as ::core::ffi::c_int,
            );
            if (*parser).m_unknownEncodingMem.is_null() {
                if info.release.is_some() {
                    info.release.expect("non-null function pointer")(info.data);
                }
                return crate::expat_h::XML_ERROR_NO_MEMORY;
            }
            enc = if (*parser).m_ns as ::core::ffi::c_int != 0 {
                Some(
                    crate::src::xmltok::XmlInitUnknownEncodingNS_ffi
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const ::core::ffi::c_int,
                            crate::src::xmltok::CONVERTER,
                            *mut ::core::ffi::c_void,
                        )
                            -> *mut crate::src::xmltok::ENCODING,
                )
            } else {
                Some(
                    crate::src::xmltok::XmlInitUnknownEncoding_ffi
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const ::core::ffi::c_int,
                            crate::src::xmltok::CONVERTER,
                            *mut ::core::ffi::c_void,
                        )
                            -> *mut crate::src::xmltok::ENCODING,
                )
            }
            .expect("non-null function pointer")(
                (*parser).m_unknownEncodingMem,
                &raw mut info.map as *mut ::core::ffi::c_int,
                info.convert as crate::src::xmltok::CONVERTER,
                info.data,
            );
            if !enc.is_null() {
                (*parser).m_unknownEncodingData = info.data;
                (*parser).m_unknownEncodingRelease = info.release;
                (*parser).m_encoding = enc;
                return crate::expat_h::XML_ERROR_NONE;
            }
        }
        if info.release.is_some() {
            info.release.expect("non-null function pointer")(info.data);
        }
    }
    return crate::expat_h::XML_ERROR_UNKNOWN_ENCODING;
}

unsafe extern "C" fn prologInitProcessor(
    mut parser: crate::expat_h::XML_Parser,
    mut s: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextPtr: *mut *const ::core::ffi::c_char,
) -> crate::expat_h::XML_Error {
    let mut result: crate::expat_h::XML_Error = initializeEncoding(parser);
    if result as ::core::ffi::c_uint
        != crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return result;
    }
    (*parser).m_processor = Some(
        prologProcessor
            as unsafe extern "C" fn(
                crate::expat_h::XML_Parser,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut *const ::core::ffi::c_char,
            ) -> crate::expat_h::XML_Error,
    );
    return prologProcessor(parser, s, end, nextPtr);
}

unsafe extern "C" fn externalParEntInitProcessor(
    mut parser: crate::expat_h::XML_Parser,
    mut s: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextPtr: *mut *const ::core::ffi::c_char,
) -> crate::expat_h::XML_Error {
    let mut result: crate::expat_h::XML_Error = initializeEncoding(parser);
    if result as ::core::ffi::c_uint
        != crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return result;
    }
    (*(*parser).m_dtd).paramEntityRead = crate::expat_h::XML_TRUE;
    if (*parser).m_prologState.inEntityValue != 0 {
        (*parser).m_processor = Some(
            entityValueInitProcessor
                as unsafe extern "C" fn(
                    crate::expat_h::XML_Parser,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> crate::expat_h::XML_Error,
        );
        return entityValueInitProcessor(parser, s, end, nextPtr);
    } else {
        (*parser).m_processor = Some(
            externalParEntProcessor
                as unsafe extern "C" fn(
                    crate::expat_h::XML_Parser,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> crate::expat_h::XML_Error,
        );
        return externalParEntProcessor(parser, s, end, nextPtr);
    };
}

unsafe extern "C" fn entityValueInitProcessor(
    mut parser: crate::expat_h::XML_Parser,
    mut s: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextPtr: *mut *const ::core::ffi::c_char,
) -> crate::expat_h::XML_Error {
    let mut tok: ::core::ffi::c_int = 0;
    let mut start: *const ::core::ffi::c_char = s;
    let mut next: *const ::core::ffi::c_char = start;
    (*parser).m_eventPtr = start;
    loop {
        tok = (*(*parser).m_encoding).scanners[0 as ::core::ffi::c_int as usize]
            .expect("non-null function pointer")(
            (*parser).m_encoding, start, end, &raw mut next
        );
        (*parser).m_eventEndPtr = next;
        if tok <= 0 as ::core::ffi::c_int {
            if (*parser).m_parsingStatus.finalBuffer == 0
                && tok != crate::src::xmltok::XML_TOK_INVALID
            {
                *nextPtr = s;
                return crate::expat_h::XML_ERROR_NONE;
            }
            match tok {
                crate::src::xmltok::XML_TOK_INVALID => {
                    return crate::expat_h::XML_ERROR_INVALID_TOKEN
                }
                crate::src::xmltok::XML_TOK_PARTIAL => {
                    return crate::expat_h::XML_ERROR_UNCLOSED_TOKEN
                }
                crate::src::xmltok::XML_TOK_PARTIAL_CHAR => {
                    return crate::expat_h::XML_ERROR_PARTIAL_CHAR
                }
                crate::src::xmltok::XML_TOK_NONE | _ => {}
            }
            return storeEntityValue(
                parser,
                (*parser).m_encoding,
                s,
                end,
                XML_ACCOUNT_DIRECT,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
        } else if tok == crate::src::xmltok::XML_TOK_XML_DECL {
            let mut result: crate::expat_h::XML_Error = crate::expat_h::XML_ERROR_NONE;
            result = processXmlDecl(parser, 0 as ::core::ffi::c_int, start, next);
            if result as ::core::ffi::c_uint
                != crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return result;
            }
            if (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint
                == crate::expat_h::XML_FINISHED as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return crate::expat_h::XML_ERROR_ABORTED;
            }
            *nextPtr = next;
            (*parser).m_processor = Some(
                entityValueProcessor
                    as unsafe extern "C" fn(
                        crate::expat_h::XML_Parser,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> crate::expat_h::XML_Error,
            );
            return entityValueProcessor(parser, next, end, nextPtr);
        } else if tok == crate::src::xmltok::XML_TOK_BOM {
            let mut accounting_levels: ::core::ffi::c_uint = 0;
            let accounting_root =
                getRootParserOf(parser, &raw mut accounting_levels) as crate::expat_h::XML_Parser;
            if accountingDiffTolerated(
                &mut *accounting_root,
                accounting_levels,
                parser == accounting_root,
                tok,
                || {
                    ::core::slice::from_raw_parts(
                        s as *const ::core::ffi::c_uchar,
                        byte_offset(next, s) as usize,
                    )
                },
                5077 as ::core::ffi::c_int,
                XML_ACCOUNT_DIRECT,
            ) == 0
            {
                accountingReportStats(&*accounting_root, ACCOUNTING_ABORTING_EPILOG);
                return crate::expat_h::XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
            }
            *nextPtr = next;
            s = next;
        } else if tok == crate::src::xmltok::XML_TOK_INSTANCE_START {
            *nextPtr = next;
            return crate::expat_h::XML_ERROR_SYNTAX;
        }
        start = next;
        (*parser).m_eventPtr = start;
    }
}

unsafe extern "C" fn externalParEntProcessor(
    mut parser: crate::expat_h::XML_Parser,
    mut s: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextPtr: *mut *const ::core::ffi::c_char,
) -> crate::expat_h::XML_Error {
    let mut next: *const ::core::ffi::c_char = s;
    let mut tok: ::core::ffi::c_int = 0;
    tok = (*(*parser).m_encoding).scanners[0 as ::core::ffi::c_int as usize]
        .expect("non-null function pointer")((*parser).m_encoding, s, end, &raw mut next);
    if tok <= 0 as ::core::ffi::c_int {
        if (*parser).m_parsingStatus.finalBuffer == 0 && tok != crate::src::xmltok::XML_TOK_INVALID
        {
            *nextPtr = s;
            return crate::expat_h::XML_ERROR_NONE;
        }
        match tok {
            crate::src::xmltok::XML_TOK_INVALID => return crate::expat_h::XML_ERROR_INVALID_TOKEN,
            crate::src::xmltok::XML_TOK_PARTIAL => return crate::expat_h::XML_ERROR_UNCLOSED_TOKEN,
            crate::src::xmltok::XML_TOK_PARTIAL_CHAR => {
                return crate::expat_h::XML_ERROR_PARTIAL_CHAR
            }
            crate::src::xmltok::XML_TOK_NONE | _ => {}
        }
    } else if tok == crate::src::xmltok::XML_TOK_BOM {
        let mut accounting_levels: ::core::ffi::c_uint = 0;
        let accounting_root =
            getRootParserOf(parser, &raw mut accounting_levels) as crate::expat_h::XML_Parser;
        if accountingDiffTolerated(
            &mut *accounting_root,
            accounting_levels,
            parser == accounting_root,
            tok,
            || {
                ::core::slice::from_raw_parts(
                    s as *const ::core::ffi::c_uchar,
                    byte_offset(next, s) as usize,
                )
            },
            5130 as ::core::ffi::c_int,
            XML_ACCOUNT_DIRECT,
        ) == 0
        {
            accountingReportStats(&*accounting_root, ACCOUNTING_ABORTING_EPILOG);
            return crate::expat_h::XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
        }
        s = next;
        tok = (*(*parser).m_encoding).scanners[0 as ::core::ffi::c_int as usize]
            .expect("non-null function pointer")(
            (*parser).m_encoding, s, end, &raw mut next
        );
    }
    (*parser).m_processor = Some(
        prologProcessor
            as unsafe extern "C" fn(
                crate::expat_h::XML_Parser,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut *const ::core::ffi::c_char,
            ) -> crate::expat_h::XML_Error,
    );
    return doProlog(
        parser,
        (*parser).m_encoding,
        s,
        end,
        tok,
        next,
        nextPtr,
        ((*parser).m_parsingStatus.finalBuffer == 0) as ::core::ffi::c_int
            as crate::expat_h::XML_Bool,
        crate::expat_h::XML_TRUE,
        XML_ACCOUNT_DIRECT,
    );
}

unsafe extern "C" fn entityValueProcessor(
    mut parser: crate::expat_h::XML_Parser,
    mut s: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextPtr: *mut *const ::core::ffi::c_char,
) -> crate::expat_h::XML_Error {
    let mut start: *const ::core::ffi::c_char = s;
    let mut next: *const ::core::ffi::c_char = s;
    let mut enc: *const crate::src::xmltok::ENCODING = (*parser).m_encoding;
    let mut tok: ::core::ffi::c_int = 0;
    loop {
        tok = (*enc).scanners[0 as ::core::ffi::c_int as usize].expect("non-null function pointer")(
            enc,
            start,
            end,
            &raw mut next,
        );
        if tok <= 0 as ::core::ffi::c_int {
            if (*parser).m_parsingStatus.finalBuffer == 0
                && tok != crate::src::xmltok::XML_TOK_INVALID
            {
                *nextPtr = s;
                return crate::expat_h::XML_ERROR_NONE;
            }
            match tok {
                crate::src::xmltok::XML_TOK_INVALID => {
                    return crate::expat_h::XML_ERROR_INVALID_TOKEN
                }
                crate::src::xmltok::XML_TOK_PARTIAL => {
                    return crate::expat_h::XML_ERROR_UNCLOSED_TOKEN
                }
                crate::src::xmltok::XML_TOK_PARTIAL_CHAR => {
                    return crate::expat_h::XML_ERROR_PARTIAL_CHAR
                }
                crate::src::xmltok::XML_TOK_NONE | _ => {}
            }
            return storeEntityValue(
                parser,
                enc,
                s,
                end,
                XML_ACCOUNT_DIRECT,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
        }
        start = next;
    }
}

unsafe extern "C" fn prologProcessor(
    mut parser: crate::expat_h::XML_Parser,
    mut s: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextPtr: *mut *const ::core::ffi::c_char,
) -> crate::expat_h::XML_Error {
    let mut next: *const ::core::ffi::c_char = s;
    let mut tok: ::core::ffi::c_int = (*(*parser).m_encoding).scanners
        [0 as ::core::ffi::c_int as usize]
        .expect("non-null function pointer")(
        (*parser).m_encoding, s, end, &raw mut next
    );
    return doProlog(
        parser,
        (*parser).m_encoding,
        s,
        end,
        tok,
        next,
        nextPtr,
        ((*parser).m_parsingStatus.finalBuffer == 0) as ::core::ffi::c_int
            as crate::expat_h::XML_Bool,
        crate::expat_h::XML_TRUE,
        XML_ACCOUNT_DIRECT,
    );
}

unsafe extern "C" fn doProlog(
    mut parser: crate::expat_h::XML_Parser,
    mut enc: *const crate::src::xmltok::ENCODING,
    mut s: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut tok: ::core::ffi::c_int,
    mut next: *const ::core::ffi::c_char,
    mut nextPtr: *mut *const ::core::ffi::c_char,
    mut haveMore: crate::expat_h::XML_Bool,
    mut allowClosingDoctype: crate::expat_h::XML_Bool,
    mut account: XML_Account,
) -> crate::expat_h::XML_Error {
    let mut c2rust_current_block: u64;
    static externalSubsetName: [crate::expat_external_h::XML_Char; 2] = [
        crate::ascii_h::ASCII_HASH as crate::expat_external_h::XML_Char,
        '\0' as i32 as crate::expat_external_h::XML_Char,
    ];
    static atypeCDATA: [crate::expat_external_h::XML_Char; 6] = [
        crate::ascii_h::ASCII_C as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_D as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_A as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_T as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_A as crate::expat_external_h::XML_Char,
        '\0' as i32 as crate::expat_external_h::XML_Char,
    ];
    static atypeID: [crate::expat_external_h::XML_Char; 3] = [
        crate::ascii_h::ASCII_I as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_D as crate::expat_external_h::XML_Char,
        '\0' as i32 as crate::expat_external_h::XML_Char,
    ];
    static atypeIDREF: [crate::expat_external_h::XML_Char; 6] = [
        crate::ascii_h::ASCII_I as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_D as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_R as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_E as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_F as crate::expat_external_h::XML_Char,
        '\0' as i32 as crate::expat_external_h::XML_Char,
    ];
    static atypeIDREFS: [crate::expat_external_h::XML_Char; 7] = [
        crate::ascii_h::ASCII_I as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_D as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_R as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_E as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_F as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_S as crate::expat_external_h::XML_Char,
        '\0' as i32 as crate::expat_external_h::XML_Char,
    ];
    static atypeENTITY: [crate::expat_external_h::XML_Char; 7] = [
        crate::ascii_h::ASCII_E as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_N as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_T as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_I as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_T as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_Y as crate::expat_external_h::XML_Char,
        '\0' as i32 as crate::expat_external_h::XML_Char,
    ];
    static atypeENTITIES: [crate::expat_external_h::XML_Char; 9] = [
        crate::ascii_h::ASCII_E as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_N as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_T as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_I as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_T as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_I as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_E as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_S as crate::expat_external_h::XML_Char,
        '\0' as i32 as crate::expat_external_h::XML_Char,
    ];
    static atypeNMTOKEN: [crate::expat_external_h::XML_Char; 8] = [
        crate::ascii_h::ASCII_N as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_M as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_T as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_O as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_K as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_E as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_N as crate::expat_external_h::XML_Char,
        '\0' as i32 as crate::expat_external_h::XML_Char,
    ];
    static atypeNMTOKENS: [crate::expat_external_h::XML_Char; 9] = [
        crate::ascii_h::ASCII_N as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_M as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_T as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_O as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_K as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_E as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_N as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_S as crate::expat_external_h::XML_Char,
        '\0' as i32 as crate::expat_external_h::XML_Char,
    ];
    static notationPrefix: [crate::expat_external_h::XML_Char; 10] = [
        crate::ascii_h::ASCII_N as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_O as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_T as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_A as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_T as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_I as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_O as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_N as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_LPAREN as crate::expat_external_h::XML_Char,
        '\0' as i32 as crate::expat_external_h::XML_Char,
    ];
    static enumValueSep: [crate::expat_external_h::XML_Char; 2] = [
        crate::ascii_h::ASCII_PIPE as crate::expat_external_h::XML_Char,
        '\0' as i32 as crate::expat_external_h::XML_Char,
    ];
    static enumValueStart: [crate::expat_external_h::XML_Char; 2] = [
        crate::ascii_h::ASCII_LPAREN as crate::expat_external_h::XML_Char,
        '\0' as i32 as crate::expat_external_h::XML_Char,
    ];
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut eventPP: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    let mut eventEndPP: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    let mut quant: crate::expat_h::XML_Content_Quant = crate::expat_h::XML_CQUANT_NONE;
    if enc == (*parser).m_encoding {
        eventPP = &raw mut (*parser).m_eventPtr;
        eventEndPP = &raw mut (*parser).m_eventEndPtr;
    } else {
        eventPP = &raw mut (*(*parser).m_openInternalEntities).internalEventPtr;
        eventEndPP = &raw mut (*(*parser).m_openInternalEntities).internalEventEndPtr;
    }
    loop {
        let mut role: ::core::ffi::c_int = 0;
        let mut handleDefault: crate::expat_h::XML_Bool = crate::expat_h::XML_TRUE;
        *eventPP = s;
        *eventEndPP = next;
        if tok <= 0 as ::core::ffi::c_int {
            if haveMore as ::core::ffi::c_int != 0 && tok != crate::src::xmltok::XML_TOK_INVALID {
                *nextPtr = s;
                return crate::expat_h::XML_ERROR_NONE;
            }
            match tok {
                crate::src::xmltok::XML_TOK_INVALID => {
                    *eventPP = next;
                    return crate::expat_h::XML_ERROR_INVALID_TOKEN;
                }
                crate::src::xmltok::XML_TOK_PARTIAL => {
                    return crate::expat_h::XML_ERROR_UNCLOSED_TOKEN
                }
                crate::src::xmltok::XML_TOK_PARTIAL_CHAR => {
                    return crate::expat_h::XML_ERROR_PARTIAL_CHAR
                }
                -15 => {
                    tok = -tok;
                }
                crate::src::xmltok::XML_TOK_NONE => {
                    if enc != (*parser).m_encoding
                        && (*(*parser).m_openInternalEntities).betweenDecl == 0
                    {
                        *nextPtr = s;
                        return crate::expat_h::XML_ERROR_NONE;
                    }
                    if (*parser).m_isParamEntity as ::core::ffi::c_int != 0
                        || enc != (*parser).m_encoding
                    {
                        if (*parser)
                            .m_prologState
                            .handler
                            .expect("non-null function pointer")(
                            &mut (*parser).m_prologState,
                            -4 as ::core::ffi::c_int,
                            end,
                            end,
                            enc,
                        ) == crate::src::xmlrole::XML_ROLE_ERROR as ::core::ffi::c_int
                        {
                            return crate::expat_h::XML_ERROR_INCOMPLETE_PE;
                        }
                        *nextPtr = s;
                        return crate::expat_h::XML_ERROR_NONE;
                    }
                    return crate::expat_h::XML_ERROR_NO_ELEMENTS;
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
            &mut (*parser).m_prologState, tok, s, next, enc
        );
        match role {
            2 | 1 | 57 => {}
            _ => {
                let mut accounting_levels: ::core::ffi::c_uint = 0;
                let accounting_root = getRootParserOf(parser, &raw mut accounting_levels)
                    as crate::expat_h::XML_Parser;
                if accountingDiffTolerated(
                    &mut *accounting_root,
                    accounting_levels,
                    parser == accounting_root,
                    tok,
                    || {
                        ::core::slice::from_raw_parts(
                            s as *const ::core::ffi::c_uchar,
                            byte_offset(next, s) as usize,
                        )
                    },
                    5301 as ::core::ffi::c_int,
                    account,
                ) == 0
                {
                    accountingReportStats(&*accounting_root, ACCOUNTING_ABORTING_EPILOG);
                    return crate::expat_h::XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
                }
            }
        }
        match role {
            1 => {
                let mut result: crate::expat_h::XML_Error =
                    processXmlDecl(parser, 0 as ::core::ffi::c_int, s, next);
                if result as ::core::ffi::c_uint
                    != crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    return result;
                }
                enc = (*parser).m_encoding;
                handleDefault = crate::expat_h::XML_FALSE;
                c2rust_current_block = 8258632986558375165;
            }
            4 => {
                if (*parser).m_startDoctypeDeclHandler.is_some() {
                    (*parser).m_doctypeName =
                        poolStoreString(&raw mut (*parser).m_tempPool, enc, s, next);
                    if (*parser).m_doctypeName.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                    (*parser).m_doctypePubid =
                        ::core::ptr::null::<crate::expat_external_h::XML_Char>();
                    handleDefault = crate::expat_h::XML_FALSE;
                }
                (*parser).m_doctypeSysid = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
                c2rust_current_block = 8258632986558375165;
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
                        1 as ::core::ffi::c_int,
                    );
                    (*parser).m_doctypeName =
                        ::core::ptr::null::<crate::expat_external_h::XML_Char>();
                    poolClear(&raw mut (*parser).m_tempPool);
                    handleDefault = crate::expat_h::XML_FALSE;
                }
                c2rust_current_block = 8258632986558375165;
            }
            57 => {
                let mut result_0: crate::expat_h::XML_Error =
                    processXmlDecl(parser, 1 as ::core::ffi::c_int, s, next);
                if result_0 as ::core::ffi::c_uint
                    != crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    return result_0;
                }
                enc = (*parser).m_encoding;
                handleDefault = crate::expat_h::XML_FALSE;
                c2rust_current_block = 8258632986558375165;
            }
            6 => {
                (*parser).m_useForeignDTD = crate::expat_h::XML_FALSE;
                (*parser).m_declEntity = lookup(
                    parser,
                    &raw mut (*dtd).paramEntities,
                    externalSubsetName.as_ptr() as KEY,
                    ::core::mem::size_of::<ENTITY>() as crate::__stddef_size_t_h::size_t,
                ) as *mut ENTITY;
                if (*parser).m_declEntity.is_null() {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
                (*dtd).hasParamEntityRefs = crate::expat_h::XML_TRUE;
                if (*parser).m_startDoctypeDeclHandler.is_some() {
                    let mut pubId: *mut crate::expat_external_h::XML_Char =
                        ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
                    if (*enc).isPublicId.expect("non-null function pointer")(enc, s, next, eventPP)
                        == 0
                    {
                        return crate::expat_h::XML_ERROR_PUBLICID;
                    }
                    pubId = poolStoreString(
                        &raw mut (*parser).m_tempPool,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if pubId.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    let pub_id_len = ::std::ffi::CStr::from_ptr(pubId).to_bytes_with_nul().len();
                    normalizePublicId(::core::slice::from_raw_parts_mut(pubId, pub_id_len));
                    (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                    (*parser).m_doctypePubid = pubId;
                    handleDefault = crate::expat_h::XML_FALSE;
                    c2rust_current_block = 14769507584970373587;
                } else {
                    c2rust_current_block = 3681359897139369850;
                }
            }
            14 => {
                c2rust_current_block = 3681359897139369850;
            }
            8 => {
                if allowClosingDoctype as ::core::ffi::c_int
                    != crate::expat_h::XML_TRUE as ::core::ffi::c_int
                {
                    return crate::expat_h::XML_ERROR_INVALID_TOKEN;
                }
                if !(*parser).m_doctypeName.is_null() {
                    (*parser)
                        .m_startDoctypeDeclHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        (*parser).m_doctypeName,
                        (*parser).m_doctypeSysid,
                        (*parser).m_doctypePubid,
                        0 as ::core::ffi::c_int,
                    );
                    poolClear(&raw mut (*parser).m_tempPool);
                    handleDefault = crate::expat_h::XML_FALSE;
                }
                if !(*parser).m_doctypeSysid.is_null()
                    || (*parser).m_useForeignDTD as ::core::ffi::c_int != 0
                {
                    let mut hadParamEntityRefs: crate::expat_h::XML_Bool =
                        (*dtd).hasParamEntityRefs;
                    (*dtd).hasParamEntityRefs = crate::expat_h::XML_TRUE;
                    if (*parser).m_paramEntityParsing as ::core::ffi::c_uint != 0
                        && (*parser).m_externalEntityRefHandler.is_some()
                    {
                        let mut entity: *mut ENTITY = lookup(
                            parser,
                            &raw mut (*dtd).paramEntities,
                            externalSubsetName.as_ptr() as KEY,
                            ::core::mem::size_of::<ENTITY>() as crate::__stddef_size_t_h::size_t,
                        ) as *mut ENTITY;
                        if entity.is_null() {
                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                        }
                        if (*parser).m_useForeignDTD != 0 {
                            (*entity).base = (*parser).m_curBase;
                        }
                        (*dtd).paramEntityRead = crate::expat_h::XML_FALSE;
                        if (*parser)
                            .m_externalEntityRefHandler
                            .expect("non-null function pointer")(
                            (*parser).m_externalEntityRefHandlerArg,
                            ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
                            (*entity).base,
                            (*entity).systemId,
                            (*entity).publicId,
                        ) == 0
                        {
                            return crate::expat_h::XML_ERROR_EXTERNAL_ENTITY_HANDLING;
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
                                return crate::expat_h::XML_ERROR_NOT_STANDALONE;
                            }
                        } else if (*parser).m_doctypeSysid.is_null() {
                            (*dtd).hasParamEntityRefs = hadParamEntityRefs;
                        }
                    }
                    (*parser).m_useForeignDTD = crate::expat_h::XML_FALSE;
                }
                if (*parser).m_endDoctypeDeclHandler.is_some() {
                    (*parser)
                        .m_endDoctypeDeclHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg
                    );
                    handleDefault = crate::expat_h::XML_FALSE;
                }
                c2rust_current_block = 8258632986558375165;
            }
            2 => {
                if (*parser).m_useForeignDTD != 0 {
                    let mut hadParamEntityRefs_0: crate::expat_h::XML_Bool =
                        (*dtd).hasParamEntityRefs;
                    (*dtd).hasParamEntityRefs = crate::expat_h::XML_TRUE;
                    if (*parser).m_paramEntityParsing as ::core::ffi::c_uint != 0
                        && (*parser).m_externalEntityRefHandler.is_some()
                    {
                        let mut entity_0: *mut ENTITY = lookup(
                            parser,
                            &raw mut (*dtd).paramEntities,
                            externalSubsetName.as_ptr() as KEY,
                            ::core::mem::size_of::<ENTITY>() as crate::__stddef_size_t_h::size_t,
                        ) as *mut ENTITY;
                        if entity_0.is_null() {
                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                        }
                        (*entity_0).base = (*parser).m_curBase;
                        (*dtd).paramEntityRead = crate::expat_h::XML_FALSE;
                        if (*parser)
                            .m_externalEntityRefHandler
                            .expect("non-null function pointer")(
                            (*parser).m_externalEntityRefHandlerArg,
                            ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
                            (*entity_0).base,
                            (*entity_0).systemId,
                            (*entity_0).publicId,
                        ) == 0
                        {
                            return crate::expat_h::XML_ERROR_EXTERNAL_ENTITY_HANDLING;
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
                                return crate::expat_h::XML_ERROR_NOT_STANDALONE;
                            }
                        } else {
                            (*dtd).hasParamEntityRefs = hadParamEntityRefs_0;
                        }
                    }
                }
                (*parser).m_processor = Some(
                    contentProcessor
                        as unsafe extern "C" fn(
                            crate::expat_h::XML_Parser,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        )
                            -> crate::expat_h::XML_Error,
                );
                return contentProcessor(parser, s, end, nextPtr);
            }
            34 => {
                (*parser).m_declElementType = getElementType(parser, enc, s, next);
                if (*parser).m_declElementType.is_null() {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
                c2rust_current_block = 11852364650731333344;
            }
            22 => {
                (*parser).m_declAttributeId = getAttributeId(parser, enc, s, next);
                if (*parser).m_declAttributeId.is_null() {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
                (*parser).m_declAttributeIsCdata = crate::expat_h::XML_FALSE;
                (*parser).m_declAttributeType =
                    ::core::ptr::null::<crate::expat_external_h::XML_Char>();
                (*parser).m_declAttributeIsId = crate::expat_h::XML_FALSE;
                c2rust_current_block = 11852364650731333344;
            }
            23 => {
                (*parser).m_declAttributeIsCdata = crate::expat_h::XML_TRUE;
                (*parser).m_declAttributeType = atypeCDATA.as_ptr();
                c2rust_current_block = 11852364650731333344;
            }
            24 => {
                (*parser).m_declAttributeIsId = crate::expat_h::XML_TRUE;
                (*parser).m_declAttributeType = atypeID.as_ptr();
                c2rust_current_block = 11852364650731333344;
            }
            25 => {
                (*parser).m_declAttributeType = atypeIDREF.as_ptr();
                c2rust_current_block = 11852364650731333344;
            }
            26 => {
                (*parser).m_declAttributeType = atypeIDREFS.as_ptr();
                c2rust_current_block = 11852364650731333344;
            }
            27 => {
                (*parser).m_declAttributeType = atypeENTITY.as_ptr();
                c2rust_current_block = 11852364650731333344;
            }
            28 => {
                (*parser).m_declAttributeType = atypeENTITIES.as_ptr();
                c2rust_current_block = 11852364650731333344;
            }
            29 => {
                (*parser).m_declAttributeType = atypeNMTOKEN.as_ptr();
                c2rust_current_block = 11852364650731333344;
            }
            30 => {
                (*parser).m_declAttributeType = atypeNMTOKENS.as_ptr();
                c2rust_current_block = 11852364650731333344;
            }
            31 | 32 => {
                if (*dtd).keepProcessing as ::core::ffi::c_int != 0
                    && (*parser).m_attlistDeclHandler.is_some()
                {
                    let mut prefix: *const crate::expat_external_h::XML_Char =
                        ::core::ptr::null::<crate::expat_external_h::XML_Char>();
                    if !(*parser).m_declAttributeType.is_null() {
                        prefix = enumValueSep.as_ptr();
                    } else {
                        prefix = if role
                            == crate::src::xmlrole::XML_ROLE_ATTRIBUTE_NOTATION_VALUE
                                as ::core::ffi::c_int
                        {
                            notationPrefix.as_ptr()
                        } else {
                            enumValueStart.as_ptr()
                        };
                    }
                    while *prefix != 0 {
                        if if (*parser).m_tempPool.ptr
                            == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
                            && poolGrow(&raw mut (*parser).m_tempPool) == 0
                        {
                            0 as ::core::ffi::c_int
                        } else {
                            let c2rust_fresh74 = (*parser).m_tempPool.ptr;
                            (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                            *c2rust_fresh74 = *prefix;
                            1 as ::core::ffi::c_int
                        } == 0
                        {
                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                        }
                        prefix = prefix.offset(1);
                    }
                    if poolAppend(&raw mut (*parser).m_tempPool, enc, s, next).is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    (*parser).m_declAttributeType = (*parser).m_tempPool.start;
                    handleDefault = crate::expat_h::XML_FALSE;
                }
                c2rust_current_block = 8258632986558375165;
            }
            35 | 36 => {
                if (*dtd).keepProcessing != 0 {
                    if defineAttribute(
                        (*parser).m_declElementType,
                        (*parser).m_declAttributeId,
                        (*parser).m_declAttributeIsCdata,
                        (*parser).m_declAttributeIsId,
                        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
                        parser,
                    ) == 0
                    {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    if (*parser).m_attlistDeclHandler.is_some()
                        && !(*parser).m_declAttributeType.is_null()
                    {
                        if *(*parser).m_declAttributeType as ::core::ffi::c_int
                            == 0x28 as ::core::ffi::c_int
                            || *(*parser).m_declAttributeType as ::core::ffi::c_int
                                == 0x4e as ::core::ffi::c_int
                                && *(*parser)
                                    .m_declAttributeType
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == 0x4f as ::core::ffi::c_int
                        {
                            if (if (*parser).m_tempPool.ptr
                                == (*parser).m_tempPool.end
                                    as *mut crate::expat_external_h::XML_Char
                                && poolGrow(&raw mut (*parser).m_tempPool) == 0
                            {
                                0 as ::core::ffi::c_int
                            } else {
                                let c2rust_fresh1 = (*parser).m_tempPool.ptr;
                                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                                *c2rust_fresh1 = 0x29 as crate::expat_external_h::XML_Char;
                                1 as ::core::ffi::c_int
                            }) == 0
                                || (if (*parser).m_tempPool.ptr
                                    == (*parser).m_tempPool.end
                                        as *mut crate::expat_external_h::XML_Char
                                    && poolGrow(&raw mut (*parser).m_tempPool) == 0
                                {
                                    0 as ::core::ffi::c_int
                                } else {
                                    let c2rust_fresh2 = (*parser).m_tempPool.ptr;
                                    (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                                    *c2rust_fresh2 =
                                        '\0' as i32 as crate::expat_external_h::XML_Char;
                                    1 as ::core::ffi::c_int
                                }) == 0
                            {
                                return crate::expat_h::XML_ERROR_NO_MEMORY;
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
                            ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
                            (role
                                == crate::src::xmlrole::XML_ROLE_REQUIRED_ATTRIBUTE_VALUE
                                    as ::core::ffi::c_int)
                                as ::core::ffi::c_int,
                        );
                        handleDefault = crate::expat_h::XML_FALSE;
                    }
                }
                poolClear(&raw mut (*parser).m_tempPool);
                c2rust_current_block = 8258632986558375165;
            }
            37 | 38 => {
                if (*dtd).keepProcessing != 0 {
                    let mut attVal: *const crate::expat_external_h::XML_Char =
                        ::core::ptr::null::<crate::expat_external_h::XML_Char>();
                    let mut result_1: crate::expat_h::XML_Error = storeAttributeValue(
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
                        crate::expat_h::XML_FALSE,
                        attVal,
                        parser,
                    ) == 0
                    {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    if (*parser).m_attlistDeclHandler.is_some()
                        && !(*parser).m_declAttributeType.is_null()
                    {
                        if *(*parser).m_declAttributeType as ::core::ffi::c_int
                            == 0x28 as ::core::ffi::c_int
                            || *(*parser).m_declAttributeType as ::core::ffi::c_int
                                == 0x4e as ::core::ffi::c_int
                                && *(*parser)
                                    .m_declAttributeType
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == 0x4f as ::core::ffi::c_int
                        {
                            if (if (*parser).m_tempPool.ptr
                                == (*parser).m_tempPool.end
                                    as *mut crate::expat_external_h::XML_Char
                                && poolGrow(&raw mut (*parser).m_tempPool) == 0
                            {
                                0 as ::core::ffi::c_int
                            } else {
                                let c2rust_fresh3 = (*parser).m_tempPool.ptr;
                                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                                *c2rust_fresh3 = 0x29 as crate::expat_external_h::XML_Char;
                                1 as ::core::ffi::c_int
                            }) == 0
                                || (if (*parser).m_tempPool.ptr
                                    == (*parser).m_tempPool.end
                                        as *mut crate::expat_external_h::XML_Char
                                    && poolGrow(&raw mut (*parser).m_tempPool) == 0
                                {
                                    0 as ::core::ffi::c_int
                                } else {
                                    let c2rust_fresh4 = (*parser).m_tempPool.ptr;
                                    (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                                    *c2rust_fresh4 =
                                        '\0' as i32 as crate::expat_external_h::XML_Char;
                                    1 as ::core::ffi::c_int
                                }) == 0
                            {
                                return crate::expat_h::XML_ERROR_NO_MEMORY;
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
                            (role
                                == crate::src::xmlrole::XML_ROLE_FIXED_ATTRIBUTE_VALUE
                                    as ::core::ffi::c_int)
                                as ::core::ffi::c_int,
                        );
                        poolClear(&raw mut (*parser).m_tempPool);
                        handleDefault = crate::expat_h::XML_FALSE;
                    }
                }
                c2rust_current_block = 8258632986558375165;
            }
            12 => {
                if (*dtd).keepProcessing != 0 {
                    let mut result_2: crate::expat_h::XML_Error = callStoreEntityValue(
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
                            as ::core::ffi::c_long
                            as ::core::ffi::c_int;
                        (*dtd).entityValuePool.start = (*dtd).entityValuePool.ptr;
                        if (*parser).m_entityDeclHandler.is_some() {
                            *eventEndPP = s;
                            (*parser)
                                .m_entityDeclHandler
                                .expect("non-null function pointer")(
                                (*parser).m_handlerArg,
                                (*(*parser).m_declEntity).name,
                                (*(*parser).m_declEntity).is_param as ::core::ffi::c_int,
                                (*(*parser).m_declEntity).textPtr,
                                (*(*parser).m_declEntity).textLen,
                                (*parser).m_curBase,
                                ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
                                ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
                                ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
                            );
                            handleDefault = crate::expat_h::XML_FALSE;
                        }
                    } else {
                        (*dtd).entityValuePool.ptr = (*dtd).entityValuePool.start;
                    }
                    if result_2 as ::core::ffi::c_uint
                        != crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int
                            as ::core::ffi::c_uint
                    {
                        return result_2;
                    }
                }
                c2rust_current_block = 8258632986558375165;
            }
            5 => {
                (*parser).m_useForeignDTD = crate::expat_h::XML_FALSE;
                (*dtd).hasParamEntityRefs = crate::expat_h::XML_TRUE;
                if (*parser).m_startDoctypeDeclHandler.is_some() {
                    (*parser).m_doctypeSysid = poolStoreString(
                        &raw mut (*parser).m_tempPool,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if (*parser).m_doctypeSysid.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                    handleDefault = crate::expat_h::XML_FALSE;
                } else {
                    (*parser).m_doctypeSysid = externalSubsetName.as_ptr();
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
                    return crate::expat_h::XML_ERROR_NOT_STANDALONE;
                }
                if (*parser).m_declEntity.is_null() {
                    (*parser).m_declEntity = lookup(
                        parser,
                        &raw mut (*dtd).paramEntities,
                        externalSubsetName.as_ptr() as KEY,
                        ::core::mem::size_of::<ENTITY>() as crate::__stddef_size_t_h::size_t,
                    ) as *mut ENTITY;
                    if (*parser).m_declEntity.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    (*(*parser).m_declEntity).publicId =
                        ::core::ptr::null::<crate::expat_external_h::XML_Char>();
                }
                c2rust_current_block = 16790621803724920579;
            }
            13 => {
                c2rust_current_block = 16790621803724920579;
            }
            15 => {
                if (*dtd).keepProcessing as ::core::ffi::c_int != 0
                    && !(*parser).m_declEntity.is_null()
                    && (*parser).m_entityDeclHandler.is_some()
                {
                    *eventEndPP = s;
                    (*parser)
                        .m_entityDeclHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        (*(*parser).m_declEntity).name,
                        (*(*parser).m_declEntity).is_param as ::core::ffi::c_int,
                        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
                        0 as ::core::ffi::c_int,
                        (*(*parser).m_declEntity).base,
                        (*(*parser).m_declEntity).systemId,
                        (*(*parser).m_declEntity).publicId,
                        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
                    );
                    handleDefault = crate::expat_h::XML_FALSE;
                }
                c2rust_current_block = 8258632986558375165;
            }
            16 => {
                if (*dtd).keepProcessing as ::core::ffi::c_int != 0
                    && !(*parser).m_declEntity.is_null()
                {
                    (*(*parser).m_declEntity).notation =
                        poolStoreString(&raw mut (*dtd).pool, enc, s, next);
                    if (*(*parser).m_declEntity).notation.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
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
                        handleDefault = crate::expat_h::XML_FALSE;
                    } else if (*parser).m_entityDeclHandler.is_some() {
                        *eventEndPP = s;
                        (*parser)
                            .m_entityDeclHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            (*(*parser).m_declEntity).name,
                            0 as ::core::ffi::c_int,
                            ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
                            0 as ::core::ffi::c_int,
                            (*(*parser).m_declEntity).base,
                            (*(*parser).m_declEntity).systemId,
                            (*(*parser).m_declEntity).publicId,
                            (*(*parser).m_declEntity).notation,
                        );
                        handleDefault = crate::expat_h::XML_FALSE;
                    }
                }
                c2rust_current_block = 8258632986558375165;
            }
            9 => {
                if (*enc)
                    .predefinedEntityName
                    .expect("non-null function pointer")(enc, s, next)
                    != 0
                {
                    (*parser).m_declEntity = ::core::ptr::null_mut::<ENTITY>();
                } else if (*dtd).keepProcessing != 0 {
                    let mut name: *const crate::expat_external_h::XML_Char =
                        poolStoreString(&raw mut (*dtd).pool, enc, s, next);
                    if name.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    (*parser).m_declEntity = lookup(
                        parser,
                        &raw mut (*dtd).generalEntities,
                        name as KEY,
                        ::core::mem::size_of::<ENTITY>() as crate::__stddef_size_t_h::size_t,
                    ) as *mut ENTITY;
                    if (*parser).m_declEntity.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    if (*(*parser).m_declEntity).name != name {
                        (*dtd).pool.ptr = (*dtd).pool.start;
                        (*parser).m_declEntity = ::core::ptr::null_mut::<ENTITY>();
                    } else {
                        (*dtd).pool.start = (*dtd).pool.ptr;
                        (*(*parser).m_declEntity).publicId =
                            ::core::ptr::null::<crate::expat_external_h::XML_Char>();
                        (*(*parser).m_declEntity).is_param = crate::expat_h::XML_FALSE;
                        (*(*parser).m_declEntity).is_internal =
                            !(!(*parser).m_parentParser.is_null()
                                || !(*parser).m_openInternalEntities.is_null())
                                as ::core::ffi::c_int
                                as crate::expat_h::XML_Bool;
                        if (*parser).m_entityDeclHandler.is_some() {
                            handleDefault = crate::expat_h::XML_FALSE;
                        }
                    }
                } else {
                    (*dtd).pool.ptr = (*dtd).pool.start;
                    (*parser).m_declEntity = ::core::ptr::null_mut::<ENTITY>();
                }
                c2rust_current_block = 8258632986558375165;
            }
            10 => {
                if (*dtd).keepProcessing != 0 {
                    let mut name_0: *const crate::expat_external_h::XML_Char =
                        poolStoreString(&raw mut (*dtd).pool, enc, s, next);
                    if name_0.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    (*parser).m_declEntity = lookup(
                        parser,
                        &raw mut (*dtd).paramEntities,
                        name_0 as KEY,
                        ::core::mem::size_of::<ENTITY>() as crate::__stddef_size_t_h::size_t,
                    ) as *mut ENTITY;
                    if (*parser).m_declEntity.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    if (*(*parser).m_declEntity).name != name_0 {
                        (*dtd).pool.ptr = (*dtd).pool.start;
                        (*parser).m_declEntity = ::core::ptr::null_mut::<ENTITY>();
                    } else {
                        (*dtd).pool.start = (*dtd).pool.ptr;
                        (*(*parser).m_declEntity).publicId =
                            ::core::ptr::null::<crate::expat_external_h::XML_Char>();
                        (*(*parser).m_declEntity).is_param = crate::expat_h::XML_TRUE;
                        (*(*parser).m_declEntity).is_internal =
                            !(!(*parser).m_parentParser.is_null()
                                || !(*parser).m_openInternalEntities.is_null())
                                as ::core::ffi::c_int
                                as crate::expat_h::XML_Bool;
                        if (*parser).m_entityDeclHandler.is_some() {
                            handleDefault = crate::expat_h::XML_FALSE;
                        }
                    }
                } else {
                    (*dtd).pool.ptr = (*dtd).pool.start;
                    (*parser).m_declEntity = ::core::ptr::null_mut::<ENTITY>();
                }
                c2rust_current_block = 8258632986558375165;
            }
            18 => {
                (*parser).m_declNotationPublicId =
                    ::core::ptr::null::<crate::expat_external_h::XML_Char>();
                (*parser).m_declNotationName =
                    ::core::ptr::null::<crate::expat_external_h::XML_Char>();
                if (*parser).m_notationDeclHandler.is_some() {
                    (*parser).m_declNotationName =
                        poolStoreString(&raw mut (*parser).m_tempPool, enc, s, next);
                    if (*parser).m_declNotationName.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                    handleDefault = crate::expat_h::XML_FALSE;
                }
                c2rust_current_block = 8258632986558375165;
            }
            21 => {
                if (*enc).isPublicId.expect("non-null function pointer")(enc, s, next, eventPP) == 0
                {
                    return crate::expat_h::XML_ERROR_PUBLICID;
                }
                if !(*parser).m_declNotationName.is_null() {
                    let mut tem_0: *mut crate::expat_external_h::XML_Char = poolStoreString(
                        &raw mut (*parser).m_tempPool,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if tem_0.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    let public_id_len = ::std::ffi::CStr::from_ptr(tem_0).to_bytes_with_nul().len();
                    normalizePublicId(::core::slice::from_raw_parts_mut(tem_0, public_id_len));
                    (*parser).m_declNotationPublicId = tem_0;
                    (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                    handleDefault = crate::expat_h::XML_FALSE;
                }
                c2rust_current_block = 8258632986558375165;
            }
            19 => {
                if !(*parser).m_declNotationName.is_null()
                    && (*parser).m_notationDeclHandler.is_some()
                {
                    let mut systemId: *const crate::expat_external_h::XML_Char = poolStoreString(
                        &raw mut (*parser).m_tempPool,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if systemId.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
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
                    handleDefault = crate::expat_h::XML_FALSE;
                }
                poolClear(&raw mut (*parser).m_tempPool);
                c2rust_current_block = 8258632986558375165;
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
                        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
                        (*parser).m_declNotationPublicId,
                    );
                    handleDefault = crate::expat_h::XML_FALSE;
                }
                poolClear(&raw mut (*parser).m_tempPool);
                c2rust_current_block = 8258632986558375165;
            }
            -1 => match tok {
                crate::src::xmltok::XML_TOK_PARAM_ENTITY_REF => {
                    return crate::expat_h::XML_ERROR_PARAM_ENTITY_REF
                }
                crate::src::xmltok::XML_TOK_XML_DECL => {
                    return crate::expat_h::XML_ERROR_MISPLACED_XML_PI
                }
                _ => return crate::expat_h::XML_ERROR_SYNTAX,
            },
            58 => {
                let mut result_3: crate::expat_h::XML_Error = crate::expat_h::XML_ERROR_NONE;
                if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
                handleDefault = crate::expat_h::XML_FALSE;
                result_3 = doIgnoreSection(parser, enc, &raw mut next, end, nextPtr, haveMore);
                if result_3 as ::core::ffi::c_uint
                    != crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    return result_3;
                } else if next.is_null() {
                    (*parser).m_processor = Some(
                        ignoreSectionProcessor
                            as unsafe extern "C" fn(
                                crate::expat_h::XML_Parser,
                                *const ::core::ffi::c_char,
                                *const ::core::ffi::c_char,
                                *mut *const ::core::ffi::c_char,
                            )
                                -> crate::expat_h::XML_Error,
                    );
                    return result_3;
                }
                c2rust_current_block = 8258632986558375165;
            }
            44 => {
                if (*parser).m_prologState.level >= (*parser).m_groupSize {
                    if (*parser).m_groupSize != 0 {
                        if (*parser).m_groupSize
                            > (-1 as ::core::ffi::c_int as ::core::ffi::c_uint)
                                .wrapping_div(2 as ::core::ffi::c_uint)
                        {
                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                        }
                        (*parser).m_groupSize =
                            (*parser).m_groupSize.wrapping_mul(2 as ::core::ffi::c_uint);
                        let new_connector: *mut ::core::ffi::c_char = expat_realloc(
                            parser,
                            (*parser).m_groupConnector as *mut ::core::ffi::c_void,
                            (*parser).m_groupSize as crate::__stddef_size_t_h::size_t,
                            5915 as ::core::ffi::c_int,
                        )
                            as *mut ::core::ffi::c_char;
                        if new_connector.is_null() {
                            (*parser).m_groupSize =
                                (*parser).m_groupSize.wrapping_div(2 as ::core::ffi::c_uint);
                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                        }
                        (*parser).m_groupConnector = new_connector;
                        if !(*dtd).scaffIndex.is_null() {
                            let new_scaff_index: *mut ::core::ffi::c_int = expat_realloc(
                                parser,
                                (*dtd).scaffIndex as *mut ::core::ffi::c_void,
                                ((*parser).m_groupSize as crate::__stddef_size_t_h::size_t)
                                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>()
                                        as crate::__stddef_size_t_h::size_t),
                                5936 as ::core::ffi::c_int,
                            )
                                as *mut ::core::ffi::c_int;
                            if new_scaff_index.is_null() {
                                (*parser).m_groupSize =
                                    (*parser).m_groupSize.wrapping_div(2 as ::core::ffi::c_uint);
                                return crate::expat_h::XML_ERROR_NO_MEMORY;
                            }
                            (*dtd).scaffIndex = new_scaff_index;
                        }
                    } else {
                        (*parser).m_groupSize = 32 as ::core::ffi::c_uint;
                        (*parser).m_groupConnector = expat_malloc(
                            parser,
                            (*parser).m_groupSize as crate::__stddef_size_t_h::size_t,
                            5944 as ::core::ffi::c_int,
                        )
                            as *mut ::core::ffi::c_char;
                        if (*parser).m_groupConnector.is_null() {
                            (*parser).m_groupSize = 0 as ::core::ffi::c_uint;
                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                        }
                    }
                }
                *(*parser)
                    .m_groupConnector
                    .offset((*parser).m_prologState.level as isize) = 0 as ::core::ffi::c_char;
                if (*dtd).in_eldecl != 0 {
                    let mut myindex: ::core::ffi::c_int = nextScaffoldPart(parser);
                    if myindex < 0 as ::core::ffi::c_int {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    '_c2rust_label: {
                        if !(*dtd).scaffIndex.is_null() {
                        } else {
                            crate::stdlib::__assert_fail(
                                b"dtd->scaffIndex != NULL\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"../../expat/lib/xmlparse.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                5956 as ::core::ffi::c_uint,
                                b"enum XML_Error doProlog(XML_Parser, const ENCODING *, const char *, const char *, int, const char *, const char **, XML_Bool, XML_Bool, enum XML_Account)\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                            );
                        }
                    };
                    *(*dtd).scaffIndex.offset((*dtd).scaffLevel as isize) = myindex;
                    (*dtd).scaffLevel += 1;
                    (*(*dtd).scaffold.offset(myindex as isize)).type_0 =
                        crate::expat_h::XML_CTYPE_SEQ;
                    if (*parser).m_elementDeclHandler.is_some() {
                        handleDefault = crate::expat_h::XML_FALSE;
                    }
                }
                c2rust_current_block = 8258632986558375165;
            }
            50 => {
                if *(*parser)
                    .m_groupConnector
                    .offset((*parser).m_prologState.level as isize)
                    as ::core::ffi::c_int
                    == crate::ascii_h::ASCII_PIPE
                {
                    return crate::expat_h::XML_ERROR_SYNTAX;
                }
                *(*parser)
                    .m_groupConnector
                    .offset((*parser).m_prologState.level as isize) =
                    crate::ascii_h::ASCII_COMMA as ::core::ffi::c_char;
                if (*dtd).in_eldecl as ::core::ffi::c_int != 0
                    && (*parser).m_elementDeclHandler.is_some()
                {
                    handleDefault = crate::expat_h::XML_FALSE;
                }
                c2rust_current_block = 8258632986558375165;
            }
            49 => {
                if *(*parser)
                    .m_groupConnector
                    .offset((*parser).m_prologState.level as isize)
                    as ::core::ffi::c_int
                    == crate::ascii_h::ASCII_COMMA
                {
                    return crate::expat_h::XML_ERROR_SYNTAX;
                }
                if (*dtd).in_eldecl as ::core::ffi::c_int != 0
                    && *(*parser)
                        .m_groupConnector
                        .offset((*parser).m_prologState.level as isize)
                        == 0
                    && (*(*dtd).scaffold.offset(
                        *(*dtd)
                            .scaffIndex
                            .offset(((*dtd).scaffLevel - 1 as ::core::ffi::c_int) as isize)
                            as isize,
                    ))
                    .type_0 as ::core::ffi::c_uint
                        != crate::expat_h::XML_CTYPE_MIXED as ::core::ffi::c_int
                            as ::core::ffi::c_uint
                {
                    (*(*dtd).scaffold.offset(
                        *(*dtd)
                            .scaffIndex
                            .offset(((*dtd).scaffLevel - 1 as ::core::ffi::c_int) as isize)
                            as isize,
                    ))
                    .type_0 = crate::expat_h::XML_CTYPE_CHOICE;
                    if (*parser).m_elementDeclHandler.is_some() {
                        handleDefault = crate::expat_h::XML_FALSE;
                    }
                }
                *(*parser)
                    .m_groupConnector
                    .offset((*parser).m_prologState.level as isize) =
                    crate::ascii_h::ASCII_PIPE as ::core::ffi::c_char;
                c2rust_current_block = 8258632986558375165;
            }
            60 | 59 => {
                (*dtd).hasParamEntityRefs = crate::expat_h::XML_TRUE;
                if (*parser).m_paramEntityParsing as u64 == 0 {
                    (*dtd).keepProcessing = (*dtd).standalone;
                    c2rust_current_block = 16953886395775657100;
                } else {
                    let mut name_1: *const crate::expat_external_h::XML_Char =
                        ::core::ptr::null::<crate::expat_external_h::XML_Char>();
                    let mut entity_1: *mut ENTITY = ::core::ptr::null_mut::<ENTITY>();
                    name_1 = poolStoreString(
                        &raw mut (*dtd).pool,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if name_1.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    entity_1 = lookup(
                        parser,
                        &raw mut (*dtd).paramEntities,
                        name_1 as KEY,
                        0 as crate::__stddef_size_t_h::size_t,
                    ) as *mut ENTITY;
                    (*dtd).pool.ptr = (*dtd).pool.start;
                    if (*parser).m_prologState.documentEntity != 0
                        && (if (*dtd).standalone as ::core::ffi::c_int != 0 {
                            (*parser).m_openInternalEntities.is_null() as ::core::ffi::c_int
                        } else {
                            ((*dtd).hasParamEntityRefs == 0) as ::core::ffi::c_int
                        }) != 0
                    {
                        if entity_1.is_null() {
                            return crate::expat_h::XML_ERROR_UNDEFINED_ENTITY;
                        } else if (*entity_1).is_internal == 0 {
                            return crate::expat_h::XML_ERROR_ENTITY_DECLARED_IN_PE;
                        }
                        c2rust_current_block = 11938645146649090955;
                    } else if entity_1.is_null() {
                        (*dtd).keepProcessing = (*dtd).standalone;
                        if role
                            == crate::src::xmlrole::XML_ROLE_PARAM_ENTITY_REF as ::core::ffi::c_int
                            && (*parser).m_skippedEntityHandler.is_some()
                        {
                            (*parser)
                                .m_skippedEntityHandler
                                .expect("non-null function pointer")(
                                (*parser).m_handlerArg,
                                name_1,
                                1 as ::core::ffi::c_int,
                            );
                            handleDefault = crate::expat_h::XML_FALSE;
                        }
                        c2rust_current_block = 8258632986558375165;
                    } else {
                        c2rust_current_block = 11938645146649090955;
                    }
                    match c2rust_current_block {
                        8258632986558375165 => {}
                        _ => {
                            if (*entity_1).open != 0 {
                                return crate::expat_h::XML_ERROR_RECURSIVE_ENTITY_REF;
                            }
                            if !(*entity_1).textPtr.is_null() {
                                let mut result_4: crate::expat_h::XML_Error =
                                    crate::expat_h::XML_ERROR_NONE;
                                let mut betweenDecl: crate::expat_h::XML_Bool = (if role
                                    == crate::src::xmlrole::XML_ROLE_PARAM_ENTITY_REF
                                        as ::core::ffi::c_int
                                {
                                    crate::expat_h::XML_TRUE as ::core::ffi::c_int
                                } else {
                                    crate::expat_h::XML_FALSE as ::core::ffi::c_int
                                })
                                    as crate::expat_h::XML_Bool;
                                result_4 =
                                    processEntity(parser, entity_1, betweenDecl, ENTITY_INTERNAL);
                                if result_4 as ::core::ffi::c_uint
                                    != crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int
                                        as ::core::ffi::c_uint
                                {
                                    return result_4;
                                }
                                handleDefault = crate::expat_h::XML_FALSE;
                                c2rust_current_block = 8258632986558375165;
                            } else if (*parser).m_externalEntityRefHandler.is_some() {
                                (*dtd).paramEntityRead = crate::expat_h::XML_FALSE;
                                (*entity_1).open = crate::expat_h::XML_TRUE;
                                let entity_root_parser = getRootParserOf(
                                    parser,
                                    ::core::ptr::null_mut::<::core::ffi::c_uint>(),
                                )
                                    as crate::expat_h::XML_Parser;
                                entityTrackingOnOpen(&mut *entity_root_parser);
                                entityTrackingReportStats(
                                    &*entity_root_parser,
                                    &*entity_1,
                                    ::std::ffi::CStr::from_ptr(
                                        (*entity_1).name as *const ::core::ffi::c_char,
                                    ),
                                    "OPEN ",
                                    6057 as ::core::ffi::c_int,
                                );
                                if (*parser)
                                    .m_externalEntityRefHandler
                                    .expect("non-null function pointer")(
                                    (*parser).m_externalEntityRefHandlerArg,
                                    ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
                                    (*entity_1).base,
                                    (*entity_1).systemId,
                                    (*entity_1).publicId,
                                ) == 0
                                {
                                    entityTrackingReportStats(
                                        &*entity_root_parser,
                                        &*entity_1,
                                        ::std::ffi::CStr::from_ptr(
                                            (*entity_1).name as *const ::core::ffi::c_char,
                                        ),
                                        "CLOSE",
                                        6061 as ::core::ffi::c_int,
                                    );
                                    entityTrackingOnClose(&mut *entity_root_parser);
                                    (*entity_1).open = crate::expat_h::XML_FALSE;
                                    return crate::expat_h::XML_ERROR_EXTERNAL_ENTITY_HANDLING;
                                }
                                entityTrackingReportStats(
                                    &*entity_root_parser,
                                    &*entity_1,
                                    ::std::ffi::CStr::from_ptr(
                                        (*entity_1).name as *const ::core::ffi::c_char,
                                    ),
                                    "CLOSE",
                                    6065 as ::core::ffi::c_int,
                                );
                                entityTrackingOnClose(&mut *entity_root_parser);
                                (*entity_1).open = crate::expat_h::XML_FALSE;
                                handleDefault = crate::expat_h::XML_FALSE;
                                if (*dtd).paramEntityRead == 0 {
                                    (*dtd).keepProcessing = (*dtd).standalone;
                                    c2rust_current_block = 8258632986558375165;
                                } else {
                                    c2rust_current_block = 16953886395775657100;
                                }
                            } else {
                                (*dtd).keepProcessing = (*dtd).standalone;
                                c2rust_current_block = 8258632986558375165;
                            }
                        }
                    }
                }
                match c2rust_current_block {
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
                            return crate::expat_h::XML_ERROR_NOT_STANDALONE;
                        }
                        c2rust_current_block = 8258632986558375165;
                    }
                }
            }
            40 => {
                if (*parser).m_elementDeclHandler.is_some() {
                    (*parser).m_declElementType = getElementType(parser, enc, s, next);
                    if (*parser).m_declElementType.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    (*dtd).scaffLevel = 0 as ::core::ffi::c_int;
                    (*dtd).scaffCount = 0 as ::core::ffi::c_uint;
                    (*dtd).in_eldecl = crate::expat_h::XML_TRUE;
                    handleDefault = crate::expat_h::XML_FALSE;
                }
                c2rust_current_block = 8258632986558375165;
            }
            41 | 42 => {
                if (*dtd).in_eldecl != 0 {
                    if (*parser).m_elementDeclHandler.is_some() {
                        let mut content: *mut crate::expat_h::XML_Content = (*parser)
                            .m_mem
                            .malloc_fcn
                            .expect("non-null function pointer")(
                            ::core::mem::size_of::<crate::expat_h::XML_Content>()
                                as crate::__stddef_size_t_h::size_t,
                        )
                            as *mut crate::expat_h::XML_Content;
                        if content.is_null() {
                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                        }
                        (*content).quant = crate::expat_h::XML_CQUANT_NONE;
                        (*content).name =
                            ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
                        (*content).numchildren = 0 as ::core::ffi::c_uint;
                        (*content).children =
                            ::core::ptr::null_mut::<crate::expat_h::XML_Content>();
                        (*content).type_0 = (if role
                            == crate::src::xmlrole::XML_ROLE_CONTENT_ANY as ::core::ffi::c_int
                        {
                            crate::expat_h::XML_CTYPE_ANY as ::core::ffi::c_int
                        } else {
                            crate::expat_h::XML_CTYPE_EMPTY as ::core::ffi::c_int
                        })
                            as crate::expat_h::XML_Content_Type;
                        *eventEndPP = s;
                        (*parser)
                            .m_elementDeclHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            (*(*parser).m_declElementType).name,
                            content,
                        );
                        handleDefault = crate::expat_h::XML_FALSE;
                    }
                    (*dtd).in_eldecl = crate::expat_h::XML_FALSE;
                }
                c2rust_current_block = 8258632986558375165;
            }
            43 => {
                if (*dtd).in_eldecl != 0 {
                    (*(*dtd).scaffold.offset(
                        *(*dtd)
                            .scaffIndex
                            .offset(((*dtd).scaffLevel - 1 as ::core::ffi::c_int) as isize)
                            as isize,
                    ))
                    .type_0 = crate::expat_h::XML_CTYPE_MIXED;
                    if (*parser).m_elementDeclHandler.is_some() {
                        handleDefault = crate::expat_h::XML_FALSE;
                    }
                }
                c2rust_current_block = 8258632986558375165;
            }
            51 => {
                quant = crate::expat_h::XML_CQUANT_NONE;
                c2rust_current_block = 9506620664607974954;
            }
            53 => {
                quant = crate::expat_h::XML_CQUANT_OPT;
                c2rust_current_block = 9506620664607974954;
            }
            52 => {
                quant = crate::expat_h::XML_CQUANT_REP;
                c2rust_current_block = 9506620664607974954;
            }
            54 => {
                quant = crate::expat_h::XML_CQUANT_PLUS;
                c2rust_current_block = 9506620664607974954;
            }
            45 => {
                quant = crate::expat_h::XML_CQUANT_NONE;
                c2rust_current_block = 2837971202649219995;
            }
            47 => {
                quant = crate::expat_h::XML_CQUANT_OPT;
                c2rust_current_block = 2837971202649219995;
            }
            46 => {
                quant = crate::expat_h::XML_CQUANT_REP;
                c2rust_current_block = 2837971202649219995;
            }
            48 => {
                quant = crate::expat_h::XML_CQUANT_PLUS;
                c2rust_current_block = 2837971202649219995;
            }
            55 => {
                if reportProcessingInstruction(parser, enc, s, next) == 0 {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
                handleDefault = crate::expat_h::XML_FALSE;
                c2rust_current_block = 8258632986558375165;
            }
            56 => {
                if reportComment(parser, enc, s, next) == 0 {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
                handleDefault = crate::expat_h::XML_FALSE;
                c2rust_current_block = 8258632986558375165;
            }
            0 => {
                match tok {
                    crate::src::xmltok::XML_TOK_BOM => {
                        handleDefault = crate::expat_h::XML_FALSE;
                    }
                    _ => {}
                }
                c2rust_current_block = 8258632986558375165;
            }
            3 => {
                if (*parser).m_startDoctypeDeclHandler.is_some() {
                    handleDefault = crate::expat_h::XML_FALSE;
                }
                c2rust_current_block = 8258632986558375165;
            }
            11 => {
                if (*dtd).keepProcessing as ::core::ffi::c_int != 0
                    && (*parser).m_entityDeclHandler.is_some()
                {
                    handleDefault = crate::expat_h::XML_FALSE;
                }
                c2rust_current_block = 8258632986558375165;
            }
            17 => {
                if (*parser).m_notationDeclHandler.is_some() {
                    handleDefault = crate::expat_h::XML_FALSE;
                }
                c2rust_current_block = 8258632986558375165;
            }
            33 => {
                if (*dtd).keepProcessing as ::core::ffi::c_int != 0
                    && (*parser).m_attlistDeclHandler.is_some()
                {
                    handleDefault = crate::expat_h::XML_FALSE;
                }
                c2rust_current_block = 8258632986558375165;
            }
            39 => {
                if (*parser).m_elementDeclHandler.is_some() {
                    handleDefault = crate::expat_h::XML_FALSE;
                }
                c2rust_current_block = 8258632986558375165;
            }
            _ => {
                c2rust_current_block = 8258632986558375165;
            }
        }
        match c2rust_current_block {
            3681359897139369850 => {
                if (*enc).isPublicId.expect("non-null function pointer")(enc, s, next, eventPP) == 0
                {
                    return crate::expat_h::XML_ERROR_PUBLICID;
                }
                c2rust_current_block = 14769507584970373587;
            }
            16790621803724920579 => {
                if (*dtd).keepProcessing as ::core::ffi::c_int != 0
                    && !(*parser).m_declEntity.is_null()
                {
                    (*(*parser).m_declEntity).systemId = poolStoreString(
                        &raw mut (*dtd).pool,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if (*(*parser).m_declEntity).systemId.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    (*(*parser).m_declEntity).base = (*parser).m_curBase;
                    (*dtd).pool.start = (*dtd).pool.ptr;
                    if (*parser).m_entityDeclHandler.is_some()
                        && role
                            == crate::src::xmlrole::XML_ROLE_ENTITY_SYSTEM_ID as ::core::ffi::c_int
                    {
                        handleDefault = crate::expat_h::XML_FALSE;
                    }
                }
                c2rust_current_block = 8258632986558375165;
            }
            11852364650731333344 => {
                if (*dtd).keepProcessing as ::core::ffi::c_int != 0
                    && (*parser).m_attlistDeclHandler.is_some()
                {
                    handleDefault = crate::expat_h::XML_FALSE;
                }
                c2rust_current_block = 8258632986558375165;
            }
            9506620664607974954 => {
                if (*dtd).in_eldecl != 0 {
                    let mut el: *mut ELEMENT_TYPE = ::core::ptr::null_mut::<ELEMENT_TYPE>();
                    let mut name_2: *const crate::expat_external_h::XML_Char =
                        ::core::ptr::null::<crate::expat_external_h::XML_Char>();
                    let mut nameLen: crate::__stddef_size_t_h::size_t = 0;
                    let mut nxt: *const ::core::ffi::c_char = if quant as ::core::ffi::c_uint
                        == crate::expat_h::XML_CQUANT_NONE as ::core::ffi::c_int
                            as ::core::ffi::c_uint
                    {
                        next
                    } else {
                        next.offset(-((*enc).minBytesPerChar as isize))
                    };
                    let mut myindex_0: ::core::ffi::c_int = nextScaffoldPart(parser);
                    if myindex_0 < 0 as ::core::ffi::c_int {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    (*(*dtd).scaffold.offset(myindex_0 as isize)).type_0 =
                        crate::expat_h::XML_CTYPE_NAME;
                    (*(*dtd).scaffold.offset(myindex_0 as isize)).quant = quant;
                    el = getElementType(parser, enc, s, nxt);
                    if el.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    name_2 = (*el).name;
                    let ref mut c2rust_fresh5 = (*(*dtd).scaffold.offset(myindex_0 as isize)).name;
                    *c2rust_fresh5 = name_2;
                    nameLen = 0 as crate::__stddef_size_t_h::size_t;
                    loop {
                        let c2rust_fresh6 = nameLen;
                        nameLen = nameLen.wrapping_add(1);
                        if !(*name_2.offset(c2rust_fresh6 as isize) != 0) {
                            break;
                        }
                    }
                    if nameLen
                        > crate::limits_h::UINT_MAX.wrapping_sub((*dtd).contentStringLen)
                            as crate::__stddef_size_t_h::size_t
                    {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    (*dtd).contentStringLen = (*dtd)
                        .contentStringLen
                        .wrapping_add(nameLen as ::core::ffi::c_uint);
                    if (*parser).m_elementDeclHandler.is_some() {
                        handleDefault = crate::expat_h::XML_FALSE;
                    }
                }
                c2rust_current_block = 8258632986558375165;
            }
            2837971202649219995 => {
                if (*dtd).in_eldecl != 0 {
                    if (*parser).m_elementDeclHandler.is_some() {
                        handleDefault = crate::expat_h::XML_FALSE;
                    }
                    (*dtd).scaffLevel -= 1;
                    (*(*dtd)
                        .scaffold
                        .offset(*(*dtd).scaffIndex.offset((*dtd).scaffLevel as isize) as isize))
                    .quant = quant;
                    if (*dtd).scaffLevel == 0 as ::core::ffi::c_int {
                        if handleDefault == 0 {
                            let mut model: *mut crate::expat_h::XML_Content = build_model(parser);
                            if model.is_null() {
                                return crate::expat_h::XML_ERROR_NO_MEMORY;
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
                        (*dtd).in_eldecl = crate::expat_h::XML_FALSE;
                        (*dtd).contentStringLen = 0 as ::core::ffi::c_uint;
                    }
                }
                c2rust_current_block = 8258632986558375165;
            }
            _ => {}
        }
        match c2rust_current_block {
            14769507584970373587 => {
                if (*dtd).keepProcessing as ::core::ffi::c_int != 0
                    && !(*parser).m_declEntity.is_null()
                {
                    let mut tem: *mut crate::expat_external_h::XML_Char = poolStoreString(
                        &raw mut (*dtd).pool,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if tem.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    let public_id_len = ::std::ffi::CStr::from_ptr(tem).to_bytes_with_nul().len();
                    normalizePublicId(::core::slice::from_raw_parts_mut(tem, public_id_len));
                    (*(*parser).m_declEntity).publicId = tem;
                    (*dtd).pool.start = (*dtd).pool.ptr;
                    if (*parser).m_entityDeclHandler.is_some()
                        && role
                            == crate::src::xmlrole::XML_ROLE_ENTITY_PUBLIC_ID as ::core::ffi::c_int
                    {
                        handleDefault = crate::expat_h::XML_FALSE;
                    }
                }
            }
            _ => {}
        }
        if handleDefault as ::core::ffi::c_int != 0 && (*parser).m_defaultHandler.is_some() {
            reportDefault(parser, enc, s, next);
        }
        match (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint {
            3 => {
                *nextPtr = next;
                return crate::expat_h::XML_ERROR_NONE;
            }
            2 => return crate::expat_h::XML_ERROR_ABORTED,
            1 => {
                if (*parser).m_reenter != 0 {
                    *nextPtr = next;
                    return crate::expat_h::XML_ERROR_NONE;
                }
            }
            _ => {}
        }
        s = next;
        tok = (*enc).scanners[0 as ::core::ffi::c_int as usize].expect("non-null function pointer")(
            enc,
            s,
            end,
            &raw mut next,
        );
    }
}

unsafe extern "C" fn epilogProcessor(
    mut parser: crate::expat_h::XML_Parser,
    mut s: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextPtr: *mut *const ::core::ffi::c_char,
) -> crate::expat_h::XML_Error {
    (*parser).m_processor = Some(
        epilogProcessor
            as unsafe extern "C" fn(
                crate::expat_h::XML_Parser,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut *const ::core::ffi::c_char,
            ) -> crate::expat_h::XML_Error,
    );
    (*parser).m_eventPtr = s;
    loop {
        let mut next: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut tok: ::core::ffi::c_int = (*(*parser).m_encoding).scanners
            [0 as ::core::ffi::c_int as usize]
            .expect("non-null function pointer")(
            (*parser).m_encoding, s, end, &raw mut next
        );
        let mut accounting_levels: ::core::ffi::c_uint = 0;
        let accounting_root =
            getRootParserOf(parser, &raw mut accounting_levels) as crate::expat_h::XML_Parser;
        if accountingDiffTolerated(
            &mut *accounting_root,
            accounting_levels,
            parser == accounting_root,
            tok,
            || {
                ::core::slice::from_raw_parts(
                    s as *const ::core::ffi::c_uchar,
                    byte_offset(next, s) as usize,
                )
            },
            6279 as ::core::ffi::c_int,
            XML_ACCOUNT_DIRECT,
        ) == 0
        {
            accountingReportStats(&*accounting_root, ACCOUNTING_ABORTING_EPILOG);
            return crate::expat_h::XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
        }
        (*parser).m_eventEndPtr = next;
        match tok {
            -15 => {
                if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, (*parser).m_encoding, s, next);
                    if (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint
                        == crate::expat_h::XML_FINISHED as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        return crate::expat_h::XML_ERROR_ABORTED;
                    }
                }
                *nextPtr = next;
                return crate::expat_h::XML_ERROR_NONE;
            }
            crate::src::xmltok::XML_TOK_NONE => {
                *nextPtr = s;
                return crate::expat_h::XML_ERROR_NONE;
            }
            crate::src::xmltok::XML_TOK_PROLOG_S => {
                if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, (*parser).m_encoding, s, next);
                }
            }
            crate::src::xmltok::XML_TOK_PI => {
                if reportProcessingInstruction(parser, (*parser).m_encoding, s, next) == 0 {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
            }
            crate::src::xmltok::XML_TOK_COMMENT => {
                if reportComment(parser, (*parser).m_encoding, s, next) == 0 {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
            }
            crate::src::xmltok::XML_TOK_INVALID => {
                (*parser).m_eventPtr = next;
                return crate::expat_h::XML_ERROR_INVALID_TOKEN;
            }
            crate::src::xmltok::XML_TOK_PARTIAL => {
                if (*parser).m_parsingStatus.finalBuffer == 0 {
                    *nextPtr = s;
                    return crate::expat_h::XML_ERROR_NONE;
                }
                return crate::expat_h::XML_ERROR_UNCLOSED_TOKEN;
            }
            crate::src::xmltok::XML_TOK_PARTIAL_CHAR => {
                if (*parser).m_parsingStatus.finalBuffer == 0 {
                    *nextPtr = s;
                    return crate::expat_h::XML_ERROR_NONE;
                }
                return crate::expat_h::XML_ERROR_PARTIAL_CHAR;
            }
            _ => return crate::expat_h::XML_ERROR_JUNK_AFTER_DOC_ELEMENT,
        }
        match (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint {
            3 => {
                (*parser).m_eventPtr = next;
                *nextPtr = next;
                return crate::expat_h::XML_ERROR_NONE;
            }
            2 => {
                (*parser).m_eventPtr = next;
                return crate::expat_h::XML_ERROR_ABORTED;
            }
            1 => {
                if (*parser).m_reenter != 0 {
                    return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
                }
            }
            _ => {}
        }
        s = next;
        (*parser).m_eventPtr = s;
    }
}

unsafe extern "C" fn processEntity(
    mut parser: crate::expat_h::XML_Parser,
    mut entity: *mut ENTITY,
    mut betweenDecl: crate::expat_h::XML_Bool,
    mut type_0: EntityType,
) -> crate::expat_h::XML_Error {
    let mut openEntity: *mut OPEN_INTERNAL_ENTITY = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
    let mut openEntityList: *mut *mut OPEN_INTERNAL_ENTITY =
        ::core::ptr::null_mut::<*mut OPEN_INTERNAL_ENTITY>();
    let mut freeEntityList: *mut *mut OPEN_INTERNAL_ENTITY =
        ::core::ptr::null_mut::<*mut OPEN_INTERNAL_ENTITY>();
    match type_0 as ::core::ffi::c_uint {
        0 => {
            (*parser).m_processor = Some(
                internalEntityProcessor
                    as unsafe extern "C" fn(
                        crate::expat_h::XML_Parser,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> crate::expat_h::XML_Error,
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
            '_c2rust_label: {
                crate::stdlib::__assert_fail(
                    b"0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../../expat/lib/xmlparse.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    6374 as ::core::ffi::c_uint,
                    b"enum XML_Error processEntity(XML_Parser, ENTITY *, XML_Bool, enum EntityType)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
        }
    }
    if !(*freeEntityList).is_null() {
        openEntity = *freeEntityList;
        *freeEntityList = (*openEntity).next as *mut OPEN_INTERNAL_ENTITY;
    } else {
        openEntity = expat_malloc(
            parser,
            ::core::mem::size_of::<OPEN_INTERNAL_ENTITY>() as crate::__stddef_size_t_h::size_t,
            6382 as ::core::ffi::c_int,
        ) as *mut OPEN_INTERNAL_ENTITY;
        if openEntity.is_null() {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
    }
    (*entity).open = crate::expat_h::XML_TRUE;
    (*entity).hasMore = crate::expat_h::XML_TRUE;
    let entity_root_parser = getRootParserOf(parser, ::core::ptr::null_mut::<::core::ffi::c_uint>())
        as crate::expat_h::XML_Parser;
    entityTrackingOnOpen(&mut *entity_root_parser);
    entityTrackingReportStats(
        &*entity_root_parser,
        &*entity,
        ::std::ffi::CStr::from_ptr((*entity).name as *const ::core::ffi::c_char),
        "OPEN ",
        6389 as ::core::ffi::c_int,
    );
    (*entity).processed = 0 as ::core::ffi::c_int;
    (*openEntity).next = *openEntityList as *mut open_internal_entity;
    *openEntityList = openEntity;
    (*openEntity).entity = entity;
    (*openEntity).type_0 = type_0;
    (*openEntity).startTagLevel = (*parser).m_tagLevel;
    (*openEntity).betweenDecl = betweenDecl;
    (*openEntity).internalEventPtr = ::core::ptr::null::<::core::ffi::c_char>();
    (*openEntity).internalEventEndPtr = ::core::ptr::null::<::core::ffi::c_char>();
    if type_0 as ::core::ffi::c_uint == ENTITY_INTERNAL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        triggerReenter(&mut *parser);
    }
    return crate::expat_h::XML_ERROR_NONE;
}

unsafe extern "C" fn internalEntityProcessor(
    mut parser: crate::expat_h::XML_Parser,
    mut s: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextPtr: *mut *const ::core::ffi::c_char,
) -> crate::expat_h::XML_Error {
    let mut entity: *mut ENTITY = ::core::ptr::null_mut::<ENTITY>();
    let mut textStart: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut textEnd: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut next: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut result: crate::expat_h::XML_Error = crate::expat_h::XML_ERROR_NONE;
    let mut openEntity: *mut OPEN_INTERNAL_ENTITY = (*parser).m_openInternalEntities;
    if openEntity.is_null() {
        return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
    }
    entity = (*openEntity).entity;
    if (*entity).hasMore != 0 {
        textStart =
            ((*entity).textPtr as *const ::core::ffi::c_char).offset((*entity).processed as isize);
        textEnd =
            (*entity).textPtr.offset((*entity).textLen as isize) as *const ::core::ffi::c_char;
        next = textStart;
        if (*entity).is_param != 0 {
            let mut tok: ::core::ffi::c_int = (*(*parser).m_internalEncoding).scanners
                [0 as ::core::ffi::c_int as usize]
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
                crate::expat_h::XML_FALSE,
                crate::expat_h::XML_FALSE,
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
                crate::expat_h::XML_FALSE,
                XML_ACCOUNT_ENTITY_EXPANSION,
            );
        }
        if result as ::core::ffi::c_uint
            != crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return result;
        }
        if textEnd != next
            && ((*parser).m_parsingStatus.parsing as ::core::ffi::c_uint
                == crate::expat_h::XML_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint
                    == crate::expat_h::XML_PARSING as ::core::ffi::c_int as ::core::ffi::c_uint
                    && (*parser).m_reenter as ::core::ffi::c_int != 0)
        {
            (*entity).processed = next.offset_from((*entity).textPtr as *const ::core::ffi::c_char)
                as ::core::ffi::c_long as ::core::ffi::c_int;
            return result;
        }
        (*entity).hasMore = crate::expat_h::XML_FALSE;
        if (*entity).is_param == 0 && (*openEntity).startTagLevel != (*parser).m_tagLevel {
            return crate::expat_h::XML_ERROR_ASYNC_ENTITY;
        }
        triggerReenter(&mut *parser);
        return result;
    }
    let entity_root_parser = getRootParserOf(parser, ::core::ptr::null_mut::<::core::ffi::c_uint>())
        as crate::expat_h::XML_Parser;
    entityTrackingReportStats(
        &*entity_root_parser,
        &*entity,
        ::std::ffi::CStr::from_ptr((*entity).name as *const ::core::ffi::c_char),
        "CLOSE",
        6470 as ::core::ffi::c_int,
    );
    entityTrackingOnClose(&mut *entity_root_parser);
    '_c2rust_label: {
        if (*parser).m_openInternalEntities == openEntity {
        } else {
            crate::stdlib::__assert_fail(
                b"parser->m_openInternalEntities == openEntity\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../../expat/lib/xmlparse.c\0".as_ptr() as *const ::core::ffi::c_char,
                6476 as ::core::ffi::c_uint,
                b"enum XML_Error internalEntityProcessor(XML_Parser, const char *, const char *, const char **)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    (*entity).open = crate::expat_h::XML_FALSE;
    (*parser).m_openInternalEntities =
        (*(*parser).m_openInternalEntities).next as *mut OPEN_INTERNAL_ENTITY;
    (*openEntity).next = (*parser).m_freeInternalEntities as *mut open_internal_entity;
    (*parser).m_freeInternalEntities = openEntity;
    if (*parser).m_openInternalEntities.is_null() {
        (*parser).m_processor = if (*entity).is_param as ::core::ffi::c_int != 0 {
            Some(
                prologProcessor
                    as unsafe extern "C" fn(
                        crate::expat_h::XML_Parser,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> crate::expat_h::XML_Error,
            )
        } else {
            Some(
                contentProcessor
                    as unsafe extern "C" fn(
                        crate::expat_h::XML_Parser,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> crate::expat_h::XML_Error,
            )
        };
    }
    triggerReenter(&mut *parser);
    return crate::expat_h::XML_ERROR_NONE;
}

unsafe extern "C" fn errorProcessor(
    mut parser: crate::expat_h::XML_Parser,
    mut s: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextPtr: *mut *const ::core::ffi::c_char,
) -> crate::expat_h::XML_Error {
    return (*parser).m_errorCode;
}

unsafe extern "C" fn storeAttributeValue(
    mut parser: crate::expat_h::XML_Parser,
    mut enc: *const crate::src::xmltok::ENCODING,
    mut isCdata: crate::expat_h::XML_Bool,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut pool: *mut STRING_POOL,
    mut account: XML_Account,
) -> crate::expat_h::XML_Error {
    let mut next: *const ::core::ffi::c_char = ptr;
    let mut result: crate::expat_h::XML_Error = crate::expat_h::XML_ERROR_NONE;
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
                return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
            }
            let entity: *mut ENTITY = (*openEntity).entity;
            let textStart: *const ::core::ffi::c_char = ((*entity).textPtr
                as *const ::core::ffi::c_char)
                .offset((*entity).processed as isize);
            let textEnd: *const ::core::ffi::c_char =
                (*entity).textPtr.offset((*entity).textLen as isize) as *const ::core::ffi::c_char;
            let mut nextInEntity: *const ::core::ffi::c_char = textStart;
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
                if result as ::core::ffi::c_uint
                    != crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    break;
                }
                if textEnd != nextInEntity {
                    (*entity).processed =
                        nextInEntity.offset_from((*entity).textPtr as *const ::core::ffi::c_char)
                            as ::core::ffi::c_long as ::core::ffi::c_int;
                    continue;
                } else {
                    (*entity).hasMore = crate::expat_h::XML_FALSE;
                    continue;
                }
            } else {
                let entity_root_parser =
                    getRootParserOf(parser, ::core::ptr::null_mut::<::core::ffi::c_uint>())
                        as crate::expat_h::XML_Parser;
                entityTrackingReportStats(
                    &*entity_root_parser,
                    &*entity,
                    ::std::ffi::CStr::from_ptr((*entity).name as *const ::core::ffi::c_char),
                    "CLOSE",
                    6547 as ::core::ffi::c_int,
                );
                entityTrackingOnClose(&mut *entity_root_parser);
                '_c2rust_label: {
                    if (*parser).m_openAttributeEntities == openEntity {
                    } else {
                        crate::stdlib::__assert_fail(
                            b"parser->m_openAttributeEntities == openEntity\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"../../expat/lib/xmlparse.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            6553 as ::core::ffi::c_uint,
                            b"enum XML_Error storeAttributeValue(XML_Parser, const ENCODING *, XML_Bool, const char *, const char *, STRING_POOL *, enum XML_Account)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                };
                (*entity).open = crate::expat_h::XML_FALSE;
                (*parser).m_openAttributeEntities =
                    (*(*parser).m_openAttributeEntities).next as *mut OPEN_INTERNAL_ENTITY;
                (*openEntity).next = (*parser).m_freeAttributeEntities as *mut open_internal_entity;
                (*parser).m_freeAttributeEntities = openEntity;
            }
        }
        if result as ::core::ffi::c_uint != 0
            || (*parser).m_openAttributeEntities.is_null() && end == next
        {
            break;
        }
    }
    if result as u64 != 0 {
        return result;
    }
    if isCdata == 0
        && (*pool).ptr.offset_from((*pool).start) as ::core::ffi::c_long != 0
        && *(*pool).ptr.offset(-1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x20 as ::core::ffi::c_int
    {
        (*pool).ptr = (*pool).ptr.offset(-1);
    }
    if if (*pool).ptr == (*pool).end as *mut crate::expat_external_h::XML_Char
        && poolGrow(pool) == 0
    {
        0 as ::core::ffi::c_int
    } else {
        let c2rust_fresh55 = (*pool).ptr;
        (*pool).ptr = (*pool).ptr.offset(1);
        *c2rust_fresh55 = '\0' as i32 as crate::expat_external_h::XML_Char;
        1 as ::core::ffi::c_int
    } == 0
    {
        return crate::expat_h::XML_ERROR_NO_MEMORY;
    }
    return crate::expat_h::XML_ERROR_NONE;
}

unsafe extern "C" fn appendAttributeValue(
    mut parser: crate::expat_h::XML_Parser,
    mut enc: *const crate::src::xmltok::ENCODING,
    mut isCdata: crate::expat_h::XML_Bool,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut pool: *mut STRING_POOL,
    mut account: XML_Account,
    mut nextPtr: *mut *const ::core::ffi::c_char,
) -> crate::expat_h::XML_Error {
    let dtd: *mut DTD = (*parser).m_dtd;
    loop {
        let mut next: *const ::core::ffi::c_char = ptr;
        let mut tok: ::core::ffi::c_int =
            (*enc).literalScanners[0 as ::core::ffi::c_int as usize]
                .expect("non-null function pointer")(enc, ptr, end, &raw mut next);
        let mut accounting_levels: ::core::ffi::c_uint = 0;
        let accounting_root =
            getRootParserOf(parser, &raw mut accounting_levels) as crate::expat_h::XML_Parser;
        if accountingDiffTolerated(
            &mut *accounting_root,
            accounting_levels,
            parser == accounting_root,
            tok,
            || {
                ::core::slice::from_raw_parts(
                    ptr as *const ::core::ffi::c_uchar,
                    byte_offset(next, ptr) as usize,
                )
            },
            6591 as ::core::ffi::c_int,
            account,
        ) == 0
        {
            accountingReportStats(&*accounting_root, ACCOUNTING_ABORTING_EPILOG);
            return crate::expat_h::XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
        }
        let mut c2rust_current_block_70: u64;
        match tok {
            crate::src::xmltok::XML_TOK_NONE => {
                if !nextPtr.is_null() {
                    *nextPtr = next;
                }
                return crate::expat_h::XML_ERROR_NONE;
            }
            crate::src::xmltok::XML_TOK_INVALID => {
                if enc == (*parser).m_encoding {
                    (*parser).m_eventPtr = next;
                }
                return crate::expat_h::XML_ERROR_INVALID_TOKEN;
            }
            crate::src::xmltok::XML_TOK_PARTIAL => {
                if enc == (*parser).m_encoding {
                    (*parser).m_eventPtr = ptr;
                }
                return crate::expat_h::XML_ERROR_INVALID_TOKEN;
            }
            crate::src::xmltok::XML_TOK_CHAR_REF => {
                let mut buf: [crate::expat_external_h::XML_Char; 4] = [0; 4];
                let mut i: ::core::ffi::c_int = 0;
                let mut n: ::core::ffi::c_int =
                    (*enc).charRefNumber.expect("non-null function pointer")(enc, ptr);
                if n < 0 as ::core::ffi::c_int {
                    if enc == (*parser).m_encoding {
                        (*parser).m_eventPtr = ptr;
                    }
                    return crate::expat_h::XML_ERROR_BAD_CHAR_REF;
                }
                if isCdata == 0
                    && n == 0x20 as ::core::ffi::c_int
                    && ((*pool).ptr.offset_from((*pool).start) as ::core::ffi::c_long
                        == 0 as ::core::ffi::c_long
                        || *(*pool).ptr.offset(-1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == 0x20 as ::core::ffi::c_int)
                {
                    c2rust_current_block_70 = 18038362259723567392;
                } else {
                    n = crate::src::xmltok::XmlUtf8Encode(n, &mut buf);
                    i = 0 as ::core::ffi::c_int;
                    while i < n {
                        if if (*pool).ptr == (*pool).end as *mut crate::expat_external_h::XML_Char
                            && poolGrow(pool) == 0
                        {
                            0 as ::core::ffi::c_int
                        } else {
                            let c2rust_fresh56 = (*pool).ptr;
                            (*pool).ptr = (*pool).ptr.offset(1);
                            *c2rust_fresh56 = buf[i as usize];
                            1 as ::core::ffi::c_int
                        } == 0
                        {
                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                        }
                        i += 1;
                    }
                    c2rust_current_block_70 = 18038362259723567392;
                }
            }
            crate::src::xmltok::XML_TOK_DATA_CHARS => {
                if poolAppend(pool, enc, ptr, next).is_null() {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
                c2rust_current_block_70 = 18038362259723567392;
            }
            crate::src::xmltok::XML_TOK_TRAILING_CR => {
                next = ptr.offset((*enc).minBytesPerChar as isize);
                c2rust_current_block_70 = 2280072329198493674;
            }
            crate::src::xmltok::XML_TOK_ATTRIBUTE_VALUE_S
            | crate::src::xmltok::XML_TOK_DATA_NEWLINE => {
                c2rust_current_block_70 = 2280072329198493674;
            }
            crate::src::xmltok::XML_TOK_ENTITY_REF => {
                let mut name: *const crate::expat_external_h::XML_Char =
                    ::core::ptr::null::<crate::expat_external_h::XML_Char>();
                let mut entity: *mut ENTITY = ::core::ptr::null_mut::<ENTITY>();
                let mut checkEntityDecl: bool = false;
                let mut ch: crate::expat_external_h::XML_Char = (*enc)
                    .predefinedEntityName
                    .expect("non-null function pointer")(
                    enc,
                    ptr.offset((*enc).minBytesPerChar as isize),
                    next.offset(-((*enc).minBytesPerChar as isize)),
                )
                    as crate::expat_external_h::XML_Char;
                if ch != 0 {
                    let mut accounting_levels: ::core::ffi::c_uint = 0;
                    let accounting_root = getRootParserOf(parser, &raw mut accounting_levels)
                        as crate::expat_h::XML_Parser;
                    let ch_start = &raw mut ch as *mut ::core::ffi::c_char;
                    let ch_end = ch_start.offset(::core::mem::size_of::<
                        crate::expat_external_h::XML_Char,
                    >() as usize as isize);
                    accountingDiffTolerated(
                        &mut *accounting_root,
                        accounting_levels,
                        parser == accounting_root,
                        tok,
                        || {
                            ::core::slice::from_raw_parts(
                                ch_start as *const ::core::ffi::c_uchar,
                                byte_offset(ch_end, ch_start) as usize,
                            )
                        },
                        6663 as ::core::ffi::c_int,
                        XML_ACCOUNT_ENTITY_EXPANSION,
                    );
                    if if (*pool).ptr == (*pool).end as *mut crate::expat_external_h::XML_Char
                        && poolGrow(pool) == 0
                    {
                        0 as ::core::ffi::c_int
                    } else {
                        let c2rust_fresh58 = (*pool).ptr;
                        (*pool).ptr = (*pool).ptr.offset(1);
                        *c2rust_fresh58 = ch;
                        1 as ::core::ffi::c_int
                    } == 0
                    {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                } else {
                    name = poolStoreString(
                        &raw mut (*parser).m_temp2Pool,
                        enc,
                        ptr.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if name.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    entity = lookup(
                        parser,
                        &raw mut (*dtd).generalEntities,
                        name as KEY,
                        0 as crate::__stddef_size_t_h::size_t,
                    ) as *mut ENTITY;
                    (*parser).m_temp2Pool.ptr = (*parser).m_temp2Pool.start;
                    if pool == &raw mut (*dtd).pool {
                        checkEntityDecl = (*parser).m_prologState.documentEntity != 0
                            && (if (*dtd).standalone as ::core::ffi::c_int != 0 {
                                (*parser).m_openInternalEntities.is_null() as ::core::ffi::c_int
                            } else {
                                ((*dtd).hasParamEntityRefs == 0) as ::core::ffi::c_int
                            }) != 0;
                    } else {
                        checkEntityDecl = (*dtd).hasParamEntityRefs == 0
                            || (*dtd).standalone as ::core::ffi::c_int != 0;
                    }
                    if checkEntityDecl {
                        if entity.is_null() {
                            return crate::expat_h::XML_ERROR_UNDEFINED_ENTITY;
                        } else if (*entity).is_internal == 0 {
                            return crate::expat_h::XML_ERROR_ENTITY_DECLARED_IN_PE;
                        }
                        c2rust_current_block_70 = 13678349939556791712;
                    } else if entity.is_null() {
                        c2rust_current_block_70 = 18038362259723567392;
                    } else {
                        c2rust_current_block_70 = 13678349939556791712;
                    }
                    match c2rust_current_block_70 {
                        18038362259723567392 => {}
                        _ => {
                            if (*entity).open != 0 {
                                if enc == (*parser).m_encoding {
                                    (*parser).m_eventPtr = ptr;
                                }
                                return crate::expat_h::XML_ERROR_RECURSIVE_ENTITY_REF;
                            }
                            if !(*entity).notation.is_null() {
                                if enc == (*parser).m_encoding {
                                    (*parser).m_eventPtr = ptr;
                                }
                                return crate::expat_h::XML_ERROR_BINARY_ENTITY_REF;
                            }
                            if (*entity).textPtr.is_null() {
                                if enc == (*parser).m_encoding {
                                    (*parser).m_eventPtr = ptr;
                                }
                                return crate::expat_h::XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF;
                            } else {
                                let mut result: crate::expat_h::XML_Error =
                                    crate::expat_h::XML_ERROR_NONE;
                                result = processEntity(
                                    parser,
                                    entity,
                                    crate::expat_h::XML_FALSE,
                                    ENTITY_ATTRIBUTE,
                                );
                                if result as ::core::ffi::c_uint
                                    == crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int
                                        as ::core::ffi::c_uint
                                    && !nextPtr.is_null()
                                {
                                    *nextPtr = next;
                                }
                                return result;
                            }
                        }
                    }
                }
                c2rust_current_block_70 = 18038362259723567392;
            }
            _ => {
                if enc == (*parser).m_encoding {
                    (*parser).m_eventPtr = ptr;
                }
                return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
            }
        }
        match c2rust_current_block_70 {
            2280072329198493674 => {
                if !(isCdata == 0
                    && ((*pool).ptr.offset_from((*pool).start) as ::core::ffi::c_long
                        == 0 as ::core::ffi::c_long
                        || *(*pool).ptr.offset(-1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == 0x20 as ::core::ffi::c_int))
                {
                    if if (*pool).ptr == (*pool).end as *mut crate::expat_external_h::XML_Char
                        && poolGrow(pool) == 0
                    {
                        0 as ::core::ffi::c_int
                    } else {
                        let c2rust_fresh57 = (*pool).ptr;
                        (*pool).ptr = (*pool).ptr.offset(1);
                        *c2rust_fresh57 = 0x20 as crate::expat_external_h::XML_Char;
                        1 as ::core::ffi::c_int
                    } == 0
                    {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                }
            }
            _ => {}
        }
        ptr = next;
    }
}

unsafe extern "C" fn storeEntityValue(
    mut parser: crate::expat_h::XML_Parser,
    mut enc: *const crate::src::xmltok::ENCODING,
    mut entityTextPtr: *const ::core::ffi::c_char,
    mut entityTextEnd: *const ::core::ffi::c_char,
    mut account: XML_Account,
    mut nextPtr: *mut *const ::core::ffi::c_char,
) -> crate::expat_h::XML_Error {
    let mut c2rust_current_block: u64;
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut pool: *mut STRING_POOL = &raw mut (*dtd).entityValuePool;
    let mut result: crate::expat_h::XML_Error = crate::expat_h::XML_ERROR_NONE;
    let mut oldInEntityValue: ::core::ffi::c_int = (*parser).m_prologState.inEntityValue;
    (*parser).m_prologState.inEntityValue = 1 as ::core::ffi::c_int;
    if (*pool).blocks.is_null() {
        if poolGrow(pool) == 0 {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
    }
    let mut next: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    's_35: loop {
        next = entityTextPtr;
        let mut tok: ::core::ffi::c_int = (*enc).literalScanners[1 as ::core::ffi::c_int as usize]
            .expect("non-null function pointer")(
            enc, entityTextPtr, entityTextEnd, &raw mut next
        );
        let mut accounting_levels: ::core::ffi::c_uint = 0;
        let accounting_root =
            getRootParserOf(parser, &raw mut accounting_levels) as crate::expat_h::XML_Parser;
        if accountingDiffTolerated(
            &mut *accounting_root,
            accounting_levels,
            parser == accounting_root,
            tok,
            || {
                ::core::slice::from_raw_parts(
                    entityTextPtr as *const ::core::ffi::c_uchar,
                    byte_offset(next, entityTextPtr) as usize,
                )
            },
            6798 as ::core::ffi::c_int,
            account,
        ) == 0
        {
            accountingReportStats(&*accounting_root, ACCOUNTING_ABORTING_EPILOG);
            result = crate::expat_h::XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
            break;
        } else {
            match tok {
                crate::src::xmltok::XML_TOK_PARAM_ENTITY_REF => {
                    if (*parser).m_isParamEntity as ::core::ffi::c_int != 0
                        || enc != (*parser).m_encoding
                    {
                        let mut name: *const crate::expat_external_h::XML_Char =
                            ::core::ptr::null::<crate::expat_external_h::XML_Char>();
                        let mut entity: *mut ENTITY = ::core::ptr::null_mut::<ENTITY>();
                        name = poolStoreString(
                            &raw mut (*parser).m_tempPool,
                            enc,
                            entityTextPtr.offset((*enc).minBytesPerChar as isize),
                            next.offset(-((*enc).minBytesPerChar as isize)),
                        );
                        if name.is_null() {
                            result = crate::expat_h::XML_ERROR_NO_MEMORY;
                            break;
                        } else {
                            entity = lookup(
                                parser,
                                &raw mut (*dtd).paramEntities,
                                name as KEY,
                                0 as crate::__stddef_size_t_h::size_t,
                            ) as *mut ENTITY;
                            (*parser).m_tempPool.ptr = (*parser).m_tempPool.start;
                            if entity.is_null() {
                                (*dtd).keepProcessing = (*dtd).standalone;
                                break;
                            } else if (*entity).open as ::core::ffi::c_int != 0
                                || entity == (*parser).m_declEntity
                            {
                                if enc == (*parser).m_encoding {
                                    (*parser).m_eventPtr = entityTextPtr;
                                }
                                result = crate::expat_h::XML_ERROR_RECURSIVE_ENTITY_REF;
                                break;
                            } else if !(*entity).systemId.is_null() {
                                if (*parser).m_externalEntityRefHandler.is_some() {
                                    (*dtd).paramEntityRead = crate::expat_h::XML_FALSE;
                                    (*entity).open = crate::expat_h::XML_TRUE;
                                    let entity_root_parser = getRootParserOf(
                                        parser,
                                        ::core::ptr::null_mut::<::core::ffi::c_uint>(),
                                    )
                                        as crate::expat_h::XML_Parser;
                                    entityTrackingOnOpen(&mut *entity_root_parser);
                                    entityTrackingReportStats(
                                        &*entity_root_parser,
                                        &*entity,
                                        ::std::ffi::CStr::from_ptr(
                                            (*entity).name as *const ::core::ffi::c_char,
                                        ),
                                        "OPEN ",
                                        6840 as ::core::ffi::c_int,
                                    );
                                    if (*parser)
                                        .m_externalEntityRefHandler
                                        .expect("non-null function pointer")(
                                        (*parser).m_externalEntityRefHandlerArg,
                                        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
                                        (*entity).base,
                                        (*entity).systemId,
                                        (*entity).publicId,
                                    ) == 0
                                    {
                                        entityTrackingReportStats(
                                            &*entity_root_parser,
                                            &*entity,
                                            ::std::ffi::CStr::from_ptr(
                                                (*entity).name as *const ::core::ffi::c_char,
                                            ),
                                            "CLOSE",
                                            6844 as ::core::ffi::c_int,
                                        );
                                        entityTrackingOnClose(&mut *entity_root_parser);
                                        (*entity).open = crate::expat_h::XML_FALSE;
                                        result = crate::expat_h::XML_ERROR_EXTERNAL_ENTITY_HANDLING;
                                        break;
                                    } else {
                                        entityTrackingReportStats(
                                            &*entity_root_parser,
                                            &*entity,
                                            ::std::ffi::CStr::from_ptr(
                                                (*entity).name as *const ::core::ffi::c_char,
                                            ),
                                            "CLOSE",
                                            6849 as ::core::ffi::c_int,
                                        );
                                        entityTrackingOnClose(&mut *entity_root_parser);
                                        (*entity).open = crate::expat_h::XML_FALSE;
                                        if (*dtd).paramEntityRead == 0 {
                                            (*dtd).keepProcessing = (*dtd).standalone;
                                        }
                                    }
                                } else {
                                    (*dtd).keepProcessing = (*dtd).standalone;
                                }
                            } else {
                                result = processEntity(
                                    parser,
                                    entity,
                                    crate::expat_h::XML_FALSE,
                                    ENTITY_VALUE,
                                );
                                break;
                            }
                        }
                    } else {
                        (*parser).m_eventPtr = entityTextPtr;
                        result = crate::expat_h::XML_ERROR_PARAM_ENTITY_REF;
                        break;
                    }
                    c2rust_current_block = 5028470053297453708;
                }
                crate::src::xmltok::XML_TOK_NONE => {
                    result = crate::expat_h::XML_ERROR_NONE;
                    break;
                }
                crate::src::xmltok::XML_TOK_ENTITY_REF | crate::src::xmltok::XML_TOK_DATA_CHARS => {
                    if poolAppend(pool, enc, entityTextPtr, next).is_null() {
                        result = crate::expat_h::XML_ERROR_NO_MEMORY;
                        break;
                    } else {
                        c2rust_current_block = 5028470053297453708;
                    }
                }
                crate::src::xmltok::XML_TOK_TRAILING_CR => {
                    next = entityTextPtr.offset((*enc).minBytesPerChar as isize);
                    c2rust_current_block = 11380413721579491564;
                }
                crate::src::xmltok::XML_TOK_DATA_NEWLINE => {
                    c2rust_current_block = 11380413721579491564;
                }
                crate::src::xmltok::XML_TOK_CHAR_REF => {
                    let mut buf: [crate::expat_external_h::XML_Char; 4] = [0; 4];
                    let mut i: ::core::ffi::c_int = 0;
                    let mut n: ::core::ffi::c_int = (*enc)
                        .charRefNumber
                        .expect("non-null function pointer")(
                        enc, entityTextPtr
                    );
                    if n < 0 as ::core::ffi::c_int {
                        if enc == (*parser).m_encoding {
                            (*parser).m_eventPtr = entityTextPtr;
                        }
                        result = crate::expat_h::XML_ERROR_BAD_CHAR_REF;
                        break;
                    } else {
                        n = crate::src::xmltok::XmlUtf8Encode(n, &mut buf);
                        i = 0 as ::core::ffi::c_int;
                        while i < n {
                            if (*pool).end
                                == (*pool).ptr as *const crate::expat_external_h::XML_Char
                                && poolGrow(pool) == 0
                            {
                                result = crate::expat_h::XML_ERROR_NO_MEMORY;
                                break 's_35;
                            } else {
                                let c2rust_fresh73 = (*pool).ptr;
                                (*pool).ptr = (*pool).ptr.offset(1);
                                *c2rust_fresh73 = buf[i as usize];
                                i += 1;
                            }
                        }
                    }
                    c2rust_current_block = 5028470053297453708;
                }
                crate::src::xmltok::XML_TOK_PARTIAL => {
                    if enc == (*parser).m_encoding {
                        (*parser).m_eventPtr = entityTextPtr;
                    }
                    result = crate::expat_h::XML_ERROR_INVALID_TOKEN;
                    break;
                }
                crate::src::xmltok::XML_TOK_INVALID => {
                    if enc == (*parser).m_encoding {
                        (*parser).m_eventPtr = next;
                    }
                    result = crate::expat_h::XML_ERROR_INVALID_TOKEN;
                    break;
                }
                _ => {
                    if enc == (*parser).m_encoding {
                        (*parser).m_eventPtr = entityTextPtr;
                    }
                    result = crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
                    break;
                }
            }
            match c2rust_current_block {
                11380413721579491564 => {
                    if (*pool).end == (*pool).ptr as *const crate::expat_external_h::XML_Char
                        && poolGrow(pool) == 0
                    {
                        result = crate::expat_h::XML_ERROR_NO_MEMORY;
                        break;
                    } else {
                        let c2rust_fresh72 = (*pool).ptr;
                        (*pool).ptr = (*pool).ptr.offset(1);
                        *c2rust_fresh72 = 0xa as crate::expat_external_h::XML_Char;
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
    mut parser: crate::expat_h::XML_Parser,
    mut enc: *const crate::src::xmltok::ENCODING,
    mut entityTextPtr: *const ::core::ffi::c_char,
    mut entityTextEnd: *const ::core::ffi::c_char,
    mut account: XML_Account,
) -> crate::expat_h::XML_Error {
    let mut next: *const ::core::ffi::c_char = entityTextPtr;
    let mut result: crate::expat_h::XML_Error = crate::expat_h::XML_ERROR_NONE;
    loop {
        if (*parser).m_openValueEntities.is_null() {
            result = storeEntityValue(parser, enc, next, entityTextEnd, account, &raw mut next);
        } else {
            let openEntity: *mut OPEN_INTERNAL_ENTITY = (*parser).m_openValueEntities;
            if openEntity.is_null() {
                return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
            }
            let entity: *mut ENTITY = (*openEntity).entity;
            let textStart: *const ::core::ffi::c_char = ((*entity).textPtr
                as *const ::core::ffi::c_char)
                .offset((*entity).processed as isize);
            let textEnd: *const ::core::ffi::c_char =
                (*entity).textPtr.offset((*entity).textLen as isize) as *const ::core::ffi::c_char;
            let mut nextInEntity: *const ::core::ffi::c_char = textStart;
            if (*entity).hasMore != 0 {
                result = storeEntityValue(
                    parser,
                    (*parser).m_internalEncoding,
                    textStart,
                    textEnd,
                    XML_ACCOUNT_ENTITY_EXPANSION,
                    &raw mut nextInEntity,
                );
                if result as ::core::ffi::c_uint
                    != crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    break;
                }
                if textEnd != nextInEntity {
                    (*entity).processed =
                        nextInEntity.offset_from((*entity).textPtr as *const ::core::ffi::c_char)
                            as ::core::ffi::c_long as ::core::ffi::c_int;
                    continue;
                } else {
                    (*entity).hasMore = crate::expat_h::XML_FALSE;
                    continue;
                }
            } else {
                let entity_root_parser =
                    getRootParserOf(parser, ::core::ptr::null_mut::<::core::ffi::c_uint>())
                        as crate::expat_h::XML_Parser;
                entityTrackingReportStats(
                    &*entity_root_parser,
                    &*entity,
                    ::std::ffi::CStr::from_ptr((*entity).name as *const ::core::ffi::c_char),
                    "CLOSE",
                    6998 as ::core::ffi::c_int,
                );
                entityTrackingOnClose(&mut *entity_root_parser);
                '_c2rust_label: {
                    if (*parser).m_openValueEntities == openEntity {
                    } else {
                        crate::stdlib::__assert_fail(
                            b"parser->m_openValueEntities == openEntity\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"../../expat/lib/xmlparse.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            7004 as ::core::ffi::c_uint,
                            b"enum XML_Error callStoreEntityValue(XML_Parser, const ENCODING *, const char *, const char *, enum XML_Account)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                };
                (*entity).open = crate::expat_h::XML_FALSE;
                (*parser).m_openValueEntities =
                    (*(*parser).m_openValueEntities).next as *mut OPEN_INTERNAL_ENTITY;
                (*openEntity).next = (*parser).m_freeValueEntities as *mut open_internal_entity;
                (*parser).m_freeValueEntities = openEntity;
            }
        }
        if result as ::core::ffi::c_uint != 0
            || (*parser).m_openValueEntities.is_null() && entityTextEnd == next
        {
            break;
        }
    }
    return result;
}

fn normalizeLines(s: &mut [crate::expat_external_h::XML_Char]) {
    let nul = s
        .iter()
        .position(|&c| c as ::core::ffi::c_int == '\0' as i32)
        .unwrap_or(s.len());
    let Some(first_cr) = s[..nul]
        .iter()
        .position(|&c| c as ::core::ffi::c_int == 0xd as ::core::ffi::c_int)
    else {
        return;
    };

    let mut read = first_cr;
    let mut write = first_cr;
    while read < nul {
        if s[read] as ::core::ffi::c_int == 0xd as ::core::ffi::c_int {
            s[write] = 0xa as crate::expat_external_h::XML_Char;
            write += 1;
            read += 1;
            if read < nul && s[read] as ::core::ffi::c_int == 0xa as ::core::ffi::c_int {
                read += 1;
            }
        } else {
            s[write] = s[read];
            write += 1;
            read += 1;
        }
    }
    if write < s.len() {
        s[write] = '\0' as i32 as crate::expat_external_h::XML_Char;
    }
}

unsafe extern "C" fn reportProcessingInstruction(
    mut parser: crate::expat_h::XML_Parser,
    mut enc: *const crate::src::xmltok::ENCODING,
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut target: *const crate::expat_external_h::XML_Char =
        ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    let mut data: *mut crate::expat_external_h::XML_Char =
        ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
    let mut tem: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if (*parser).m_processingInstructionHandler.is_none() {
        if (*parser).m_defaultHandler.is_some() {
            reportDefault(parser, enc, start, end);
        }
        return 1 as ::core::ffi::c_int;
    }
    start = start.offset(((*enc).minBytesPerChar * 2 as ::core::ffi::c_int) as isize);
    tem = start.offset((*enc).nameLength.expect("non-null function pointer")(enc, start) as isize);
    target = poolStoreString(&raw mut (*parser).m_tempPool, enc, start, tem);
    if target.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
    data = poolStoreString(
        &raw mut (*parser).m_tempPool,
        enc,
        (*enc).skipS.expect("non-null function pointer")(enc, tem),
        end.offset(-(((*enc).minBytesPerChar * 2 as ::core::ffi::c_int) as isize)),
    );
    if data.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let data_len = ::std::ffi::CStr::from_ptr(data).to_bytes_with_nul().len();
    normalizeLines(::core::slice::from_raw_parts_mut(data, data_len));
    (*parser)
        .m_processingInstructionHandler
        .expect("non-null function pointer")((*parser).m_handlerArg, target, data);
    poolClear(&raw mut (*parser).m_tempPool);
    return 1 as ::core::ffi::c_int;
}

unsafe extern "C" fn reportComment(
    mut parser: crate::expat_h::XML_Parser,
    mut enc: *const crate::src::xmltok::ENCODING,
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut data: *mut crate::expat_external_h::XML_Char =
        ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
    if (*parser).m_commentHandler.is_none() {
        if (*parser).m_defaultHandler.is_some() {
            reportDefault(parser, enc, start, end);
        }
        return 1 as ::core::ffi::c_int;
    }
    data = poolStoreString(
        &raw mut (*parser).m_tempPool,
        enc,
        start.offset(((*enc).minBytesPerChar * 4 as ::core::ffi::c_int) as isize),
        end.offset(-(((*enc).minBytesPerChar * 3 as ::core::ffi::c_int) as isize)),
    );
    if data.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let data_len = ::std::ffi::CStr::from_ptr(data).to_bytes_with_nul().len();
    normalizeLines(::core::slice::from_raw_parts_mut(data, data_len));
    (*parser)
        .m_commentHandler
        .expect("non-null function pointer")((*parser).m_handlerArg, data);
    poolClear(&raw mut (*parser).m_tempPool);
    return 1 as ::core::ffi::c_int;
}

unsafe extern "C" fn reportDefault(
    mut parser: crate::expat_h::XML_Parser,
    mut enc: *const crate::src::xmltok::ENCODING,
    mut s: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) {
    if (*enc).isUtf8 == 0 {
        let mut convert_res: crate::src::xmltok::XML_Convert_Result =
            crate::src::xmltok::XML_CONVERT_COMPLETED;
        let mut eventPP: *mut *const ::core::ffi::c_char =
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
        let mut eventEndPP: *mut *const ::core::ffi::c_char =
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
        if enc == (*parser).m_encoding {
            eventPP = &raw mut (*parser).m_eventPtr;
            eventEndPP = &raw mut (*parser).m_eventEndPtr;
        } else {
            eventPP = &raw mut (*(*parser).m_openInternalEntities).internalEventPtr;
            eventEndPP = &raw mut (*(*parser).m_openInternalEntities).internalEventEndPtr;
        }
        loop {
            let mut dataPtr: *mut ICHAR = (*parser).m_dataBuf as *mut ICHAR;
            convert_res = (*enc).utf8Convert.expect("non-null function pointer")(
                enc,
                &raw mut s,
                end,
                &raw mut dataPtr,
                (*parser).m_dataBufEnd as *mut ICHAR,
            );
            *eventEndPP = s;
            (*parser)
                .m_defaultHandler
                .expect("non-null function pointer")(
                (*parser).m_handlerArg,
                (*parser).m_dataBuf,
                dataPtr.offset_from((*parser).m_dataBuf as *mut ICHAR) as ::core::ffi::c_long
                    as ::core::ffi::c_int,
            );
            *eventPP = s;
            if !(convert_res as ::core::ffi::c_uint
                != crate::src::xmltok::XML_CONVERT_COMPLETED as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                && convert_res as ::core::ffi::c_uint
                    != crate::src::xmltok::XML_CONVERT_INPUT_INCOMPLETE as ::core::ffi::c_int
                        as ::core::ffi::c_uint)
            {
                break;
            }
        }
    } else {
        (*parser)
            .m_defaultHandler
            .expect("non-null function pointer")(
            (*parser).m_handlerArg,
            s as *const crate::expat_external_h::XML_Char,
            (end as *const crate::expat_external_h::XML_Char)
                .offset_from(s as *const crate::expat_external_h::XML_Char)
                as ::core::ffi::c_long as ::core::ffi::c_int,
        );
    };
}

unsafe extern "C" fn defineAttribute(
    mut type_0: *mut ELEMENT_TYPE,
    mut attId: *mut ATTRIBUTE_ID,
    mut isCdata: crate::expat_h::XML_Bool,
    mut isId: crate::expat_h::XML_Bool,
    mut value: *const crate::expat_external_h::XML_Char,
    mut parser: crate::expat_h::XML_Parser,
) -> ::core::ffi::c_int {
    let mut att: *mut DEFAULT_ATTRIBUTE = ::core::ptr::null_mut::<DEFAULT_ATTRIBUTE>();
    if !value.is_null() || isId as ::core::ffi::c_int != 0 {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*type_0).nDefaultAtts {
            if attId == (*(*type_0).defaultAtts.offset(i as isize)).id as *mut ATTRIBUTE_ID {
                return 1 as ::core::ffi::c_int;
            }
            i += 1;
        }
        if isId as ::core::ffi::c_int != 0 && (*type_0).idAtt.is_null() && (*attId).xmlns == 0 {
            (*type_0).idAtt = attId;
        }
    }
    if (*type_0).nDefaultAtts == (*type_0).allocDefaultAtts {
        if (*type_0).allocDefaultAtts == 0 as ::core::ffi::c_int {
            (*type_0).allocDefaultAtts = 8 as ::core::ffi::c_int;
            (*type_0).defaultAtts = expat_malloc(
                parser,
                ((*type_0).allocDefaultAtts as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(::core::mem::size_of::<DEFAULT_ATTRIBUTE>()
                        as crate::__stddef_size_t_h::size_t),
                7182 as ::core::ffi::c_int,
            ) as *mut DEFAULT_ATTRIBUTE;
            if (*type_0).defaultAtts.is_null() {
                (*type_0).allocDefaultAtts = 0 as ::core::ffi::c_int;
                return 0 as ::core::ffi::c_int;
            }
        } else {
            let mut temp: *mut DEFAULT_ATTRIBUTE = ::core::ptr::null_mut::<DEFAULT_ATTRIBUTE>();
            if (*type_0).allocDefaultAtts > crate::limits_h::INT_MAX / 2 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            }
            let mut count: ::core::ffi::c_int =
                (*type_0).allocDefaultAtts * 2 as ::core::ffi::c_int;
            temp = expat_realloc(
                parser,
                (*type_0).defaultAtts as *mut ::core::ffi::c_void,
                (count as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(::core::mem::size_of::<DEFAULT_ATTRIBUTE>()
                        as crate::__stddef_size_t_h::size_t),
                7208 as ::core::ffi::c_int,
            ) as *mut DEFAULT_ATTRIBUTE;
            if temp.is_null() {
                return 0 as ::core::ffi::c_int;
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
        (*attId).maybeTokenized = crate::expat_h::XML_TRUE;
    }
    (*type_0).nDefaultAtts += 1 as ::core::ffi::c_int;
    return 1 as ::core::ffi::c_int;
}

unsafe extern "C" fn setElementTypePrefix(
    mut parser: crate::expat_h::XML_Parser,
    mut elementType: *mut ELEMENT_TYPE,
) -> ::core::ffi::c_int {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut name: *const crate::expat_external_h::XML_Char =
        ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    name = (*elementType).name;
    while *name != 0 {
        if *name as ::core::ffi::c_int == 0x3a as ::core::ffi::c_int {
            let mut prefix: *mut PREFIX = ::core::ptr::null_mut::<PREFIX>();
            let mut s: *const crate::expat_external_h::XML_Char =
                ::core::ptr::null::<crate::expat_external_h::XML_Char>();
            s = (*elementType).name;
            while s != name {
                if if (*dtd).pool.ptr == (*dtd).pool.end as *mut crate::expat_external_h::XML_Char
                    && poolGrow(&raw mut (*dtd).pool) == 0
                {
                    0 as ::core::ffi::c_int
                } else {
                    let c2rust_fresh15 = (*dtd).pool.ptr;
                    (*dtd).pool.ptr = (*dtd).pool.ptr.offset(1);
                    *c2rust_fresh15 = *s;
                    1 as ::core::ffi::c_int
                } == 0
                {
                    return 0 as ::core::ffi::c_int;
                }
                s = s.offset(1);
            }
            if if (*dtd).pool.ptr == (*dtd).pool.end as *mut crate::expat_external_h::XML_Char
                && poolGrow(&raw mut (*dtd).pool) == 0
            {
                0 as ::core::ffi::c_int
            } else {
                let c2rust_fresh16 = (*dtd).pool.ptr;
                (*dtd).pool.ptr = (*dtd).pool.ptr.offset(1);
                *c2rust_fresh16 = '\0' as i32 as crate::expat_external_h::XML_Char;
                1 as ::core::ffi::c_int
            } == 0
            {
                return 0 as ::core::ffi::c_int;
            }
            prefix = lookup(
                parser,
                &raw mut (*dtd).prefixes,
                (*dtd).pool.start as KEY,
                ::core::mem::size_of::<PREFIX>() as crate::__stddef_size_t_h::size_t,
            ) as *mut PREFIX;
            if prefix.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            if (*prefix).name == (*dtd).pool.start as *const crate::expat_external_h::XML_Char {
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
    return 1 as ::core::ffi::c_int;
}

unsafe extern "C" fn getAttributeId(
    mut parser: crate::expat_h::XML_Parser,
    mut enc: *const crate::src::xmltok::ENCODING,
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> *mut ATTRIBUTE_ID {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut id: *mut ATTRIBUTE_ID = ::core::ptr::null_mut::<ATTRIBUTE_ID>();
    let mut name: *const crate::expat_external_h::XML_Char =
        ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    if if (*dtd).pool.ptr == (*dtd).pool.end as *mut crate::expat_external_h::XML_Char
        && poolGrow(&raw mut (*dtd).pool) == 0
    {
        0 as ::core::ffi::c_int
    } else {
        let c2rust_fresh52 = (*dtd).pool.ptr;
        (*dtd).pool.ptr = (*dtd).pool.ptr.offset(1);
        *c2rust_fresh52 = '\0' as i32 as crate::expat_external_h::XML_Char;
        1 as ::core::ffi::c_int
    } == 0
    {
        return ::core::ptr::null_mut::<ATTRIBUTE_ID>();
    }
    name = poolStoreString(&raw mut (*dtd).pool, enc, start, end);
    if name.is_null() {
        return ::core::ptr::null_mut::<ATTRIBUTE_ID>();
    }
    name = name.offset(1);
    id = lookup(
        parser,
        &raw mut (*dtd).attributeIds,
        name as KEY,
        ::core::mem::size_of::<ATTRIBUTE_ID>() as crate::__stddef_size_t_h::size_t,
    ) as *mut ATTRIBUTE_ID;
    if id.is_null() {
        return ::core::ptr::null_mut::<ATTRIBUTE_ID>();
    }
    if (*id).name != name as *mut crate::expat_external_h::XML_Char {
        (*dtd).pool.ptr = (*dtd).pool.start;
    } else {
        (*dtd).pool.start = (*dtd).pool.ptr;
        if !((*parser).m_ns == 0) {
            if *name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0x78 as ::core::ffi::c_int
                && *name.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0x6d as ::core::ffi::c_int
                && *name.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0x6c as ::core::ffi::c_int
                && *name.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0x6e as ::core::ffi::c_int
                && *name.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0x73 as ::core::ffi::c_int
                && (*name.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\0' as i32
                    || *name.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 0x3a as ::core::ffi::c_int)
            {
                if *name.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\0' as i32
                {
                    (*id).prefix = &raw mut (*dtd).defaultPrefix;
                } else {
                    (*id).prefix = lookup(
                        parser,
                        &raw mut (*dtd).prefixes,
                        name.offset(6 as ::core::ffi::c_int as isize),
                        ::core::mem::size_of::<PREFIX>() as crate::__stddef_size_t_h::size_t,
                    ) as *mut PREFIX;
                }
                (*id).xmlns = crate::expat_h::XML_TRUE;
            } else {
                let mut i: ::core::ffi::c_int = 0;
                i = 0 as ::core::ffi::c_int;
                while *name.offset(i as isize) != 0 {
                    if *name.offset(i as isize) as ::core::ffi::c_int == 0x3a as ::core::ffi::c_int
                    {
                        let mut j: ::core::ffi::c_int = 0;
                        j = 0 as ::core::ffi::c_int;
                        while j < i {
                            if if (*dtd).pool.ptr
                                == (*dtd).pool.end as *mut crate::expat_external_h::XML_Char
                                && poolGrow(&raw mut (*dtd).pool) == 0
                            {
                                0 as ::core::ffi::c_int
                            } else {
                                let c2rust_fresh53 = (*dtd).pool.ptr;
                                (*dtd).pool.ptr = (*dtd).pool.ptr.offset(1);
                                *c2rust_fresh53 = *name.offset(j as isize);
                                1 as ::core::ffi::c_int
                            } == 0
                            {
                                return ::core::ptr::null_mut::<ATTRIBUTE_ID>();
                            }
                            j += 1;
                        }
                        if if (*dtd).pool.ptr
                            == (*dtd).pool.end as *mut crate::expat_external_h::XML_Char
                            && poolGrow(&raw mut (*dtd).pool) == 0
                        {
                            0 as ::core::ffi::c_int
                        } else {
                            let c2rust_fresh54 = (*dtd).pool.ptr;
                            (*dtd).pool.ptr = (*dtd).pool.ptr.offset(1);
                            *c2rust_fresh54 = '\0' as i32 as crate::expat_external_h::XML_Char;
                            1 as ::core::ffi::c_int
                        } == 0
                        {
                            return ::core::ptr::null_mut::<ATTRIBUTE_ID>();
                        }
                        (*id).prefix = lookup(
                            parser,
                            &raw mut (*dtd).prefixes,
                            (*dtd).pool.start as KEY,
                            ::core::mem::size_of::<PREFIX>() as crate::__stddef_size_t_h::size_t,
                        ) as *mut PREFIX;
                        if (*id).prefix.is_null() {
                            return ::core::ptr::null_mut::<ATTRIBUTE_ID>();
                        }
                        if (*(*id).prefix).name
                            == (*dtd).pool.start as *const crate::expat_external_h::XML_Char
                        {
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

unsafe extern "C" fn getContext(
    mut parser: crate::expat_h::XML_Parser,
) -> *const crate::expat_external_h::XML_Char {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut iter: HASH_TABLE_ITER = HASH_TABLE_ITER {
        p: ::core::ptr::null_mut::<*mut NAMED>(),
        end: ::core::ptr::null_mut::<*mut NAMED>(),
    };
    let mut needSep: crate::expat_h::XML_Bool = crate::expat_h::XML_FALSE;
    if !(*dtd).defaultPrefix.binding.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        let mut len: ::core::ffi::c_int = 0;
        if if (*parser).m_tempPool.ptr
            == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
            && poolGrow(&raw mut (*parser).m_tempPool) == 0
        {
            0 as ::core::ffi::c_int
        } else {
            let c2rust_fresh61 = (*parser).m_tempPool.ptr;
            (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
            *c2rust_fresh61 = 0x3d as crate::expat_external_h::XML_Char;
            1 as ::core::ffi::c_int
        } == 0
        {
            return ::core::ptr::null::<crate::expat_external_h::XML_Char>();
        }
        len = (*(*dtd).defaultPrefix.binding).uriLen;
        if (*parser).m_namespaceSeparator != 0 {
            len -= 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < len {
            if if (*parser).m_tempPool.ptr
                == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
                && poolGrow(&raw mut (*parser).m_tempPool) == 0
            {
                0 as ::core::ffi::c_int
            } else {
                let c2rust_fresh62 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *c2rust_fresh62 = *(*(*dtd).defaultPrefix.binding).uri.offset(i as isize);
                1 as ::core::ffi::c_int
            } == 0
            {
                return ::core::ptr::null::<crate::expat_external_h::XML_Char>();
            }
            i += 1;
        }
        needSep = crate::expat_h::XML_TRUE;
    }
    hashTableIterInit(&mut iter, &(*dtd).prefixes);
    loop {
        let mut i_0: ::core::ffi::c_int = 0;
        let mut len_0: ::core::ffi::c_int = 0;
        let mut s: *const crate::expat_external_h::XML_Char =
            ::core::ptr::null::<crate::expat_external_h::XML_Char>();
        let mut prefix: *mut PREFIX = hashTableIterNext(&raw mut iter) as *mut PREFIX;
        if prefix.is_null() {
            break;
        }
        if (*prefix).binding.is_null() {
            continue;
        }
        if needSep as ::core::ffi::c_int != 0
            && (if (*parser).m_tempPool.ptr
                == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
                && poolGrow(&raw mut (*parser).m_tempPool) == 0
            {
                0 as ::core::ffi::c_int
            } else {
                let c2rust_fresh63 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *c2rust_fresh63 = 0xc as crate::expat_external_h::XML_Char;
                1 as ::core::ffi::c_int
            }) == 0
        {
            return ::core::ptr::null::<crate::expat_external_h::XML_Char>();
        }
        s = (*prefix).name;
        while *s != 0 {
            if if (*parser).m_tempPool.ptr
                == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
                && poolGrow(&raw mut (*parser).m_tempPool) == 0
            {
                0 as ::core::ffi::c_int
            } else {
                let c2rust_fresh64 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *c2rust_fresh64 = *s;
                1 as ::core::ffi::c_int
            } == 0
            {
                return ::core::ptr::null::<crate::expat_external_h::XML_Char>();
            }
            s = s.offset(1);
        }
        if if (*parser).m_tempPool.ptr
            == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
            && poolGrow(&raw mut (*parser).m_tempPool) == 0
        {
            0 as ::core::ffi::c_int
        } else {
            let c2rust_fresh65 = (*parser).m_tempPool.ptr;
            (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
            *c2rust_fresh65 = 0x3d as crate::expat_external_h::XML_Char;
            1 as ::core::ffi::c_int
        } == 0
        {
            return ::core::ptr::null::<crate::expat_external_h::XML_Char>();
        }
        len_0 = (*(*prefix).binding).uriLen;
        if (*parser).m_namespaceSeparator != 0 {
            len_0 -= 1;
        }
        i_0 = 0 as ::core::ffi::c_int;
        while i_0 < len_0 {
            if if (*parser).m_tempPool.ptr
                == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
                && poolGrow(&raw mut (*parser).m_tempPool) == 0
            {
                0 as ::core::ffi::c_int
            } else {
                let c2rust_fresh66 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *c2rust_fresh66 = *(*(*prefix).binding).uri.offset(i_0 as isize);
                1 as ::core::ffi::c_int
            } == 0
            {
                return ::core::ptr::null::<crate::expat_external_h::XML_Char>();
            }
            i_0 += 1;
        }
        needSep = crate::expat_h::XML_TRUE;
    }
    hashTableIterInit(&mut iter, &(*dtd).generalEntities);
    loop {
        let mut s_0: *const crate::expat_external_h::XML_Char =
            ::core::ptr::null::<crate::expat_external_h::XML_Char>();
        let mut e: *mut ENTITY = hashTableIterNext(&raw mut iter) as *mut ENTITY;
        if e.is_null() {
            break;
        }
        if (*e).open == 0 {
            continue;
        }
        if needSep as ::core::ffi::c_int != 0
            && (if (*parser).m_tempPool.ptr
                == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
                && poolGrow(&raw mut (*parser).m_tempPool) == 0
            {
                0 as ::core::ffi::c_int
            } else {
                let c2rust_fresh67 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *c2rust_fresh67 = 0xc as crate::expat_external_h::XML_Char;
                1 as ::core::ffi::c_int
            }) == 0
        {
            return ::core::ptr::null::<crate::expat_external_h::XML_Char>();
        }
        s_0 = (*e).name;
        while *s_0 != 0 {
            if if (*parser).m_tempPool.ptr
                == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
                && poolGrow(&raw mut (*parser).m_tempPool) == 0
            {
                0 as ::core::ffi::c_int
            } else {
                let c2rust_fresh68 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *c2rust_fresh68 = *s_0;
                1 as ::core::ffi::c_int
            } == 0
            {
                return ::core::ptr::null::<crate::expat_external_h::XML_Char>();
            }
            s_0 = s_0.offset(1);
        }
        needSep = crate::expat_h::XML_TRUE;
    }
    if if (*parser).m_tempPool.ptr
        == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
        && poolGrow(&raw mut (*parser).m_tempPool) == 0
    {
        0 as ::core::ffi::c_int
    } else {
        let c2rust_fresh69 = (*parser).m_tempPool.ptr;
        (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
        *c2rust_fresh69 = '\0' as i32 as crate::expat_external_h::XML_Char;
        1 as ::core::ffi::c_int
    } == 0
    {
        return ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    }
    return (*parser).m_tempPool.start;
}

unsafe extern "C" fn setContext(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Bool {
    if context.is_null() {
        return crate::expat_h::XML_FALSE;
    }
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut s: *const crate::expat_external_h::XML_Char = context;
    while *context as ::core::ffi::c_int != '\0' as i32 {
        if *s as ::core::ffi::c_int == 0xc as ::core::ffi::c_int
            || *s as ::core::ffi::c_int == '\0' as i32
        {
            let mut e: *mut ENTITY = ::core::ptr::null_mut::<ENTITY>();
            if if (*parser).m_tempPool.ptr
                == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
                && poolGrow(&raw mut (*parser).m_tempPool) == 0
            {
                0 as ::core::ffi::c_int
            } else {
                let c2rust_fresh76 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *c2rust_fresh76 = '\0' as i32 as crate::expat_external_h::XML_Char;
                1 as ::core::ffi::c_int
            } == 0
            {
                return crate::expat_h::XML_FALSE;
            }
            e = lookup(
                parser,
                &raw mut (*dtd).generalEntities,
                (*parser).m_tempPool.start as KEY,
                0 as crate::__stddef_size_t_h::size_t,
            ) as *mut ENTITY;
            if !e.is_null() {
                (*e).open = crate::expat_h::XML_TRUE;
            }
            if *s as ::core::ffi::c_int != '\0' as i32 {
                s = s.offset(1);
            }
            context = s;
            (*parser).m_tempPool.ptr = (*parser).m_tempPool.start;
        } else if *s as ::core::ffi::c_int == 0x3d as ::core::ffi::c_int {
            let mut prefix: *mut PREFIX = ::core::ptr::null_mut::<PREFIX>();
            if (*parser)
                .m_tempPool
                .ptr
                .offset_from((*parser).m_tempPool.start) as ::core::ffi::c_long
                == 0 as ::core::ffi::c_long
            {
                prefix = &raw mut (*dtd).defaultPrefix;
            } else {
                if if (*parser).m_tempPool.ptr
                    == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
                    && poolGrow(&raw mut (*parser).m_tempPool) == 0
                {
                    0 as ::core::ffi::c_int
                } else {
                    let c2rust_fresh77 = (*parser).m_tempPool.ptr;
                    (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                    *c2rust_fresh77 = '\0' as i32 as crate::expat_external_h::XML_Char;
                    1 as ::core::ffi::c_int
                } == 0
                {
                    return crate::expat_h::XML_FALSE;
                }
                prefix = lookup(
                    parser,
                    &raw mut (*dtd).prefixes,
                    (*parser).m_tempPool.start as KEY,
                    ::core::mem::size_of::<PREFIX>() as crate::__stddef_size_t_h::size_t,
                ) as *mut PREFIX;
                if prefix.is_null() {
                    return crate::expat_h::XML_FALSE;
                }
                if (*prefix).name
                    == (*parser).m_tempPool.start as *const crate::expat_external_h::XML_Char
                {
                    (*prefix).name = poolCopyString(&raw mut (*dtd).pool, (*prefix).name);
                    if (*prefix).name.is_null() {
                        return crate::expat_h::XML_FALSE;
                    }
                }
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.start;
            }
            context = s.offset(1 as ::core::ffi::c_int as isize);
            while *context as ::core::ffi::c_int != 0xc as ::core::ffi::c_int
                && *context as ::core::ffi::c_int != '\0' as i32
            {
                if if (*parser).m_tempPool.ptr
                    == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
                    && poolGrow(&raw mut (*parser).m_tempPool) == 0
                {
                    0 as ::core::ffi::c_int
                } else {
                    let c2rust_fresh78 = (*parser).m_tempPool.ptr;
                    (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                    *c2rust_fresh78 = *context;
                    1 as ::core::ffi::c_int
                } == 0
                {
                    return crate::expat_h::XML_FALSE;
                }
                context = context.offset(1);
            }
            if if (*parser).m_tempPool.ptr
                == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
                && poolGrow(&raw mut (*parser).m_tempPool) == 0
            {
                0 as ::core::ffi::c_int
            } else {
                let c2rust_fresh79 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *c2rust_fresh79 = '\0' as i32 as crate::expat_external_h::XML_Char;
                1 as ::core::ffi::c_int
            } == 0
            {
                return crate::expat_h::XML_FALSE;
            }
            if addBinding(
                parser,
                prefix,
                ::core::ptr::null::<ATTRIBUTE_ID>(),
                (*parser).m_tempPool.start,
                &raw mut (*parser).m_inheritedBindings,
            ) as ::core::ffi::c_uint
                != crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return crate::expat_h::XML_FALSE;
            }
            (*parser).m_tempPool.ptr = (*parser).m_tempPool.start;
            if *context as ::core::ffi::c_int != '\0' as i32 {
                context = context.offset(1);
            }
            s = context;
        } else {
            if if (*parser).m_tempPool.ptr
                == (*parser).m_tempPool.end as *mut crate::expat_external_h::XML_Char
                && poolGrow(&raw mut (*parser).m_tempPool) == 0
            {
                0 as ::core::ffi::c_int
            } else {
                let c2rust_fresh80 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *c2rust_fresh80 = *s;
                1 as ::core::ffi::c_int
            } == 0
            {
                return crate::expat_h::XML_FALSE;
            }
            s = s.offset(1);
        }
    }
    return crate::expat_h::XML_TRUE;
}

fn normalizePublicId(public_id: &mut [crate::expat_external_h::XML_Char]) {
    let nul = public_id
        .iter()
        .position(|&c| c as ::core::ffi::c_int == '\0' as i32)
        .unwrap_or(public_id.len());
    let mut write = 0;
    for read in 0..nul {
        match public_id[read] as ::core::ffi::c_int {
            32 | 13 | 10 => {
                if write != 0
                    && public_id[write - 1] as ::core::ffi::c_int != 0x20 as ::core::ffi::c_int
                {
                    public_id[write] = 0x20 as crate::expat_external_h::XML_Char;
                    write += 1;
                }
            }
            _ => {
                public_id[write] = public_id[read];
                write += 1;
            }
        }
    }
    if write != 0 && public_id[write - 1] as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int {
        write -= 1;
    }
    if write < public_id.len() {
        public_id[write] = '\0' as i32 as crate::expat_external_h::XML_Char;
    }
}

unsafe extern "C" fn dtdCreate(mut parser: crate::expat_h::XML_Parser) -> *mut DTD {
    let mut p: *mut DTD = expat_malloc(
        parser,
        ::core::mem::size_of::<DTD>() as crate::__stddef_size_t_h::size_t,
        7500 as ::core::ffi::c_int,
    ) as *mut DTD;
    if p.is_null() {
        return p;
    }
    poolInit(&mut (*p).pool, parser);
    poolInit(&mut (*p).entityValuePool, parser);
    hashTableInit(&mut (*p).generalEntities, parser);
    hashTableInit(&mut (*p).elementTypes, parser);
    hashTableInit(&mut (*p).attributeIds, parser);
    hashTableInit(&mut (*p).prefixes, parser);
    (*p).paramEntityRead = crate::expat_h::XML_FALSE;
    hashTableInit(&mut (*p).paramEntities, parser);
    (*p).defaultPrefix.name = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    (*p).defaultPrefix.binding = ::core::ptr::null_mut::<BINDING>();
    (*p).in_eldecl = crate::expat_h::XML_FALSE;
    (*p).scaffIndex = ::core::ptr::null_mut::<::core::ffi::c_int>();
    (*p).scaffold = ::core::ptr::null_mut::<CONTENT_SCAFFOLD>();
    (*p).scaffLevel = 0 as ::core::ffi::c_int;
    (*p).scaffSize = 0 as ::core::ffi::c_uint;
    (*p).scaffCount = 0 as ::core::ffi::c_uint;
    (*p).contentStringLen = 0 as ::core::ffi::c_uint;
    (*p).keepProcessing = crate::expat_h::XML_TRUE;
    (*p).hasParamEntityRefs = crate::expat_h::XML_FALSE;
    (*p).standalone = crate::expat_h::XML_FALSE;
    return p;
}

unsafe extern "C" fn dtdReset(mut p: *mut DTD, mut parser: crate::expat_h::XML_Parser) {
    let mut iter: HASH_TABLE_ITER = HASH_TABLE_ITER {
        p: ::core::ptr::null_mut::<*mut NAMED>(),
        end: ::core::ptr::null_mut::<*mut NAMED>(),
    };
    hashTableIterInit(&mut iter, &(*p).elementTypes);
    loop {
        let mut e: *mut ELEMENT_TYPE = hashTableIterNext(&raw mut iter) as *mut ELEMENT_TYPE;
        if e.is_null() {
            break;
        }
        if (*e).allocDefaultAtts != 0 as ::core::ffi::c_int {
            expat_free(
                parser,
                (*e).defaultAtts as *mut ::core::ffi::c_void,
                7539 as ::core::ffi::c_int,
            );
        }
    }
    for table in [
        &raw mut (*p).generalEntities,
        &raw mut (*p).paramEntities,
        &raw mut (*p).elementTypes,
        &raw mut (*p).attributeIds,
        &raw mut (*p).prefixes,
    ] {
        let mut i: crate::__stddef_size_t_h::size_t = 0;
        while i < (*table).size {
            expat_free(
                (*table).parser,
                *(*table).v.offset(i as isize) as *mut ::core::ffi::c_void,
                7927 as ::core::ffi::c_int,
            );
            *(*table).v.offset(i as isize) = ::core::ptr::null_mut::<NAMED>();
            i = i.wrapping_add(1);
        }
        (*table).used = 0 as crate::__stddef_size_t_h::size_t;
    }
    (*p).paramEntityRead = crate::expat_h::XML_FALSE;
    poolClear(&raw mut (*p).pool);
    poolClear(&raw mut (*p).entityValuePool);
    (*p).defaultPrefix.name = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    (*p).defaultPrefix.binding = ::core::ptr::null_mut::<BINDING>();
    (*p).in_eldecl = crate::expat_h::XML_FALSE;
    expat_free(
        parser,
        (*p).scaffIndex as *mut ::core::ffi::c_void,
        7556 as ::core::ffi::c_int,
    );
    (*p).scaffIndex = ::core::ptr::null_mut::<::core::ffi::c_int>();
    expat_free(
        parser,
        (*p).scaffold as *mut ::core::ffi::c_void,
        7558 as ::core::ffi::c_int,
    );
    (*p).scaffold = ::core::ptr::null_mut::<CONTENT_SCAFFOLD>();
    (*p).scaffLevel = 0 as ::core::ffi::c_int;
    (*p).scaffSize = 0 as ::core::ffi::c_uint;
    (*p).scaffCount = 0 as ::core::ffi::c_uint;
    (*p).contentStringLen = 0 as ::core::ffi::c_uint;
    (*p).keepProcessing = crate::expat_h::XML_TRUE;
    (*p).hasParamEntityRefs = crate::expat_h::XML_FALSE;
    (*p).standalone = crate::expat_h::XML_FALSE;
}

unsafe extern "C" fn dtdDestroy(
    mut p: *mut DTD,
    mut isDocEntity: crate::expat_h::XML_Bool,
    mut parser: crate::expat_h::XML_Parser,
) {
    let mut iter: HASH_TABLE_ITER = HASH_TABLE_ITER {
        p: ::core::ptr::null_mut::<*mut NAMED>(),
        end: ::core::ptr::null_mut::<*mut NAMED>(),
    };
    hashTableIterInit(&mut iter, &(*p).elementTypes);
    loop {
        let mut e: *mut ELEMENT_TYPE = hashTableIterNext(&raw mut iter) as *mut ELEMENT_TYPE;
        if e.is_null() {
            break;
        }
        if (*e).allocDefaultAtts != 0 as ::core::ffi::c_int {
            expat_free(
                parser,
                (*e).defaultAtts as *mut ::core::ffi::c_void,
                7580 as ::core::ffi::c_int,
            );
        }
    }
    for table in [
        &raw mut (*p).generalEntities,
        &raw mut (*p).paramEntities,
        &raw mut (*p).elementTypes,
        &raw mut (*p).attributeIds,
        &raw mut (*p).prefixes,
    ] {
        let mut i: crate::__stddef_size_t_h::size_t = 0;
        while i < (*table).size {
            expat_free(
                (*table).parser,
                *(*table).v.offset(i as isize) as *mut ::core::ffi::c_void,
                7937 as ::core::ffi::c_int,
            );
            i = i.wrapping_add(1);
        }
        expat_free(
            (*table).parser,
            (*table).v as *mut ::core::ffi::c_void,
            7938 as ::core::ffi::c_int,
        );
    }
    poolDestroy(&raw mut (*p).pool);
    poolDestroy(&raw mut (*p).entityValuePool);
    if isDocEntity != 0 {
        expat_free(
            parser,
            (*p).scaffIndex as *mut ::core::ffi::c_void,
            7592 as ::core::ffi::c_int,
        );
        expat_free(
            parser,
            (*p).scaffold as *mut ::core::ffi::c_void,
            7593 as ::core::ffi::c_int,
        );
    }
    expat_free(
        parser,
        p as *mut ::core::ffi::c_void,
        7595 as ::core::ffi::c_int,
    );
}

unsafe extern "C" fn dtdCopy(
    mut oldParser: crate::expat_h::XML_Parser,
    mut newDtd: *mut DTD,
    mut oldDtd: *const DTD,
    mut parser: crate::expat_h::XML_Parser,
) -> ::core::ffi::c_int {
    let mut iter: HASH_TABLE_ITER = HASH_TABLE_ITER {
        p: ::core::ptr::null_mut::<*mut NAMED>(),
        end: ::core::ptr::null_mut::<*mut NAMED>(),
    };
    hashTableIterInit(&mut iter, &(*oldDtd).prefixes);
    loop {
        let mut name: *const crate::expat_external_h::XML_Char =
            ::core::ptr::null::<crate::expat_external_h::XML_Char>();
        let mut oldP: *const PREFIX = hashTableIterNext(&raw mut iter) as *mut PREFIX;
        if oldP.is_null() {
            break;
        }
        name = poolCopyString(&raw mut (*newDtd).pool, (*oldP).name);
        if name.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if lookup(
            oldParser,
            &raw mut (*newDtd).prefixes,
            name as KEY,
            ::core::mem::size_of::<PREFIX>() as crate::__stddef_size_t_h::size_t,
        )
        .is_null()
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    hashTableIterInit(&mut iter, &(*oldDtd).attributeIds);
    loop {
        let mut newA: *mut ATTRIBUTE_ID = ::core::ptr::null_mut::<ATTRIBUTE_ID>();
        let mut name_0: *const crate::expat_external_h::XML_Char =
            ::core::ptr::null::<crate::expat_external_h::XML_Char>();
        let mut oldA: *const ATTRIBUTE_ID = hashTableIterNext(&raw mut iter) as *mut ATTRIBUTE_ID;
        if oldA.is_null() {
            break;
        }
        if if (*newDtd).pool.ptr == (*newDtd).pool.end as *mut crate::expat_external_h::XML_Char
            && poolGrow(&raw mut (*newDtd).pool) == 0
        {
            0 as ::core::ffi::c_int
        } else {
            let c2rust_fresh81 = (*newDtd).pool.ptr;
            (*newDtd).pool.ptr = (*newDtd).pool.ptr.offset(1);
            *c2rust_fresh81 = '\0' as i32 as crate::expat_external_h::XML_Char;
            1 as ::core::ffi::c_int
        } == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        name_0 = poolCopyString(&raw mut (*newDtd).pool, (*oldA).name);
        if name_0.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        name_0 = name_0.offset(1);
        newA = lookup(
            oldParser,
            &raw mut (*newDtd).attributeIds,
            name_0 as KEY,
            ::core::mem::size_of::<ATTRIBUTE_ID>() as crate::__stddef_size_t_h::size_t,
        ) as *mut ATTRIBUTE_ID;
        if newA.is_null() {
            return 0 as ::core::ffi::c_int;
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
                    (*(*oldA).prefix).name as KEY,
                    0 as crate::__stddef_size_t_h::size_t,
                ) as *mut PREFIX;
            }
        }
    }
    hashTableIterInit(&mut iter, &(*oldDtd).elementTypes);
    loop {
        let mut i: ::core::ffi::c_int = 0;
        let mut newE: *mut ELEMENT_TYPE = ::core::ptr::null_mut::<ELEMENT_TYPE>();
        let mut name_1: *const crate::expat_external_h::XML_Char =
            ::core::ptr::null::<crate::expat_external_h::XML_Char>();
        let mut oldE: *const ELEMENT_TYPE = hashTableIterNext(&raw mut iter) as *mut ELEMENT_TYPE;
        if oldE.is_null() {
            break;
        }
        name_1 = poolCopyString(&raw mut (*newDtd).pool, (*oldE).name);
        if name_1.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        newE = lookup(
            oldParser,
            &raw mut (*newDtd).elementTypes,
            name_1 as KEY,
            ::core::mem::size_of::<ELEMENT_TYPE>() as crate::__stddef_size_t_h::size_t,
        ) as *mut ELEMENT_TYPE;
        if newE.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if (*oldE).nDefaultAtts != 0 {
            (*newE).defaultAtts = expat_malloc(
                parser,
                ((*oldE).nDefaultAtts as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(::core::mem::size_of::<DEFAULT_ATTRIBUTE>()
                        as crate::__stddef_size_t_h::size_t),
                7683 as ::core::ffi::c_int,
            ) as *mut DEFAULT_ATTRIBUTE;
            if (*newE).defaultAtts.is_null() {
                return 0 as ::core::ffi::c_int;
            }
        }
        if !(*oldE).idAtt.is_null() {
            (*newE).idAtt = lookup(
                oldParser,
                &raw mut (*newDtd).attributeIds,
                (*(*oldE).idAtt).name as KEY,
                0 as crate::__stddef_size_t_h::size_t,
            ) as *mut ATTRIBUTE_ID;
        }
        (*newE).nDefaultAtts = (*oldE).nDefaultAtts;
        (*newE).allocDefaultAtts = (*newE).nDefaultAtts;
        if !(*oldE).prefix.is_null() {
            (*newE).prefix = lookup(
                oldParser,
                &raw mut (*newDtd).prefixes,
                (*(*oldE).prefix).name as KEY,
                0 as crate::__stddef_size_t_h::size_t,
            ) as *mut PREFIX;
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*newE).nDefaultAtts {
            let ref mut c2rust_fresh82 = (*(*newE).defaultAtts.offset(i as isize)).id;
            *c2rust_fresh82 = lookup(
                oldParser,
                &raw mut (*newDtd).attributeIds,
                (*(*(*oldE).defaultAtts.offset(i as isize)).id).name as KEY,
                0 as crate::__stddef_size_t_h::size_t,
            ) as *mut ATTRIBUTE_ID;
            (*(*newE).defaultAtts.offset(i as isize)).isCdata =
                (*(*oldE).defaultAtts.offset(i as isize)).isCdata;
            if !(*(*oldE).defaultAtts.offset(i as isize)).value.is_null() {
                let ref mut c2rust_fresh83 = (*(*newE).defaultAtts.offset(i as isize)).value;
                *c2rust_fresh83 = poolCopyString(
                    &raw mut (*newDtd).pool,
                    (*(*oldE).defaultAtts.offset(i as isize)).value,
                );
                if (*(*newE).defaultAtts.offset(i as isize)).value.is_null() {
                    return 0 as ::core::ffi::c_int;
                }
            } else {
                let ref mut c2rust_fresh84 = (*(*newE).defaultAtts.offset(i as isize)).value;
                *c2rust_fresh84 = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
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
        return 0 as ::core::ffi::c_int;
    }
    if copyEntityTable(
        oldParser,
        &raw mut (*newDtd).paramEntities,
        &raw mut (*newDtd).pool,
        &raw const (*oldDtd).paramEntities,
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
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
    return 1 as ::core::ffi::c_int;
}

unsafe extern "C" fn copyEntityTable(
    mut oldParser: crate::expat_h::XML_Parser,
    mut newTable: *mut HASH_TABLE,
    mut newPool: *mut STRING_POOL,
    mut oldTable: *const HASH_TABLE,
) -> ::core::ffi::c_int {
    let mut iter: HASH_TABLE_ITER = HASH_TABLE_ITER {
        p: ::core::ptr::null_mut::<*mut NAMED>(),
        end: ::core::ptr::null_mut::<*mut NAMED>(),
    };
    let mut cachedOldBase: *const crate::expat_external_h::XML_Char =
        ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    let mut cachedNewBase: *const crate::expat_external_h::XML_Char =
        ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    hashTableIterInit(&mut iter, &*oldTable);
    loop {
        let mut newE: *mut ENTITY = ::core::ptr::null_mut::<ENTITY>();
        let mut name: *const crate::expat_external_h::XML_Char =
            ::core::ptr::null::<crate::expat_external_h::XML_Char>();
        let mut oldE: *const ENTITY = hashTableIterNext(&raw mut iter) as *mut ENTITY;
        if oldE.is_null() {
            break;
        }
        name = poolCopyString(newPool, (*oldE).name);
        if name.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        newE = lookup(
            oldParser,
            newTable,
            name as KEY,
            ::core::mem::size_of::<ENTITY>() as crate::__stddef_size_t_h::size_t,
        ) as *mut ENTITY;
        if newE.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if !(*oldE).systemId.is_null() {
            let mut tem: *const crate::expat_external_h::XML_Char =
                poolCopyString(newPool, (*oldE).systemId);
            if tem.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            (*newE).systemId = tem;
            if !(*oldE).base.is_null() {
                if (*oldE).base == cachedOldBase {
                    (*newE).base = cachedNewBase;
                } else {
                    cachedOldBase = (*oldE).base;
                    tem = poolCopyString(newPool, cachedOldBase);
                    if tem.is_null() {
                        return 0 as ::core::ffi::c_int;
                    }
                    (*newE).base = tem;
                    cachedNewBase = (*newE).base;
                }
            }
            if !(*oldE).publicId.is_null() {
                tem = poolCopyString(newPool, (*oldE).publicId);
                if tem.is_null() {
                    return 0 as ::core::ffi::c_int;
                }
                (*newE).publicId = tem;
            }
        } else {
            let mut tem_0: *const crate::expat_external_h::XML_Char =
                poolCopyStringN(newPool, (*oldE).textPtr, (*oldE).textLen);
            if tem_0.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            (*newE).textPtr = tem_0;
            (*newE).textLen = (*oldE).textLen;
        }
        if !(*oldE).notation.is_null() {
            let mut tem_1: *const crate::expat_external_h::XML_Char =
                poolCopyString(newPool, (*oldE).notation);
            if tem_1.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            (*newE).notation = tem_1;
        }
        (*newE).is_param = (*oldE).is_param;
        (*newE).is_internal = (*oldE).is_internal;
    }
    return 1 as ::core::ffi::c_int;
}

pub const INIT_POWER: ::core::ffi::c_int = 6 as ::core::ffi::c_int;

unsafe extern "C" fn hash(
    mut parser: crate::expat_h::XML_Parser,
    mut s: KEY,
) -> ::core::ffi::c_ulong {
    let mut key: crate::siphash_h::sipkey = crate::siphash_h::sipkey { k: [0; 2] };
    let root_parser: crate::expat_h::XML_Parser =
        getRootParserOf(parser, ::core::ptr::null_mut::<::core::ffi::c_uint>())
            as crate::expat_h::XML_Parser;
    '_c2rust_label: {
        if (*root_parser).m_parentParser.is_null() {
        } else {
            crate::stdlib::__assert_fail(
                b"! rootParser->m_parentParser\0".as_ptr() as *const ::core::ffi::c_char,
                b"../../expat/lib/xmlparse.c\0".as_ptr() as *const ::core::ffi::c_char,
                1251 as ::core::ffi::c_uint,
                b"unsigned long get_hash_secret_salt(XML_Parser)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    key.k[0 as ::core::ffi::c_int as usize] = 0 as crate::stdlib::uint64_t;
    key.k[1 as ::core::ffi::c_int as usize] =
        (*root_parser).m_hash_secret_salt as crate::stdlib::uint64_t;
    let key_bytes = std::ffi::CStr::from_ptr(s).to_bytes();
    return siphash24(key_bytes, &key) as ::core::ffi::c_ulong;
}

unsafe extern "C" fn lookup(
    mut parser: crate::expat_h::XML_Parser,
    mut table: *mut HASH_TABLE,
    mut name: KEY,
    mut createSize: crate::__stddef_size_t_h::size_t,
) -> *mut NAMED {
    let mut i: crate::__stddef_size_t_h::size_t = 0;
    if (*table).size == 0 as crate::__stddef_size_t_h::size_t {
        let mut tsize: crate::__stddef_size_t_h::size_t = 0;
        if createSize == 0 {
            return ::core::ptr::null_mut::<NAMED>();
        }
        (*table).power = INIT_POWER as ::core::ffi::c_uchar;
        (*table).size = (1 as ::core::ffi::c_int as crate::__stddef_size_t_h::size_t) << INIT_POWER;
        tsize = (*table)
            .size
            .wrapping_mul(::core::mem::size_of::<*mut NAMED>() as crate::__stddef_size_t_h::size_t);
        (*table).v =
            expat_malloc((*table).parser, tsize, 7845 as ::core::ffi::c_int) as *mut *mut NAMED;
        if (*table).v.is_null() {
            (*table).size = 0 as crate::__stddef_size_t_h::size_t;
            return ::core::ptr::null_mut::<NAMED>();
        }
        crate::stdlib::memset(
            (*table).v as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            tsize,
        );
        i = (hash(parser, name)
            & ((*table).size as ::core::ffi::c_ulong).wrapping_sub(1 as ::core::ffi::c_ulong))
            as crate::__stddef_size_t_h::size_t;
    } else {
        let mut h: ::core::ffi::c_ulong = hash(parser, name);
        let mut mask: ::core::ffi::c_ulong =
            ((*table).size as ::core::ffi::c_ulong).wrapping_sub(1 as ::core::ffi::c_ulong);
        let mut step: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
        i = (h & mask) as crate::__stddef_size_t_h::size_t;
        while !(*(*table).v.offset(i as isize)).is_null() {
            if std::ffi::CStr::from_ptr(name)
                == std::ffi::CStr::from_ptr((**(*table).v.offset(i as isize)).name)
            {
                return *(*table).v.offset(i as isize);
            }
            if step == 0 {
                step = ((h & !mask)
                    >> (*table).power as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    & mask >> 2 as ::core::ffi::c_int
                    | 1 as ::core::ffi::c_ulong) as ::core::ffi::c_uchar;
            }
            if i < step as crate::__stddef_size_t_h::size_t {
                i = i.wrapping_add(
                    (*table)
                        .size
                        .wrapping_sub(step as crate::__stddef_size_t_h::size_t),
                );
            } else {
                i = i.wrapping_sub(step as crate::__stddef_size_t_h::size_t);
            };
        }
        if createSize == 0 {
            return ::core::ptr::null_mut::<NAMED>();
        }
        if (*table).used >> (*table).power as ::core::ffi::c_int - 1 as ::core::ffi::c_int != 0 {
            let mut newPower: ::core::ffi::c_uchar = ((*table).power as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
            if newPower as usize
                >= (::core::mem::size_of::<::core::ffi::c_ulong>() as usize)
                    .wrapping_mul(8 as usize)
            {
                return ::core::ptr::null_mut::<NAMED>();
            }
            let mut newSize: crate::__stddef_size_t_h::size_t = (1 as ::core::ffi::c_int
                as crate::__stddef_size_t_h::size_t)
                << newPower as ::core::ffi::c_int;
            let mut newMask: ::core::ffi::c_ulong =
                (newSize as ::core::ffi::c_ulong).wrapping_sub(1 as ::core::ffi::c_ulong);
            if newSize
                > (crate::stdlib::SIZE_MAX as usize)
                    .wrapping_div(::core::mem::size_of::<*mut NAMED>() as usize)
            {
                return ::core::ptr::null_mut::<NAMED>();
            }
            let mut tsize_0: crate::__stddef_size_t_h::size_t = newSize.wrapping_mul(
                ::core::mem::size_of::<*mut NAMED>() as crate::__stddef_size_t_h::size_t,
            );
            let mut newV: *mut *mut NAMED =
                expat_malloc((*table).parser, tsize_0, 7885 as ::core::ffi::c_int)
                    as *mut *mut NAMED;
            if newV.is_null() {
                return ::core::ptr::null_mut::<NAMED>();
            }
            crate::stdlib::memset(
                newV as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                tsize_0,
            );
            i = 0 as crate::__stddef_size_t_h::size_t;
            while i < (*table).size {
                if !(*(*table).v.offset(i as isize)).is_null() {
                    let mut newHash: ::core::ffi::c_ulong =
                        hash(parser, (**(*table).v.offset(i as isize)).name);
                    let mut j: crate::__stddef_size_t_h::size_t = newHash
                        as crate::__stddef_size_t_h::size_t
                        & newMask as crate::__stddef_size_t_h::size_t;
                    step = 0 as ::core::ffi::c_uchar;
                    while !(*newV.offset(j as isize)).is_null() {
                        if step == 0 {
                            step = ((newHash & !newMask)
                                >> newPower as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                & newMask >> 2 as ::core::ffi::c_int
                                | 1 as ::core::ffi::c_ulong)
                                as ::core::ffi::c_uchar;
                        }
                        if j < step as crate::__stddef_size_t_h::size_t {
                            j = j.wrapping_add(
                                newSize.wrapping_sub(step as crate::__stddef_size_t_h::size_t),
                            );
                        } else {
                            j = j.wrapping_sub(step as crate::__stddef_size_t_h::size_t);
                        };
                    }
                    let ref mut c2rust_fresh17 = *newV.offset(j as isize);
                    *c2rust_fresh17 = *(*table).v.offset(i as isize);
                }
                i = i.wrapping_add(1);
            }
            expat_free(
                (*table).parser,
                (*table).v as *mut ::core::ffi::c_void,
                7901 as ::core::ffi::c_int,
            );
            (*table).v = newV;
            (*table).power = newPower;
            (*table).size = newSize;
            i = (h & newMask) as crate::__stddef_size_t_h::size_t;
            step = 0 as ::core::ffi::c_uchar;
            while !(*(*table).v.offset(i as isize)).is_null() {
                if step == 0 {
                    step = ((h & !newMask)
                        >> newPower as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                        & newMask >> 2 as ::core::ffi::c_int
                        | 1 as ::core::ffi::c_ulong)
                        as ::core::ffi::c_uchar;
                }
                if i < step as crate::__stddef_size_t_h::size_t {
                    i = i.wrapping_add(
                        newSize.wrapping_sub(step as crate::__stddef_size_t_h::size_t),
                    );
                } else {
                    i = i.wrapping_sub(step as crate::__stddef_size_t_h::size_t);
                };
            }
        }
    }
    let ref mut c2rust_fresh18 = *(*table).v.offset(i as isize);
    *c2rust_fresh18 =
        expat_malloc((*table).parser, createSize, 7914 as ::core::ffi::c_int) as *mut NAMED;
    if (*(*table).v.offset(i as isize)).is_null() {
        return ::core::ptr::null_mut::<NAMED>();
    }
    crate::stdlib::memset(
        *(*table).v.offset(i as isize) as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        createSize,
    );
    let ref mut c2rust_fresh19 = (**(*table).v.offset(i as isize)).name;
    *c2rust_fresh19 = name;
    (*table).used = (*table).used.wrapping_add(1);
    return *(*table).v.offset(i as isize);
}

fn hashTableInit(p: &mut HASH_TABLE, parser: crate::expat_h::XML_Parser) {
    p.power = 0 as ::core::ffi::c_uchar;
    p.size = 0 as crate::__stddef_size_t_h::size_t;
    p.used = 0 as crate::__stddef_size_t_h::size_t;
    p.v = ::core::ptr::null_mut::<*mut NAMED>();
    p.parser = parser;
}

fn hashTableIterInit(iter: &mut HASH_TABLE_ITER, table: &HASH_TABLE) {
    iter.p = table.v;
    iter.end = if !iter.p.is_null() {
        iter.p.wrapping_add(table.size)
    } else {
        ::core::ptr::null_mut::<*mut NAMED>()
    };
}

unsafe extern "C" fn hashTableIterNext(mut iter: *mut HASH_TABLE_ITER) -> *mut NAMED {
    while (*iter).p != (*iter).end {
        let c2rust_fresh0 = (*iter).p;
        (*iter).p = (*iter).p.offset(1);
        let mut tem: *mut NAMED = *c2rust_fresh0;
        if !tem.is_null() {
            return tem;
        }
    }
    return ::core::ptr::null_mut::<NAMED>();
}

fn poolInit(pool: &mut STRING_POOL, parser: crate::expat_h::XML_Parser) {
    pool.blocks = ::core::ptr::null_mut::<BLOCK>();
    pool.freeBlocks = ::core::ptr::null_mut::<BLOCK>();
    pool.start = ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
    pool.ptr = ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
    pool.end = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    pool.parser = parser;
}

unsafe extern "C" fn poolClear(mut pool: *mut STRING_POOL) {
    if (*pool).freeBlocks.is_null() {
        (*pool).freeBlocks = (*pool).blocks;
    } else {
        let mut p: *mut BLOCK = (*pool).blocks;
        while !p.is_null() {
            let mut tem: *mut BLOCK = (*p).next as *mut BLOCK;
            (*p).next = (*pool).freeBlocks as *mut block;
            (*pool).freeBlocks = p;
            p = tem;
        }
    }
    (*pool).blocks = ::core::ptr::null_mut::<BLOCK>();
    (*pool).start = ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
    (*pool).ptr = ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
    (*pool).end = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
}

unsafe extern "C" fn poolDestroy(mut pool: *mut STRING_POOL) {
    let mut p: *mut BLOCK = (*pool).blocks;
    while !p.is_null() {
        let mut tem: *mut BLOCK = (*p).next as *mut BLOCK;
        expat_free(
            (*pool).parser,
            p as *mut ::core::ffi::c_void,
            8000 as ::core::ffi::c_int,
        );
        p = tem;
    }
    p = (*pool).freeBlocks;
    while !p.is_null() {
        let mut tem_0: *mut BLOCK = (*p).next as *mut BLOCK;
        expat_free(
            (*pool).parser,
            p as *mut ::core::ffi::c_void,
            8006 as ::core::ffi::c_int,
        );
        p = tem_0;
    }
}

unsafe extern "C" fn poolAppend(
    mut pool: *mut STRING_POOL,
    mut enc: *const crate::src::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> *mut crate::expat_external_h::XML_Char {
    if (*pool).ptr.is_null() && poolGrow(pool) == 0 {
        return ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
    }
    loop {
        let convert_res: crate::src::xmltok::XML_Convert_Result =
            (*enc).utf8Convert.expect("non-null function pointer")(
                enc,
                &raw mut ptr,
                end,
                &raw mut (*pool).ptr as *mut *mut ::core::ffi::c_char,
                (*pool).end as *const ::core::ffi::c_char,
            ) as crate::src::xmltok::XML_Convert_Result;
        if convert_res as ::core::ffi::c_uint
            == crate::src::xmltok::XML_CONVERT_COMPLETED as ::core::ffi::c_int
                as ::core::ffi::c_uint
            || convert_res as ::core::ffi::c_uint
                == crate::src::xmltok::XML_CONVERT_INPUT_INCOMPLETE as ::core::ffi::c_int
                    as ::core::ffi::c_uint
        {
            break;
        }
        if poolGrow(pool) == 0 {
            return ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
        }
    }
    return (*pool).start;
}

unsafe extern "C" fn poolCopyString(
    mut pool: *mut STRING_POOL,
    mut s: *const crate::expat_external_h::XML_Char,
) -> *const crate::expat_external_h::XML_Char {
    loop {
        if if (*pool).ptr == (*pool).end as *mut crate::expat_external_h::XML_Char
            && poolGrow(pool) == 0
        {
            0 as ::core::ffi::c_int
        } else {
            let c2rust_fresh59 = (*pool).ptr;
            (*pool).ptr = (*pool).ptr.offset(1);
            *c2rust_fresh59 = *s;
            1 as ::core::ffi::c_int
        } == 0
        {
            return ::core::ptr::null::<crate::expat_external_h::XML_Char>();
        }
        let c2rust_fresh60 = s;
        s = s.offset(1);
        if !(*c2rust_fresh60 != 0) {
            break;
        }
    }
    s = (*pool).start;
    (*pool).start = (*pool).ptr;
    return s;
}

unsafe extern "C" fn poolCopyStringN(
    mut pool: *mut STRING_POOL,
    mut s: *const crate::expat_external_h::XML_Char,
    mut n: ::core::ffi::c_int,
) -> *const crate::expat_external_h::XML_Char {
    if (*pool).ptr.is_null() && poolGrow(pool) == 0 {
        return ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    }
    while n > 0 as ::core::ffi::c_int {
        if if (*pool).ptr == (*pool).end as *mut crate::expat_external_h::XML_Char
            && poolGrow(pool) == 0
        {
            0 as ::core::ffi::c_int
        } else {
            let c2rust_fresh85 = (*pool).ptr;
            (*pool).ptr = (*pool).ptr.offset(1);
            *c2rust_fresh85 = *s;
            1 as ::core::ffi::c_int
        } == 0
        {
            return ::core::ptr::null::<crate::expat_external_h::XML_Char>();
        }
        n -= 1;
        s = s.offset(1);
    }
    s = (*pool).start;
    (*pool).start = (*pool).ptr;
    return s;
}

unsafe extern "C" fn poolStoreString(
    mut pool: *mut STRING_POOL,
    mut enc: *const crate::src::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> *mut crate::expat_external_h::XML_Char {
    if poolAppend(pool, enc, ptr, end).is_null() {
        return ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
    }
    if (*pool).ptr == (*pool).end as *mut crate::expat_external_h::XML_Char && poolGrow(pool) == 0 {
        return ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
    }
    let c2rust_fresh10 = (*pool).ptr;
    (*pool).ptr = (*pool).ptr.offset(1);
    *c2rust_fresh10 = 0 as crate::expat_external_h::XML_Char;
    return (*pool).start;
}

fn poolBytesToAllocateFor(mut blockSize: ::core::ffi::c_int) -> crate::__stddef_size_t_h::size_t {
    let stretch: crate::__stddef_size_t_h::size_t = ::core::mem::size_of::<
        crate::expat_external_h::XML_Char,
    >() as crate::__stddef_size_t_h::size_t;
    if blockSize <= 0 as ::core::ffi::c_int {
        return 0 as crate::__stddef_size_t_h::size_t;
    }
    if blockSize
        > (crate::limits_h::INT_MAX as crate::__stddef_size_t_h::size_t).wrapping_div(stretch)
            as ::core::ffi::c_int
    {
        return 0 as crate::__stddef_size_t_h::size_t;
    }
    let stretchedBlockSize: ::core::ffi::c_int = blockSize * stretch as ::core::ffi::c_int;
    let bytesToAllocate: ::core::ffi::c_int = (12 as ::core::ffi::c_ulong)
        .wrapping_add(stretchedBlockSize as ::core::ffi::c_uint as ::core::ffi::c_ulong)
        as ::core::ffi::c_int;
    if bytesToAllocate < 0 as ::core::ffi::c_int {
        return 0 as crate::__stddef_size_t_h::size_t;
    }
    return bytesToAllocate as crate::__stddef_size_t_h::size_t;
}

unsafe extern "C" fn poolGrow(mut pool: *mut STRING_POOL) -> crate::expat_h::XML_Bool {
    if !(*pool).freeBlocks.is_null() {
        if (*pool).start.is_null() {
            (*pool).blocks = (*pool).freeBlocks;
            (*pool).freeBlocks = (*(*pool).freeBlocks).next as *mut BLOCK;
            (*(*pool).blocks).next = ::core::ptr::null_mut::<block>();
            (*pool).start = &raw mut (*(*pool).blocks).s as *mut crate::expat_external_h::XML_Char;
            (*pool).end = (*pool).start.offset((*(*pool).blocks).size as isize);
            (*pool).ptr = (*pool).start;
            return crate::expat_h::XML_TRUE;
        }
        if ((*pool).end.offset_from((*pool).start) as ::core::ffi::c_long)
            < (*(*pool).freeBlocks).size as ::core::ffi::c_long
        {
            let mut tem: *mut BLOCK = (*(*pool).freeBlocks).next as *mut BLOCK;
            (*(*pool).freeBlocks).next = (*pool).blocks as *mut block;
            (*pool).blocks = (*pool).freeBlocks;
            (*pool).freeBlocks = tem;
            crate::stdlib::memcpy(
                &raw mut (*(*pool).blocks).s as *mut crate::expat_external_h::XML_Char
                    as *mut ::core::ffi::c_void,
                (*pool).start as *const ::core::ffi::c_void,
                ((*pool).end.offset_from((*pool).start) as ::core::ffi::c_long
                    as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(::core::mem::size_of::<crate::expat_external_h::XML_Char>()
                        as crate::__stddef_size_t_h::size_t),
            );
            (*pool).ptr = (&raw mut (*(*pool).blocks).s as *mut crate::expat_external_h::XML_Char)
                .offset((*pool).ptr.offset_from((*pool).start) as ::core::ffi::c_long as isize);
            (*pool).start = &raw mut (*(*pool).blocks).s as *mut crate::expat_external_h::XML_Char;
            (*pool).end = (*pool).start.offset((*(*pool).blocks).size as isize);
            return crate::expat_h::XML_TRUE;
        }
    }
    if !(*pool).blocks.is_null()
        && (*pool).start == &raw mut (*(*pool).blocks).s as *mut crate::expat_external_h::XML_Char
    {
        let mut temp: *mut BLOCK = ::core::ptr::null_mut::<BLOCK>();
        let mut blockSize: ::core::ffi::c_int =
            ((*pool).end.offset_from((*pool).start) as ::core::ffi::c_long as ::core::ffi::c_uint)
                .wrapping_mul(2 as ::core::ffi::c_uint) as ::core::ffi::c_int;
        let mut bytesToAllocate: crate::__stddef_size_t_h::size_t = 0;
        let offsetInsideBlock: crate::__stddef_ptrdiff_t_h::ptrdiff_t =
            (*pool).ptr.offset_from((*pool).start) as crate::__stddef_ptrdiff_t_h::ptrdiff_t;
        if blockSize < 0 as ::core::ffi::c_int {
            return crate::expat_h::XML_FALSE;
        }
        bytesToAllocate = poolBytesToAllocateFor(blockSize);
        if bytesToAllocate == 0 as crate::__stddef_size_t_h::size_t {
            return crate::expat_h::XML_FALSE;
        }
        temp = expat_realloc(
            (*pool).parser,
            (*pool).blocks as *mut ::core::ffi::c_void,
            bytesToAllocate,
            8161 as ::core::ffi::c_int,
        ) as *mut BLOCK;
        if temp.is_null() {
            return crate::expat_h::XML_FALSE;
        }
        (*pool).blocks = temp;
        (*(*pool).blocks).size = blockSize;
        (*pool).ptr = (&raw mut (*(*pool).blocks).s as *mut crate::expat_external_h::XML_Char)
            .offset(offsetInsideBlock as isize);
        (*pool).start = &raw mut (*(*pool).blocks).s as *mut crate::expat_external_h::XML_Char;
        (*pool).end = (*pool).start.offset(blockSize as isize);
    } else {
        let mut tem_0: *mut BLOCK = ::core::ptr::null_mut::<BLOCK>();
        let mut blockSize_0: ::core::ffi::c_int =
            (*pool).end.offset_from((*pool).start) as ::core::ffi::c_long as ::core::ffi::c_int;
        let mut bytesToAllocate_0: crate::__stddef_size_t_h::size_t = 0;
        if blockSize_0 < 0 as ::core::ffi::c_int {
            return crate::expat_h::XML_FALSE;
        }
        if blockSize_0 < INIT_BLOCK_SIZE {
            blockSize_0 = INIT_BLOCK_SIZE;
        } else {
            if ((blockSize_0 as ::core::ffi::c_uint).wrapping_mul(2 as ::core::ffi::c_uint)
                as ::core::ffi::c_int)
                < 0 as ::core::ffi::c_int
            {
                return crate::expat_h::XML_FALSE;
            }
            blockSize_0 *= 2 as ::core::ffi::c_int;
        }
        bytesToAllocate_0 = poolBytesToAllocateFor(blockSize_0);
        if bytesToAllocate_0 == 0 as crate::__stddef_size_t_h::size_t {
            return crate::expat_h::XML_FALSE;
        }
        tem_0 = expat_malloc(
            (*pool).parser,
            bytesToAllocate_0,
            8201 as ::core::ffi::c_int,
        ) as *mut BLOCK;
        if tem_0.is_null() {
            return crate::expat_h::XML_FALSE;
        }
        (*tem_0).size = blockSize_0;
        (*tem_0).next = (*pool).blocks as *mut block;
        (*pool).blocks = tem_0;
        if (*pool).ptr != (*pool).start {
            crate::stdlib::memcpy(
                &raw mut (*tem_0).s as *mut crate::expat_external_h::XML_Char
                    as *mut ::core::ffi::c_void,
                (*pool).start as *const ::core::ffi::c_void,
                ((*pool).ptr.offset_from((*pool).start) as ::core::ffi::c_long
                    as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(::core::mem::size_of::<crate::expat_external_h::XML_Char>()
                        as crate::__stddef_size_t_h::size_t),
            );
        }
        (*pool).ptr = (&raw mut (*tem_0).s as *mut crate::expat_external_h::XML_Char)
            .offset((*pool).ptr.offset_from((*pool).start) as ::core::ffi::c_long as isize);
        (*pool).start = &raw mut (*tem_0).s as *mut crate::expat_external_h::XML_Char;
        (*pool).end = (&raw mut (*tem_0).s as *mut crate::expat_external_h::XML_Char)
            .offset(blockSize_0 as isize);
    }
    return crate::expat_h::XML_TRUE;
}

unsafe extern "C" fn nextScaffoldPart(
    mut parser: crate::expat_h::XML_Parser,
) -> ::core::ffi::c_int {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut me: *mut CONTENT_SCAFFOLD = ::core::ptr::null_mut::<CONTENT_SCAFFOLD>();
    let mut next: ::core::ffi::c_int = 0;
    if (*dtd).scaffIndex.is_null() {
        (*dtd).scaffIndex = expat_malloc(
            parser,
            ((*parser).m_groupSize as crate::__stddef_size_t_h::size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>()
                    as crate::__stddef_size_t_h::size_t),
            8232 as ::core::ffi::c_int,
        ) as *mut ::core::ffi::c_int;
        if (*dtd).scaffIndex.is_null() {
            return -1 as ::core::ffi::c_int;
        }
        *(*dtd).scaffIndex.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int;
    }
    if (*dtd).scaffCount > crate::limits_h::INT_MAX as ::core::ffi::c_uint {
        return -1 as ::core::ffi::c_int;
    }
    if (*dtd).scaffCount >= (*dtd).scaffSize {
        let mut temp: *mut CONTENT_SCAFFOLD = ::core::ptr::null_mut::<CONTENT_SCAFFOLD>();
        if !(*dtd).scaffold.is_null() {
            if (*dtd).scaffSize > crate::limits_h::UINT_MAX.wrapping_div(2 as ::core::ffi::c_uint) {
                return -1 as ::core::ffi::c_int;
            }
            temp = expat_realloc(
                parser,
                (*dtd).scaffold as *mut ::core::ffi::c_void,
                ((*dtd).scaffSize.wrapping_mul(2 as ::core::ffi::c_uint)
                    as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(::core::mem::size_of::<CONTENT_SCAFFOLD>()
                        as crate::__stddef_size_t_h::size_t),
                8261 as ::core::ffi::c_int,
            ) as *mut CONTENT_SCAFFOLD;
            if temp.is_null() {
                return -1 as ::core::ffi::c_int;
            }
            (*dtd).scaffSize = (*dtd).scaffSize.wrapping_mul(2 as ::core::ffi::c_uint);
        } else {
            temp = expat_malloc(
                parser,
                (32 as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(::core::mem::size_of::<CONTENT_SCAFFOLD>()
                        as crate::__stddef_size_t_h::size_t),
                8266 as ::core::ffi::c_int,
            ) as *mut CONTENT_SCAFFOLD;
            if temp.is_null() {
                return -1 as ::core::ffi::c_int;
            }
            (*dtd).scaffSize = INIT_SCAFFOLD_ELEMENTS as ::core::ffi::c_uint;
        }
        (*dtd).scaffold = temp;
    }
    let c2rust_fresh14 = (*dtd).scaffCount;
    (*dtd).scaffCount = (*dtd).scaffCount.wrapping_add(1);
    next = c2rust_fresh14 as ::core::ffi::c_int;
    me = (*dtd).scaffold.offset(next as isize) as *mut CONTENT_SCAFFOLD;
    if (*dtd).scaffLevel != 0 {
        let mut parent: *mut CONTENT_SCAFFOLD = (*dtd).scaffold.offset(
            *(*dtd)
                .scaffIndex
                .offset(((*dtd).scaffLevel - 1 as ::core::ffi::c_int) as isize)
                as isize,
        ) as *mut CONTENT_SCAFFOLD;
        if (*parent).lastchild != 0 {
            (*(*dtd).scaffold.offset((*parent).lastchild as isize)).nextsib = next;
        }
        if (*parent).childcnt == 0 {
            (*parent).firstchild = next;
        }
        (*parent).lastchild = next;
        (*parent).childcnt += 1;
    }
    (*me).nextsib = 0 as ::core::ffi::c_int;
    (*me).childcnt = (*me).nextsib;
    (*me).lastchild = (*me).childcnt;
    (*me).firstchild = (*me).lastchild;
    return next;
}

unsafe extern "C" fn build_model(
    mut parser: crate::expat_h::XML_Parser,
) -> *mut crate::expat_h::XML_Content {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut ret: *mut crate::expat_h::XML_Content =
        ::core::ptr::null_mut::<crate::expat_h::XML_Content>();
    let mut str: *mut crate::expat_external_h::XML_Char =
        ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
    if ((*dtd).scaffCount as usize)
        .wrapping_mul(::core::mem::size_of::<crate::expat_h::XML_Content>() as usize)
        > (crate::stdlib::SIZE_MAX as usize).wrapping_sub(
            ((*dtd).contentStringLen as usize)
                .wrapping_mul(::core::mem::size_of::<crate::expat_external_h::XML_Char>() as usize),
        )
    {
        return ::core::ptr::null_mut::<crate::expat_h::XML_Content>();
    }
    let allocsize: crate::__stddef_size_t_h::size_t = ((*dtd).scaffCount
        as crate::__stddef_size_t_h::size_t)
        .wrapping_mul(::core::mem::size_of::<crate::expat_h::XML_Content>()
            as crate::__stddef_size_t_h::size_t)
        .wrapping_add(
            ((*dtd).contentStringLen as crate::__stddef_size_t_h::size_t)
                .wrapping_mul(::core::mem::size_of::<crate::expat_external_h::XML_Char>()
                    as crate::__stddef_size_t_h::size_t),
        );
    ret = (*parser)
        .m_mem
        .malloc_fcn
        .expect("non-null function pointer")(allocsize)
        as *mut crate::expat_h::XML_Content;
    if ret.is_null() {
        return ::core::ptr::null_mut::<crate::expat_h::XML_Content>();
    }
    let mut dest: *mut crate::expat_h::XML_Content = ret;
    let destLimit: *mut crate::expat_h::XML_Content =
        ret.offset((*dtd).scaffCount as isize) as *mut crate::expat_h::XML_Content;
    let mut jobDest: *mut crate::expat_h::XML_Content = ret;
    str = ret.offset((*dtd).scaffCount as isize) as *mut crate::expat_h::XML_Content
        as *mut crate::expat_external_h::XML_Char;
    let c2rust_fresh11 = jobDest;
    jobDest = jobDest.offset(1);
    (*c2rust_fresh11).numchildren = 0 as ::core::ffi::c_uint;
    while dest < destLimit {
        let src_node: ::core::ffi::c_int = (*dest).numchildren as ::core::ffi::c_int;
        (*dest).type_0 = (*(*dtd).scaffold.offset(src_node as isize)).type_0;
        (*dest).quant = (*(*dtd).scaffold.offset(src_node as isize)).quant;
        if (*dest).type_0 as ::core::ffi::c_uint
            == crate::expat_h::XML_CTYPE_NAME as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut src: *const crate::expat_external_h::XML_Char =
                ::core::ptr::null::<crate::expat_external_h::XML_Char>();
            (*dest).name = str;
            src = (*(*dtd).scaffold.offset(src_node as isize)).name;
            loop {
                let c2rust_fresh12 = str;
                str = str.offset(1);
                *c2rust_fresh12 = *src;
                if *src == 0 {
                    break;
                }
                src = src.offset(1);
            }
            (*dest).numchildren = 0 as ::core::ffi::c_uint;
            (*dest).children = ::core::ptr::null_mut::<crate::expat_h::XML_Content>();
        } else {
            let mut i: ::core::ffi::c_uint = 0;
            let mut cn: ::core::ffi::c_int = 0;
            (*dest).name = ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
            (*dest).numchildren =
                (*(*dtd).scaffold.offset(src_node as isize)).childcnt as ::core::ffi::c_uint;
            (*dest).children = jobDest;
            i = 0 as ::core::ffi::c_uint;
            cn = (*(*dtd).scaffold.offset(src_node as isize)).firstchild;
            while i < (*dest).numchildren {
                let c2rust_fresh13 = jobDest;
                jobDest = jobDest.offset(1);
                (*c2rust_fresh13).numchildren = cn as ::core::ffi::c_uint;
                i = i.wrapping_add(1);
                cn = (*(*dtd).scaffold.offset(cn as isize)).nextsib;
            }
        }
        dest = dest.offset(1);
    }
    return ret;
}

unsafe extern "C" fn getElementType(
    mut parser: crate::expat_h::XML_Parser,
    mut enc: *const crate::src::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> *mut ELEMENT_TYPE {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut name: *const crate::expat_external_h::XML_Char =
        poolStoreString(&raw mut (*dtd).pool, enc, ptr, end);
    let mut ret: *mut ELEMENT_TYPE = ::core::ptr::null_mut::<ELEMENT_TYPE>();
    if name.is_null() {
        return ::core::ptr::null_mut::<ELEMENT_TYPE>();
    }
    ret = lookup(
        parser,
        &raw mut (*dtd).elementTypes,
        name as KEY,
        ::core::mem::size_of::<ELEMENT_TYPE>() as crate::__stddef_size_t_h::size_t,
    ) as *mut ELEMENT_TYPE;
    if ret.is_null() {
        return ::core::ptr::null_mut::<ELEMENT_TYPE>();
    }
    if (*ret).name != name {
        (*dtd).pool.ptr = (*dtd).pool.start;
    } else {
        (*dtd).pool.start = (*dtd).pool.ptr;
        if setElementTypePrefix(parser, ret) == 0 {
            return ::core::ptr::null_mut::<ELEMENT_TYPE>();
        }
    }
    return ret;
}

fn accountingGetCurrentAmplification(rootParser: &XML_ParserStruct) -> ::core::ffi::c_float {
    let lenOfShortestInclude: crate::__stddef_size_t_h::size_t =
        (::core::mem::size_of::<[::core::ffi::c_char; 23]>() as crate::__stddef_size_t_h::size_t)
            .wrapping_sub(1 as crate::__stddef_size_t_h::size_t);
    let countBytesOutput: XmlBigCount = rootParser
        .m_accounting
        .countBytesDirect
        .wrapping_add(rootParser.m_accounting.countBytesIndirect);
    let amplificationFactor: ::core::ffi::c_float = if rootParser.m_accounting.countBytesDirect != 0
    {
        countBytesOutput as ::core::ffi::c_float
            / rootParser.m_accounting.countBytesDirect as ::core::ffi::c_float
    } else {
        (lenOfShortestInclude as XmlBigCount)
            .wrapping_add(rootParser.m_accounting.countBytesIndirect)
            as ::core::ffi::c_float
            / lenOfShortestInclude as ::core::ffi::c_float
    };
    if !rootParser.m_parentParser.is_null() {
        ::std::process::abort();
    }
    return amplificationFactor;
}

fn accountingReportStats(rootParser: &XML_ParserStruct, epilog: &str) {
    if !rootParser.m_parentParser.is_null() {
        std::process::abort();
    }
    if rootParser.m_accounting.debugLevel == 0 as ::core::ffi::c_ulong {
        return;
    }
    let amplificationFactor: ::core::ffi::c_float =
        accountingGetCurrentAmplification(rootParser) as ::core::ffi::c_float;
    use std::io::Write as _;

    let _ = write!(
        std::io::stderr(),
        "expat: Accounting({:p}): Direct {:10}, indirect {:10}, amplification {:8.2}{}",
        rootParser,
        rootParser.m_accounting.countBytesDirect,
        rootParser.m_accounting.countBytesIndirect,
        amplificationFactor,
        epilog,
    );
}

fn write_printable_byte(
    writer: &mut impl std::io::Write,
    c: ::core::ffi::c_uchar,
) -> std::io::Result<()> {
    match c {
        0 => writer.write_all(b"\\0"),
        9 => writer.write_all(b"\\t"),
        10 => writer.write_all(b"\\n"),
        13 => writer.write_all(b"\\r"),
        b'"' => writer.write_all(b"\\\""),
        b'\\' => writer.write_all(b"\\\\"),
        32..=126 => writer.write_all(&[c]),
        1..=15 => write!(writer, "\\x{:X}", c),
        _ => write!(writer, "\\x{:02X}", c),
    }
}

fn accountingReportDiff(
    rootParser: &XML_ParserStruct,
    levelsAwayFromRootParser: ::core::ffi::c_uint,
    context: &[::core::ffi::c_uchar],
    bytesMore: crate::__stddef_ptrdiff_t_h::ptrdiff_t,
    source_line: ::core::ffi::c_int,
    account: XML_Account,
) {
    if !rootParser.m_parentParser.is_null() {
        std::process::abort();
    }

    use std::io::Write as _;

    let account_label = if account as ::core::ffi::c_uint
        == XML_ACCOUNT_DIRECT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        "DIR"
    } else {
        "EXP"
    };
    let contextLength = 10usize;
    let ellipsis = b"[..]";
    let mut stderr = std::io::stderr();

    let _ = write!(
        stderr,
        " (+{:6} bytes {}|{}, xmlparse.c:{}) {:10}\"",
        bytesMore, account_label, levelsAwayFromRootParser, source_line, ""
    );

    if rootParser.m_accounting.debugLevel >= 3 as ::core::ffi::c_ulong
        || context.len() <= contextLength + ellipsis.len() + contextLength
    {
        for &byte in context {
            let _ = write_printable_byte(&mut stderr, byte);
        }
    } else {
        for &byte in &context[..contextLength] {
            let _ = write_printable_byte(&mut stderr, byte);
        }
        let _ = stderr.write_all(ellipsis);
        for &byte in &context[context.len() - contextLength..] {
            let _ = write_printable_byte(&mut stderr, byte);
        }
    }
    let _ = stderr.write_all(b"\"\n");
}

fn accountingDiffTolerated<'a, F>(
    rootParser: &mut XML_ParserStruct,
    levelsAwayFromRootParser: ::core::ffi::c_uint,
    origin_is_root_parser: bool,
    tok: ::core::ffi::c_int,
    context: F,
    source_line: ::core::ffi::c_int,
    account: XML_Account,
) -> crate::expat_h::XML_Bool
where
    F: FnOnce() -> &'a [::core::ffi::c_uchar],
{
    match tok {
        crate::src::xmltok::XML_TOK_INVALID
        | crate::src::xmltok::XML_TOK_PARTIAL
        | crate::src::xmltok::XML_TOK_PARTIAL_CHAR
        | crate::src::xmltok::XML_TOK_NONE => {
            return crate::expat_h::XML_TRUE;
        }
        _ => {}
    }
    if account as ::core::ffi::c_uint
        == XML_ACCOUNT_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return crate::expat_h::XML_TRUE;
    }
    if !rootParser.m_parentParser.is_null() {
        std::process::abort();
    }
    let isDirect: ::core::ffi::c_int = (account as ::core::ffi::c_uint
        == XML_ACCOUNT_DIRECT as ::core::ffi::c_int as ::core::ffi::c_uint
        && origin_is_root_parser) as ::core::ffi::c_int;
    let context = context();
    let bytesMore: crate::__stddef_ptrdiff_t_h::ptrdiff_t =
        context.len() as crate::__stddef_ptrdiff_t_h::ptrdiff_t;
    let additionTarget = if isDirect != 0 {
        &mut rootParser.m_accounting.countBytesDirect
    } else {
        &mut rootParser.m_accounting.countBytesIndirect
    };
    if *additionTarget
        > (-1 as ::core::ffi::c_int as XmlBigCount).wrapping_sub(bytesMore as XmlBigCount)
    {
        return crate::expat_h::XML_FALSE;
    }
    *additionTarget = (*additionTarget).wrapping_add(bytesMore as XmlBigCount);
    let countBytesOutput: XmlBigCount = rootParser
        .m_accounting
        .countBytesDirect
        .wrapping_add(rootParser.m_accounting.countBytesIndirect);
    let amplificationFactor: ::core::ffi::c_float =
        accountingGetCurrentAmplification(rootParser) as ::core::ffi::c_float;
    let tolerated: crate::expat_h::XML_Bool =
        (countBytesOutput < rootParser.m_accounting.activationThresholdBytes
            || amplificationFactor <= rootParser.m_accounting.maximumAmplificationFactor)
            as ::core::ffi::c_int as crate::expat_h::XML_Bool;
    if rootParser.m_accounting.debugLevel >= 2 as ::core::ffi::c_ulong {
        accountingReportStats(rootParser, "");
        accountingReportDiff(
            rootParser,
            levelsAwayFromRootParser,
            context,
            bytesMore,
            source_line,
            account,
        );
    }
    return tolerated;
}
pub fn testingAccountingGetCountBytesDirect(
    parser: Option<&XML_ParserStruct>,
) -> ::core::ffi::c_ulonglong {
    match parser {
        Some(parser) => parser.m_accounting.countBytesDirect as ::core::ffi::c_ulonglong,
        None => 0 as ::core::ffi::c_ulonglong,
    }
}
#[export_name = "testingAccountingGetCountBytesDirect"]

pub unsafe extern "C" fn testingAccountingGetCountBytesDirect_ffi(
    mut parser: crate::expat_h::XML_Parser,
) -> ::core::ffi::c_ulonglong {
    testingAccountingGetCountBytesDirect(parser.as_ref())
}
pub fn testingAccountingGetCountBytesIndirect(
    parser: Option<&XML_ParserStruct>,
) -> ::core::ffi::c_ulonglong {
    match parser {
        Some(parser) => parser.m_accounting.countBytesIndirect as ::core::ffi::c_ulonglong,
        None => 0 as ::core::ffi::c_ulonglong,
    }
}
#[export_name = "testingAccountingGetCountBytesIndirect"]

pub unsafe extern "C" fn testingAccountingGetCountBytesIndirect_ffi(
    mut parser: crate::expat_h::XML_Parser,
) -> ::core::ffi::c_ulonglong {
    testingAccountingGetCountBytesIndirect(parser.as_ref())
}
fn entityTrackingReportStats(
    rootParser: &XML_ParserStruct,
    entity: &ENTITY,
    entity_name: &::std::ffi::CStr,
    action: &str,
    sourceLine: ::core::ffi::c_int,
) {
    if !rootParser.m_parentParser.is_null() {
        ::std::process::abort();
    }
    if rootParser.m_entity_stats.debugLevel == 0 as ::core::ffi::c_ulong {
        return;
    }

    let indent_width =
        ((rootParser.m_entity_stats.currentDepth as ::core::ffi::c_int - 1) * 2).max(0) as usize;
    let entity_prefix = if entity.is_param as ::core::ffi::c_int != 0 {
        "%"
    } else {
        "&"
    };
    use std::io::Write as _;

    let mut stderr = std::io::stderr();
    let _ = write!(
        stderr,
        "expat: Entities({:p}): Count {:9}, depth {:2}/{:2} {:indent_width$}{}",
        rootParser,
        rootParser.m_entity_stats.countEverOpened,
        rootParser.m_entity_stats.currentDepth,
        rootParser.m_entity_stats.maximumDepthSeen,
        "",
        entity_prefix,
    );
    let _ = stderr.write_all(entity_name.to_bytes());
    let _ = writeln!(
        stderr,
        "; {} length {} (xmlparse.c:{})",
        action, entity.textLen, sourceLine,
    );
}

fn entityTrackingOnOpen(rootParser: &mut XML_ParserStruct) {
    if !rootParser.m_parentParser.is_null() {
        std::process::abort();
    }
    rootParser.m_entity_stats.countEverOpened =
        rootParser.m_entity_stats.countEverOpened.wrapping_add(1);
    rootParser.m_entity_stats.currentDepth = rootParser.m_entity_stats.currentDepth.wrapping_add(1);
    if rootParser.m_entity_stats.currentDepth > rootParser.m_entity_stats.maximumDepthSeen {
        rootParser.m_entity_stats.maximumDepthSeen =
            rootParser.m_entity_stats.maximumDepthSeen.wrapping_add(1);
    }
}

fn entityTrackingOnClose(rootParser: &mut XML_ParserStruct) {
    if !rootParser.m_parentParser.is_null() {
        std::process::abort();
    }
    rootParser.m_entity_stats.currentDepth = rootParser.m_entity_stats.currentDepth.wrapping_sub(1);
}

unsafe extern "C" fn getRootParserOf(
    mut parser: crate::expat_h::XML_Parser,
    mut outLevelDiff: *mut ::core::ffi::c_uint,
) -> crate::expat_h::XML_Parser {
    let mut rootParser: crate::expat_h::XML_Parser = parser;
    let mut stepsTakenUpwards: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while !(*rootParser).m_parentParser.is_null() {
        rootParser = (*rootParser).m_parentParser;
        stepsTakenUpwards = stepsTakenUpwards.wrapping_add(1);
    }
    '_c2rust_label: {
        if (*rootParser).m_parentParser.is_null() {
        } else {
            crate::stdlib::__assert_fail(
                b"! rootParser->m_parentParser\0".as_ptr() as *const ::core::ffi::c_char,
                b"../../expat/lib/xmlparse.c\0".as_ptr() as *const ::core::ffi::c_char,
                8672 as ::core::ffi::c_uint,
                b"XML_Parser getRootParserOf(XML_Parser, unsigned int *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if !outLevelDiff.is_null() {
        *outLevelDiff = stepsTakenUpwards;
    }
    return rootParser;
}
pub extern "C" fn unsignedCharToPrintable(
    mut c: ::core::ffi::c_uchar,
) -> *const ::core::ffi::c_char {
    match c as ::core::ffi::c_int {
        0 => return b"\\0\0".as_ptr() as *const ::core::ffi::c_char,
        1 => return b"\\x1\0".as_ptr() as *const ::core::ffi::c_char,
        2 => return b"\\x2\0".as_ptr() as *const ::core::ffi::c_char,
        3 => return b"\\x3\0".as_ptr() as *const ::core::ffi::c_char,
        4 => return b"\\x4\0".as_ptr() as *const ::core::ffi::c_char,
        5 => return b"\\x5\0".as_ptr() as *const ::core::ffi::c_char,
        6 => return b"\\x6\0".as_ptr() as *const ::core::ffi::c_char,
        7 => return b"\\x7\0".as_ptr() as *const ::core::ffi::c_char,
        8 => return b"\\x8\0".as_ptr() as *const ::core::ffi::c_char,
        9 => return b"\\t\0".as_ptr() as *const ::core::ffi::c_char,
        10 => return b"\\n\0".as_ptr() as *const ::core::ffi::c_char,
        11 => return b"\\xB\0".as_ptr() as *const ::core::ffi::c_char,
        12 => return b"\\xC\0".as_ptr() as *const ::core::ffi::c_char,
        13 => return b"\\r\0".as_ptr() as *const ::core::ffi::c_char,
        14 => return b"\\xE\0".as_ptr() as *const ::core::ffi::c_char,
        15 => return b"\\xF\0".as_ptr() as *const ::core::ffi::c_char,
        16 => return b"\\x10\0".as_ptr() as *const ::core::ffi::c_char,
        17 => return b"\\x11\0".as_ptr() as *const ::core::ffi::c_char,
        18 => return b"\\x12\0".as_ptr() as *const ::core::ffi::c_char,
        19 => return b"\\x13\0".as_ptr() as *const ::core::ffi::c_char,
        20 => return b"\\x14\0".as_ptr() as *const ::core::ffi::c_char,
        21 => return b"\\x15\0".as_ptr() as *const ::core::ffi::c_char,
        22 => return b"\\x16\0".as_ptr() as *const ::core::ffi::c_char,
        23 => return b"\\x17\0".as_ptr() as *const ::core::ffi::c_char,
        24 => return b"\\x18\0".as_ptr() as *const ::core::ffi::c_char,
        25 => return b"\\x19\0".as_ptr() as *const ::core::ffi::c_char,
        26 => return b"\\x1A\0".as_ptr() as *const ::core::ffi::c_char,
        27 => return b"\\x1B\0".as_ptr() as *const ::core::ffi::c_char,
        28 => return b"\\x1C\0".as_ptr() as *const ::core::ffi::c_char,
        29 => return b"\\x1D\0".as_ptr() as *const ::core::ffi::c_char,
        30 => return b"\\x1E\0".as_ptr() as *const ::core::ffi::c_char,
        31 => return b"\\x1F\0".as_ptr() as *const ::core::ffi::c_char,
        32 => return b" \0".as_ptr() as *const ::core::ffi::c_char,
        33 => return b"!\0".as_ptr() as *const ::core::ffi::c_char,
        34 => return b"\\\"\0".as_ptr() as *const ::core::ffi::c_char,
        35 => return b"#\0".as_ptr() as *const ::core::ffi::c_char,
        36 => return b"$\0".as_ptr() as *const ::core::ffi::c_char,
        37 => return b"%\0".as_ptr() as *const ::core::ffi::c_char,
        38 => return b"&\0".as_ptr() as *const ::core::ffi::c_char,
        39 => return b"'\0".as_ptr() as *const ::core::ffi::c_char,
        40 => return b"(\0".as_ptr() as *const ::core::ffi::c_char,
        41 => return b")\0".as_ptr() as *const ::core::ffi::c_char,
        42 => return b"*\0".as_ptr() as *const ::core::ffi::c_char,
        43 => return b"+\0".as_ptr() as *const ::core::ffi::c_char,
        44 => return b",\0".as_ptr() as *const ::core::ffi::c_char,
        45 => return b"-\0".as_ptr() as *const ::core::ffi::c_char,
        46 => return b".\0".as_ptr() as *const ::core::ffi::c_char,
        47 => return b"/\0".as_ptr() as *const ::core::ffi::c_char,
        48 => return b"0\0".as_ptr() as *const ::core::ffi::c_char,
        49 => return b"1\0".as_ptr() as *const ::core::ffi::c_char,
        50 => return b"2\0".as_ptr() as *const ::core::ffi::c_char,
        51 => return b"3\0".as_ptr() as *const ::core::ffi::c_char,
        52 => return b"4\0".as_ptr() as *const ::core::ffi::c_char,
        53 => return b"5\0".as_ptr() as *const ::core::ffi::c_char,
        54 => return b"6\0".as_ptr() as *const ::core::ffi::c_char,
        55 => return b"7\0".as_ptr() as *const ::core::ffi::c_char,
        56 => return b"8\0".as_ptr() as *const ::core::ffi::c_char,
        57 => return b"9\0".as_ptr() as *const ::core::ffi::c_char,
        58 => return b":\0".as_ptr() as *const ::core::ffi::c_char,
        59 => return b";\0".as_ptr() as *const ::core::ffi::c_char,
        60 => return b"<\0".as_ptr() as *const ::core::ffi::c_char,
        61 => return b"=\0".as_ptr() as *const ::core::ffi::c_char,
        62 => return b">\0".as_ptr() as *const ::core::ffi::c_char,
        63 => return b"?\0".as_ptr() as *const ::core::ffi::c_char,
        64 => return b"@\0".as_ptr() as *const ::core::ffi::c_char,
        65 => return b"A\0".as_ptr() as *const ::core::ffi::c_char,
        66 => return b"B\0".as_ptr() as *const ::core::ffi::c_char,
        67 => return b"C\0".as_ptr() as *const ::core::ffi::c_char,
        68 => return b"D\0".as_ptr() as *const ::core::ffi::c_char,
        69 => return b"E\0".as_ptr() as *const ::core::ffi::c_char,
        70 => return b"F\0".as_ptr() as *const ::core::ffi::c_char,
        71 => return b"G\0".as_ptr() as *const ::core::ffi::c_char,
        72 => return b"H\0".as_ptr() as *const ::core::ffi::c_char,
        73 => return b"I\0".as_ptr() as *const ::core::ffi::c_char,
        74 => return b"J\0".as_ptr() as *const ::core::ffi::c_char,
        75 => return b"K\0".as_ptr() as *const ::core::ffi::c_char,
        76 => return b"L\0".as_ptr() as *const ::core::ffi::c_char,
        77 => return b"M\0".as_ptr() as *const ::core::ffi::c_char,
        78 => return b"N\0".as_ptr() as *const ::core::ffi::c_char,
        79 => return b"O\0".as_ptr() as *const ::core::ffi::c_char,
        80 => return b"P\0".as_ptr() as *const ::core::ffi::c_char,
        81 => return b"Q\0".as_ptr() as *const ::core::ffi::c_char,
        82 => return b"R\0".as_ptr() as *const ::core::ffi::c_char,
        83 => return b"S\0".as_ptr() as *const ::core::ffi::c_char,
        84 => return b"T\0".as_ptr() as *const ::core::ffi::c_char,
        85 => return b"U\0".as_ptr() as *const ::core::ffi::c_char,
        86 => return b"V\0".as_ptr() as *const ::core::ffi::c_char,
        87 => return b"W\0".as_ptr() as *const ::core::ffi::c_char,
        88 => return b"X\0".as_ptr() as *const ::core::ffi::c_char,
        89 => return b"Y\0".as_ptr() as *const ::core::ffi::c_char,
        90 => return b"Z\0".as_ptr() as *const ::core::ffi::c_char,
        91 => return b"[\0".as_ptr() as *const ::core::ffi::c_char,
        92 => return b"\\\\\0".as_ptr() as *const ::core::ffi::c_char,
        93 => return b"]\0".as_ptr() as *const ::core::ffi::c_char,
        94 => return b"^\0".as_ptr() as *const ::core::ffi::c_char,
        95 => return b"_\0".as_ptr() as *const ::core::ffi::c_char,
        96 => return b"`\0".as_ptr() as *const ::core::ffi::c_char,
        97 => return b"a\0".as_ptr() as *const ::core::ffi::c_char,
        98 => return b"b\0".as_ptr() as *const ::core::ffi::c_char,
        99 => return b"c\0".as_ptr() as *const ::core::ffi::c_char,
        100 => return b"d\0".as_ptr() as *const ::core::ffi::c_char,
        101 => return b"e\0".as_ptr() as *const ::core::ffi::c_char,
        102 => return b"f\0".as_ptr() as *const ::core::ffi::c_char,
        103 => return b"g\0".as_ptr() as *const ::core::ffi::c_char,
        104 => return b"h\0".as_ptr() as *const ::core::ffi::c_char,
        105 => return b"i\0".as_ptr() as *const ::core::ffi::c_char,
        106 => return b"j\0".as_ptr() as *const ::core::ffi::c_char,
        107 => return b"k\0".as_ptr() as *const ::core::ffi::c_char,
        108 => return b"l\0".as_ptr() as *const ::core::ffi::c_char,
        109 => return b"m\0".as_ptr() as *const ::core::ffi::c_char,
        110 => return b"n\0".as_ptr() as *const ::core::ffi::c_char,
        111 => return b"o\0".as_ptr() as *const ::core::ffi::c_char,
        112 => return b"p\0".as_ptr() as *const ::core::ffi::c_char,
        113 => return b"q\0".as_ptr() as *const ::core::ffi::c_char,
        114 => return b"r\0".as_ptr() as *const ::core::ffi::c_char,
        115 => return b"s\0".as_ptr() as *const ::core::ffi::c_char,
        116 => return b"t\0".as_ptr() as *const ::core::ffi::c_char,
        117 => return b"u\0".as_ptr() as *const ::core::ffi::c_char,
        118 => return b"v\0".as_ptr() as *const ::core::ffi::c_char,
        119 => return b"w\0".as_ptr() as *const ::core::ffi::c_char,
        120 => return b"x\0".as_ptr() as *const ::core::ffi::c_char,
        121 => return b"y\0".as_ptr() as *const ::core::ffi::c_char,
        122 => return b"z\0".as_ptr() as *const ::core::ffi::c_char,
        123 => return b"{\0".as_ptr() as *const ::core::ffi::c_char,
        124 => return b"|\0".as_ptr() as *const ::core::ffi::c_char,
        125 => return b"}\0".as_ptr() as *const ::core::ffi::c_char,
        126 => return b"~\0".as_ptr() as *const ::core::ffi::c_char,
        127 => return b"\\x7F\0".as_ptr() as *const ::core::ffi::c_char,
        128 => return b"\\x80\0".as_ptr() as *const ::core::ffi::c_char,
        129 => return b"\\x81\0".as_ptr() as *const ::core::ffi::c_char,
        130 => return b"\\x82\0".as_ptr() as *const ::core::ffi::c_char,
        131 => return b"\\x83\0".as_ptr() as *const ::core::ffi::c_char,
        132 => return b"\\x84\0".as_ptr() as *const ::core::ffi::c_char,
        133 => return b"\\x85\0".as_ptr() as *const ::core::ffi::c_char,
        134 => return b"\\x86\0".as_ptr() as *const ::core::ffi::c_char,
        135 => return b"\\x87\0".as_ptr() as *const ::core::ffi::c_char,
        136 => return b"\\x88\0".as_ptr() as *const ::core::ffi::c_char,
        137 => return b"\\x89\0".as_ptr() as *const ::core::ffi::c_char,
        138 => return b"\\x8A\0".as_ptr() as *const ::core::ffi::c_char,
        139 => return b"\\x8B\0".as_ptr() as *const ::core::ffi::c_char,
        140 => return b"\\x8C\0".as_ptr() as *const ::core::ffi::c_char,
        141 => return b"\\x8D\0".as_ptr() as *const ::core::ffi::c_char,
        142 => return b"\\x8E\0".as_ptr() as *const ::core::ffi::c_char,
        143 => return b"\\x8F\0".as_ptr() as *const ::core::ffi::c_char,
        144 => return b"\\x90\0".as_ptr() as *const ::core::ffi::c_char,
        145 => return b"\\x91\0".as_ptr() as *const ::core::ffi::c_char,
        146 => return b"\\x92\0".as_ptr() as *const ::core::ffi::c_char,
        147 => return b"\\x93\0".as_ptr() as *const ::core::ffi::c_char,
        148 => return b"\\x94\0".as_ptr() as *const ::core::ffi::c_char,
        149 => return b"\\x95\0".as_ptr() as *const ::core::ffi::c_char,
        150 => return b"\\x96\0".as_ptr() as *const ::core::ffi::c_char,
        151 => return b"\\x97\0".as_ptr() as *const ::core::ffi::c_char,
        152 => return b"\\x98\0".as_ptr() as *const ::core::ffi::c_char,
        153 => return b"\\x99\0".as_ptr() as *const ::core::ffi::c_char,
        154 => return b"\\x9A\0".as_ptr() as *const ::core::ffi::c_char,
        155 => return b"\\x9B\0".as_ptr() as *const ::core::ffi::c_char,
        156 => return b"\\x9C\0".as_ptr() as *const ::core::ffi::c_char,
        157 => return b"\\x9D\0".as_ptr() as *const ::core::ffi::c_char,
        158 => return b"\\x9E\0".as_ptr() as *const ::core::ffi::c_char,
        159 => return b"\\x9F\0".as_ptr() as *const ::core::ffi::c_char,
        160 => return b"\\xA0\0".as_ptr() as *const ::core::ffi::c_char,
        161 => return b"\\xA1\0".as_ptr() as *const ::core::ffi::c_char,
        162 => return b"\\xA2\0".as_ptr() as *const ::core::ffi::c_char,
        163 => return b"\\xA3\0".as_ptr() as *const ::core::ffi::c_char,
        164 => return b"\\xA4\0".as_ptr() as *const ::core::ffi::c_char,
        165 => return b"\\xA5\0".as_ptr() as *const ::core::ffi::c_char,
        166 => return b"\\xA6\0".as_ptr() as *const ::core::ffi::c_char,
        167 => return b"\\xA7\0".as_ptr() as *const ::core::ffi::c_char,
        168 => return b"\\xA8\0".as_ptr() as *const ::core::ffi::c_char,
        169 => return b"\\xA9\0".as_ptr() as *const ::core::ffi::c_char,
        170 => return b"\\xAA\0".as_ptr() as *const ::core::ffi::c_char,
        171 => return b"\\xAB\0".as_ptr() as *const ::core::ffi::c_char,
        172 => return b"\\xAC\0".as_ptr() as *const ::core::ffi::c_char,
        173 => return b"\\xAD\0".as_ptr() as *const ::core::ffi::c_char,
        174 => return b"\\xAE\0".as_ptr() as *const ::core::ffi::c_char,
        175 => return b"\\xAF\0".as_ptr() as *const ::core::ffi::c_char,
        176 => return b"\\xB0\0".as_ptr() as *const ::core::ffi::c_char,
        177 => return b"\\xB1\0".as_ptr() as *const ::core::ffi::c_char,
        178 => return b"\\xB2\0".as_ptr() as *const ::core::ffi::c_char,
        179 => return b"\\xB3\0".as_ptr() as *const ::core::ffi::c_char,
        180 => return b"\\xB4\0".as_ptr() as *const ::core::ffi::c_char,
        181 => return b"\\xB5\0".as_ptr() as *const ::core::ffi::c_char,
        182 => return b"\\xB6\0".as_ptr() as *const ::core::ffi::c_char,
        183 => return b"\\xB7\0".as_ptr() as *const ::core::ffi::c_char,
        184 => return b"\\xB8\0".as_ptr() as *const ::core::ffi::c_char,
        185 => return b"\\xB9\0".as_ptr() as *const ::core::ffi::c_char,
        186 => return b"\\xBA\0".as_ptr() as *const ::core::ffi::c_char,
        187 => return b"\\xBB\0".as_ptr() as *const ::core::ffi::c_char,
        188 => return b"\\xBC\0".as_ptr() as *const ::core::ffi::c_char,
        189 => return b"\\xBD\0".as_ptr() as *const ::core::ffi::c_char,
        190 => return b"\\xBE\0".as_ptr() as *const ::core::ffi::c_char,
        191 => return b"\\xBF\0".as_ptr() as *const ::core::ffi::c_char,
        192 => return b"\\xC0\0".as_ptr() as *const ::core::ffi::c_char,
        193 => return b"\\xC1\0".as_ptr() as *const ::core::ffi::c_char,
        194 => return b"\\xC2\0".as_ptr() as *const ::core::ffi::c_char,
        195 => return b"\\xC3\0".as_ptr() as *const ::core::ffi::c_char,
        196 => return b"\\xC4\0".as_ptr() as *const ::core::ffi::c_char,
        197 => return b"\\xC5\0".as_ptr() as *const ::core::ffi::c_char,
        198 => return b"\\xC6\0".as_ptr() as *const ::core::ffi::c_char,
        199 => return b"\\xC7\0".as_ptr() as *const ::core::ffi::c_char,
        200 => return b"\\xC8\0".as_ptr() as *const ::core::ffi::c_char,
        201 => return b"\\xC9\0".as_ptr() as *const ::core::ffi::c_char,
        202 => return b"\\xCA\0".as_ptr() as *const ::core::ffi::c_char,
        203 => return b"\\xCB\0".as_ptr() as *const ::core::ffi::c_char,
        204 => return b"\\xCC\0".as_ptr() as *const ::core::ffi::c_char,
        205 => return b"\\xCD\0".as_ptr() as *const ::core::ffi::c_char,
        206 => return b"\\xCE\0".as_ptr() as *const ::core::ffi::c_char,
        207 => return b"\\xCF\0".as_ptr() as *const ::core::ffi::c_char,
        208 => return b"\\xD0\0".as_ptr() as *const ::core::ffi::c_char,
        209 => return b"\\xD1\0".as_ptr() as *const ::core::ffi::c_char,
        210 => return b"\\xD2\0".as_ptr() as *const ::core::ffi::c_char,
        211 => return b"\\xD3\0".as_ptr() as *const ::core::ffi::c_char,
        212 => return b"\\xD4\0".as_ptr() as *const ::core::ffi::c_char,
        213 => return b"\\xD5\0".as_ptr() as *const ::core::ffi::c_char,
        214 => return b"\\xD6\0".as_ptr() as *const ::core::ffi::c_char,
        215 => return b"\\xD7\0".as_ptr() as *const ::core::ffi::c_char,
        216 => return b"\\xD8\0".as_ptr() as *const ::core::ffi::c_char,
        217 => return b"\\xD9\0".as_ptr() as *const ::core::ffi::c_char,
        218 => return b"\\xDA\0".as_ptr() as *const ::core::ffi::c_char,
        219 => return b"\\xDB\0".as_ptr() as *const ::core::ffi::c_char,
        220 => return b"\\xDC\0".as_ptr() as *const ::core::ffi::c_char,
        221 => return b"\\xDD\0".as_ptr() as *const ::core::ffi::c_char,
        222 => return b"\\xDE\0".as_ptr() as *const ::core::ffi::c_char,
        223 => return b"\\xDF\0".as_ptr() as *const ::core::ffi::c_char,
        224 => return b"\\xE0\0".as_ptr() as *const ::core::ffi::c_char,
        225 => return b"\\xE1\0".as_ptr() as *const ::core::ffi::c_char,
        226 => return b"\\xE2\0".as_ptr() as *const ::core::ffi::c_char,
        227 => return b"\\xE3\0".as_ptr() as *const ::core::ffi::c_char,
        228 => return b"\\xE4\0".as_ptr() as *const ::core::ffi::c_char,
        229 => return b"\\xE5\0".as_ptr() as *const ::core::ffi::c_char,
        230 => return b"\\xE6\0".as_ptr() as *const ::core::ffi::c_char,
        231 => return b"\\xE7\0".as_ptr() as *const ::core::ffi::c_char,
        232 => return b"\\xE8\0".as_ptr() as *const ::core::ffi::c_char,
        233 => return b"\\xE9\0".as_ptr() as *const ::core::ffi::c_char,
        234 => return b"\\xEA\0".as_ptr() as *const ::core::ffi::c_char,
        235 => return b"\\xEB\0".as_ptr() as *const ::core::ffi::c_char,
        236 => return b"\\xEC\0".as_ptr() as *const ::core::ffi::c_char,
        237 => return b"\\xED\0".as_ptr() as *const ::core::ffi::c_char,
        238 => return b"\\xEE\0".as_ptr() as *const ::core::ffi::c_char,
        239 => return b"\\xEF\0".as_ptr() as *const ::core::ffi::c_char,
        240 => return b"\\xF0\0".as_ptr() as *const ::core::ffi::c_char,
        241 => return b"\\xF1\0".as_ptr() as *const ::core::ffi::c_char,
        242 => return b"\\xF2\0".as_ptr() as *const ::core::ffi::c_char,
        243 => return b"\\xF3\0".as_ptr() as *const ::core::ffi::c_char,
        244 => return b"\\xF4\0".as_ptr() as *const ::core::ffi::c_char,
        245 => return b"\\xF5\0".as_ptr() as *const ::core::ffi::c_char,
        246 => return b"\\xF6\0".as_ptr() as *const ::core::ffi::c_char,
        247 => return b"\\xF7\0".as_ptr() as *const ::core::ffi::c_char,
        248 => return b"\\xF8\0".as_ptr() as *const ::core::ffi::c_char,
        249 => return b"\\xF9\0".as_ptr() as *const ::core::ffi::c_char,
        250 => return b"\\xFA\0".as_ptr() as *const ::core::ffi::c_char,
        251 => return b"\\xFB\0".as_ptr() as *const ::core::ffi::c_char,
        252 => return b"\\xFC\0".as_ptr() as *const ::core::ffi::c_char,
        253 => return b"\\xFD\0".as_ptr() as *const ::core::ffi::c_char,
        254 => return b"\\xFE\0".as_ptr() as *const ::core::ffi::c_char,
        255 => return b"\\xFF\0".as_ptr() as *const ::core::ffi::c_char,
        _ => ::std::process::abort(),
    };
}
#[export_name = "unsignedCharToPrintable"]

pub unsafe extern "C" fn unsignedCharToPrintable_ffi(
    mut c: ::core::ffi::c_uchar,
) -> *const ::core::ffi::c_char {
    unsignedCharToPrintable(c)
}
fn parseDebugLevel(value: &[u8]) -> Option<::core::ffi::c_ulong> {
    let mut index = 0usize;
    while matches!(
        value.get(index),
        Some(b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
    ) {
        index += 1;
    }

    let negative = match value.get(index) {
        Some(b'+') => {
            index += 1;
            false
        }
        Some(b'-') => {
            index += 1;
            true
        }
        _ => false,
    };

    let mut parsed_any = false;
    let mut parsed = 0 as ::core::ffi::c_ulong;
    while let Some(byte) = value.get(index) {
        let Some(digit) = byte.checked_sub(b'0').filter(|digit| *digit <= 9) else {
            break;
        };
        parsed_any = true;
        parsed = parsed.checked_mul(10)?;
        parsed = parsed.checked_add(digit as ::core::ffi::c_ulong)?;
        index += 1;
    }

    if !parsed_any || index != value.len() {
        return None;
    }

    Some(if negative {
        (0 as ::core::ffi::c_ulong).wrapping_sub(parsed)
    } else {
        parsed
    })
}

fn getDebugLevel(
    variableName: &str,
    defaultDebugLevel: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    ::std::env::var_os(variableName)
        .and_then(|value| parseDebugLevel(value.as_bytes()))
        .unwrap_or(defaultDebugLevel)
}
