pub mod siphash_h {

    pub fn sip_tokey(
        key: &mut crate::siphash_h::sipkey,
        src: &[::core::ffi::c_uchar; 16],
    ) {
        key.k[0] = crate::stdlib::uint64_t::from_le_bytes([
            src[0], src[1], src[2], src[3], src[4], src[5], src[6], src[7],
        ]);
        key.k[1] = crate::stdlib::uint64_t::from_le_bytes([
            src[8], src[9], src[10], src[11], src[12], src[13], src[14], src[15],
        ]);
    }

    pub unsafe extern "C" fn sip_round(
        mut H: *mut crate::siphash_h::siphash,
        rounds: ::core::ffi::c_int,
    ) {
        let state = &mut *H;
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < rounds {
            state.v0 = state.v0.wrapping_add(state.v1);
            state.v1 = state.v1 << 13 as ::core::ffi::c_int
                | state.v1 >> 64 as ::core::ffi::c_int - 13 as ::core::ffi::c_int;
            state.v1 ^= state.v0;
            state.v0 = state.v0 << 32 as ::core::ffi::c_int
                | state.v0 >> 64 as ::core::ffi::c_int - 32 as ::core::ffi::c_int;
            state.v2 = state.v2.wrapping_add(state.v3);
            state.v3 = state.v3 << 16 as ::core::ffi::c_int
                | state.v3 >> 64 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
            state.v3 ^= state.v2;
            state.v0 = state.v0.wrapping_add(state.v3);
            state.v3 = state.v3 << 21 as ::core::ffi::c_int
                | state.v3 >> 64 as ::core::ffi::c_int - 21 as ::core::ffi::c_int;
            state.v3 ^= state.v0;
            state.v2 = state.v2.wrapping_add(state.v1);
            state.v1 = state.v1 << 17 as ::core::ffi::c_int
                | state.v1 >> 64 as ::core::ffi::c_int - 17 as ::core::ffi::c_int;
            state.v1 ^= state.v2;
            state.v2 = state.v2 << 32 as ::core::ffi::c_int
                | state.v2 >> 64 as ::core::ffi::c_int - 32 as ::core::ffi::c_int;
            i += 1;
        }
    }

    pub unsafe extern "C" fn sip24_init(
        mut H: *mut crate::siphash_h::siphash,
        mut key: *const crate::siphash_h::sipkey,
    ) -> *mut crate::siphash_h::siphash {
        (*H).v0 = ((0x736f6d65 as ::core::ffi::c_uint as crate::stdlib::uint64_t)
            << 32 as ::core::ffi::c_int
            | 0x70736575 as crate::stdlib::uint64_t)
            ^ (*key).k[0 as usize];
        (*H).v1 = ((0x646f7261 as ::core::ffi::c_uint as crate::stdlib::uint64_t)
            << 32 as ::core::ffi::c_int
            | 0x6e646f6d as crate::stdlib::uint64_t)
            ^ (*key).k[1 as usize];
        (*H).v2 = ((0x6c796765 as ::core::ffi::c_uint as crate::stdlib::uint64_t)
            << 32 as ::core::ffi::c_int
            | 0x6e657261 as crate::stdlib::uint64_t)
            ^ (*key).k[0 as usize];
        (*H).v3 = ((0x74656462 as ::core::ffi::c_uint as crate::stdlib::uint64_t)
            << 32 as ::core::ffi::c_int
            | 0x79746573 as crate::stdlib::uint64_t)
            ^ (*key).k[1 as usize];
        (*H).p = &raw mut (*H).buf as *mut ::core::ffi::c_uchar;
        (*H).c = 0 as crate::stdlib::uint64_t;
        return H;
    }

    pub unsafe extern "C" fn sip24_update(
        mut H: *mut crate::siphash_h::siphash,
        mut src: *const ::core::ffi::c_void,
        mut len: crate::__stddef_size_t_h::size_t,
    ) -> *mut crate::siphash_h::siphash {
        let mut p: *const ::core::ffi::c_uchar = src as *const ::core::ffi::c_uchar;
        let mut pe: *const ::core::ffi::c_uchar = p.offset(len as isize);
        let mut m: crate::stdlib::uint64_t = 0;
        loop {
            while p < pe
                && (*H).p
                    < (&raw mut (*H).buf as *mut ::core::ffi::c_uchar).offset(
                        ::core::mem::size_of::<[::core::ffi::c_uchar; 8]>()
                            .wrapping_div(::core::mem::size_of::<::core::ffi::c_uchar>())
                            as isize,
                    )
            {
                let c2rust_fresh16 = p;
                p = p.offset(1);
                let c2rust_fresh17 = (*H).p;
                (*H).p = (*H).p.offset(1);
                *c2rust_fresh17 = *c2rust_fresh16;
            }
            if (*H).p
                < (&raw mut (*H).buf as *mut ::core::ffi::c_uchar).offset(
                    ::core::mem::size_of::<[::core::ffi::c_uchar; 8]>()
                        .wrapping_div(::core::mem::size_of::<::core::ffi::c_uchar>())
                        as isize,
                )
            {
                break;
            }
            m = ((*H).buf[0 as usize] as crate::stdlib::uint64_t) << 0 as ::core::ffi::c_int
                | ((*H).buf[1 as usize] as crate::stdlib::uint64_t) << 8 as ::core::ffi::c_int
                | ((*H).buf[2 as usize] as crate::stdlib::uint64_t) << 16 as ::core::ffi::c_int
                | ((*H).buf[3 as usize] as crate::stdlib::uint64_t) << 24 as ::core::ffi::c_int
                | ((*H).buf[4 as usize] as crate::stdlib::uint64_t) << 32 as ::core::ffi::c_int
                | ((*H).buf[5 as usize] as crate::stdlib::uint64_t) << 40 as ::core::ffi::c_int
                | ((*H).buf[6 as usize] as crate::stdlib::uint64_t) << 48 as ::core::ffi::c_int
                | ((*H).buf[7 as usize] as crate::stdlib::uint64_t) << 56 as ::core::ffi::c_int;
            (*H).v3 ^= m;
            sip_round(H, 2 as ::core::ffi::c_int);
            (*H).v0 ^= m;
            (*H).p = &raw mut (*H).buf as *mut ::core::ffi::c_uchar;
            (*H).c = (*H).c.wrapping_add(8 as crate::stdlib::uint64_t);
            if p >= pe {
                break;
            }
        }
        return H;
    }

    pub unsafe extern "C" fn sip24_final(
        mut H: *mut crate::siphash_h::siphash,
    ) -> crate::stdlib::uint64_t {
        let left: ::core::ffi::c_char = (*H)
            .p
            .offset_from(&raw mut (*H).buf as *mut ::core::ffi::c_uchar)
            as ::core::ffi::c_char;
        let mut b: crate::stdlib::uint64_t =
            (*H).c.wrapping_add(left as crate::stdlib::uint64_t) << 56 as ::core::ffi::c_int;
        's_46: {
            'c_16515: {
                'c_16514: {
                    'c_16513: {
                        'c_16512: {
                            'c_16511: {
                                match left as ::core::ffi::c_int {
                                    7 => {
                                        b |= ((*H).buf[6 as usize] as crate::stdlib::uint64_t)
                                            << 48 as ::core::ffi::c_int;
                                    }
                                    6 => {}
                                    5 => {
                                        break 'c_16511;
                                    }
                                    4 => {
                                        break 'c_16512;
                                    }
                                    3 => {
                                        break 'c_16513;
                                    }
                                    2 => {
                                        break 'c_16514;
                                    }
                                    1 => {
                                        break 'c_16515;
                                    }
                                    0 | _ => {
                                        break 's_46;
                                    }
                                }
                                b |= ((*H).buf[5 as usize] as crate::stdlib::uint64_t)
                                    << 40 as ::core::ffi::c_int;
                            }
                            b |= ((*H).buf[4 as usize] as crate::stdlib::uint64_t)
                                << 32 as ::core::ffi::c_int;
                        }
                        b |= ((*H).buf[3 as usize] as crate::stdlib::uint64_t)
                            << 24 as ::core::ffi::c_int;
                    }
                    b |= ((*H).buf[2 as usize] as crate::stdlib::uint64_t)
                        << 16 as ::core::ffi::c_int;
                }
                b |= ((*H).buf[1 as usize] as crate::stdlib::uint64_t) << 8 as ::core::ffi::c_int;
            }
            b |= ((*H).buf[0 as usize] as crate::stdlib::uint64_t) << 0 as ::core::ffi::c_int;
        }
        (*H).v3 ^= b;
        sip_round(H, 2 as ::core::ffi::c_int);
        (*H).v0 ^= b;
        (*H).v2 ^= 0xff as crate::stdlib::uint64_t;
        sip_round(H, 4 as ::core::ffi::c_int);
        return (*H).v0 ^ (*H).v1 ^ (*H).v2 ^ (*H).v3;
    }

    pub unsafe extern "C" fn siphash24(
        mut src: *const ::core::ffi::c_void,
        mut len: crate::__stddef_size_t_h::size_t,
        mut key: *const crate::siphash_h::sipkey,
    ) -> crate::stdlib::uint64_t {
        let mut state: crate::siphash_h::siphash = crate::siphash_h::SIPHASH_INITIALIZER;
        return sip24_final(sip24_update(sip24_init(&raw mut state, key), src, len));
    }

    pub unsafe extern "C" fn sip24_valid() -> ::core::ffi::c_int {
        pub static mut vectors: [[::core::ffi::c_uchar; 8]; 64] = [
            [
                0x31 as ::core::ffi::c_uchar,
                0xe as ::core::ffi::c_uchar,
                0xe as ::core::ffi::c_uchar,
                0xdd as ::core::ffi::c_uchar,
                0x47 as ::core::ffi::c_uchar,
                0xdb as ::core::ffi::c_uchar,
                0x6f as ::core::ffi::c_uchar,
                0x72 as ::core::ffi::c_uchar,
            ],
            [
                0xfd as ::core::ffi::c_uchar,
                0x67 as ::core::ffi::c_uchar,
                0xdc as ::core::ffi::c_uchar,
                0x93 as ::core::ffi::c_uchar,
                0xc5 as ::core::ffi::c_uchar,
                0x39 as ::core::ffi::c_uchar,
                0xf8 as ::core::ffi::c_uchar,
                0x74 as ::core::ffi::c_uchar,
            ],
            [
                0x5a as ::core::ffi::c_uchar,
                0x4f as ::core::ffi::c_uchar,
                0xa9 as ::core::ffi::c_uchar,
                0xd9 as ::core::ffi::c_uchar,
                0x9 as ::core::ffi::c_uchar,
                0x80 as ::core::ffi::c_uchar,
                0x6c as ::core::ffi::c_uchar,
                0xd as ::core::ffi::c_uchar,
            ],
            [
                0x2d as ::core::ffi::c_uchar,
                0x7e as ::core::ffi::c_uchar,
                0xfb as ::core::ffi::c_uchar,
                0xd7 as ::core::ffi::c_uchar,
                0x96 as ::core::ffi::c_uchar,
                0x66 as ::core::ffi::c_uchar,
                0x67 as ::core::ffi::c_uchar,
                0x85 as ::core::ffi::c_uchar,
            ],
            [
                0xb7 as ::core::ffi::c_uchar,
                0x87 as ::core::ffi::c_uchar,
                0x71 as ::core::ffi::c_uchar,
                0x27 as ::core::ffi::c_uchar,
                0xe0 as ::core::ffi::c_uchar,
                0x94 as ::core::ffi::c_uchar,
                0x27 as ::core::ffi::c_uchar,
                0xcf as ::core::ffi::c_uchar,
            ],
            [
                0x8d as ::core::ffi::c_uchar,
                0xa6 as ::core::ffi::c_uchar,
                0x99 as ::core::ffi::c_uchar,
                0xcd as ::core::ffi::c_uchar,
                0x64 as ::core::ffi::c_uchar,
                0x55 as ::core::ffi::c_uchar,
                0x76 as ::core::ffi::c_uchar,
                0x18 as ::core::ffi::c_uchar,
            ],
            [
                0xce as ::core::ffi::c_uchar,
                0xe3 as ::core::ffi::c_uchar,
                0xfe as ::core::ffi::c_uchar,
                0x58 as ::core::ffi::c_uchar,
                0x6e as ::core::ffi::c_uchar,
                0x46 as ::core::ffi::c_uchar,
                0xc9 as ::core::ffi::c_uchar,
                0xcb as ::core::ffi::c_uchar,
            ],
            [
                0x37 as ::core::ffi::c_uchar,
                0xd1 as ::core::ffi::c_uchar,
                0x1 as ::core::ffi::c_uchar,
                0x8b as ::core::ffi::c_uchar,
                0xf5 as ::core::ffi::c_uchar,
                0 as ::core::ffi::c_uchar,
                0x2 as ::core::ffi::c_uchar,
                0xab as ::core::ffi::c_uchar,
            ],
            [
                0x62 as ::core::ffi::c_uchar,
                0x24 as ::core::ffi::c_uchar,
                0x93 as ::core::ffi::c_uchar,
                0x9a as ::core::ffi::c_uchar,
                0x79 as ::core::ffi::c_uchar,
                0xf5 as ::core::ffi::c_uchar,
                0xf5 as ::core::ffi::c_uchar,
                0x93 as ::core::ffi::c_uchar,
            ],
            [
                0xb0 as ::core::ffi::c_uchar,
                0xe4 as ::core::ffi::c_uchar,
                0xa9 as ::core::ffi::c_uchar,
                0xb as ::core::ffi::c_uchar,
                0xdf as ::core::ffi::c_uchar,
                0x82 as ::core::ffi::c_uchar,
                0 as ::core::ffi::c_uchar,
                0x9e as ::core::ffi::c_uchar,
            ],
            [
                0xf3 as ::core::ffi::c_uchar,
                0xb9 as ::core::ffi::c_uchar,
                0xdd as ::core::ffi::c_uchar,
                0x94 as ::core::ffi::c_uchar,
                0xc5 as ::core::ffi::c_uchar,
                0xbb as ::core::ffi::c_uchar,
                0x5d as ::core::ffi::c_uchar,
                0x7a as ::core::ffi::c_uchar,
            ],
            [
                0xa7 as ::core::ffi::c_uchar,
                0xad as ::core::ffi::c_uchar,
                0x6b as ::core::ffi::c_uchar,
                0x22 as ::core::ffi::c_uchar,
                0x46 as ::core::ffi::c_uchar,
                0x2f as ::core::ffi::c_uchar,
                0xb3 as ::core::ffi::c_uchar,
                0xf4 as ::core::ffi::c_uchar,
            ],
            [
                0xfb as ::core::ffi::c_uchar,
                0xe5 as ::core::ffi::c_uchar,
                0xe as ::core::ffi::c_uchar,
                0x86 as ::core::ffi::c_uchar,
                0xbc as ::core::ffi::c_uchar,
                0x8f as ::core::ffi::c_uchar,
                0x1e as ::core::ffi::c_uchar,
                0x75 as ::core::ffi::c_uchar,
            ],
            [
                0x90 as ::core::ffi::c_uchar,
                0x3d as ::core::ffi::c_uchar,
                0x84 as ::core::ffi::c_uchar,
                0xc0 as ::core::ffi::c_uchar,
                0x27 as ::core::ffi::c_uchar,
                0x56 as ::core::ffi::c_uchar,
                0xea as ::core::ffi::c_uchar,
                0x14 as ::core::ffi::c_uchar,
            ],
            [
                0xee as ::core::ffi::c_uchar,
                0xf2 as ::core::ffi::c_uchar,
                0x7a as ::core::ffi::c_uchar,
                0x8e as ::core::ffi::c_uchar,
                0x90 as ::core::ffi::c_uchar,
                0xca as ::core::ffi::c_uchar,
                0x23 as ::core::ffi::c_uchar,
                0xf7 as ::core::ffi::c_uchar,
            ],
            [
                0xe5 as ::core::ffi::c_uchar,
                0x45 as ::core::ffi::c_uchar,
                0xbe as ::core::ffi::c_uchar,
                0x49 as ::core::ffi::c_uchar,
                0x61 as ::core::ffi::c_uchar,
                0xca as ::core::ffi::c_uchar,
                0x29 as ::core::ffi::c_uchar,
                0xa1 as ::core::ffi::c_uchar,
            ],
            [
                0xdb as ::core::ffi::c_uchar,
                0x9b as ::core::ffi::c_uchar,
                0xc2 as ::core::ffi::c_uchar,
                0x57 as ::core::ffi::c_uchar,
                0x7f as ::core::ffi::c_uchar,
                0xcc as ::core::ffi::c_uchar,
                0x2a as ::core::ffi::c_uchar,
                0x3f as ::core::ffi::c_uchar,
            ],
            [
                0x94 as ::core::ffi::c_uchar,
                0x47 as ::core::ffi::c_uchar,
                0xbe as ::core::ffi::c_uchar,
                0x2c as ::core::ffi::c_uchar,
                0xf5 as ::core::ffi::c_uchar,
                0xe9 as ::core::ffi::c_uchar,
                0x9a as ::core::ffi::c_uchar,
                0x69 as ::core::ffi::c_uchar,
            ],
            [
                0x9c as ::core::ffi::c_uchar,
                0xd3 as ::core::ffi::c_uchar,
                0x8d as ::core::ffi::c_uchar,
                0x96 as ::core::ffi::c_uchar,
                0xf0 as ::core::ffi::c_uchar,
                0xb3 as ::core::ffi::c_uchar,
                0xc1 as ::core::ffi::c_uchar,
                0x4b as ::core::ffi::c_uchar,
            ],
            [
                0xbd as ::core::ffi::c_uchar,
                0x61 as ::core::ffi::c_uchar,
                0x79 as ::core::ffi::c_uchar,
                0xa7 as ::core::ffi::c_uchar,
                0x1d as ::core::ffi::c_uchar,
                0xc9 as ::core::ffi::c_uchar,
                0x6d as ::core::ffi::c_uchar,
                0xbb as ::core::ffi::c_uchar,
            ],
            [
                0x98 as ::core::ffi::c_uchar,
                0xee as ::core::ffi::c_uchar,
                0xa2 as ::core::ffi::c_uchar,
                0x1a as ::core::ffi::c_uchar,
                0xf2 as ::core::ffi::c_uchar,
                0x5c as ::core::ffi::c_uchar,
                0xd6 as ::core::ffi::c_uchar,
                0xbe as ::core::ffi::c_uchar,
            ],
            [
                0xc7 as ::core::ffi::c_uchar,
                0x67 as ::core::ffi::c_uchar,
                0x3b as ::core::ffi::c_uchar,
                0x2e as ::core::ffi::c_uchar,
                0xb0 as ::core::ffi::c_uchar,
                0xcb as ::core::ffi::c_uchar,
                0xf2 as ::core::ffi::c_uchar,
                0xd0 as ::core::ffi::c_uchar,
            ],
            [
                0x88 as ::core::ffi::c_uchar,
                0x3e as ::core::ffi::c_uchar,
                0xa3 as ::core::ffi::c_uchar,
                0xe3 as ::core::ffi::c_uchar,
                0x95 as ::core::ffi::c_uchar,
                0x67 as ::core::ffi::c_uchar,
                0x53 as ::core::ffi::c_uchar,
                0x93 as ::core::ffi::c_uchar,
            ],
            [
                0xc8 as ::core::ffi::c_uchar,
                0xce as ::core::ffi::c_uchar,
                0x5c as ::core::ffi::c_uchar,
                0xcd as ::core::ffi::c_uchar,
                0x8c as ::core::ffi::c_uchar,
                0x3 as ::core::ffi::c_uchar,
                0xc as ::core::ffi::c_uchar,
                0xa8 as ::core::ffi::c_uchar,
            ],
            [
                0x94 as ::core::ffi::c_uchar,
                0xaf as ::core::ffi::c_uchar,
                0x49 as ::core::ffi::c_uchar,
                0xf6 as ::core::ffi::c_uchar,
                0xc6 as ::core::ffi::c_uchar,
                0x50 as ::core::ffi::c_uchar,
                0xad as ::core::ffi::c_uchar,
                0xb8 as ::core::ffi::c_uchar,
            ],
            [
                0xea as ::core::ffi::c_uchar,
                0xb8 as ::core::ffi::c_uchar,
                0x85 as ::core::ffi::c_uchar,
                0x8a as ::core::ffi::c_uchar,
                0xde as ::core::ffi::c_uchar,
                0x92 as ::core::ffi::c_uchar,
                0xe1 as ::core::ffi::c_uchar,
                0xbc as ::core::ffi::c_uchar,
            ],
            [
                0xf3 as ::core::ffi::c_uchar,
                0x15 as ::core::ffi::c_uchar,
                0xbb as ::core::ffi::c_uchar,
                0x5b as ::core::ffi::c_uchar,
                0xb8 as ::core::ffi::c_uchar,
                0x35 as ::core::ffi::c_uchar,
                0xd8 as ::core::ffi::c_uchar,
                0x17 as ::core::ffi::c_uchar,
            ],
            [
                0xad as ::core::ffi::c_uchar,
                0xcf as ::core::ffi::c_uchar,
                0x6b as ::core::ffi::c_uchar,
                0x7 as ::core::ffi::c_uchar,
                0x63 as ::core::ffi::c_uchar,
                0x61 as ::core::ffi::c_uchar,
                0x2e as ::core::ffi::c_uchar,
                0x2f as ::core::ffi::c_uchar,
            ],
            [
                0xa5 as ::core::ffi::c_uchar,
                0xc9 as ::core::ffi::c_uchar,
                0x1d as ::core::ffi::c_uchar,
                0xa7 as ::core::ffi::c_uchar,
                0xac as ::core::ffi::c_uchar,
                0xaa as ::core::ffi::c_uchar,
                0x4d as ::core::ffi::c_uchar,
                0xde as ::core::ffi::c_uchar,
            ],
            [
                0x71 as ::core::ffi::c_uchar,
                0x65 as ::core::ffi::c_uchar,
                0x95 as ::core::ffi::c_uchar,
                0x87 as ::core::ffi::c_uchar,
                0x66 as ::core::ffi::c_uchar,
                0x50 as ::core::ffi::c_uchar,
                0xa2 as ::core::ffi::c_uchar,
                0xa6 as ::core::ffi::c_uchar,
            ],
            [
                0x28 as ::core::ffi::c_uchar,
                0xef as ::core::ffi::c_uchar,
                0x49 as ::core::ffi::c_uchar,
                0x5c as ::core::ffi::c_uchar,
                0x53 as ::core::ffi::c_uchar,
                0xa3 as ::core::ffi::c_uchar,
                0x87 as ::core::ffi::c_uchar,
                0xad as ::core::ffi::c_uchar,
            ],
            [
                0x42 as ::core::ffi::c_uchar,
                0xc3 as ::core::ffi::c_uchar,
                0x41 as ::core::ffi::c_uchar,
                0xd8 as ::core::ffi::c_uchar,
                0xfa as ::core::ffi::c_uchar,
                0x92 as ::core::ffi::c_uchar,
                0xd8 as ::core::ffi::c_uchar,
                0x32 as ::core::ffi::c_uchar,
            ],
            [
                0xce as ::core::ffi::c_uchar,
                0x7c as ::core::ffi::c_uchar,
                0xf2 as ::core::ffi::c_uchar,
                0x72 as ::core::ffi::c_uchar,
                0x2f as ::core::ffi::c_uchar,
                0x51 as ::core::ffi::c_uchar,
                0x27 as ::core::ffi::c_uchar,
                0x71 as ::core::ffi::c_uchar,
            ],
            [
                0xe3 as ::core::ffi::c_uchar,
                0x78 as ::core::ffi::c_uchar,
                0x59 as ::core::ffi::c_uchar,
                0xf9 as ::core::ffi::c_uchar,
                0x46 as ::core::ffi::c_uchar,
                0x23 as ::core::ffi::c_uchar,
                0xf3 as ::core::ffi::c_uchar,
                0xa7 as ::core::ffi::c_uchar,
            ],
            [
                0x38 as ::core::ffi::c_uchar,
                0x12 as ::core::ffi::c_uchar,
                0x5 as ::core::ffi::c_uchar,
                0xbb as ::core::ffi::c_uchar,
                0x1a as ::core::ffi::c_uchar,
                0xb0 as ::core::ffi::c_uchar,
                0xe0 as ::core::ffi::c_uchar,
                0x12 as ::core::ffi::c_uchar,
            ],
            [
                0xae as ::core::ffi::c_uchar,
                0x97 as ::core::ffi::c_uchar,
                0xa1 as ::core::ffi::c_uchar,
                0xf as ::core::ffi::c_uchar,
                0xd4 as ::core::ffi::c_uchar,
                0x34 as ::core::ffi::c_uchar,
                0xe0 as ::core::ffi::c_uchar,
                0x15 as ::core::ffi::c_uchar,
            ],
            [
                0xb4 as ::core::ffi::c_uchar,
                0xa3 as ::core::ffi::c_uchar,
                0x15 as ::core::ffi::c_uchar,
                0x8 as ::core::ffi::c_uchar,
                0xbe as ::core::ffi::c_uchar,
                0xff as ::core::ffi::c_uchar,
                0x4d as ::core::ffi::c_uchar,
                0x31 as ::core::ffi::c_uchar,
            ],
            [
                0x81 as ::core::ffi::c_uchar,
                0x39 as ::core::ffi::c_uchar,
                0x62 as ::core::ffi::c_uchar,
                0x29 as ::core::ffi::c_uchar,
                0xf0 as ::core::ffi::c_uchar,
                0x90 as ::core::ffi::c_uchar,
                0x79 as ::core::ffi::c_uchar,
                0x2 as ::core::ffi::c_uchar,
            ],
            [
                0x4d as ::core::ffi::c_uchar,
                0xc as ::core::ffi::c_uchar,
                0xf4 as ::core::ffi::c_uchar,
                0x9e as ::core::ffi::c_uchar,
                0xe5 as ::core::ffi::c_uchar,
                0xd4 as ::core::ffi::c_uchar,
                0xdc as ::core::ffi::c_uchar,
                0xca as ::core::ffi::c_uchar,
            ],
            [
                0x5c as ::core::ffi::c_uchar,
                0x73 as ::core::ffi::c_uchar,
                0x33 as ::core::ffi::c_uchar,
                0x6a as ::core::ffi::c_uchar,
                0x76 as ::core::ffi::c_uchar,
                0xd8 as ::core::ffi::c_uchar,
                0xbf as ::core::ffi::c_uchar,
                0x9a as ::core::ffi::c_uchar,
            ],
            [
                0xd0 as ::core::ffi::c_uchar,
                0xa7 as ::core::ffi::c_uchar,
                0x4 as ::core::ffi::c_uchar,
                0x53 as ::core::ffi::c_uchar,
                0x6b as ::core::ffi::c_uchar,
                0xa9 as ::core::ffi::c_uchar,
                0x3e as ::core::ffi::c_uchar,
                0xe as ::core::ffi::c_uchar,
            ],
            [
                0x92 as ::core::ffi::c_uchar,
                0x59 as ::core::ffi::c_uchar,
                0x58 as ::core::ffi::c_uchar,
                0xfc as ::core::ffi::c_uchar,
                0xd6 as ::core::ffi::c_uchar,
                0x42 as ::core::ffi::c_uchar,
                0xc as ::core::ffi::c_uchar,
                0xad as ::core::ffi::c_uchar,
            ],
            [
                0xa9 as ::core::ffi::c_uchar,
                0x15 as ::core::ffi::c_uchar,
                0xc2 as ::core::ffi::c_uchar,
                0x9b as ::core::ffi::c_uchar,
                0xc8 as ::core::ffi::c_uchar,
                0x6 as ::core::ffi::c_uchar,
                0x73 as ::core::ffi::c_uchar,
                0x18 as ::core::ffi::c_uchar,
            ],
            [
                0x95 as ::core::ffi::c_uchar,
                0x2b as ::core::ffi::c_uchar,
                0x79 as ::core::ffi::c_uchar,
                0xf3 as ::core::ffi::c_uchar,
                0xbc as ::core::ffi::c_uchar,
                0xa as ::core::ffi::c_uchar,
                0xa6 as ::core::ffi::c_uchar,
                0xd4 as ::core::ffi::c_uchar,
            ],
            [
                0xf2 as ::core::ffi::c_uchar,
                0x1d as ::core::ffi::c_uchar,
                0xf2 as ::core::ffi::c_uchar,
                0xe4 as ::core::ffi::c_uchar,
                0x1d as ::core::ffi::c_uchar,
                0x45 as ::core::ffi::c_uchar,
                0x35 as ::core::ffi::c_uchar,
                0xf9 as ::core::ffi::c_uchar,
            ],
            [
                0x87 as ::core::ffi::c_uchar,
                0x57 as ::core::ffi::c_uchar,
                0x75 as ::core::ffi::c_uchar,
                0x19 as ::core::ffi::c_uchar,
                0x4 as ::core::ffi::c_uchar,
                0x8f as ::core::ffi::c_uchar,
                0x53 as ::core::ffi::c_uchar,
                0xa9 as ::core::ffi::c_uchar,
            ],
            [
                0x10 as ::core::ffi::c_uchar,
                0xa5 as ::core::ffi::c_uchar,
                0x6c as ::core::ffi::c_uchar,
                0xf5 as ::core::ffi::c_uchar,
                0xdf as ::core::ffi::c_uchar,
                0xcd as ::core::ffi::c_uchar,
                0x9a as ::core::ffi::c_uchar,
                0xdb as ::core::ffi::c_uchar,
            ],
            [
                0xeb as ::core::ffi::c_uchar,
                0x75 as ::core::ffi::c_uchar,
                0x9 as ::core::ffi::c_uchar,
                0x5c as ::core::ffi::c_uchar,
                0xcd as ::core::ffi::c_uchar,
                0x98 as ::core::ffi::c_uchar,
                0x6c as ::core::ffi::c_uchar,
                0xd0 as ::core::ffi::c_uchar,
            ],
            [
                0x51 as ::core::ffi::c_uchar,
                0xa9 as ::core::ffi::c_uchar,
                0xcb as ::core::ffi::c_uchar,
                0x9e as ::core::ffi::c_uchar,
                0xcb as ::core::ffi::c_uchar,
                0xa3 as ::core::ffi::c_uchar,
                0x12 as ::core::ffi::c_uchar,
                0xe6 as ::core::ffi::c_uchar,
            ],
            [
                0x96 as ::core::ffi::c_uchar,
                0xaf as ::core::ffi::c_uchar,
                0xad as ::core::ffi::c_uchar,
                0xfc as ::core::ffi::c_uchar,
                0x2c as ::core::ffi::c_uchar,
                0xe6 as ::core::ffi::c_uchar,
                0x66 as ::core::ffi::c_uchar,
                0xc7 as ::core::ffi::c_uchar,
            ],
            [
                0x72 as ::core::ffi::c_uchar,
                0xfe as ::core::ffi::c_uchar,
                0x52 as ::core::ffi::c_uchar,
                0x97 as ::core::ffi::c_uchar,
                0x5a as ::core::ffi::c_uchar,
                0x43 as ::core::ffi::c_uchar,
                0x64 as ::core::ffi::c_uchar,
                0xee as ::core::ffi::c_uchar,
            ],
            [
                0x5a as ::core::ffi::c_uchar,
                0x16 as ::core::ffi::c_uchar,
                0x45 as ::core::ffi::c_uchar,
                0xb2 as ::core::ffi::c_uchar,
                0x76 as ::core::ffi::c_uchar,
                0xd5 as ::core::ffi::c_uchar,
                0x92 as ::core::ffi::c_uchar,
                0xa1 as ::core::ffi::c_uchar,
            ],
            [
                0xb2 as ::core::ffi::c_uchar,
                0x74 as ::core::ffi::c_uchar,
                0xcb as ::core::ffi::c_uchar,
                0x8e as ::core::ffi::c_uchar,
                0xbf as ::core::ffi::c_uchar,
                0x87 as ::core::ffi::c_uchar,
                0x87 as ::core::ffi::c_uchar,
                0xa as ::core::ffi::c_uchar,
            ],
            [
                0x6f as ::core::ffi::c_uchar,
                0x9b as ::core::ffi::c_uchar,
                0xb4 as ::core::ffi::c_uchar,
                0x20 as ::core::ffi::c_uchar,
                0x3d as ::core::ffi::c_uchar,
                0xe7 as ::core::ffi::c_uchar,
                0xb3 as ::core::ffi::c_uchar,
                0x81 as ::core::ffi::c_uchar,
            ],
            [
                0xea as ::core::ffi::c_uchar,
                0xec as ::core::ffi::c_uchar,
                0xb2 as ::core::ffi::c_uchar,
                0xa3 as ::core::ffi::c_uchar,
                0xb as ::core::ffi::c_uchar,
                0x22 as ::core::ffi::c_uchar,
                0xa8 as ::core::ffi::c_uchar,
                0x7f as ::core::ffi::c_uchar,
            ],
            [
                0x99 as ::core::ffi::c_uchar,
                0x24 as ::core::ffi::c_uchar,
                0xa4 as ::core::ffi::c_uchar,
                0x3c as ::core::ffi::c_uchar,
                0xc1 as ::core::ffi::c_uchar,
                0x31 as ::core::ffi::c_uchar,
                0x57 as ::core::ffi::c_uchar,
                0x24 as ::core::ffi::c_uchar,
            ],
            [
                0xbd as ::core::ffi::c_uchar,
                0x83 as ::core::ffi::c_uchar,
                0x8d as ::core::ffi::c_uchar,
                0x3a as ::core::ffi::c_uchar,
                0xaf as ::core::ffi::c_uchar,
                0xbf as ::core::ffi::c_uchar,
                0x8d as ::core::ffi::c_uchar,
                0xb7 as ::core::ffi::c_uchar,
            ],
            [
                0xb as ::core::ffi::c_uchar,
                0x1a as ::core::ffi::c_uchar,
                0x2a as ::core::ffi::c_uchar,
                0x32 as ::core::ffi::c_uchar,
                0x65 as ::core::ffi::c_uchar,
                0xd5 as ::core::ffi::c_uchar,
                0x1a as ::core::ffi::c_uchar,
                0xea as ::core::ffi::c_uchar,
            ],
            [
                0x13 as ::core::ffi::c_uchar,
                0x50 as ::core::ffi::c_uchar,
                0x79 as ::core::ffi::c_uchar,
                0xa3 as ::core::ffi::c_uchar,
                0x23 as ::core::ffi::c_uchar,
                0x1c as ::core::ffi::c_uchar,
                0xe6 as ::core::ffi::c_uchar,
                0x60 as ::core::ffi::c_uchar,
            ],
            [
                0x93 as ::core::ffi::c_uchar,
                0x2b as ::core::ffi::c_uchar,
                0x28 as ::core::ffi::c_uchar,
                0x46 as ::core::ffi::c_uchar,
                0xe4 as ::core::ffi::c_uchar,
                0xd7 as ::core::ffi::c_uchar,
                0x6 as ::core::ffi::c_uchar,
                0x66 as ::core::ffi::c_uchar,
            ],
            [
                0xe1 as ::core::ffi::c_uchar,
                0x91 as ::core::ffi::c_uchar,
                0x5f as ::core::ffi::c_uchar,
                0x5c as ::core::ffi::c_uchar,
                0xb1 as ::core::ffi::c_uchar,
                0xec as ::core::ffi::c_uchar,
                0xa4 as ::core::ffi::c_uchar,
                0x6c as ::core::ffi::c_uchar,
            ],
            [
                0xf3 as ::core::ffi::c_uchar,
                0x25 as ::core::ffi::c_uchar,
                0x96 as ::core::ffi::c_uchar,
                0x5c as ::core::ffi::c_uchar,
                0xa1 as ::core::ffi::c_uchar,
                0x6d as ::core::ffi::c_uchar,
                0x62 as ::core::ffi::c_uchar,
                0x9f as ::core::ffi::c_uchar,
            ],
            [
                0x57 as ::core::ffi::c_uchar,
                0x5f as ::core::ffi::c_uchar,
                0xf2 as ::core::ffi::c_uchar,
                0x8e as ::core::ffi::c_uchar,
                0x60 as ::core::ffi::c_uchar,
                0x38 as ::core::ffi::c_uchar,
                0x1b as ::core::ffi::c_uchar,
                0xe5 as ::core::ffi::c_uchar,
            ],
            [
                0x72 as ::core::ffi::c_uchar,
                0x45 as ::core::ffi::c_uchar,
                0x6 as ::core::ffi::c_uchar,
                0xeb as ::core::ffi::c_uchar,
                0x4c as ::core::ffi::c_uchar,
                0x32 as ::core::ffi::c_uchar,
                0x8a as ::core::ffi::c_uchar,
                0x95 as ::core::ffi::c_uchar,
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
        while i < ::core::mem::size_of::<[::core::ffi::c_uchar; 64]>() {
            in_0[i] = i as ::core::ffi::c_uchar;
            if siphash24(
                &raw mut in_0 as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                i,
                &raw mut k,
            ) != (vectors[i][0 as usize] as crate::stdlib::uint64_t) << 0 as ::core::ffi::c_int
                | (vectors[i][1 as usize] as crate::stdlib::uint64_t) << 8 as ::core::ffi::c_int
                | (vectors[i][2 as usize] as crate::stdlib::uint64_t) << 16 as ::core::ffi::c_int
                | (vectors[i][3 as usize] as crate::stdlib::uint64_t) << 24 as ::core::ffi::c_int
                | (vectors[i][4 as usize] as crate::stdlib::uint64_t) << 32 as ::core::ffi::c_int
                | (vectors[i][5 as usize] as crate::stdlib::uint64_t) << 40 as ::core::ffi::c_int
                | (vectors[i][6 as usize] as crate::stdlib::uint64_t) << 48 as ::core::ffi::c_int
                | (vectors[i][7 as usize] as crate::stdlib::uint64_t) << 56 as ::core::ffi::c_int
            {
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
pub use crate::siphash_h::SIPHASH_INITIALIZER;
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

pub use crate::src::xmlrole::prolog_handler_dispatch;
pub use crate::src::xmlrole::prolog_state;
pub use crate::src::xmlrole::prolog_state_init;
pub use crate::src::xmlrole::prolog_state_init_external_entity;
pub use crate::src::xmlrole::C2Rust_Unnamed_0;
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
pub use crate::stdlib::_IO_FILE;
pub use crate::stdlib::FILE;
trait StartElementCallback: Send + Sync {
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        name: *const crate::expat_external_h::XML_Char,
        atts: *mut *const crate::expat_external_h::XML_Char,
    );
}

impl StartElementCallback
    for unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const crate::expat_external_h::XML_Char,
        *mut *const crate::expat_external_h::XML_Char,
    )
{
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        name: *const crate::expat_external_h::XML_Char,
        atts: *mut *const crate::expat_external_h::XML_Char,
    ) {
        self(user_data, name, atts);
    }
}

static START_ELEMENT_HANDLERS: std::sync::OnceLock<
    std::sync::Mutex<std::collections::HashMap<usize, std::sync::Arc<dyn StartElementCallback>>>,
> = std::sync::OnceLock::new();

// Foreign callback values remain in this boundary registry; parser state only
// records whether a start-namespace callback is installed.
static START_NAMESPACE_DECL_HANDLERS: std::sync::OnceLock<
    std::sync::Mutex<
        std::collections::HashMap<usize, std::sync::Arc<TwoXmlCharCallback>>,
    >,
> = std::sync::OnceLock::new();

trait EndElementCallback: Send + Sync {
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        name: *const crate::expat_external_h::XML_Char,
    );
}

impl EndElementCallback
    for unsafe extern "C" fn(*mut ::core::ffi::c_void, *const crate::expat_external_h::XML_Char)
{
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        name: *const crate::expat_external_h::XML_Char,
    ) {
        self(user_data, name);
    }
}

// Foreign callback values remain in this boundary registry; parser state only
// records whether an end-element callback is installed.
static END_ELEMENT_HANDLERS: std::sync::OnceLock<
    std::sync::Mutex<std::collections::HashMap<usize, std::sync::Arc<dyn EndElementCallback>>>,
> = std::sync::OnceLock::new();

trait EndNamespaceDeclCallback: Send + Sync {
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        prefix: *const crate::expat_external_h::XML_Char,
    );
}

impl EndNamespaceDeclCallback
    for unsafe extern "C" fn(*mut ::core::ffi::c_void, *const crate::expat_external_h::XML_Char)
{
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        prefix: *const crate::expat_external_h::XML_Char,
    ) {
        self(user_data, prefix);
    }
}

// Foreign callback values remain in this boundary registry; parser state only
// records whether an end-namespace callback is installed.
static END_NAMESPACE_DECL_HANDLERS: std::sync::OnceLock<
    std::sync::Mutex<
        std::collections::HashMap<usize, std::sync::Arc<dyn EndNamespaceDeclCallback>>,
    >,
> = std::sync::OnceLock::new();

trait CharacterDataCallback: Send + Sync {
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        data: *const crate::expat_external_h::XML_Char,
        len: ::core::ffi::c_int,
    );
}

impl CharacterDataCallback
    for unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const crate::expat_external_h::XML_Char,
        ::core::ffi::c_int,
    ) -> ()
{
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        data: *const crate::expat_external_h::XML_Char,
        len: ::core::ffi::c_int,
    ) {
        self(user_data, data, len);
    }
}

// Foreign callback values remain in this boundary registry; parser state only
// records whether a character-data callback is installed.
static CHARACTER_DATA_HANDLERS: std::sync::OnceLock<
    std::sync::Mutex<std::collections::HashMap<usize, std::sync::Arc<dyn CharacterDataCallback>>>,
> = std::sync::OnceLock::new();

// Foreign callback values remain in this boundary registry; parser state only
// records whether a processing-instruction callback is installed.
trait ProcessingInstructionCallback: Send + Sync {
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        target: *const crate::expat_external_h::XML_Char,
        data: *const crate::expat_external_h::XML_Char,
    );
}

impl ProcessingInstructionCallback
    for unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const crate::expat_external_h::XML_Char,
        *const crate::expat_external_h::XML_Char,
    ) -> ()
{
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        target: *const crate::expat_external_h::XML_Char,
        data: *const crate::expat_external_h::XML_Char,
    ) {
        self(user_data, target, data);
    }
}

static PROCESSING_INSTRUCTION_HANDLERS: std::sync::OnceLock<
    std::sync::Mutex<
        std::collections::HashMap<usize, std::sync::Arc<dyn ProcessingInstructionCallback>>,
    >,
> = std::sync::OnceLock::new();

// Start-namespace and processing-instruction handlers use the same C callback
// ABI: an opaque user context followed by two XML character pointers.
type TwoXmlCharCallback = dyn ProcessingInstructionCallback;

// Foreign callback values remain in this boundary registry; parser state only
// records whether a comment callback is installed.
trait CommentCallback: Send + Sync {
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        data: *const crate::expat_external_h::XML_Char,
    );
}

impl CommentCallback
    for unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const crate::expat_external_h::XML_Char,
    ) -> ()
{
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        data: *const crate::expat_external_h::XML_Char,
    ) {
        self(user_data, data);
    }
}

static COMMENT_HANDLERS: std::sync::OnceLock<
    std::sync::Mutex<std::collections::HashMap<usize, std::sync::Arc<dyn CommentCallback>>>,
> = std::sync::OnceLock::new();

// Foreign callback values remain in this boundary registry; parser state only
// records whether a default callback is installed.
trait DefaultCallback: Send + Sync {
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        data: *const crate::expat_external_h::XML_Char,
        len: ::core::ffi::c_int,
    );
}

impl DefaultCallback
    for unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const crate::expat_external_h::XML_Char,
        ::core::ffi::c_int,
    ) -> ()
{
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        data: *const crate::expat_external_h::XML_Char,
        len: ::core::ffi::c_int,
    ) {
        self(user_data, data, len);
    }
}

static DEFAULT_HANDLERS: std::sync::OnceLock<
    std::sync::Mutex<std::collections::HashMap<usize, std::sync::Arc<dyn DefaultCallback>>>,
> = std::sync::OnceLock::new();

// Foreign callback values remain in this boundary registry; parser state only
// records whether a start-doctype callback is installed.
trait StartDoctypeDeclCallback: Send + Sync {
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        doctype_name: *const crate::expat_external_h::XML_Char,
        system_id: *const crate::expat_external_h::XML_Char,
        public_id: *const crate::expat_external_h::XML_Char,
        has_internal_subset: ::core::ffi::c_int,
    );
}

impl StartDoctypeDeclCallback
    for unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const crate::expat_external_h::XML_Char,
        *const crate::expat_external_h::XML_Char,
        *const crate::expat_external_h::XML_Char,
        ::core::ffi::c_int,
    )
{
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        doctype_name: *const crate::expat_external_h::XML_Char,
        system_id: *const crate::expat_external_h::XML_Char,
        public_id: *const crate::expat_external_h::XML_Char,
        has_internal_subset: ::core::ffi::c_int,
    ) {
        self(user_data, doctype_name, system_id, public_id, has_internal_subset);
    }
}

static START_DOCTYPE_DECL_HANDLERS: std::sync::OnceLock<
    std::sync::Mutex<
        std::collections::HashMap<usize, std::sync::Arc<dyn StartDoctypeDeclCallback>>,
    >,
> = std::sync::OnceLock::new();

unsafe fn callCharacterDataHandler(
    parser: crate::expat_h::XML_Parser,
    data: *const crate::expat_external_h::XML_Char,
    len: ::core::ffi::c_int,
) {
    let callback = CHARACTER_DATA_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&(parser as usize))
        .cloned();
    if let Some(callback) = callback {
        callback.invoke((*parser).m_handlerArg, data, len);
    }
}

unsafe fn callElementDeclHandler(
    parser: crate::expat_h::XML_Parser,
    name: *const crate::expat_external_h::XML_Char,
    model: *mut crate::expat_h::XML_Content,
) {
    let callback = ELEMENT_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&(parser as usize))
        .cloned();
    if let Some(callback) = callback {
        callback.invoke((*parser).m_handlerArg, name, model);
    }
}

trait EntityDeclCallback: Send + Sync {
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        entity_name: *const crate::expat_external_h::XML_Char,
        is_parameter_entity: ::core::ffi::c_int,
        value: *const crate::expat_external_h::XML_Char,
        value_length: ::core::ffi::c_int,
        base: *const crate::expat_external_h::XML_Char,
        system_id: *const crate::expat_external_h::XML_Char,
        public_id: *const crate::expat_external_h::XML_Char,
        notation_name: *const crate::expat_external_h::XML_Char,
    );
}

impl EntityDeclCallback
    for unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const crate::expat_external_h::XML_Char,
        ::core::ffi::c_int,
        *const crate::expat_external_h::XML_Char,
        ::core::ffi::c_int,
        *const crate::expat_external_h::XML_Char,
        *const crate::expat_external_h::XML_Char,
        *const crate::expat_external_h::XML_Char,
        *const crate::expat_external_h::XML_Char,
    )
{
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        entity_name: *const crate::expat_external_h::XML_Char,
        is_parameter_entity: ::core::ffi::c_int,
        value: *const crate::expat_external_h::XML_Char,
        value_length: ::core::ffi::c_int,
        base: *const crate::expat_external_h::XML_Char,
        system_id: *const crate::expat_external_h::XML_Char,
        public_id: *const crate::expat_external_h::XML_Char,
        notation_name: *const crate::expat_external_h::XML_Char,
    ) {
        self(
            user_data,
            entity_name,
            is_parameter_entity,
            value,
            value_length,
            base,
            system_id,
            public_id,
            notation_name,
        );
    }
}

// Foreign callback values remain in this boundary registry; parser state only
// records whether an entity-declaration callback is installed.
static ENTITY_DECL_HANDLERS: std::sync::OnceLock<
    std::sync::Mutex<std::collections::HashMap<usize, std::sync::Arc<dyn EntityDeclCallback>>>,
> = std::sync::OnceLock::new();

trait UnparsedEntityDeclCallback: Send + Sync {
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        entity_name: *const crate::expat_external_h::XML_Char,
        base: *const crate::expat_external_h::XML_Char,
        system_id: *const crate::expat_external_h::XML_Char,
        public_id: *const crate::expat_external_h::XML_Char,
        notation_name: *const crate::expat_external_h::XML_Char,
    );
}

impl UnparsedEntityDeclCallback
    for unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const crate::expat_external_h::XML_Char,
        *const crate::expat_external_h::XML_Char,
        *const crate::expat_external_h::XML_Char,
        *const crate::expat_external_h::XML_Char,
        *const crate::expat_external_h::XML_Char,
    ) -> ()
{
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        entity_name: *const crate::expat_external_h::XML_Char,
        base: *const crate::expat_external_h::XML_Char,
        system_id: *const crate::expat_external_h::XML_Char,
        public_id: *const crate::expat_external_h::XML_Char,
        notation_name: *const crate::expat_external_h::XML_Char,
    ) {
        self(
            user_data,
            entity_name,
            base,
            system_id,
            public_id,
            notation_name,
        );
    }
}

// Foreign callback values remain in this boundary registry; parser state only
// records whether an unparsed-entity callback is installed.
static UNPARSED_ENTITY_DECL_HANDLERS: std::sync::OnceLock<
    std::sync::Mutex<
        std::collections::HashMap<usize, std::sync::Arc<dyn UnparsedEntityDeclCallback>>,
    >,
> = std::sync::OnceLock::new();

trait NotationDeclCallback: Send + Sync {
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        notation_name: *const crate::expat_external_h::XML_Char,
        base: *const crate::expat_external_h::XML_Char,
        system_id: *const crate::expat_external_h::XML_Char,
        public_id: *const crate::expat_external_h::XML_Char,
    );
}

impl NotationDeclCallback
    for unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const crate::expat_external_h::XML_Char,
        *const crate::expat_external_h::XML_Char,
        *const crate::expat_external_h::XML_Char,
        *const crate::expat_external_h::XML_Char,
    ) -> ()
{
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        notation_name: *const crate::expat_external_h::XML_Char,
        base: *const crate::expat_external_h::XML_Char,
        system_id: *const crate::expat_external_h::XML_Char,
        public_id: *const crate::expat_external_h::XML_Char,
    ) {
        self(user_data, notation_name, base, system_id, public_id);
    }
}

// Foreign callback values remain in this boundary registry; parser state only
// records whether a notation-declaration callback is installed.
static NOTATION_DECL_HANDLERS: std::sync::OnceLock<
    std::sync::Mutex<
        std::collections::HashMap<usize, std::sync::Arc<dyn NotationDeclCallback>>,
    >,
> = std::sync::OnceLock::new();

trait ElementDeclCallback: Send + Sync {
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        name: *const crate::expat_external_h::XML_Char,
        model: *mut crate::expat_h::XML_Content,
    );
}

impl ElementDeclCallback
    for unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const crate::expat_external_h::XML_Char,
        *mut crate::expat_h::XML_Content,
    )
{
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        name: *const crate::expat_external_h::XML_Char,
        model: *mut crate::expat_h::XML_Content,
    ) {
        self(user_data, name, model);
    }
}

// Foreign callback values remain in this boundary registry; parser state only
// records whether an element-declaration callback is installed.
static ELEMENT_DECL_HANDLERS: std::sync::OnceLock<
    std::sync::Mutex<std::collections::HashMap<usize, std::sync::Arc<dyn ElementDeclCallback>>>,
> = std::sync::OnceLock::new();

trait AttlistDeclCallback: Send + Sync {
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        element_name: *const crate::expat_external_h::XML_Char,
        attribute_name: *const crate::expat_external_h::XML_Char,
        attribute_type: *const crate::expat_external_h::XML_Char,
        default_value: *const crate::expat_external_h::XML_Char,
        is_required: ::core::ffi::c_int,
    );
}

impl AttlistDeclCallback
    for unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const crate::expat_external_h::XML_Char,
        *const crate::expat_external_h::XML_Char,
        *const crate::expat_external_h::XML_Char,
        *const crate::expat_external_h::XML_Char,
        ::core::ffi::c_int,
    )
{
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        element_name: *const crate::expat_external_h::XML_Char,
        attribute_name: *const crate::expat_external_h::XML_Char,
        attribute_type: *const crate::expat_external_h::XML_Char,
        default_value: *const crate::expat_external_h::XML_Char,
        is_required: ::core::ffi::c_int,
    ) {
        self(
            user_data,
            element_name,
            attribute_name,
            attribute_type,
            default_value,
            is_required,
        );
    }
}

// Foreign callback values remain in this boundary registry; parser state only
// records whether an attribute-list callback is installed.
static ATTLIST_DECL_HANDLERS: std::sync::OnceLock<
    std::sync::Mutex<std::collections::HashMap<usize, std::sync::Arc<dyn AttlistDeclCallback>>>,
> = std::sync::OnceLock::new();

fn attlist_decl_handler(parser_key: usize) -> Option<std::sync::Arc<dyn AttlistDeclCallback>> {
    ATTLIST_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&parser_key)
        .cloned()
}

trait XmlDeclCallback: Send + Sync {
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        version: *const crate::expat_external_h::XML_Char,
        encoding: *const crate::expat_external_h::XML_Char,
        standalone: ::core::ffi::c_int,
    );
}

impl XmlDeclCallback
    for unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const crate::expat_external_h::XML_Char,
        *const crate::expat_external_h::XML_Char,
        ::core::ffi::c_int,
    )
{
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        version: *const crate::expat_external_h::XML_Char,
        encoding: *const crate::expat_external_h::XML_Char,
        standalone: ::core::ffi::c_int,
    ) {
        self(user_data, version, encoding, standalone);
    }
}

// Foreign callback values remain in this boundary registry; parser state only
// records whether an XML-declaration callback is installed.
static XML_DECL_HANDLERS: std::sync::OnceLock<
    std::sync::Mutex<std::collections::HashMap<usize, std::sync::Arc<dyn XmlDeclCallback>>>,
> = std::sync::OnceLock::new();

trait UnknownEncodingCallback: Send + Sync {
    unsafe fn invoke(
        &self,
        data: *mut ::core::ffi::c_void,
        encoding_name: *const crate::expat_external_h::XML_Char,
        info: *mut crate::expat_h::XML_Encoding,
    ) -> ::core::ffi::c_int;
}

impl UnknownEncodingCallback
    for unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const crate::expat_external_h::XML_Char,
        *mut crate::expat_h::XML_Encoding,
    ) -> ::core::ffi::c_int
{
    unsafe fn invoke(
        &self,
        data: *mut ::core::ffi::c_void,
        encoding_name: *const crate::expat_external_h::XML_Char,
        info: *mut crate::expat_h::XML_Encoding,
    ) -> ::core::ffi::c_int {
        self(data, encoding_name, info)
    }
}

// Foreign callback values remain in this boundary registry; parser state only
// records whether an unknown-encoding callback is installed.
static UNKNOWN_ENCODING_HANDLERS: std::sync::OnceLock<
    std::sync::Mutex<std::collections::HashMap<usize, std::sync::Arc<dyn UnknownEncodingCallback>>>,
> = std::sync::OnceLock::new();

trait ExternalEntityRefCallback: Send + Sync {
    unsafe fn invoke(
        &self,
        parser: crate::expat_h::XML_Parser,
        context: *const crate::expat_external_h::XML_Char,
        base: *const crate::expat_external_h::XML_Char,
        system_id: *const crate::expat_external_h::XML_Char,
        public_id: *const crate::expat_external_h::XML_Char,
    ) -> ::core::ffi::c_int;
}

impl ExternalEntityRefCallback
    for unsafe extern "C" fn(
        crate::expat_h::XML_Parser,
        *const crate::expat_external_h::XML_Char,
        *const crate::expat_external_h::XML_Char,
        *const crate::expat_external_h::XML_Char,
        *const crate::expat_external_h::XML_Char,
    ) -> ::core::ffi::c_int
{
    unsafe fn invoke(
        &self,
        parser: crate::expat_h::XML_Parser,
        context: *const crate::expat_external_h::XML_Char,
        base: *const crate::expat_external_h::XML_Char,
        system_id: *const crate::expat_external_h::XML_Char,
        public_id: *const crate::expat_external_h::XML_Char,
    ) -> ::core::ffi::c_int {
        self(parser, context, base, system_id, public_id)
    }
}

// Foreign callback values live outside parser state.  The parser itself only
// records whether a handler is installed, while this registry preserves the
// C callback ABI and lets call sites clone the callback before re-entry.
static EXTERNAL_ENTITY_REF_HANDLERS: std::sync::OnceLock<
    std::sync::Mutex<
        std::collections::HashMap<usize, std::sync::Arc<dyn ExternalEntityRefCallback>>,
    >,
> = std::sync::OnceLock::new();

trait SkippedEntityCallback: Send + Sync {
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        entity_name: *const crate::expat_external_h::XML_Char,
        is_parameter_entity: ::core::ffi::c_int,
    );
}

impl SkippedEntityCallback
    for unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const crate::expat_external_h::XML_Char,
        ::core::ffi::c_int,
    )
{
    unsafe fn invoke(
        &self,
        user_data: *mut ::core::ffi::c_void,
        entity_name: *const crate::expat_external_h::XML_Char,
        is_parameter_entity: ::core::ffi::c_int,
    ) {
        self(user_data, entity_name, is_parameter_entity);
    }
}

// Foreign callback values live outside parser state.  The parser itself only
// records whether a handler is installed, while this registry preserves the
// C callback ABI and lets call sites clone the callback before re-entry.
static SKIPPED_ENTITY_HANDLERS: std::sync::OnceLock<
    std::sync::Mutex<std::collections::HashMap<usize, std::sync::Arc<dyn SkippedEntityCallback>>>,
> = std::sync::OnceLock::new();

// The initial tokenizer chooses a built-in encoding after inspecting the
// first bytes.  Keep that choice as an index in INIT_ENCODING rather than
// copying a pointer to a static encoding table into parser state.  An unknown
// encoding remains backed by the allocation already owned by the parser.
#[derive(Copy, Clone)]
enum EncodingState {
    Initial,
    Unknown,
}

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
    pub m_startElementHandler: bool,
    pub m_endElementHandler: bool,
    pub m_characterDataHandler: bool,
    pub m_processingInstructionHandler: bool,
    pub m_commentHandler: bool,
    pub m_startCdataSectionHandler: crate::expat_h::XML_StartCdataSectionHandler,
    pub m_endCdataSectionHandler: crate::expat_h::XML_EndCdataSectionHandler,
    pub m_defaultHandler: bool,
    pub m_startDoctypeDeclHandler: bool,
    pub m_endDoctypeDeclHandler: crate::expat_h::XML_EndDoctypeDeclHandler,
    pub m_unparsedEntityDeclHandler: bool,
    pub m_notationDeclHandler: bool,
    pub m_startNamespaceDeclHandler: bool,
    pub m_endNamespaceDeclHandler: bool,
    pub m_notStandaloneHandler: crate::expat_h::XML_NotStandaloneHandler,
    pub m_externalEntityRefHandler: bool,
    pub m_externalEntityRefHandlerArg: crate::expat_h::XML_Parser,
    pub m_skippedEntityHandler: bool,
    pub m_unknownEncodingHandler: bool,
    pub m_elementDeclHandler: bool,
    pub m_attlistDeclHandler: bool,
    pub m_entityDeclHandler: bool,
    pub m_xmlDeclHandler: bool,
    m_encoding: EncodingState,
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
    pub m_processor: ProcessorState,
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
#[repr(C)]

pub struct STRING_POOL {
    // Text lives in Rust-owned slabs.  Each slab also owns an opaque
    // custom-allocator token, so the configured Expat allocator observes the
    // same block allocation, reallocation, and release lifecycle.
    storage: StringPoolStorage,
    // The write cursor is an offset from this checked slab location.  A
    // completed empty value is therefore still representable when `start` is
    // the valid one-past-end position of a full slab.
    ptr_offset: usize,
    start: Option<PoolStringRef>,
    pub parser: crate::expat_h::XML_Parser,
    // Active blocks grow only at the head.  This count lets pool clients use
    // a stable ordinal from the tail without retaining a pointer into a block.
    pub blockCount: usize,
}

impl STRING_POOL {
    // `start` can advance after a string is committed, including to the
    // one-past-end position of its slab.
    fn remaining_capacity(&self) -> Option<usize> {
        let Some(start) = self.start else {
            return Some(0);
        };
        self.storage
            .active
            .get(start.block_from_tail.get() - 1)?
            .chars
            .len()
            .checked_sub(start.offset)
    }

    fn is_full(&self) -> bool {
        self.remaining_capacity()
            .is_none_or(|capacity| self.ptr_offset >= capacity)
    }

    fn rewind(&mut self) {
        self.ptr_offset = 0;
    }

    fn commit(&mut self) {
        self.start = self.start.and_then(|start| {
            Some(PoolStringRef {
                block_from_tail: start.block_from_tail,
                offset: start.offset.checked_add(self.ptr_offset)?,
            })
        });
        self.ptr_offset = 0;
    }

    fn write_cursor(&mut self, value: crate::expat_external_h::XML_Char) -> bool {
        let Some(start) = self.start_ref(true) else {
            return false;
        };
        let block_index = start.block_from_tail.get() - 1;
        let Some(slot) = start.offset.checked_add(self.ptr_offset) else {
            return false;
        };
        let Some(block) = self.storage.active.get_mut(block_index) else {
            return false;
        };
        let Some(destination) = block.chars.get_mut(slot) else {
            return false;
        };
        *destination = value;
        self.ptr_offset += 1;
        true
    }

    fn last_cursor_char(&self) -> Option<crate::expat_external_h::XML_Char> {
        let start = self.start_ref(true)?;
        let slot = start.offset.checked_add(self.ptr_offset.checked_sub(1)?)?;
        self.storage
            .active
            .get(start.block_from_tail.get() - 1)?
            .chars
            .get(slot)
            .copied()
    }

    fn discard_last_cursor_char(&mut self) -> bool {
        let Some(offset) = self.ptr_offset.checked_sub(1) else {
            return false;
        };
        self.ptr_offset = offset;
        true
    }

    fn replace_last_cursor_char(&mut self, value: crate::expat_external_h::XML_Char) -> bool {
        let Some(start) = self.start_ref(true) else {
            return false;
        };
        let Some(cursor) = self.ptr_offset.checked_sub(1) else {
            return false;
        };
        let Some(slot) = start.offset.checked_add(cursor) else {
            return false;
        };
        let Some(destination) = self
            .storage
            .active
            .get_mut(start.block_from_tail.get() - 1)
            .and_then(|block| block.chars.get_mut(slot))
        else {
            return false;
        };
        *destination = value;
        true
    }

    fn chars_at(
        &self,
        string: PoolStringRef,
        length: ::core::ffi::c_int,
    ) -> Option<&[crate::expat_external_h::XML_Char]> {
        let block_index = string.block_from_tail.get().checked_sub(1)?;
        let length = usize::try_from(length).ok()?;
        let block = self.storage.active.get(block_index)?;
        let end = string.offset.checked_add(length)?;
        block.chars.get(string.offset..end)
    }

    fn chars_from(
        &self,
        string: PoolStringRef,
    ) -> Option<&[crate::expat_external_h::XML_Char]> {
        let block_index = string.block_from_tail.get().checked_sub(1)?;
        self.storage.active.get(block_index)?.chars.get(string.offset..)
    }

    fn start_ref(&self, allow_block_end: bool) -> Option<PoolStringRef> {
        let start = self.start?;
        let block = self.storage.active.get(start.block_from_tail.get() - 1)?;
        if start.offset < block.chars.len()
            || allow_block_end && start.offset == block.chars.len()
        {
            Some(start)
        } else {
            None
        }
    }
}

struct StringPoolStorage {
    // Both vectors are tail-to-head: popping `free` produces the same LIFO
    // reuse order as Expat's former free-block list.
    active: Vec<StringPoolBlock>,
    free: Vec<StringPoolBlock>,
}

struct StringPoolBlock {
    chars: Vec<crate::expat_external_h::XML_Char>,
    backing: Box<dyn FnMut(StringPoolAllocationAction) -> bool>,
}

enum StringPoolAllocationAction {
    Grow(crate::__stddef_size_t_h::size_t),
    Free(::core::ffi::c_int),
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PoolStringRef {
    // A zeroed `Option<PoolStringRef>` is used for freshly allocated hash
    // table records.  Reserve zero as its niche so that initialization stays
    // valid while the actual tail-relative ordinal remains lossless.
    block_from_tail: std::num::NonZeroUsize,
    offset: usize,
}

// Entity replacement text can be committed either to the dedicated value
// pool during declaration parsing or to the general DTD pool while copying a
// DTD for an external entity parser.  Retain the pool identity together with
// its checked location so `ENTITY` never keeps an address into either slab.
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum EntityTextPool {
    Dtd = 0,
    EntityValue = 1,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct EntityTextRef {
    pool: EntityTextPool,
    // Hash-table records are zero-initialized before their declaration is
    // known.  Keep the nullable niche inside this C-initialized record so an
    // all-zero `ENTITY` unambiguously remains an external entity.
    string: Option<PoolStringRef>,
}

impl EntityTextRef {
    fn is_some(self) -> bool {
        self.string.is_some()
    }

    fn is_none(self) -> bool {
        self.string.is_none()
    }

    fn present(self) -> Option<Self> {
        self.string?;
        Some(self)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct NS_ATT {
    pub version: ::core::ffi::c_ulong,
    pub hash: ::core::ffi::c_ulong,
    // Namespace-attribute cache entries only live while the temporary pool is
    // live.  Keep a checked location in that pool rather than an address into
    // its custom-allocator-owned storage.
    pub uriName: Option<PoolStringRef>,
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
    // The buffer base remains allocated and freed through the parser's
    // configured allocator.  `bufEnd` is the base pointer; `bufSize` is its
    // capacity in bytes.
    pub bufEnd: *mut ::core::ffi::c_char,
    pub bufSize: crate::__stddef_size_t_h::size_t,
    pub bindings: *mut BINDING,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct C2Rust_Unnamed_1 {
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct TAG_NAME {
    // Tag names either reside in the tag's allocator-backed buffer or in the
    // temporary pool used for an empty element.  An offset/reference keeps
    // that distinction explicit without retaining an address into either
    // allocation.
    pub str: TagNameStorage,
    // When namespace processing is active, this is the character offset of
    // the local part within the tag's allocator-backed name buffer.  Keeping
    // an offset rather than a pointer lets that buffer move during
    // `storeRawNames` without leaving a stale interior pointer behind.
    pub localPart: Option<usize>,
    pub strLen: ::core::ffi::c_int,
    pub uriLen: ::core::ffi::c_int,
    pub prefixLen: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
pub enum TagNameStorage {
    Unset,
    TagBuffer { offset: usize },
    TempPool(PoolStringRef),
    // The expanded namespace name remains in the active binding's
    // allocator-backed URI buffer.  Its binding is resolved from the DTD
    // when the callback needs an address, rather than retained here.
    NamespaceUri,
}
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
    // The content-model graph is shared when a child parser inherits its
    // parent's DTD.  Its nodes are ordinary indexed Rust storage; `backing`
    // below is retained only to preserve the observable allocation schedule
    // of the configured Expat memory suite.
    scaffold: std::sync::Arc<std::sync::Mutex<ScaffoldStorage>>,
    pub contentStringLen: ::core::ffi::c_uint,
    pub scaffSize: ::core::ffi::c_uint,
    pub scaffCount: ::core::ffi::c_uint,
    pub scaffLevel: ::core::ffi::c_int,
    pub scaffIndex: std::sync::Arc<std::sync::Mutex<Vec<::core::ffi::c_int>>>,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct CONTENT_SCAFFOLD {
    pub type_0: crate::expat_h::XML_Content_Type,
    pub quant: crate::expat_h::XML_Content_Quant,
    // Name scaffolds refer to a NUL-terminated value committed in the DTD
    // string pool.  The handle keeps the custom-allocator-owned block alive
    // without retaining a raw address; other scaffold kinds have no name.
    pub name: Option<PoolStringRef>,
    pub firstchild: ::core::ffi::c_int,
    pub lastchild: ::core::ffi::c_int,
    pub childcnt: ::core::ffi::c_int,
    pub nextsib: ::core::ffi::c_int,
}

// `backing` is deliberately opaque: content-model nodes live in `nodes`, but
// Expat's custom allocator still sees the same malloc/realloc/free sequence
// (and can still inject allocation failure).  Its captured allocation is
// never dereferenced or exposed to DTD state.
struct ScaffoldStorage {
    nodes: Vec<CONTENT_SCAFFOLD>,
    backing: Option<Box<dyn FnMut(ScaffoldAllocationAction) -> bool>>,
}

enum ScaffoldAllocationAction {
    Grow(usize),
    Free(::core::ffi::c_int),
}

impl ScaffoldStorage {
    fn empty() -> Self {
        Self {
            nodes: Vec::new(),
            backing: None,
        }
    }
}

fn empty_scaffold() -> std::sync::Arc<std::sync::Mutex<ScaffoldStorage>> {
    std::sync::Arc::new(std::sync::Mutex::new(ScaffoldStorage::empty()))
}
#[repr(C)]

pub struct HASH_TABLE {
    // The table owns a sized slot collection, so lookup and iteration can use
    // checked indexing instead of pointer arithmetic.  `backing` keeps the
    // corresponding allocation in the parser's configured memory suite.
    pub v: Option<HashTableSlots>,
    pub power: ::core::ffi::c_uchar,
    pub size: crate::__stddef_size_t_h::size_t,
    pub used: crate::__stddef_size_t_h::size_t,
    // This token retains the parser-specific allocation route without
    // retaining a dereferenceable parser back-pointer in the table.
    allocator: Option<HashTableAllocator>,
}

pub struct HashTableSlots {
    entries: Vec<Option<NamedAllocation>>,
    backing: Box<dyn FnMut(::core::ffi::c_int)>,
}

struct HashTableAllocator {
    allocate: Box<
        dyn FnMut(
            crate::__stddef_size_t_h::size_t,
            ::core::ffi::c_int,
        ) -> Option<Box<dyn FnMut(::core::ffi::c_int)>>,
    >,
}

impl HashTableAllocator {
    fn allocate(
        &mut self,
        size: crate::__stddef_size_t_h::size_t,
        source_line: ::core::ffi::c_int,
    ) -> Option<Box<dyn FnMut(::core::ffi::c_int)>> {
        (self.allocate)(size, source_line)
    }
}

struct NamedAllocation {
    bytes: Vec<usize>,
    backing: Box<dyn FnMut(::core::ffi::c_int)>,
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
    // Element types live in the name table allocation, whose leading header
    // owns the pool-backed key.  Keeping that header here avoids a duplicate
    // pointer-bearing view of the same allocation while retaining the table's
    // layout and configured allocator.
    pub named: NAMED,
    // Prefix records are owned by the DTD prefix table.  Retain their
    // pool-backed name rather than an address into that table, so the link
    // remains allocator-neutral and can be resolved when needed.
    pub prefix: PoolStringRef,
    pub hasPrefix: crate::expat_h::XML_Bool,
    pub idAtt: *const ATTRIBUTE_ID,
    pub nDefaultAtts: ::core::ffi::c_int,
    pub allocDefaultAtts: ::core::ffi::c_int,
    // The storage remains allocated and released through the parser's
    // configured allocator.  NonNull makes its nullable ownership state
    // explicit without changing that allocator pairing.
    pub defaultAtts: Option<std::ptr::NonNull<DEFAULT_ATTRIBUTE>>,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct DEFAULT_ATTRIBUTE {
    // A default attribute names an identifier in the DTD attribute table.
    // Retain the identifier's pool-owned name, rather than an address into
    // that table: the handle stays valid across table growth and can be
    // resolved when the default is applied.
    //
    // This remains optional so an all-zero allocation representation is valid
    // until `defineAttribute` initializes the record.
    pub id: Option<PoolStringRef>,
    pub isCdata: crate::expat_h::XML_Bool,
    // Default values are pool-owned and nullable.  `NonNull` retains the
    // pointer-sized optional representation without exposing a raw field.
    pub value: Option<std::ptr::NonNull<crate::expat_external_h::XML_Char>>,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct ENTITY {
    // Hash-table records begin with their pool-backed key.  Reusing the
    // shared header keeps the hash-table view and entity view of this
    // allocator-owned allocation in sync without a second pointer field.
    pub named: NAMED,
    // Internal replacement text is a checked location in one of the DTD's
    // configured-allocator pools.  Its nullability continues to denote an
    // external entity without retaining an allocator-owned address.
    pub textPtr: EntityTextRef,
    pub textLen: ::core::ffi::c_int,
    pub processed: ::core::ffi::c_int,
    // System identifiers are owned by the DTD string pool.  Retain their
    // stable pool location instead of an address into allocator-managed
    // storage; callback adapters resolve it immediately before invocation.
    pub systemId: Option<PoolStringRef>,
    // A base identifier is nullable and, when present, is identified by its
    // stable location in the DTD string pool.  The address is recovered only
    // at callback boundaries, after checking that the pool block is live.
    pub base: Option<PoolStringRef>,
    // Public identifiers are normalized in, and owned by, the DTD pool.
    // Store a checked pool location instead of retaining its allocator-owned
    // address between callbacks.
    pub publicId: Option<PoolStringRef>,
    // Unparsed-entity notation names are committed to the DTD string pool.
    // Keep their stable pool location rather than retaining a raw address.
    pub notation: Option<PoolStringRef>,
    pub open: crate::expat_h::XML_Bool,
    pub hasMore: crate::expat_h::XML_Bool,
    pub is_param: crate::expat_h::XML_Bool,
    pub is_internal: crate::expat_h::XML_Bool,
}

fn entity_text_chars(
    dtd: &DTD,
    text: EntityTextRef,
    length: ::core::ffi::c_int,
) -> Option<&[crate::expat_external_h::XML_Char]> {
    let string = text.string?;
    match text.pool {
        EntityTextPool::Dtd => dtd.pool.chars_at(string, length),
        EntityTextPool::EntityValue => dtd.entityValuePool.chars_at(string, length),
    }
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

#[derive(Copy, Clone)]
pub enum ProcessorState {
    PrologInit,
    Content,
    ExternalEntityInit,
    ExternalEntityInit2,
    ExternalEntityInit3,
    ExternalEntityContent,
    ExternalParEntInit,
    ExternalParEnt,
    EntityValueInit,
    EntityValue,
    CdataSection,
    IgnoreSection,
    Prolog,
    Epilog,
    InternalEntity,
    Error,
}

type Processor = unsafe extern "C" fn(
    crate::expat_h::XML_Parser,
    *const ::core::ffi::c_char,
    *const ::core::ffi::c_char,
    *mut *const ::core::ffi::c_char,
) -> crate::expat_h::XML_Error;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct HASH_TABLE_ITER<'a> {
    pub table: Option<&'a HASH_TABLE>,
    pub next: crate::__stddef_size_t_h::size_t,
}

pub type XML_Account = ::core::ffi::c_uint;

pub const XML_ACCOUNT_NONE: XML_Account = 2;

pub const XML_ACCOUNT_ENTITY_EXPANSION: XML_Account = 1;

pub const XML_ACCOUNT_DIRECT: XML_Account = 0;

pub type ICHAR = ::core::ffi::c_char;
const XML_NAMESPACE_LEN: ::core::ffi::c_int = 36;
const XMLNS_NAMESPACE_LEN: ::core::ffi::c_int = 29;

pub const INIT_TAG_BUF_SIZE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;

pub const INIT_DATA_BUF_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;

pub const INIT_ATTS_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;

pub const INIT_ATTS_VERSION: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;

pub const INIT_BLOCK_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;

pub const INIT_BUFFER_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;

pub const EXPAND_SPARE: ::core::ffi::c_int = 24 as ::core::ffi::c_int;

pub const INIT_SCAFFOLD_ELEMENTS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
#[no_mangle]

pub static mut g_reparseDeferralEnabledDefault: crate::expat_h::XML_Bool = crate::expat_h::XML_TRUE;
#[no_mangle]

pub static mut g_bytesScanned: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;

unsafe extern "C" fn expat_heap_stat(
    mut rootParser: crate::expat_h::XML_Parser,
    mut operator: ::core::ffi::c_char,
    mut absDiff: XmlBigCount,
    mut newTotal: XmlBigCount,
    mut peakTotal: XmlBigCount,
    mut sourceLine: ::core::ffi::c_int,
) {
    let amplification: ::core::ffi::c_float = newTotal as ::core::ffi::c_float
        / (*rootParser).m_accounting.countBytesDirect as ::core::ffi::c_float;
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"expat: Allocations(%p): Direct %10llu, allocated %c%10llu to %10llu (%10llu peak), amplification %8.2f (xmlparse.c:%d)\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        rootParser as *mut ::core::ffi::c_void,
        (*rootParser).m_accounting.countBytesDirect,
        operator as ::core::ffi::c_int,
        absDiff,
        newTotal,
        peakTotal,
        amplification as ::core::ffi::c_double,
        sourceLine,
    );
}

unsafe extern "C" fn expat_heap_increase_tolerable(
    mut rootParser: crate::expat_h::XML_Parser,
    mut increase: XmlBigCount,
    mut sourceLine: ::core::ffi::c_int,
) -> bool {
    '_c2rust_label: {
        if !rootParser.is_null() {
        } else {
            crate::stdlib::__assert_fail(
                b"rootParser != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../../expat/lib/xmlparse.c\0".as_ptr() as *const ::core::ffi::c_char,
                815 as ::core::ffi::c_uint,
                b"_Bool expat_heap_increase_tolerable(XML_Parser, XmlBigCount, int)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if increase > 0 as XmlBigCount {
        } else {
            crate::stdlib::__assert_fail(
                b"increase > 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"../../expat/lib/xmlparse.c\0".as_ptr() as *const ::core::ffi::c_char,
                816 as ::core::ffi::c_uint,
                b"_Bool expat_heap_increase_tolerable(XML_Parser, XmlBigCount, int)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    let mut newTotal: XmlBigCount = 0 as XmlBigCount;
    let mut tolerable: bool = crate::stdbool_h::true_0 != 0;
    if (-1 as ::core::ffi::c_int as XmlBigCount)
        .wrapping_sub((*rootParser).m_alloc_tracker.bytesAllocated)
        < increase
    {
        tolerable = crate::stdbool_h::false_0 != 0;
    } else {
        newTotal = (*rootParser)
            .m_alloc_tracker
            .bytesAllocated
            .wrapping_add(increase);
        if newTotal >= (*rootParser).m_alloc_tracker.activationThresholdBytes {
            '_c2rust_label_1: {
                if newTotal > 0 as XmlBigCount {
                } else {
                    crate::stdlib::__assert_fail(
                        b"newTotal > 0\0".as_ptr() as *const ::core::ffi::c_char,
                        b"../../expat/lib/xmlparse.c\0".as_ptr() as *const ::core::ffi::c_char,
                        828 as ::core::ffi::c_uint,
                        b"_Bool expat_heap_increase_tolerable(XML_Parser, XmlBigCount, int)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            };
            let amplification: ::core::ffi::c_float = newTotal as ::core::ffi::c_float
                / (*rootParser).m_accounting.countBytesDirect as ::core::ffi::c_float;
            if amplification > (*rootParser).m_alloc_tracker.maximumAmplificationFactor {
                tolerable = crate::stdbool_h::false_0 != 0;
            }
        }
    }
    if !tolerable && (*rootParser).m_alloc_tracker.debugLevel >= 1 as ::core::ffi::c_ulong {
        expat_heap_stat(
            rootParser,
            '+' as ::core::ffi::c_char,
            increase,
            newTotal,
            newTotal,
            sourceLine,
        );
    }
    return tolerable;
}
pub unsafe extern "C" fn expat_malloc(
    mut parser: crate::expat_h::XML_Parser,
    mut size: crate::__stddef_size_t_h::size_t,
    mut sourceLine: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    if (crate::stdlib::SIZE_MAX as crate::__stddef_size_t_h::size_t).wrapping_sub(size)
        < ::core::mem::size_of::<crate::__stddef_size_t_h::size_t>()
            .wrapping_add(crate::internal_h::EXPAT_MALLOC_PADDING)
    {
        return crate::__stddef_null_h::NULL;
    }
    let rootParser: crate::expat_h::XML_Parser =
        getRootParserOf(parser, ::core::ptr::null_mut::<::core::ffi::c_uint>());
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
        ::core::mem::size_of::<crate::__stddef_size_t_h::size_t>()
            .wrapping_add(crate::internal_h::EXPAT_MALLOC_PADDING)
            .wrapping_add(size);
    if (-1 as ::core::ffi::c_int as XmlBigCount)
        .wrapping_sub((*rootParser).m_alloc_tracker.bytesAllocated)
        < bytesToAllocate as XmlBigCount
    {
        return crate::__stddef_null_h::NULL;
    }
    if !expat_heap_increase_tolerable(rootParser, bytesToAllocate as XmlBigCount, sourceLine) {
        return crate::__stddef_null_h::NULL;
    }
    let mallocedPtr: *mut ::core::ffi::c_void =
        (*parser)
            .m_mem
            .malloc_fcn
            .expect("non-null function pointer")(bytesToAllocate);
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
            rootParser,
            '+' as ::core::ffi::c_char,
            bytesToAllocate as XmlBigCount,
            (*rootParser).m_alloc_tracker.bytesAllocated,
            (*rootParser).m_alloc_tracker.peakBytesAllocated,
            sourceLine,
        );
    }
    return (mallocedPtr as *mut ::core::ffi::c_char)
        .offset(::core::mem::size_of::<crate::__stddef_size_t_h::size_t>() as isize)
        .offset(crate::internal_h::EXPAT_MALLOC_PADDING as isize)
        as *mut ::core::ffi::c_void;
}
#[export_name = "expat_malloc"]

pub unsafe extern "C" fn expat_malloc_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut size: crate::__stddef_size_t_h::size_t,
    mut sourceLine: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    expat_malloc(parser, size, sourceLine)
}
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
        getRootParserOf(parser, ::core::ptr::null_mut::<::core::ffi::c_uint>());
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
        .offset(-(::core::mem::size_of::<crate::__stddef_size_t_h::size_t>() as isize))
        as *mut ::core::ffi::c_void;
    let bytesAllocated: crate::__stddef_size_t_h::size_t =
        ::core::mem::size_of::<crate::__stddef_size_t_h::size_t>()
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
            rootParser,
            '-' as ::core::ffi::c_char,
            bytesAllocated as XmlBigCount,
            (*rootParser).m_alloc_tracker.bytesAllocated,
            (*rootParser).m_alloc_tracker.peakBytesAllocated,
            sourceLine,
        );
    }
    (*parser).m_mem.free_fcn.expect("non-null function pointer")(mallocedPtr);
}
#[export_name = "expat_free"]

pub unsafe extern "C" fn expat_free_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut ptr: *mut ::core::ffi::c_void,
    mut sourceLine: ::core::ffi::c_int,
) {
    expat_free(parser, ptr, sourceLine)
}
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
        getRootParserOf(parser, ::core::ptr::null_mut::<::core::ffi::c_uint>());
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
        .offset(-(::core::mem::size_of::<crate::__stddef_size_t_h::size_t>() as isize))
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
        if !expat_heap_increase_tolerable(rootParser, absDiff as XmlBigCount, sourceLine) {
            return crate::__stddef_null_h::NULL;
        }
    }
    '_c2rust_label_1: {
        if (18446744073709551615 as usize)
            .wrapping_sub(::core::mem::size_of::<crate::__stddef_size_t_h::size_t>())
            .wrapping_sub(
                ::core::mem::size_of::<::core::ffi::c_longlong>()
                    .wrapping_sub(::core::mem::size_of::<crate::__stddef_size_t_h::size_t>()),
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
        ::core::mem::size_of::<crate::__stddef_size_t_h::size_t>()
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
            rootParser,
            (if isIncrease as ::core::ffi::c_int != 0 {
                '+' as ::core::ffi::c_int
            } else {
                '-' as ::core::ffi::c_int
            }) as ::core::ffi::c_char,
            absDiff as XmlBigCount,
            (*rootParser).m_alloc_tracker.bytesAllocated,
            (*rootParser).m_alloc_tracker.peakBytesAllocated,
            sourceLine,
        );
    }
    *(mallocedPtr as *mut crate::__stddef_size_t_h::size_t) = size;
    return (mallocedPtr as *mut ::core::ffi::c_char)
        .offset(::core::mem::size_of::<crate::__stddef_size_t_h::size_t>() as isize)
        .offset(crate::internal_h::EXPAT_MALLOC_PADDING as isize)
        as *mut ::core::ffi::c_void;
}
#[export_name = "expat_realloc"]

pub unsafe extern "C" fn expat_realloc_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut ptr: *mut ::core::ffi::c_void,
    mut size: crate::__stddef_size_t_h::size_t,
    mut sourceLine: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    expat_realloc(parser, ptr, size, sourceLine)
}
pub unsafe extern "C" fn XML_ParserCreate(
    mut encodingName: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Parser {
    return XML_ParserCreate_MM(
        encodingName,
        ::core::ptr::null::<crate::expat_h::XML_Memory_Handling_Suite>(),
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    );
}
#[export_name = "XML_ParserCreate"]

pub unsafe extern "C" fn XML_ParserCreate_ffi(
    mut encodingName: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Parser {
    XML_ParserCreate(encodingName)
}
pub unsafe extern "C" fn XML_ParserCreateNS(
    mut encodingName: *const crate::expat_external_h::XML_Char,
    mut nsSep: crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Parser {
    let mut tmp: [crate::expat_external_h::XML_Char; 2] =
        [nsSep, 0 as crate::expat_external_h::XML_Char];
    return XML_ParserCreate_MM(
        encodingName,
        ::core::ptr::null::<crate::expat_h::XML_Memory_Handling_Suite>(),
        &raw mut tmp as *mut crate::expat_external_h::XML_Char,
    );
}
#[export_name = "XML_ParserCreateNS"]

pub unsafe extern "C" fn XML_ParserCreateNS_ffi(
    mut encodingName: *const crate::expat_external_h::XML_Char,
    mut nsSep: crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Parser {
    XML_ParserCreateNS(encodingName, nsSep)
}
static mut implicitContext: [crate::expat_external_h::XML_Char; 41] = [
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
    '\0' as crate::expat_external_h::XML_Char,
];

unsafe extern "C" fn ENTROPY_DEBUG(
    mut label: *const ::core::ffi::c_char,
    mut entropy: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    if getDebugLevel(
        b"EXPAT_ENTROPY_DEBUG\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_ulong,
    ) >= 1 as ::core::ffi::c_ulong
    {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"expat: Entropy: %s --> 0x%0*lx (%lu bytes)\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            label,
            ::core::mem::size_of::<::core::ffi::c_ulong>() as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int,
            entropy,
            ::core::mem::size_of::<::core::ffi::c_ulong>() as ::core::ffi::c_ulong,
        );
    }
    return entropy;
}

unsafe extern "C" fn generate_hash_secret_salt(
    _parser: crate::expat_h::XML_Parser,
) -> ::core::ffi::c_ulong {
    let mut entropy: ::core::ffi::c_ulong = 0;
    crate::stdlib::arc4random_buf(
        &raw mut entropy as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_ulong>(),
    );
    return ENTROPY_DEBUG(
        b"arc4random_buf\0".as_ptr() as *const ::core::ffi::c_char,
        entropy,
    );
}

unsafe extern "C" fn get_hash_secret_salt(
    mut parser: crate::expat_h::XML_Parser,
) -> ::core::ffi::c_ulong {
    let rootParser: crate::expat_h::XML_Parser =
        getRootParserOf(parser, ::core::ptr::null_mut::<::core::ffi::c_uint>());
    '_c2rust_label: {
        if (*rootParser).m_parentParser.is_null() {
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
    return (*rootParser).m_hash_secret_salt;
}

unsafe extern "C" fn callProcessor(
    mut parser: crate::expat_h::XML_Parser,
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut endPtr: *mut *const ::core::ffi::c_char,
) -> crate::expat_h::XML_Error {
    let have_now: crate::__stddef_size_t_h::size_t = (if !end.is_null() && !start.is_null() {
        end.offset_from(start)
    } else {
        0 as isize
    }) as crate::__stddef_size_t_h::size_t;
    if (*parser).m_reparseDeferralEnabled as ::core::ffi::c_int != 0
        && (*parser).m_parsingStatus.finalBuffer == 0
    {
        let had_before: crate::__stddef_size_t_h::size_t = (*parser).m_partialTokenBytesBefore;
        let mut available_buffer: crate::__stddef_size_t_h::size_t =
            (if !(*parser).m_bufferPtr.is_null() && !(*parser).m_buffer.is_null() {
                (*parser).m_bufferPtr.offset_from((*parser).m_buffer)
            } else {
                0 as isize
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
                (*parser).m_bufferLim.offset_from((*parser).m_bufferEnd)
            } else {
                0 as isize
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
    g_bytesScanned = g_bytesScanned.wrapping_add(have_now as ::core::ffi::c_uint);
    let mut ret: crate::expat_h::XML_Error = crate::expat_h::XML_ERROR_NONE;
    *endPtr = start;
    loop {
        let processor: Processor = match (*parser).m_processor {
            ProcessorState::PrologInit => prologInitProcessor,
            ProcessorState::Content => contentProcessor,
            ProcessorState::ExternalEntityInit => externalEntityInitProcessor,
            ProcessorState::ExternalEntityInit2 => externalEntityInitProcessor2,
            ProcessorState::ExternalEntityInit3 => externalEntityInitProcessor3,
            ProcessorState::ExternalEntityContent => externalEntityContentProcessor,
            ProcessorState::ExternalParEntInit => externalParEntInitProcessor,
            ProcessorState::ExternalParEnt => externalParEntProcessor,
            ProcessorState::EntityValueInit => entityValueInitProcessor,
            ProcessorState::EntityValue => entityValueProcessor,
            ProcessorState::CdataSection => cdataSectionProcessor,
            ProcessorState::IgnoreSection => ignoreSectionProcessor,
            ProcessorState::Prolog => prologProcessor,
            ProcessorState::Epilog => epilogProcessor,
            ProcessorState::InternalEntity => internalEntityProcessor,
            ProcessorState::Error => errorProcessor,
        };
        ret = processor(parser, *endPtr, end, endPtr);
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
        (*parser).m_hash_secret_salt = generate_hash_secret_salt(parser);
    }
    if (*parser).m_ns != 0 {
        return setContext(
            parser,
            &raw const implicitContext as *const crate::expat_external_h::XML_Char,
        );
    }
    return crate::expat_h::XML_TRUE;
}
pub unsafe extern "C" fn XML_ParserCreate_MM(
    mut encodingName: *const crate::expat_external_h::XML_Char,
    mut memsuite: *const crate::expat_h::XML_Memory_Handling_Suite,
    mut nameSep: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Parser {
    return parserCreate(
        encodingName,
        memsuite,
        nameSep,
        ::core::ptr::null_mut::<DTD>(),
        ::core::ptr::null_mut::<XML_ParserStruct>(),
    );
}
#[export_name = "XML_ParserCreate_MM"]

pub unsafe extern "C" fn XML_ParserCreate_MM_ffi(
    mut encodingName: *const crate::expat_external_h::XML_Char,
    mut memsuite: *const crate::expat_h::XML_Memory_Handling_Suite,
    mut nameSep: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Parser {
    XML_ParserCreate_MM(encodingName, memsuite, nameSep)
}

// Parser storage has the same header used by expat_malloc/expat_free.  Keep
// that allocation under the configured memory suite: in particular, a parser
// created with XML_ParserCreate_MM must be released through the matching
// callback even when construction later fails.
unsafe fn allocate_parser_storage(
    memory_suite: crate::expat_h::XML_Memory_Handling_Suite,
) -> Option<crate::expat_h::XML_Parser> {
    let allocation_size = ::core::mem::size_of::<crate::__stddef_size_t_h::size_t>()
        .wrapping_add(crate::internal_h::EXPAT_MALLOC_PADDING)
        .wrapping_add(::core::mem::size_of::<XML_ParserStruct>());
    let allocation = memory_suite.malloc_fcn.expect("non-null function pointer")(allocation_size);
    if allocation.is_null() {
        return None;
    }
    let allocation = allocation.cast::<u8>();
    allocation
        .cast::<crate::__stddef_size_t_h::size_t>()
        .write(::core::mem::size_of::<XML_ParserStruct>());
    Some(
        allocation
            .add(::core::mem::size_of::<crate::__stddef_size_t_h::size_t>())
            .add(crate::internal_h::EXPAT_MALLOC_PADDING)
            .cast::<XML_ParserStruct>(),
    )
}

fn empty_string_pool() -> STRING_POOL {
    STRING_POOL {
        storage: StringPoolStorage {
            active: Vec::new(),
            free: Vec::new(),
        },
        ptr_offset: 0,
        start: None,
        parser: ::core::ptr::null_mut::<XML_ParserStruct>(),
        blockCount: 0,
    }
}

fn initial_encoding() -> crate::src::xmltok::INIT_ENCODING {
    use crate::src::xmltok::{
        AttributeScanner, CharRefNumberDecoder, LiteralScanner, NameLength, NameMatcher,
        PositionUpdater, PredefinedEntityNameMatcher, PublicIdChecker, Scanner, Utf16Converter,
        Utf8Converter, WhitespaceSkipper,
    };

    crate::src::xmltok::INIT_ENCODING {
        initEnc: crate::src::xmltok::ENCODING {
            scanners: [Scanner::NormalProlog; 4],
            literalScanners: [LiteralScanner::NormalAttributeValue; 2],
            nameMatchesAscii: NameMatcher::Normal,
            nameLength: NameLength::Normal,
            skipS: WhitespaceSkipper::Normal,
            getAtts: AttributeScanner::Normal,
            charRefNumber: CharRefNumberDecoder::Normal,
            predefinedEntityName: PredefinedEntityNameMatcher::Normal,
            updatePosition: PositionUpdater::Init,
            isPublicId: PublicIdChecker::Normal,
            utf8Convert: Utf8Converter::Utf8,
            utf16Convert: Utf16Converter::Utf8,
            minBytesPerChar: 0,
            isUtf8: 0,
            isUtf16: 0,
        },
        selected_encoding: None,
    }
}

fn initial_parser_struct(
    memory_suite: crate::expat_h::XML_Memory_Handling_Suite,
) -> XML_ParserStruct {
    XML_ParserStruct {
        m_userData: crate::__stddef_null_h::NULL,
        m_handlerArg: crate::__stddef_null_h::NULL,
        m_buffer: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        m_mem: memory_suite,
        m_bufferPtr: ::core::ptr::null::<::core::ffi::c_char>(),
        m_bufferEnd: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        m_bufferLim: ::core::ptr::null::<::core::ffi::c_char>(),
        m_parseEndByteIndex: 0,
        m_parseEndPtr: ::core::ptr::null::<::core::ffi::c_char>(),
        m_partialTokenBytesBefore: 0,
        m_reparseDeferralEnabled: crate::expat_h::XML_FALSE,
        m_lastBufferRequestSize: 0,
        m_dataBuf: ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>(),
        m_dataBufEnd: ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>(),
        m_startElementHandler: false,
        m_endElementHandler: false,
        m_characterDataHandler: false,
        m_processingInstructionHandler: false,
        m_commentHandler: false,
        m_startCdataSectionHandler: None,
        m_endCdataSectionHandler: None,
        m_defaultHandler: false,
        m_startDoctypeDeclHandler: false,
        m_endDoctypeDeclHandler: None,
        m_unparsedEntityDeclHandler: false,
        m_notationDeclHandler: false,
        m_startNamespaceDeclHandler: false,
        m_endNamespaceDeclHandler: false,
        m_notStandaloneHandler: None,
        m_externalEntityRefHandler: false,
        m_externalEntityRefHandlerArg: ::core::ptr::null_mut::<XML_ParserStruct>(),
        m_skippedEntityHandler: false,
        m_unknownEncodingHandler: false,
        m_elementDeclHandler: false,
        m_attlistDeclHandler: false,
        m_entityDeclHandler: false,
        m_xmlDeclHandler: false,
        m_encoding: EncodingState::Initial,
        m_initEncoding: initial_encoding(),
        m_internalEncoding: ::core::ptr::null::<crate::src::xmltok::ENCODING>(),
        m_protocolEncodingName: ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
        m_ns: crate::expat_h::XML_FALSE,
        m_ns_triplets: crate::expat_h::XML_FALSE,
        m_unknownEncodingMem: crate::__stddef_null_h::NULL,
        m_unknownEncodingData: crate::__stddef_null_h::NULL,
        m_unknownEncodingHandlerData: crate::__stddef_null_h::NULL,
        m_unknownEncodingRelease: None,
        m_prologState: crate::src::xmlrole::PROLOG_STATE {
            handler: None,
            level: 0,
            role_none: 0,
            includeLevel: 0,
            documentEntity: 0,
            inEntityValue: 0,
        },
        m_processor: ProcessorState::PrologInit,
        m_errorCode: crate::expat_h::XML_ERROR_NONE,
        m_eventPtr: ::core::ptr::null::<::core::ffi::c_char>(),
        m_eventEndPtr: ::core::ptr::null::<::core::ffi::c_char>(),
        m_positionPtr: ::core::ptr::null::<::core::ffi::c_char>(),
        m_openInternalEntities: ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>(),
        m_freeInternalEntities: ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>(),
        m_openAttributeEntities: ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>(),
        m_freeAttributeEntities: ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>(),
        m_openValueEntities: ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>(),
        m_freeValueEntities: ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>(),
        m_defaultExpandInternalEntities: crate::expat_h::XML_TRUE,
        m_tagLevel: 0,
        m_declEntity: ::core::ptr::null_mut::<ENTITY>(),
        m_doctypeName: ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
        m_doctypeSysid: ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
        m_doctypePubid: ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
        m_declAttributeType: ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
        m_declNotationName: ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
        m_declNotationPublicId: ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
        m_declElementType: ::core::ptr::null_mut::<ELEMENT_TYPE>(),
        m_declAttributeId: ::core::ptr::null_mut::<ATTRIBUTE_ID>(),
        m_declAttributeIsCdata: crate::expat_h::XML_FALSE,
        m_declAttributeIsId: crate::expat_h::XML_FALSE,
        m_dtd: ::core::ptr::null_mut::<DTD>(),
        m_curBase: ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
        m_tagStack: ::core::ptr::null_mut::<TAG>(),
        m_freeTagList: ::core::ptr::null_mut::<TAG>(),
        m_inheritedBindings: ::core::ptr::null_mut::<BINDING>(),
        m_freeBindingList: ::core::ptr::null_mut::<BINDING>(),
        m_attsSize: 0,
        m_nSpecifiedAtts: 0,
        m_idAttIndex: 0,
        m_atts: ::core::ptr::null_mut::<crate::src::xmltok::ATTRIBUTE>(),
        m_nsAtts: ::core::ptr::null_mut::<NS_ATT>(),
        m_nsAttsVersion: 0,
        m_nsAttsPower: 0,
        m_position: crate::src::xmltok::POSITION { lineNumber: 0, columnNumber: 0 },
        m_tempPool: empty_string_pool(),
        m_temp2Pool: empty_string_pool(),
        m_groupConnector: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        m_groupSize: 0,
        m_namespaceSeparator: crate::ascii_h::ASCII_EXCL as crate::expat_external_h::XML_Char,
        m_parentParser: ::core::ptr::null_mut::<XML_ParserStruct>(),
        m_parsingStatus: crate::expat_h::XML_ParsingStatus {
            parsing: crate::expat_h::XML_INITIALIZED,
            finalBuffer: crate::expat_h::XML_FALSE,
        },
        m_isParamEntity: crate::expat_h::XML_FALSE,
        m_useForeignDTD: crate::expat_h::XML_FALSE,
        m_paramEntityParsing: crate::expat_h::XML_PARAM_ENTITY_PARSING_NEVER,
        m_hash_secret_salt: 0,
        m_accounting: ACCOUNTING {
            countBytesDirect: 0,
            countBytesIndirect: 0,
            debugLevel: 0,
            maximumAmplificationFactor: 0.0,
            activationThresholdBytes: 0,
        },
        m_alloc_tracker: MALLOC_TRACKER {
            bytesAllocated: 0,
            peakBytesAllocated: 0,
            debugLevel: 0,
            maximumAmplificationFactor: 0.0,
            activationThresholdBytes: 0,
        },
        m_entity_stats: ENTITY_STATS {
            countEverOpened: 0,
            currentDepth: 0,
            maximumDepthSeen: 0,
            debugLevel: 0,
        },
        m_reenter: crate::expat_h::XML_FALSE,
    }
}

unsafe extern "C" fn parserCreate(
    mut encodingName: *const crate::expat_external_h::XML_Char,
    mut memsuite: *const crate::expat_h::XML_Memory_Handling_Suite,
    mut nameSep: *const crate::expat_external_h::XML_Char,
    mut dtd: *mut DTD,
    mut parentParser: crate::expat_h::XML_Parser,
) -> crate::expat_h::XML_Parser {
    let increase: crate::__stddef_size_t_h::size_t =
        ::core::mem::size_of::<crate::__stddef_size_t_h::size_t>()
            .wrapping_add(crate::internal_h::EXPAT_MALLOC_PADDING)
            .wrapping_add(::core::mem::size_of::<XML_ParserStruct>());
    if !parentParser.is_null() {
        let rootParser: crate::expat_h::XML_Parser =
            getRootParserOf(parentParser, ::core::ptr::null_mut::<::core::ffi::c_uint>());
        if !expat_heap_increase_tolerable(
            rootParser,
            increase as XmlBigCount,
            1354 as ::core::ffi::c_int,
        ) {
            return ::core::ptr::null_mut::<XML_ParserStruct>();
        }
    }
    let memory_suite = if memsuite.is_null() {
        crate::expat_h::XML_Memory_Handling_Suite {
            malloc_fcn: Some(crate::stdlib::malloc),
            realloc_fcn: Some(crate::stdlib::realloc),
            free_fcn: Some(crate::stdlib::free),
        }
    } else {
        *memsuite
    };
    let parser = match allocate_parser_storage(memory_suite) {
        Some(parser) => parser,
        None => return ::core::ptr::null_mut::<XML_ParserStruct>(),
    };
    ::core::ptr::write(parser, initial_parser_struct(memory_suite));
    let parser = &mut *parser;
    parser.m_alloc_tracker = MALLOC_TRACKER {
        bytesAllocated: 0 as XmlBigCount,
        peakBytesAllocated: 0 as XmlBigCount,
        debugLevel: if parentParser.is_null() {
            getDebugLevel(
                b"EXPAT_MALLOC_DEBUG\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_ulong,
            )
        } else {
            0 as ::core::ffi::c_ulong
        },
        maximumAmplificationFactor: if parentParser.is_null() {
            crate::internal_h::EXPAT_ALLOC_TRACKER_MAXIMUM_AMPLIFICATION_DEFAULT
        } else {
            0.0
        },
        activationThresholdBytes: if parentParser.is_null() {
            crate::internal_h::EXPAT_ALLOC_TRACKER_ACTIVATION_THRESHOLD_DEFAULT as XmlBigCount
        } else {
            0 as XmlBigCount
        },
    };
    parser.m_parentParser = parentParser;
    if parentParser.is_null() {
        parser.m_accounting.countBytesDirect = 0 as XmlBigCount;
    }
    let rootParser_0: crate::expat_h::XML_Parser =
        getRootParserOf(parser, ::core::ptr::null_mut::<::core::ffi::c_uint>());
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
            rootParser_0,
            '+' as ::core::ffi::c_char,
            increase as XmlBigCount,
            (*rootParser_0).m_alloc_tracker.bytesAllocated,
            (*rootParser_0).m_alloc_tracker.peakBytesAllocated,
            1439 as ::core::ffi::c_int,
        );
    }
    parser.m_buffer = ::core::ptr::null_mut::<::core::ffi::c_char>();
    parser.m_bufferLim = ::core::ptr::null::<::core::ffi::c_char>();
    parser.m_attsSize = INIT_ATTS_SIZE;
    parser.m_atts = expat_malloc(
        parser,
        (parser.m_attsSize as crate::__stddef_size_t_h::size_t)
            .wrapping_mul(::core::mem::size_of::<crate::src::xmltok::ATTRIBUTE>()),
        1449 as ::core::ffi::c_int,
    ) as *mut crate::src::xmltok::ATTRIBUTE;
    if parser.m_atts.is_null() {
        expat_free(
            parser,
            parser as *mut XML_ParserStruct as *mut ::core::ffi::c_void,
            1451 as ::core::ffi::c_int,
        );
        return ::core::ptr::null_mut::<XML_ParserStruct>();
    }
    parser.m_dataBuf = expat_malloc(
        parser,
        (1024 as crate::__stddef_size_t_h::size_t)
            .wrapping_mul(::core::mem::size_of::<crate::expat_external_h::XML_Char>()),
        1462 as ::core::ffi::c_int,
    ) as *mut crate::expat_external_h::XML_Char;
    if parser.m_dataBuf.is_null() {
        expat_free(
            parser,
            parser.m_atts as *mut ::core::ffi::c_void,
            1464 as ::core::ffi::c_int,
        );
        expat_free(
            parser,
            parser as *mut XML_ParserStruct as *mut ::core::ffi::c_void,
            1468 as ::core::ffi::c_int,
        );
        return ::core::ptr::null_mut::<XML_ParserStruct>();
    }
    parser.m_dataBufEnd = parser.m_dataBuf.offset(INIT_DATA_BUF_SIZE as isize);
    if !dtd.is_null() {
        parser.m_dtd = dtd;
    } else {
        parser.m_dtd = dtdCreate(parser);
        if parser.m_dtd.is_null() {
            expat_free(
                parser,
                parser.m_dataBuf as *mut ::core::ffi::c_void,
                1478 as ::core::ffi::c_int,
            );
            expat_free(
                parser,
                parser.m_atts as *mut ::core::ffi::c_void,
                1479 as ::core::ffi::c_int,
            );
            expat_free(
                parser,
                parser as *mut XML_ParserStruct as *mut ::core::ffi::c_void,
                1483 as ::core::ffi::c_int,
            );
            return ::core::ptr::null_mut::<XML_ParserStruct>();
        }
    }
    parser.m_freeBindingList = ::core::ptr::null_mut::<BINDING>();
    parser.m_freeTagList = ::core::ptr::null_mut::<TAG>();
    parser.m_freeInternalEntities = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
    parser.m_freeAttributeEntities = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
    parser.m_freeValueEntities = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
    parser.m_groupSize = 0 as ::core::ffi::c_uint;
    parser.m_groupConnector = ::core::ptr::null_mut::<::core::ffi::c_char>();
    parser.m_unknownEncodingHandler = false;
    parser.m_unknownEncodingHandlerData = crate::__stddef_null_h::NULL;
    UNKNOWN_ENCODING_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&(parser as *mut XML_ParserStruct as usize));
    parser.m_namespaceSeparator =
        crate::ascii_h::ASCII_EXCL as crate::expat_external_h::XML_Char;
    parser.m_ns = crate::expat_h::XML_FALSE;
    parser.m_ns_triplets = crate::expat_h::XML_FALSE;
    parser.m_nsAtts = ::core::ptr::null_mut::<NS_ATT>();
    parser.m_nsAttsVersion = 0 as ::core::ffi::c_ulong;
    parser.m_nsAttsPower = 0 as ::core::ffi::c_uchar;
    parser.m_protocolEncodingName = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    poolInit(&raw mut parser.m_tempPool, parser);
    poolInit(&raw mut parser.m_temp2Pool, parser);
    parserInit(parser, encodingName);
    if !encodingName.is_null() && parser.m_protocolEncodingName.is_null() {
        if !dtd.is_null() {
            parser.m_dtd = ::core::ptr::null_mut::<DTD>();
        }
        XML_ParserFree(parser);
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

fn parser_init(
    parser: &mut XML_ParserStruct,
    parser_key: usize,
    reparse_deferral_enabled: crate::expat_h::XML_Bool,
    accounting_debug_level: ::core::ffi::c_ulong,
    entity_debug_level: ::core::ffi::c_ulong,
) {
    parser.m_processor = ProcessorState::PrologInit;
    crate::src::xmlrole::prolog_state_init(&mut parser.m_prologState);
    parser.m_curBase = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    parser.m_initEncoding.initEnc.isUtf16 = crate::src::xmltok::NO_ENC as ::core::ffi::c_char;
    parser.m_initEncoding.initEnc.scanners[crate::src::xmltok::XML_PROLOG_STATE as usize] =
        crate::src::xmltok::Scanner::InitProlog;
    parser.m_initEncoding.initEnc.scanners[crate::src::xmltok::XML_CONTENT_STATE as usize] =
        crate::src::xmltok::Scanner::InitContent;
    parser.m_initEncoding.initEnc.updatePosition = crate::src::xmltok::PositionUpdater::Init;
    parser.m_initEncoding.selected_encoding = None;
    parser.m_encoding = EncodingState::Initial;
    parser.m_userData = crate::__stddef_null_h::NULL;
    parser.m_handlerArg = crate::__stddef_null_h::NULL;
    parser.m_startElementHandler = false;
    START_ELEMENT_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    parser.m_endElementHandler = false;
    END_ELEMENT_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    parser.m_characterDataHandler = false;
    CHARACTER_DATA_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    parser.m_processingInstructionHandler = false;
    PROCESSING_INSTRUCTION_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    parser.m_commentHandler = false;
    COMMENT_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    parser.m_startCdataSectionHandler = None;
    parser.m_endCdataSectionHandler = None;
    parser.m_defaultHandler = false;
    DEFAULT_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    parser.m_startDoctypeDeclHandler = false;
    START_DOCTYPE_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    parser.m_endDoctypeDeclHandler = None;
    parser.m_unparsedEntityDeclHandler = false;
    UNPARSED_ENTITY_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    parser.m_notationDeclHandler = false;
    NOTATION_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    parser.m_startNamespaceDeclHandler = false;
    START_NAMESPACE_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    parser.m_endNamespaceDeclHandler = false;
    END_NAMESPACE_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    parser.m_notStandaloneHandler = None;
    parser.m_externalEntityRefHandler = false;
    EXTERNAL_ENTITY_REF_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    parser.m_skippedEntityHandler = false;
    SKIPPED_ENTITY_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    parser.m_elementDeclHandler = false;
    ELEMENT_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    parser.m_attlistDeclHandler = false;
    ATTLIST_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    parser.m_entityDeclHandler = false;
    ENTITY_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    parser.m_xmlDeclHandler = false;
    XML_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    parser.m_bufferPtr = parser.m_buffer;
    parser.m_bufferEnd = parser.m_buffer;
    parser.m_parseEndByteIndex = 0 as crate::expat_external_h::XML_Index;
    parser.m_parseEndPtr = ::core::ptr::null::<::core::ffi::c_char>();
    parser.m_partialTokenBytesBefore = 0 as crate::__stddef_size_t_h::size_t;
    parser.m_reparseDeferralEnabled = reparse_deferral_enabled;
    parser.m_lastBufferRequestSize = 0 as ::core::ffi::c_int;
    parser.m_declElementType = ::core::ptr::null_mut::<ELEMENT_TYPE>();
    parser.m_declAttributeId = ::core::ptr::null_mut::<ATTRIBUTE_ID>();
    parser.m_declEntity = ::core::ptr::null_mut::<ENTITY>();
    parser.m_doctypeName = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    parser.m_doctypeSysid = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    parser.m_doctypePubid = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    parser.m_declAttributeType = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    parser.m_declNotationName = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    parser.m_declNotationPublicId = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    parser.m_declAttributeIsCdata = crate::expat_h::XML_FALSE;
    parser.m_declAttributeIsId = crate::expat_h::XML_FALSE;
    parser.m_position = crate::src::xmltok::POSITION {
        lineNumber: 0,
        columnNumber: 0,
    };
    parser.m_errorCode = crate::expat_h::XML_ERROR_NONE;
    parser.m_eventPtr = ::core::ptr::null::<::core::ffi::c_char>();
    parser.m_eventEndPtr = ::core::ptr::null::<::core::ffi::c_char>();
    parser.m_positionPtr = ::core::ptr::null::<::core::ffi::c_char>();
    parser.m_openInternalEntities = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
    parser.m_openAttributeEntities = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
    parser.m_openValueEntities = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
    parser.m_defaultExpandInternalEntities = crate::expat_h::XML_TRUE;
    parser.m_tagLevel = 0 as ::core::ffi::c_int;
    parser.m_tagStack = ::core::ptr::null_mut::<TAG>();
    parser.m_inheritedBindings = ::core::ptr::null_mut::<BINDING>();
    parser.m_nSpecifiedAtts = 0 as ::core::ffi::c_int;
    parser.m_unknownEncodingMem = crate::__stddef_null_h::NULL;
    parser.m_unknownEncodingRelease = None;
    parser.m_unknownEncodingData = crate::__stddef_null_h::NULL;
    parser.m_parsingStatus.parsing = crate::expat_h::XML_INITIALIZED;
    parser.m_reenter = crate::expat_h::XML_FALSE;
    parser.m_isParamEntity = crate::expat_h::XML_FALSE;
    parser.m_useForeignDTD = crate::expat_h::XML_FALSE;
    parser.m_paramEntityParsing = crate::expat_h::XML_PARAM_ENTITY_PARSING_NEVER;
    parser.m_hash_secret_salt = 0 as ::core::ffi::c_ulong;
    parser.m_accounting = ACCOUNTING {
        countBytesDirect: 0,
        countBytesIndirect: 0,
        debugLevel: 0,
        maximumAmplificationFactor: 0.0,
        activationThresholdBytes: 0,
    };
    parser.m_accounting.debugLevel = accounting_debug_level;
    parser.m_accounting.maximumAmplificationFactor =
        crate::internal_h::EXPAT_BILLION_LAUGHS_ATTACK_PROTECTION_MAXIMUM_AMPLIFICATION_DEFAULT;
    parser.m_accounting.activationThresholdBytes =
        crate::internal_h::EXPAT_BILLION_LAUGHS_ATTACK_PROTECTION_ACTIVATION_THRESHOLD_DEFAULT
            as ::core::ffi::c_ulonglong;
    parser.m_entity_stats = ENTITY_STATS {
        countEverOpened: 0,
        currentDepth: 0,
        maximumDepthSeen: 0,
        debugLevel: 0,
    };
    parser.m_entity_stats.debugLevel = entity_debug_level;
}

unsafe extern "C" fn parserInit(
    mut parser: crate::expat_h::XML_Parser,
    mut encodingName: *const crate::expat_external_h::XML_Char,
) {
    let protocol_encoding_name = if encodingName.is_null() {
        ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>()
    } else {
        copyString(encodingName, parser)
    };
    let accounting_debug_level = getDebugLevel(
        b"EXPAT_ACCOUNTING_DEBUG\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_ulong,
    );
    let entity_debug_level = getDebugLevel(
        b"EXPAT_ENTITY_DEBUG\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_ulong,
    );
    let reparse_deferral_enabled = g_reparseDeferralEnabledDefault;
    let parser_key = parser as usize;
    let parser_state = &mut *parser;
    parser_state.m_protocolEncodingName = protocol_encoding_name;
    parser_state.m_externalEntityRefHandlerArg = parser;
    parser_init(
        parser_state,
        parser_key,
        reparse_deferral_enabled,
        accounting_debug_level,
        entity_debug_level,
    );
}

unsafe extern "C" fn moveToFreeBindingList(
    parser: &mut XML_ParserStruct,
    mut bindings: *mut BINDING,
) {
    while !bindings.is_null() {
        let mut b: *mut BINDING = bindings;
        bindings = (*bindings).nextTagBinding as *mut BINDING;
        (*b).nextTagBinding = parser.m_freeBindingList as *mut binding;
        parser.m_freeBindingList = b;
    }
}
pub unsafe extern "C" fn XML_ParserReset(
    mut parser: crate::expat_h::XML_Parser,
    mut encodingName: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Bool {
    if parser.is_null() {
        return crate::expat_h::XML_FALSE;
    }
    // This is the one short-lived exclusive borrow of the parser.  It ends before
    // invoking allocator or release callbacks below, which may inspect the parser.
    let (unknown_encoding_mem, unknown_encoding_release, unknown_encoding_data, protocol_encoding_name, dtd) = {
        let parser_state = &mut *parser;
        if !parser_state.m_parentParser.is_null() {
            return crate::expat_h::XML_FALSE;
        }
        let mut tag_stack = parser_state.m_tagStack;
        while !tag_stack.is_null() {
            let tag = tag_stack;
            tag_stack = (*tag).parent as *mut TAG;
            (*tag).parent = parser_state.m_freeTagList as *mut tag;
            moveToFreeBindingList(parser_state, (*tag).bindings);
            (*tag).bindings = ::core::ptr::null_mut::<BINDING>();
            parser_state.m_freeTagList = tag;
        }
        let mut open_entity_list = parser_state.m_openInternalEntities;
        while !open_entity_list.is_null() {
            let open_entity = open_entity_list;
            open_entity_list = (*open_entity).next as *mut OPEN_INTERNAL_ENTITY;
            (*open_entity).next = parser_state.m_freeInternalEntities as *mut open_internal_entity;
            parser_state.m_freeInternalEntities = open_entity;
        }
        open_entity_list = parser_state.m_openAttributeEntities;
        while !open_entity_list.is_null() {
            let open_entity = open_entity_list;
            open_entity_list = (*open_entity).next as *mut OPEN_INTERNAL_ENTITY;
            (*open_entity).next = parser_state.m_freeAttributeEntities as *mut open_internal_entity;
            parser_state.m_freeAttributeEntities = open_entity;
        }
        open_entity_list = parser_state.m_openValueEntities;
        while !open_entity_list.is_null() {
            let open_entity = open_entity_list;
            open_entity_list = (*open_entity).next as *mut OPEN_INTERNAL_ENTITY;
            (*open_entity).next = parser_state.m_freeValueEntities as *mut open_internal_entity;
            parser_state.m_freeValueEntities = open_entity;
        }
        moveToFreeBindingList(parser_state, parser_state.m_inheritedBindings);
        let unknown_encoding_mem = parser_state.m_unknownEncodingMem;
        let unknown_encoding_release = parser_state.m_unknownEncodingRelease;
        let unknown_encoding_data = parser_state.m_unknownEncodingData;
        let protocol_encoding_name = parser_state.m_protocolEncodingName;
        parser_state.m_protocolEncodingName = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
        (
            unknown_encoding_mem,
            unknown_encoding_release,
            unknown_encoding_data,
            protocol_encoding_name,
            parser_state.m_dtd,
        )
    };
    crate::src::xmltok::unregister_unknown_encoding_converter(unknown_encoding_mem as usize);
    expat_free(
        parser,
        unknown_encoding_mem,
        1686 as ::core::ffi::c_int,
    );
    if let Some(release) = unknown_encoding_release {
        release(unknown_encoding_data);
    }
    poolClear(&raw mut (*parser).m_tempPool);
    poolClear(&raw mut (*parser).m_temp2Pool);
    expat_free(
        parser,
        protocol_encoding_name as *mut ::core::ffi::c_void,
        1691 as ::core::ffi::c_int,
    );
    parserInit(parser, encodingName);
    dtdReset(dtd, parser);
    return crate::expat_h::XML_TRUE;
}
#[export_name = "XML_ParserReset"]

pub unsafe extern "C" fn XML_ParserReset_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut encodingName: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Bool {
    XML_ParserReset(parser, encodingName)
}
unsafe extern "C" fn parserBusy(
    mut parser: crate::expat_h::XML_Parser,
) -> crate::expat_h::XML_Bool {
    match (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint {
        1 | 3 => return crate::expat_h::XML_TRUE,
        0 | 2 | _ => return crate::expat_h::XML_FALSE,
    };
}
pub unsafe extern "C" fn XML_SetEncoding(
    mut parser: crate::expat_h::XML_Parser,
    mut encodingName: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Status {
    if parser.is_null() {
        return crate::expat_h::XML_STATUS_ERROR;
    }
    if parserBusy(parser) != 0 {
        return crate::expat_h::XML_STATUS_ERROR;
    }
    expat_free(
        parser,
        (*parser).m_protocolEncodingName as *mut ::core::ffi::c_void,
        1723 as ::core::ffi::c_int,
    );
    if encodingName.is_null() {
        (*parser).m_protocolEncodingName = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    } else {
        (*parser).m_protocolEncodingName = copyString(encodingName, parser);
        if (*parser).m_protocolEncodingName.is_null() {
            return crate::expat_h::XML_STATUS_ERROR;
        }
    }
    return crate::expat_h::XML_STATUS_OK;
}
#[export_name = "XML_SetEncoding"]

pub unsafe extern "C" fn XML_SetEncoding_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut encodingName: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Status {
    XML_SetEncoding(parser, encodingName)
}
pub unsafe extern "C" fn XML_ExternalEntityParserCreate(
    mut oldParser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut encodingName: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Parser {
    let mut parser: crate::expat_h::XML_Parser = oldParser;
    let mut newDtd: *mut DTD = ::core::ptr::null_mut::<DTD>();
    let mut oldDtd: *mut DTD = ::core::ptr::null_mut::<DTD>();
    let mut oldStartElementHandler = false;
    let mut oldStartElementCallback: Option<std::sync::Arc<dyn StartElementCallback>> = None;
    let mut oldEndElementCallback: Option<std::sync::Arc<dyn EndElementCallback>> = None;
    let mut oldCharacterDataHandler = false;
    let mut oldCharacterDataCallback: Option<std::sync::Arc<dyn CharacterDataCallback>> = None;
    let mut oldProcessingInstructionHandler = false;
    let mut oldProcessingInstructionCallback: Option<
        std::sync::Arc<dyn ProcessingInstructionCallback>,
    > = None;
    let mut oldCommentHandler = false;
    let mut oldCommentCallback: Option<std::sync::Arc<dyn CommentCallback>> = None;
    let mut oldStartCdataSectionHandler: crate::expat_h::XML_StartCdataSectionHandler = None;
    let mut oldEndCdataSectionHandler: crate::expat_h::XML_EndCdataSectionHandler = None;
    let mut oldDefaultHandler = false;
    let mut oldDefaultCallback: Option<std::sync::Arc<dyn DefaultCallback>> = None;
    let mut oldUnparsedEntityDeclHandler = false;
    let mut oldUnparsedEntityDeclCallback: Option<
        std::sync::Arc<dyn UnparsedEntityDeclCallback>,
    > = None;
    let mut oldNotationDeclHandler = false;
    let mut oldNotationDeclCallback: Option<std::sync::Arc<dyn NotationDeclCallback>> = None;
    let mut oldStartNamespaceDeclHandler = false;
    let mut oldStartNamespaceDeclCallback: Option<
        std::sync::Arc<TwoXmlCharCallback>,
    > = None;
    let mut oldEndNamespaceDeclHandler = false;
    let mut oldEndNamespaceDeclCallback: Option<std::sync::Arc<dyn EndNamespaceDeclCallback>> =
        None;
    let mut oldNotStandaloneHandler: crate::expat_h::XML_NotStandaloneHandler = None;
    let mut oldExternalEntityRefHandler: Option<std::sync::Arc<dyn ExternalEntityRefCallback>> =
        None;
    let mut oldSkippedEntityCallback: Option<std::sync::Arc<dyn SkippedEntityCallback>> = None;
    let mut oldUnknownEncodingHandler: Option<std::sync::Arc<dyn UnknownEncodingCallback>> = None;
    let mut oldUnknownEncodingHandlerData: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut oldElementDeclHandler = false;
    let mut oldElementDeclCallback: Option<std::sync::Arc<dyn ElementDeclCallback>> = None;
    let mut oldAttlistDeclHandler = false;
    let mut oldAttlistDeclCallback: Option<std::sync::Arc<dyn AttlistDeclCallback>> = None;
    let mut oldEntityDeclHandler: Option<std::sync::Arc<dyn EntityDeclCallback>> = None;
    let mut oldXmlDeclHandler: Option<std::sync::Arc<dyn XmlDeclCallback>> = None;
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
    oldStartElementCallback = START_ELEMENT_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&(parser as usize))
        .cloned();
    oldEndElementCallback = END_ELEMENT_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&(parser as usize))
        .cloned();
    oldCharacterDataHandler = (*parser).m_characterDataHandler;
    oldCharacterDataCallback = CHARACTER_DATA_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&(parser as usize))
        .cloned();
    oldProcessingInstructionHandler = (*parser).m_processingInstructionHandler;
    oldProcessingInstructionCallback = PROCESSING_INSTRUCTION_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&(parser as usize))
        .cloned();
    oldCommentHandler = (*parser).m_commentHandler;
    oldCommentCallback = COMMENT_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&(parser as usize))
        .cloned();
    oldStartCdataSectionHandler = (*parser).m_startCdataSectionHandler;
    oldEndCdataSectionHandler = (*parser).m_endCdataSectionHandler;
    oldDefaultHandler = (*parser).m_defaultHandler;
    oldDefaultCallback = DEFAULT_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&(parser as usize))
        .cloned();
    oldUnparsedEntityDeclHandler = (*parser).m_unparsedEntityDeclHandler;
    oldUnparsedEntityDeclCallback = UNPARSED_ENTITY_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&(parser as usize))
        .cloned();
    oldNotationDeclHandler = (*parser).m_notationDeclHandler;
    oldNotationDeclCallback = NOTATION_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&(parser as usize))
        .cloned();
    oldStartNamespaceDeclHandler = (*parser).m_startNamespaceDeclHandler;
    oldStartNamespaceDeclCallback = START_NAMESPACE_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&(parser as usize))
        .cloned();
    oldEndNamespaceDeclHandler = (*parser).m_endNamespaceDeclHandler;
    oldEndNamespaceDeclCallback = END_NAMESPACE_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&(parser as usize))
        .cloned();
    oldNotStandaloneHandler = (*parser).m_notStandaloneHandler;
    oldExternalEntityRefHandler = EXTERNAL_ENTITY_REF_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&(parser as usize))
        .cloned();
    oldSkippedEntityCallback = SKIPPED_ENTITY_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&(parser as usize))
        .cloned();
    oldUnknownEncodingHandler = UNKNOWN_ENCODING_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&(parser as usize))
        .cloned();
    oldUnknownEncodingHandlerData = (*parser).m_unknownEncodingHandlerData;
    oldElementDeclHandler = (*parser).m_elementDeclHandler;
    oldElementDeclCallback = ELEMENT_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&(parser as usize))
        .cloned();
    oldAttlistDeclHandler = (*parser).m_attlistDeclHandler;
    oldAttlistDeclCallback = ATTLIST_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&(parser as usize))
        .cloned();
    oldEntityDeclHandler = ENTITY_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&(parser as usize))
        .cloned();
    oldXmlDeclHandler = XML_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&(parser as usize))
        .cloned();
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
            0 as crate::expat_external_h::XML_Char,
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
    if let Some(callback) = oldStartElementCallback {
        START_ELEMENT_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .insert(parser as usize, callback);
    }
    (*parser).m_endElementHandler = oldEndElementCallback.is_some();
    if let Some(callback) = oldEndElementCallback {
        END_ELEMENT_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .insert(parser as usize, callback);
    }
    (*parser).m_characterDataHandler = oldCharacterDataHandler;
    if let Some(callback) = oldCharacterDataCallback {
        CHARACTER_DATA_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .insert(parser as usize, callback);
    }
    (*parser).m_processingInstructionHandler = oldProcessingInstructionHandler;
    if let Some(callback) = oldProcessingInstructionCallback {
        PROCESSING_INSTRUCTION_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .insert(parser as usize, callback);
    }
    (*parser).m_commentHandler = oldCommentHandler;
    if let Some(callback) = oldCommentCallback {
        COMMENT_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .insert(parser as usize, callback);
    }
    (*parser).m_startCdataSectionHandler = oldStartCdataSectionHandler;
    (*parser).m_endCdataSectionHandler = oldEndCdataSectionHandler;
    (*parser).m_defaultHandler = oldDefaultHandler;
    if let Some(callback) = oldDefaultCallback {
        DEFAULT_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .insert(parser as usize, callback);
    }
    (*parser).m_unparsedEntityDeclHandler = oldUnparsedEntityDeclHandler;
    if let Some(callback) = oldUnparsedEntityDeclCallback {
        UNPARSED_ENTITY_DECL_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .insert(parser as usize, callback);
    }
    (*parser).m_notationDeclHandler = oldNotationDeclHandler;
    if let Some(callback) = oldNotationDeclCallback {
        NOTATION_DECL_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .insert(parser as usize, callback);
    }
    (*parser).m_startNamespaceDeclHandler = oldStartNamespaceDeclHandler;
    if let Some(callback) = oldStartNamespaceDeclCallback {
        START_NAMESPACE_DECL_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .insert(parser as usize, callback);
    }
    (*parser).m_endNamespaceDeclHandler = oldEndNamespaceDeclHandler;
    if let Some(callback) = oldEndNamespaceDeclCallback {
        END_NAMESPACE_DECL_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .insert(parser as usize, callback);
    }
    (*parser).m_notStandaloneHandler = oldNotStandaloneHandler;
    (*parser).m_externalEntityRefHandler = oldExternalEntityRefHandler.is_some();
    if let Some(callback) = oldExternalEntityRefHandler {
        EXTERNAL_ENTITY_REF_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .insert(parser as usize, callback);
    }
    (*parser).m_skippedEntityHandler = oldSkippedEntityCallback.is_some();
    if let Some(callback) = oldSkippedEntityCallback {
        SKIPPED_ENTITY_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .insert(parser as usize, callback);
    }
    (*parser).m_unknownEncodingHandler = oldUnknownEncodingHandler.is_some();
    if let Some(callback) = oldUnknownEncodingHandler {
        UNKNOWN_ENCODING_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .insert(parser as usize, callback);
    }
    (*parser).m_unknownEncodingHandlerData = oldUnknownEncodingHandlerData;
    (*parser).m_elementDeclHandler = oldElementDeclHandler;
    if let Some(callback) = oldElementDeclCallback {
        ELEMENT_DECL_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .insert(parser as usize, callback);
    }
    (*parser).m_attlistDeclHandler = oldAttlistDeclHandler;
    if let Some(callback) = oldAttlistDeclCallback {
        ATTLIST_DECL_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .insert(parser as usize, callback);
    }
    (*parser).m_entityDeclHandler = oldEntityDeclHandler.is_some();
    if let Some(callback) = oldEntityDeclHandler {
        ENTITY_DECL_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .insert(parser as usize, callback);
    }
    (*parser).m_xmlDeclHandler = oldXmlDeclHandler.is_some();
    if let Some(callback) = oldXmlDeclHandler {
        XML_DECL_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .insert(parser as usize, callback);
    }
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
            XML_ParserFree(parser);
            return ::core::ptr::null_mut::<XML_ParserStruct>();
        }
        (*parser).m_processor = ProcessorState::ExternalEntityInit;
    } else {
        (*parser).m_isParamEntity = crate::expat_h::XML_TRUE;
        crate::src::xmlrole::prolog_state_init_external_entity(&mut (*parser).m_prologState);
        (*parser).m_processor = ProcessorState::ExternalParEntInit;
    }
    return parser;
}
#[export_name = "XML_ExternalEntityParserCreate"]

pub unsafe extern "C" fn XML_ExternalEntityParserCreate_ffi(
    mut oldParser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut encodingName: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Parser {
    XML_ExternalEntityParserCreate(oldParser, context, encodingName)
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
pub unsafe extern "C" fn XML_ParserFree(mut parser: crate::expat_h::XML_Parser) {
    let mut tagList: *mut TAG = ::core::ptr::null_mut::<TAG>();
    let mut entityList: *mut OPEN_INTERNAL_ENTITY = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
    if parser.is_null() {
        return;
    }
    let parser_key = parser as usize;
    let parser = &mut *parser;
    START_ELEMENT_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    END_ELEMENT_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    START_NAMESPACE_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    END_NAMESPACE_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    CHARACTER_DATA_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    PROCESSING_INSTRUCTION_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    COMMENT_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    DEFAULT_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    START_DOCTYPE_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    ELEMENT_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    ATTLIST_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    ENTITY_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    UNPARSED_ENTITY_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    NOTATION_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    XML_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    UNKNOWN_ENCODING_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    EXTERNAL_ENTITY_REF_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    SKIPPED_ENTITY_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&parser_key);
    tagList = parser.m_tagStack;
    loop {
        let mut p: *mut TAG = ::core::ptr::null_mut::<TAG>();
        if tagList.is_null() {
            if parser.m_freeTagList.is_null() {
                break;
            }
            tagList = parser.m_freeTagList;
            parser.m_freeTagList = ::core::ptr::null_mut::<TAG>();
        }
        p = tagList;
        tagList = (*tagList).parent as *mut TAG;
        expat_free(
            parser as *mut XML_ParserStruct,
            (*p).bufEnd as *mut ::core::ffi::c_void,
            1942 as ::core::ffi::c_int,
        );
        destroyBindings((*p).bindings, parser as *mut XML_ParserStruct);
        expat_free(
            parser as *mut XML_ParserStruct,
            p as *mut ::core::ffi::c_void,
            1944 as ::core::ffi::c_int,
        );
    }
    entityList = parser.m_openInternalEntities;
    loop {
        let mut openEntity: *mut OPEN_INTERNAL_ENTITY =
            ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
        if entityList.is_null() {
            if parser.m_freeInternalEntities.is_null() {
                break;
            }
            entityList = parser.m_freeInternalEntities;
            parser.m_freeInternalEntities = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
        }
        openEntity = entityList;
        entityList = (*entityList).next as *mut OPEN_INTERNAL_ENTITY;
        expat_free(
            parser as *mut XML_ParserStruct,
            openEntity as *mut ::core::ffi::c_void,
            1958 as ::core::ffi::c_int,
        );
    }
    entityList = parser.m_openAttributeEntities;
    loop {
        let mut openEntity_0: *mut OPEN_INTERNAL_ENTITY =
            ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
        if entityList.is_null() {
            if parser.m_freeAttributeEntities.is_null() {
                break;
            }
            entityList = parser.m_freeAttributeEntities;
            parser.m_freeAttributeEntities = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
        }
        openEntity_0 = entityList;
        entityList = (*entityList).next as *mut OPEN_INTERNAL_ENTITY;
        expat_free(
            parser as *mut XML_ParserStruct,
            openEntity_0 as *mut ::core::ffi::c_void,
            1972 as ::core::ffi::c_int,
        );
    }
    entityList = parser.m_openValueEntities;
    loop {
        let mut openEntity_1: *mut OPEN_INTERNAL_ENTITY =
            ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
        if entityList.is_null() {
            if parser.m_freeValueEntities.is_null() {
                break;
            }
            entityList = parser.m_freeValueEntities;
            parser.m_freeValueEntities = ::core::ptr::null_mut::<OPEN_INTERNAL_ENTITY>();
        }
        openEntity_1 = entityList;
        entityList = (*entityList).next as *mut OPEN_INTERNAL_ENTITY;
        expat_free(
            parser as *mut XML_ParserStruct,
            openEntity_1 as *mut ::core::ffi::c_void,
            1986 as ::core::ffi::c_int,
        );
    }
    destroyBindings(parser.m_freeBindingList, parser as *mut XML_ParserStruct);
    destroyBindings(parser.m_inheritedBindings, parser as *mut XML_ParserStruct);
    poolDestroy(&raw mut parser.m_tempPool);
    poolDestroy(&raw mut parser.m_temp2Pool);
    expat_free(
        parser as *mut XML_ParserStruct,
        parser.m_protocolEncodingName as *mut ::core::ffi::c_void,
        1992 as ::core::ffi::c_int,
    );
    if parser.m_isParamEntity == 0 && !parser.m_dtd.is_null() {
        dtdDestroy(
            parser.m_dtd,
            parser.m_parentParser.is_null() as ::core::ffi::c_int as crate::expat_h::XML_Bool,
            parser as *mut XML_ParserStruct,
        );
    }
    expat_free(
        parser as *mut XML_ParserStruct,
        parser.m_atts as *mut ::core::ffi::c_void,
        2002 as ::core::ffi::c_int,
    );
    expat_free(
        parser as *mut XML_ParserStruct,
        parser.m_groupConnector as *mut ::core::ffi::c_void,
        2006 as ::core::ffi::c_int,
    );
    parser.m_mem.free_fcn.expect("non-null function pointer")(
        parser.m_buffer as *mut ::core::ffi::c_void,
    );
    expat_free(
        parser as *mut XML_ParserStruct,
        parser.m_dataBuf as *mut ::core::ffi::c_void,
        2011 as ::core::ffi::c_int,
    );
    expat_free(
        parser as *mut XML_ParserStruct,
        parser.m_nsAtts as *mut ::core::ffi::c_void,
        2012 as ::core::ffi::c_int,
    );
    crate::src::xmltok::unregister_unknown_encoding_converter(parser.m_unknownEncodingMem as usize);
    expat_free(
        parser as *mut XML_ParserStruct,
        parser.m_unknownEncodingMem,
        2013 as ::core::ffi::c_int,
    );
    if parser.m_unknownEncodingRelease.is_some() {
        parser
            .m_unknownEncodingRelease
            .expect("non-null function pointer")(parser.m_unknownEncodingData);
    }
    expat_free(
        parser as *mut XML_ParserStruct,
        parser as *mut XML_ParserStruct as *mut ::core::ffi::c_void,
        2016 as ::core::ffi::c_int,
    );
}
#[export_name = "XML_ParserFree"]

pub unsafe extern "C" fn XML_ParserFree_ffi(mut parser: crate::expat_h::XML_Parser) {
    XML_ParserFree(parser)
}
pub unsafe extern "C" fn XML_UseParserAsHandlerArg(mut parser: crate::expat_h::XML_Parser) {
    if !parser.is_null() {
        (*parser).m_handlerArg = parser as *mut ::core::ffi::c_void;
    }
}
#[export_name = "XML_UseParserAsHandlerArg"]

pub unsafe extern "C" fn XML_UseParserAsHandlerArg_ffi(mut parser: crate::expat_h::XML_Parser) {
    XML_UseParserAsHandlerArg(parser)
}
pub unsafe extern "C" fn XML_UseForeignDTD(
    mut parser: crate::expat_h::XML_Parser,
    mut useDTD: crate::expat_h::XML_Bool,
) -> crate::expat_h::XML_Error {
    if parser.is_null() {
        return crate::expat_h::XML_ERROR_INVALID_ARGUMENT;
    }
    if parserBusy(parser) != 0 {
        return crate::expat_h::XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING;
    }
    (*parser).m_useForeignDTD = useDTD;
    return crate::expat_h::XML_ERROR_NONE;
}
#[export_name = "XML_UseForeignDTD"]

pub unsafe extern "C" fn XML_UseForeignDTD_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut useDTD: crate::expat_h::XML_Bool,
) -> crate::expat_h::XML_Error {
    XML_UseForeignDTD(parser, useDTD)
}
pub unsafe extern "C" fn XML_SetReturnNSTriplet(
    mut parser: crate::expat_h::XML_Parser,
    mut do_nst: ::core::ffi::c_int,
) {
    if parser.is_null() {
        return;
    }
    if parserBusy(parser) != 0 {
        return;
    }
    (*parser).m_ns_triplets = (if do_nst != 0 {
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
    XML_SetReturnNSTriplet(parser, do_nst)
}
pub unsafe extern "C" fn XML_SetUserData(
    mut parser: crate::expat_h::XML_Parser,
    mut p: *mut ::core::ffi::c_void,
) {
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
#[export_name = "XML_SetUserData"]

pub unsafe extern "C" fn XML_SetUserData_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut p: *mut ::core::ffi::c_void,
) {
    XML_SetUserData(parser, p)
}
pub unsafe extern "C" fn XML_SetBase(
    mut parser: crate::expat_h::XML_Parser,
    mut p: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Status {
    if parser.is_null() {
        return crate::expat_h::XML_STATUS_ERROR;
    }
    if !p.is_null() {
        p = poolCopyString(&raw mut (*(*parser).m_dtd).pool, p);
        if p.is_null() {
            return crate::expat_h::XML_STATUS_ERROR;
        }
        (*parser).m_curBase = p;
    } else {
        (*parser).m_curBase = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    }
    return crate::expat_h::XML_STATUS_OK;
}
#[export_name = "XML_SetBase"]

pub unsafe extern "C" fn XML_SetBase_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut p: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Status {
    XML_SetBase(parser, p)
}
pub unsafe extern "C" fn XML_GetBase(
    mut parser: crate::expat_h::XML_Parser,
) -> *const crate::expat_external_h::XML_Char {
    if parser.is_null() {
        return ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    }
    return (*parser).m_curBase;
}
#[export_name = "XML_GetBase"]

pub unsafe extern "C" fn XML_GetBase_ffi(
    mut parser: crate::expat_h::XML_Parser,
) -> *const crate::expat_external_h::XML_Char {
    XML_GetBase(parser)
}
pub unsafe extern "C" fn XML_GetSpecifiedAttributeCount(
    mut parser: crate::expat_h::XML_Parser,
) -> ::core::ffi::c_int {
    if parser.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    return (*parser).m_nSpecifiedAtts;
}
#[export_name = "XML_GetSpecifiedAttributeCount"]

pub unsafe extern "C" fn XML_GetSpecifiedAttributeCount_ffi(
    mut parser: crate::expat_h::XML_Parser,
) -> ::core::ffi::c_int {
    XML_GetSpecifiedAttributeCount(parser)
}
pub unsafe extern "C" fn XML_GetIdAttributeIndex(
    mut parser: crate::expat_h::XML_Parser,
) -> ::core::ffi::c_int {
    if parser.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    return (*parser).m_idAttIndex;
}
#[export_name = "XML_GetIdAttributeIndex"]

pub unsafe extern "C" fn XML_GetIdAttributeIndex_ffi(
    mut parser: crate::expat_h::XML_Parser,
) -> ::core::ffi::c_int {
    XML_GetIdAttributeIndex(parser)
}
pub unsafe extern "C" fn XML_SetElementHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartElementHandler,
    mut end: crate::expat_h::XML_EndElementHandler,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_startElementHandler = start.is_some();
    let mut handlers = START_ELEMENT_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    match start {
        Some(callback) => {
            handlers.insert(parser as usize, std::sync::Arc::new(callback));
        }
        None => {
            handlers.remove(&(parser as usize));
        }
    }
    (*parser).m_endElementHandler = end.is_some();
    let mut handlers = END_ELEMENT_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    match end {
        Some(callback) => {
            handlers.insert(parser as usize, std::sync::Arc::new(callback));
        }
        None => {
            handlers.remove(&(parser as usize));
        }
    }
}
#[export_name = "XML_SetElementHandler"]

pub unsafe extern "C" fn XML_SetElementHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartElementHandler,
    mut end: crate::expat_h::XML_EndElementHandler,
) {
    XML_SetElementHandler(parser, start, end)
}
pub unsafe extern "C" fn XML_SetStartElementHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartElementHandler,
) {
    if !parser.is_null() {
        (*parser).m_startElementHandler = start.is_some();
        let mut handlers = START_ELEMENT_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        match start {
            Some(callback) => {
                handlers.insert(parser as usize, std::sync::Arc::new(callback));
            }
            None => {
                handlers.remove(&(parser as usize));
            }
        }
    }
}
#[export_name = "XML_SetStartElementHandler"]

pub unsafe extern "C" fn XML_SetStartElementHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartElementHandler,
) {
    XML_SetStartElementHandler(parser, start)
}
pub unsafe extern "C" fn XML_SetEndElementHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut end: crate::expat_h::XML_EndElementHandler,
) {
    if !parser.is_null() {
        (*parser).m_endElementHandler = end.is_some();
        let mut handlers = END_ELEMENT_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        match end {
            Some(callback) => {
                handlers.insert(parser as usize, std::sync::Arc::new(callback));
            }
            None => {
                handlers.remove(&(parser as usize));
            }
        }
    }
}
#[export_name = "XML_SetEndElementHandler"]

pub unsafe extern "C" fn XML_SetEndElementHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut end: crate::expat_h::XML_EndElementHandler,
) {
    XML_SetEndElementHandler(parser, end)
}
pub unsafe extern "C" fn XML_SetCharacterDataHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_CharacterDataHandler,
) {
    if !parser.is_null() {
        (*parser).m_characterDataHandler = handler.is_some();
        let mut handlers = CHARACTER_DATA_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        match handler {
            Some(callback) => {
                handlers.insert(parser as usize, std::sync::Arc::new(callback));
            }
            None => {
                handlers.remove(&(parser as usize));
            }
        }
    }
}
#[export_name = "XML_SetCharacterDataHandler"]

pub unsafe extern "C" fn XML_SetCharacterDataHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_CharacterDataHandler,
) {
    XML_SetCharacterDataHandler(parser, handler)
}
pub unsafe extern "C" fn XML_SetProcessingInstructionHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_ProcessingInstructionHandler,
) {
    if !parser.is_null() {
        (*parser).m_processingInstructionHandler = handler.is_some();
        let mut handlers = PROCESSING_INSTRUCTION_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        match handler {
            Some(callback) => {
                handlers.insert(parser as usize, std::sync::Arc::new(callback));
            }
            None => {
                handlers.remove(&(parser as usize));
            }
        }
    }
}
#[export_name = "XML_SetProcessingInstructionHandler"]

pub unsafe extern "C" fn XML_SetProcessingInstructionHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_ProcessingInstructionHandler,
) {
    XML_SetProcessingInstructionHandler(parser, handler)
}
pub unsafe extern "C" fn XML_SetCommentHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_CommentHandler,
) {
    if !parser.is_null() {
        (*parser).m_commentHandler = handler.is_some();
        let mut handlers = COMMENT_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        match handler {
            Some(callback) => {
                handlers.insert(parser as usize, std::sync::Arc::new(callback));
            }
            None => {
                handlers.remove(&(parser as usize));
            }
        }
    }
}
#[export_name = "XML_SetCommentHandler"]

pub unsafe extern "C" fn XML_SetCommentHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_CommentHandler,
) {
    XML_SetCommentHandler(parser, handler)
}
pub unsafe extern "C" fn XML_SetCdataSectionHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartCdataSectionHandler,
    mut end: crate::expat_h::XML_EndCdataSectionHandler,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_startCdataSectionHandler = start;
    (*parser).m_endCdataSectionHandler = end;
}
#[export_name = "XML_SetCdataSectionHandler"]

pub unsafe extern "C" fn XML_SetCdataSectionHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartCdataSectionHandler,
    mut end: crate::expat_h::XML_EndCdataSectionHandler,
) {
    XML_SetCdataSectionHandler(parser, start, end)
}
pub unsafe extern "C" fn XML_SetStartCdataSectionHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartCdataSectionHandler,
) {
    if !parser.is_null() {
        (*parser).m_startCdataSectionHandler = start;
    }
}
#[export_name = "XML_SetStartCdataSectionHandler"]

pub unsafe extern "C" fn XML_SetStartCdataSectionHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartCdataSectionHandler,
) {
    XML_SetStartCdataSectionHandler(parser, start)
}
pub unsafe extern "C" fn XML_SetEndCdataSectionHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut end: crate::expat_h::XML_EndCdataSectionHandler,
) {
    if !parser.is_null() {
        (*parser).m_endCdataSectionHandler = end;
    }
}
#[export_name = "XML_SetEndCdataSectionHandler"]

pub unsafe extern "C" fn XML_SetEndCdataSectionHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut end: crate::expat_h::XML_EndCdataSectionHandler,
) {
    XML_SetEndCdataSectionHandler(parser, end)
}
pub unsafe extern "C" fn XML_SetDefaultHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_DefaultHandler,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_defaultHandler = handler.is_some();
    let mut handlers = DEFAULT_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    match handler {
        Some(callback) => {
            handlers.insert(parser as usize, std::sync::Arc::new(callback));
        }
        None => {
            handlers.remove(&(parser as usize));
        }
    }
    (*parser).m_defaultExpandInternalEntities = crate::expat_h::XML_FALSE;
}
#[export_name = "XML_SetDefaultHandler"]

pub unsafe extern "C" fn XML_SetDefaultHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_DefaultHandler,
) {
    XML_SetDefaultHandler(parser, handler)
}
pub unsafe extern "C" fn XML_SetDefaultHandlerExpand(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_DefaultHandler,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_defaultHandler = handler.is_some();
    let mut handlers = DEFAULT_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    match handler {
        Some(callback) => {
            handlers.insert(parser as usize, std::sync::Arc::new(callback));
        }
        None => {
            handlers.remove(&(parser as usize));
        }
    }
    (*parser).m_defaultExpandInternalEntities = crate::expat_h::XML_TRUE;
}
#[export_name = "XML_SetDefaultHandlerExpand"]

pub unsafe extern "C" fn XML_SetDefaultHandlerExpand_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_DefaultHandler,
) {
    XML_SetDefaultHandlerExpand(parser, handler)
}
pub unsafe extern "C" fn XML_SetDoctypeDeclHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartDoctypeDeclHandler,
    mut end: crate::expat_h::XML_EndDoctypeDeclHandler,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_startDoctypeDeclHandler = start.is_some();
    let mut handlers = START_DOCTYPE_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    match start {
        Some(callback) => {
            handlers.insert(parser as usize, std::sync::Arc::new(callback));
        }
        None => {
            handlers.remove(&(parser as usize));
        }
    }
    (*parser).m_endDoctypeDeclHandler = end;
}
#[export_name = "XML_SetDoctypeDeclHandler"]

pub unsafe extern "C" fn XML_SetDoctypeDeclHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartDoctypeDeclHandler,
    mut end: crate::expat_h::XML_EndDoctypeDeclHandler,
) {
    XML_SetDoctypeDeclHandler(parser, start, end)
}
pub unsafe extern "C" fn XML_SetStartDoctypeDeclHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartDoctypeDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_startDoctypeDeclHandler = start.is_some();
        let mut handlers = START_DOCTYPE_DECL_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        match start {
            Some(callback) => {
                handlers.insert(parser as usize, std::sync::Arc::new(callback));
            }
            None => {
                handlers.remove(&(parser as usize));
            }
        }
    }
}
#[export_name = "XML_SetStartDoctypeDeclHandler"]

pub unsafe extern "C" fn XML_SetStartDoctypeDeclHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartDoctypeDeclHandler,
) {
    XML_SetStartDoctypeDeclHandler(parser, start)
}
pub unsafe extern "C" fn XML_SetEndDoctypeDeclHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut end: crate::expat_h::XML_EndDoctypeDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_endDoctypeDeclHandler = end;
    }
}
#[export_name = "XML_SetEndDoctypeDeclHandler"]

pub unsafe extern "C" fn XML_SetEndDoctypeDeclHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut end: crate::expat_h::XML_EndDoctypeDeclHandler,
) {
    XML_SetEndDoctypeDeclHandler(parser, end)
}
pub unsafe extern "C" fn XML_SetUnparsedEntityDeclHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_UnparsedEntityDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_unparsedEntityDeclHandler = handler.is_some();
        let mut handlers = UNPARSED_ENTITY_DECL_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        match handler {
            Some(callback) => {
                handlers.insert(parser as usize, std::sync::Arc::new(callback));
            }
            None => {
                handlers.remove(&(parser as usize));
            }
        }
    }
}
#[export_name = "XML_SetUnparsedEntityDeclHandler"]

pub unsafe extern "C" fn XML_SetUnparsedEntityDeclHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_UnparsedEntityDeclHandler,
) {
    XML_SetUnparsedEntityDeclHandler(parser, handler)
}
pub unsafe extern "C" fn XML_SetNotationDeclHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_NotationDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_notationDeclHandler = handler.is_some();
        let mut handlers = NOTATION_DECL_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        match handler {
            Some(callback) => {
                handlers.insert(parser as usize, std::sync::Arc::new(callback));
            }
            None => {
                handlers.remove(&(parser as usize));
            }
        }
    }
}
#[export_name = "XML_SetNotationDeclHandler"]

pub unsafe extern "C" fn XML_SetNotationDeclHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_NotationDeclHandler,
) {
    XML_SetNotationDeclHandler(parser, handler)
}
pub unsafe extern "C" fn XML_SetNamespaceDeclHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartNamespaceDeclHandler,
    mut end: crate::expat_h::XML_EndNamespaceDeclHandler,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_startNamespaceDeclHandler = start.is_some();
    let mut start_handlers = START_NAMESPACE_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    match start {
        Some(callback) => {
            start_handlers.insert(parser as usize, std::sync::Arc::new(callback));
        }
        None => {
            start_handlers.remove(&(parser as usize));
        }
    }
    (*parser).m_endNamespaceDeclHandler = end.is_some();
    let mut handlers = END_NAMESPACE_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    match end {
        Some(callback) => {
            handlers.insert(parser as usize, std::sync::Arc::new(callback));
        }
        None => {
            handlers.remove(&(parser as usize));
        }
    }
}
#[export_name = "XML_SetNamespaceDeclHandler"]

pub unsafe extern "C" fn XML_SetNamespaceDeclHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartNamespaceDeclHandler,
    mut end: crate::expat_h::XML_EndNamespaceDeclHandler,
) {
    XML_SetNamespaceDeclHandler(parser, start, end)
}
pub unsafe extern "C" fn XML_SetStartNamespaceDeclHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartNamespaceDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_startNamespaceDeclHandler = start.is_some();
        let mut handlers = START_NAMESPACE_DECL_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        match start {
            Some(callback) => {
                handlers.insert(parser as usize, std::sync::Arc::new(callback));
            }
            None => {
                handlers.remove(&(parser as usize));
            }
        }
    }
}
#[export_name = "XML_SetStartNamespaceDeclHandler"]

pub unsafe extern "C" fn XML_SetStartNamespaceDeclHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut start: crate::expat_h::XML_StartNamespaceDeclHandler,
) {
    XML_SetStartNamespaceDeclHandler(parser, start)
}
pub unsafe extern "C" fn XML_SetEndNamespaceDeclHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut end: crate::expat_h::XML_EndNamespaceDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_endNamespaceDeclHandler = end.is_some();
        let mut handlers = END_NAMESPACE_DECL_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        match end {
            Some(callback) => {
                handlers.insert(parser as usize, std::sync::Arc::new(callback));
            }
            None => {
                handlers.remove(&(parser as usize));
            }
        }
    }
}
#[export_name = "XML_SetEndNamespaceDeclHandler"]

pub unsafe extern "C" fn XML_SetEndNamespaceDeclHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut end: crate::expat_h::XML_EndNamespaceDeclHandler,
) {
    XML_SetEndNamespaceDeclHandler(parser, end)
}
pub unsafe extern "C" fn XML_SetNotStandaloneHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_NotStandaloneHandler,
) {
    if !parser.is_null() {
        (*parser).m_notStandaloneHandler = handler;
    }
}
#[export_name = "XML_SetNotStandaloneHandler"]

pub unsafe extern "C" fn XML_SetNotStandaloneHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_NotStandaloneHandler,
) {
    XML_SetNotStandaloneHandler(parser, handler)
}
pub unsafe extern "C" fn XML_SetExternalEntityRefHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_ExternalEntityRefHandler,
) {
    if !parser.is_null() {
        (*parser).m_externalEntityRefHandler = handler.is_some();
        let mut handlers = EXTERNAL_ENTITY_REF_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        match handler {
            Some(callback) => {
                handlers.insert(parser as usize, std::sync::Arc::new(callback));
            }
            None => {
                handlers.remove(&(parser as usize));
            }
        }
    }
}
#[export_name = "XML_SetExternalEntityRefHandler"]

pub unsafe extern "C" fn XML_SetExternalEntityRefHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_ExternalEntityRefHandler,
) {
    XML_SetExternalEntityRefHandler(parser, handler)
}
pub unsafe extern "C" fn XML_SetExternalEntityRefHandlerArg(
    mut parser: crate::expat_h::XML_Parser,
    mut arg: *mut ::core::ffi::c_void,
) {
    if parser.is_null() {
        return;
    }
    if !arg.is_null() {
        (*parser).m_externalEntityRefHandlerArg = arg as crate::expat_h::XML_Parser;
    } else {
        (*parser).m_externalEntityRefHandlerArg = parser;
    };
}
#[export_name = "XML_SetExternalEntityRefHandlerArg"]

pub unsafe extern "C" fn XML_SetExternalEntityRefHandlerArg_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut arg: *mut ::core::ffi::c_void,
) {
    XML_SetExternalEntityRefHandlerArg(parser, arg)
}
pub unsafe extern "C" fn XML_SetSkippedEntityHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_SkippedEntityHandler,
) {
    if !parser.is_null() {
        (*parser).m_skippedEntityHandler = handler.is_some();
        let mut handlers = SKIPPED_ENTITY_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        match handler {
            Some(callback) => {
                handlers.insert(parser as usize, std::sync::Arc::new(callback));
            }
            None => {
                handlers.remove(&(parser as usize));
            }
        }
    }
}
#[export_name = "XML_SetSkippedEntityHandler"]

pub unsafe extern "C" fn XML_SetSkippedEntityHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_SkippedEntityHandler,
) {
    XML_SetSkippedEntityHandler(parser, handler)
}
pub unsafe extern "C" fn XML_SetUnknownEncodingHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_UnknownEncodingHandler,
    mut data: *mut ::core::ffi::c_void,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_unknownEncodingHandler = handler.is_some();
    (*parser).m_unknownEncodingHandlerData = data;
    let mut handlers = UNKNOWN_ENCODING_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(callback) = handler {
        handlers.insert(parser as usize, std::sync::Arc::new(callback));
    } else {
        handlers.remove(&(parser as usize));
    }
}
#[export_name = "XML_SetUnknownEncodingHandler"]

pub unsafe extern "C" fn XML_SetUnknownEncodingHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_UnknownEncodingHandler,
    mut data: *mut ::core::ffi::c_void,
) {
    XML_SetUnknownEncodingHandler(parser, handler, data)
}
fn set_element_decl_handler(
    parser: &mut XML_ParserStruct,
    parser_key: usize,
    handler: Option<std::sync::Arc<dyn ElementDeclCallback>>,
) {
    parser.m_elementDeclHandler = handler.is_some();
    let mut handlers = ELEMENT_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(callback) = handler {
        handlers.insert(parser_key, callback);
    } else {
        handlers.remove(&parser_key);
    }
}
#[export_name = "XML_SetElementDeclHandler"]

pub unsafe extern "C" fn XML_SetElementDeclHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut eldecl: crate::expat_h::XML_ElementDeclHandler,
) {
    let parser_key = parser as usize;
    if let Some(parser) = parser.as_mut() {
        let handler = eldecl.map(|callback| {
            std::sync::Arc::new(callback) as std::sync::Arc<dyn ElementDeclCallback>
        });
        set_element_decl_handler(parser, parser_key, handler);
    }
}
fn set_attlist_decl_handler(
    parser: &mut XML_ParserStruct,
    parser_key: usize,
    handler: Option<std::sync::Arc<dyn AttlistDeclCallback>>,
) {
    parser.m_attlistDeclHandler = handler.is_some();
    let mut handlers = ATTLIST_DECL_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(callback) = handler {
        handlers.insert(parser_key, callback);
    } else {
        handlers.remove(&parser_key);
    }
}
pub unsafe extern "C" fn XML_SetAttlistDeclHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut attdecl: crate::expat_h::XML_AttlistDeclHandler,
) {
    let parser_key = parser as usize;
    if !parser.is_null() {
        let parser = &mut *parser;
        let handler = attdecl.map(|callback| {
            std::sync::Arc::new(callback) as std::sync::Arc<dyn AttlistDeclCallback>
        });
        set_attlist_decl_handler(parser, parser_key, handler);
    }
}
#[export_name = "XML_SetAttlistDeclHandler"]

pub unsafe extern "C" fn XML_SetAttlistDeclHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut attdecl: crate::expat_h::XML_AttlistDeclHandler,
) {
    XML_SetAttlistDeclHandler(parser, attdecl)
}
pub unsafe extern "C" fn XML_SetEntityDeclHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_EntityDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_entityDeclHandler = handler.is_some();
        let mut handlers = ENTITY_DECL_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        match handler {
            Some(callback) => {
                handlers.insert(parser as usize, std::sync::Arc::new(callback));
            }
            None => {
                handlers.remove(&(parser as usize));
            }
        }
    }
}
#[export_name = "XML_SetEntityDeclHandler"]

pub unsafe extern "C" fn XML_SetEntityDeclHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_EntityDeclHandler,
) {
    XML_SetEntityDeclHandler(parser, handler)
}
pub unsafe extern "C" fn XML_SetXmlDeclHandler(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_XmlDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_xmlDeclHandler = handler.is_some();
        let mut handlers = XML_DECL_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        match handler {
            Some(callback) => {
                handlers.insert(parser as usize, std::sync::Arc::new(callback));
            }
            None => {
                handlers.remove(&(parser as usize));
            }
        }
    }
}
#[export_name = "XML_SetXmlDeclHandler"]

pub unsafe extern "C" fn XML_SetXmlDeclHandler_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut handler: crate::expat_h::XML_XmlDeclHandler,
) {
    XML_SetXmlDeclHandler(parser, handler)
}
pub unsafe extern "C" fn XML_SetParamEntityParsing(
    mut parser: crate::expat_h::XML_Parser,
    mut peParsing: crate::expat_h::XML_ParamEntityParsing,
) -> ::core::ffi::c_int {
    if parser.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if parserBusy(parser) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    (*parser).m_paramEntityParsing = peParsing;
    return 1 as ::core::ffi::c_int;
}
#[export_name = "XML_SetParamEntityParsing"]

pub unsafe extern "C" fn XML_SetParamEntityParsing_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut peParsing: crate::expat_h::XML_ParamEntityParsing,
) -> ::core::ffi::c_int {
    XML_SetParamEntityParsing(parser, peParsing)
}
pub unsafe extern "C" fn XML_SetHashSalt(
    mut parser: crate::expat_h::XML_Parser,
    mut hash_salt: ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    if parser.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let rootParser: crate::expat_h::XML_Parser =
        getRootParserOf(parser, ::core::ptr::null_mut::<::core::ffi::c_uint>());
    '_c2rust_label: {
        if (*rootParser).m_parentParser.is_null() {
        } else {
            crate::stdlib::__assert_fail(
                b"! rootParser->m_parentParser\0".as_ptr() as *const ::core::ffi::c_char,
                b"../../expat/lib/xmlparse.c\0".as_ptr() as *const ::core::ffi::c_char,
                2331 as ::core::ffi::c_uint,
                b"int XML_SetHashSalt(XML_Parser, unsigned long)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if parserBusy(rootParser) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    (*rootParser).m_hash_secret_salt = hash_salt;
    return 1 as ::core::ffi::c_int;
}
#[export_name = "XML_SetHashSalt"]

pub unsafe extern "C" fn XML_SetHashSalt_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut hash_salt: ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    XML_SetHashSalt(parser, hash_salt)
}
pub unsafe extern "C" fn XML_Parse(
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
    let mut buff: *mut ::core::ffi::c_void = XML_GetBuffer(parser, len);
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
    return XML_ParseBuffer(parser, len, isFinal);
}
#[export_name = "XML_Parse"]

pub unsafe extern "C" fn XML_Parse_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut s: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
    mut isFinal: ::core::ffi::c_int,
) -> crate::expat_h::XML_Status {
    XML_Parse(parser, s, len, isFinal)
}
/// Processes the caller-owned portion of the parser input buffer.
///
/// The parser may call user handlers from `callProcessor`, which can re-enter
/// the parser.  Keep the typed borrows below confined to the work before and
/// after that call; no `&mut XML_ParserStruct` is held while handlers run.
unsafe fn parse_buffer_impl(
    parser: crate::expat_h::XML_Parser,
    len: ::core::ffi::c_int,
    is_final: ::core::ffi::c_int,
) -> crate::expat_h::XML_Status {
    let mut result: crate::expat_h::XML_Status = crate::expat_h::XML_STATUS_OK;
    let needs_start = {
        let parser_ref = &mut *parser;
        if len < 0 as ::core::ffi::c_int {
            parser_ref.m_errorCode = crate::expat_h::XML_ERROR_INVALID_ARGUMENT;
            return crate::expat_h::XML_STATUS_ERROR;
        }
        match parser_ref.m_parsingStatus.parsing as ::core::ffi::c_uint {
            3 => {
                parser_ref.m_errorCode = crate::expat_h::XML_ERROR_SUSPENDED;
                return crate::expat_h::XML_STATUS_ERROR;
            }
            2 => {
                parser_ref.m_errorCode = crate::expat_h::XML_ERROR_FINISHED;
                return crate::expat_h::XML_STATUS_ERROR;
            }
            0 => {
                if parser_ref.m_bufferPtr.is_null() {
                    parser_ref.m_errorCode = crate::expat_h::XML_ERROR_NO_BUFFER;
                    return crate::expat_h::XML_STATUS_ERROR;
                }
                parser_ref.m_parentParser.is_null()
            }
            _ => false,
        }
    };
    if needs_start && startParsing(parser) == 0 {
        (&mut *parser).m_errorCode = crate::expat_h::XML_ERROR_NO_MEMORY;
        return crate::expat_h::XML_STATUS_ERROR;
    }
    let (start, parse_end) = {
        let parser_ref = &mut *parser;
        parser_ref.m_parsingStatus.parsing = crate::expat_h::XML_PARSING;
        let start = parser_ref.m_bufferPtr;
        parser_ref.m_positionPtr = start;
        parser_ref.m_bufferEnd = parser_ref.m_bufferEnd.wrapping_add(len as usize);
        parser_ref.m_parseEndPtr = parser_ref.m_bufferEnd;
        parser_ref.m_parseEndByteIndex += len as crate::expat_external_h::XML_Index;
        parser_ref.m_parsingStatus.finalBuffer = is_final as crate::expat_h::XML_Bool;
        (start, parser_ref.m_parseEndPtr)
    };

    // The setup borrow has ended before a user callback can re-enter through
    // callProcessor.
    let error = callProcessor(
        parser,
        start,
        parse_end,
        &raw mut (*parser).m_bufferPtr,
    );
    {
        let parser_ref = &mut *parser;
        parser_ref.m_errorCode = error;
        if parser_ref.m_errorCode as ::core::ffi::c_uint
            != crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            parser_ref.m_eventEndPtr = parser_ref.m_eventPtr;
            parser_ref.m_processor = ProcessorState::Error;
            return crate::expat_h::XML_STATUS_ERROR;
        } else {
            match parser_ref.m_parsingStatus.parsing as ::core::ffi::c_uint {
                3 => {
                    result = crate::expat_h::XML_STATUS_SUSPENDED;
                }
                0 | 1 => {
                    if is_final != 0 {
                        parser_ref.m_parsingStatus.parsing = crate::expat_h::XML_FINISHED;
                        return result;
                    }
                }
                _ => {}
            }
        }
    }
    let encoding = parser_encoding(parser);
    let parser_ref = &mut *parser;
    let position_ptr = parser_ref.m_positionPtr;
    let buffer_ptr = parser_ref.m_bufferPtr;
    crate::src::xmltok::initUpdatePosition(
        (*encoding).updatePosition,
        encoding,
        position_ptr,
        buffer_ptr,
        &raw mut parser_ref.m_position,
    );
    parser_ref.m_positionPtr = buffer_ptr;
    return result;
}
pub unsafe extern "C" fn XML_ParseBuffer(
    parser: crate::expat_h::XML_Parser,
    len: ::core::ffi::c_int,
    isFinal: ::core::ffi::c_int,
) -> crate::expat_h::XML_Status {
    if parser.is_null() {
        return crate::expat_h::XML_STATUS_ERROR;
    }
    parse_buffer_impl(parser, len, isFinal)
}
#[export_name = "XML_ParseBuffer"]

pub unsafe extern "C" fn XML_ParseBuffer_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut len: ::core::ffi::c_int,
    mut isFinal: ::core::ffi::c_int,
) -> crate::expat_h::XML_Status {
    XML_ParseBuffer(parser, len, isFinal)
}
pub unsafe extern "C" fn XML_GetBuffer(
    mut parser: crate::expat_h::XML_Parser,
    mut len: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    if parser.is_null() {
        return crate::__stddef_null_h::NULL;
    }
    let parser_ref = &mut *parser;
    if len < 0 as ::core::ffi::c_int {
        parser_ref.m_errorCode = crate::expat_h::XML_ERROR_NO_MEMORY;
        return crate::__stddef_null_h::NULL;
    }
    match parser_ref.m_parsingStatus.parsing as ::core::ffi::c_uint {
        3 => {
            parser_ref.m_errorCode = crate::expat_h::XML_ERROR_SUSPENDED;
            return crate::__stddef_null_h::NULL;
        }
        2 => {
            parser_ref.m_errorCode = crate::expat_h::XML_ERROR_FINISHED;
            return crate::__stddef_null_h::NULL;
        }
        _ => {}
    }
    parser_ref.m_lastBufferRequestSize = len;
    if len as isize
        > (if !parser_ref.m_bufferLim.is_null() && !parser_ref.m_bufferEnd.is_null() {
            parser_ref.m_bufferLim.offset_from(parser_ref.m_bufferEnd)
        } else {
            0 as isize
        })
        || parser_ref.m_buffer.is_null()
    {
        let mut keep: ::core::ffi::c_int = 0;
        let mut neededSize: ::core::ffi::c_int = (len as ::core::ffi::c_uint).wrapping_add(
            (if !parser_ref.m_bufferEnd.is_null() && !parser_ref.m_bufferPtr.is_null() {
                parser_ref.m_bufferEnd.offset_from(parser_ref.m_bufferPtr)
            } else {
                0 as isize
            }) as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int;
        if neededSize < 0 as ::core::ffi::c_int {
            parser_ref.m_errorCode = crate::expat_h::XML_ERROR_NO_MEMORY;
            return crate::__stddef_null_h::NULL;
        }
        keep = (if !parser_ref.m_bufferPtr.is_null() && !parser_ref.m_buffer.is_null() {
            parser_ref.m_bufferPtr.offset_from(parser_ref.m_buffer)
        } else {
            0 as isize
        }) as ::core::ffi::c_int;
        if keep > crate::stdlib::XML_CONTEXT_BYTES {
            keep = crate::stdlib::XML_CONTEXT_BYTES;
        }
        if keep > crate::limits_h::INT_MAX - neededSize {
            parser_ref.m_errorCode = crate::expat_h::XML_ERROR_NO_MEMORY;
            return crate::__stddef_null_h::NULL;
        }
        neededSize += keep;
        if !parser_ref.m_buffer.is_null()
            && !parser_ref.m_bufferPtr.is_null()
            && neededSize as isize
                <= (if !parser_ref.m_bufferLim.is_null() && !parser_ref.m_buffer.is_null() {
                    parser_ref.m_bufferLim.offset_from(parser_ref.m_buffer)
                } else {
                    0 as isize
                })
        {
            if (keep as isize)
                < (if !parser_ref.m_bufferPtr.is_null() && !parser_ref.m_buffer.is_null() {
                    parser_ref.m_bufferPtr.offset_from(parser_ref.m_buffer)
                } else {
                    0 as isize
                })
            {
                let mut offset: ::core::ffi::c_int =
                    (if !parser_ref.m_bufferPtr.is_null() && !parser_ref.m_buffer.is_null() {
                        parser_ref.m_bufferPtr.offset_from(parser_ref.m_buffer)
                    } else {
                        0 as isize
                    }) as ::core::ffi::c_int
                        - keep;
                crate::stdlib::memmove(
                    parser_ref.m_buffer as *mut ::core::ffi::c_void,
                    parser_ref.m_buffer.offset(offset as isize) as *const ::core::ffi::c_void,
                    (parser_ref.m_bufferEnd.offset_from(parser_ref.m_bufferPtr) + keep as isize)
                        as crate::__stddef_size_t_h::size_t,
                );
                parser_ref.m_bufferEnd = parser_ref.m_bufferEnd.offset(-(offset as isize));
                parser_ref.m_bufferPtr = parser_ref.m_bufferPtr.offset(-(offset as isize));
            }
        } else {
            let mut newBuf: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut bufferSize: ::core::ffi::c_int =
                (if !parser_ref.m_bufferLim.is_null() && !parser_ref.m_buffer.is_null() {
                    parser_ref.m_bufferLim.offset_from(parser_ref.m_buffer)
                } else {
                    0 as isize
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
                parser_ref.m_errorCode = crate::expat_h::XML_ERROR_NO_MEMORY;
                return crate::__stddef_null_h::NULL;
            }
            newBuf = parser_ref
                .m_mem
                .malloc_fcn
                .expect("non-null function pointer")(
                bufferSize as crate::__stddef_size_t_h::size_t
            ) as *mut ::core::ffi::c_char;
            if newBuf.is_null() {
                parser_ref.m_errorCode = crate::expat_h::XML_ERROR_NO_MEMORY;
                return crate::__stddef_null_h::NULL;
            }
            parser_ref.m_bufferLim = newBuf.offset(bufferSize as isize);
            if !parser_ref.m_bufferPtr.is_null() {
                crate::stdlib::memcpy(
                    newBuf as *mut ::core::ffi::c_void,
                    parser_ref.m_bufferPtr.offset(-keep as isize) as *const ::core::ffi::c_void,
                    ((if !parser_ref.m_bufferEnd.is_null() && !parser_ref.m_bufferPtr.is_null() {
                        parser_ref.m_bufferEnd.offset_from(parser_ref.m_bufferPtr)
                    } else {
                        0 as isize
                    }) + keep as isize) as crate::__stddef_size_t_h::size_t,
                );
                parser_ref.m_mem.free_fcn.expect("non-null function pointer")(
                    parser_ref.m_buffer as *mut ::core::ffi::c_void,
                );
                parser_ref.m_buffer = newBuf;
                parser_ref.m_bufferEnd = parser_ref
                    .m_buffer
                    .offset(
                        (if !parser_ref.m_bufferEnd.is_null() && !parser_ref.m_bufferPtr.is_null() {
                            parser_ref.m_bufferEnd.offset_from(parser_ref.m_bufferPtr)
                        } else {
                            0 as isize
                        }) as isize,
                    )
                    .offset(keep as isize);
                parser_ref.m_bufferPtr = parser_ref.m_buffer.offset(keep as isize);
            } else {
                parser_ref.m_bufferEnd = newBuf;
                parser_ref.m_buffer = newBuf;
                parser_ref.m_bufferPtr = parser_ref.m_buffer;
            }
        }
        parser_ref.m_eventEndPtr = ::core::ptr::null::<::core::ffi::c_char>();
        parser_ref.m_eventPtr = parser_ref.m_eventEndPtr;
        parser_ref.m_positionPtr = ::core::ptr::null::<::core::ffi::c_char>();
    }
    return parser_ref.m_bufferEnd as *mut ::core::ffi::c_void;
}
#[export_name = "XML_GetBuffer"]

pub unsafe extern "C" fn XML_GetBuffer_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut len: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    XML_GetBuffer(parser, len)
}
unsafe extern "C" fn triggerReenter(mut parser: crate::expat_h::XML_Parser) {
    (*parser).m_reenter = crate::expat_h::XML_TRUE;
}
pub unsafe extern "C" fn XML_StopParser(
    mut parser: crate::expat_h::XML_Parser,
    mut resumable: crate::expat_h::XML_Bool,
) -> crate::expat_h::XML_Status {
    if parser.is_null() {
        return crate::expat_h::XML_STATUS_ERROR;
    }
    match (*parser).m_parsingStatus.parsing as ::core::ffi::c_uint {
        0 => {
            (*parser).m_errorCode = crate::expat_h::XML_ERROR_NOT_STARTED;
            return crate::expat_h::XML_STATUS_ERROR;
        }
        3 => {
            if resumable != 0 {
                (*parser).m_errorCode = crate::expat_h::XML_ERROR_SUSPENDED;
                return crate::expat_h::XML_STATUS_ERROR;
            }
            (*parser).m_parsingStatus.parsing = crate::expat_h::XML_FINISHED;
        }
        2 => {
            (*parser).m_errorCode = crate::expat_h::XML_ERROR_FINISHED;
            return crate::expat_h::XML_STATUS_ERROR;
        }
        1 => {
            if resumable != 0 {
                if (*parser).m_isParamEntity != 0 {
                    (*parser).m_errorCode = crate::expat_h::XML_ERROR_SUSPEND_PE;
                    return crate::expat_h::XML_STATUS_ERROR;
                }
                (*parser).m_parsingStatus.parsing = crate::expat_h::XML_SUSPENDED;
            } else {
                (*parser).m_parsingStatus.parsing = crate::expat_h::XML_FINISHED;
            }
        }
        _ => {
            '_c2rust_label: {
                crate::stdlib::__assert_fail(
                    b"0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../../expat/lib/xmlparse.c\0".as_ptr() as *const ::core::ffi::c_char,
                    2692 as ::core::ffi::c_uint,
                    b"enum XML_Status XML_StopParser(XML_Parser, XML_Bool)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
        }
    }
    return crate::expat_h::XML_STATUS_OK;
}
#[export_name = "XML_StopParser"]

pub unsafe extern "C" fn XML_StopParser_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut resumable: crate::expat_h::XML_Bool,
) -> crate::expat_h::XML_Status {
    XML_StopParser(parser, resumable)
}
pub unsafe extern "C" fn XML_ResumeParser(
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
        (*parser).m_processor = ProcessorState::Error;
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
    crate::src::xmltok::initUpdatePosition(
        (*parser_encoding(parser)).updatePosition,
        parser_encoding(parser),
        (*parser).m_positionPtr,
        (*parser).m_bufferPtr,
        &raw mut (*parser).m_position,
    );
    (*parser).m_positionPtr = (*parser).m_bufferPtr;
    return result;
}
#[export_name = "XML_ResumeParser"]

pub unsafe extern "C" fn XML_ResumeParser_ffi(
    mut parser: crate::expat_h::XML_Parser,
) -> crate::expat_h::XML_Status {
    XML_ResumeParser(parser)
}
pub unsafe extern "C" fn XML_GetParsingStatus(
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
    *status = (*parser).m_parsingStatus;
}
#[export_name = "XML_GetParsingStatus"]

pub unsafe extern "C" fn XML_GetParsingStatus_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut status: *mut crate::expat_h::XML_ParsingStatus,
) {
    XML_GetParsingStatus(parser, status)
}
pub unsafe extern "C" fn XML_GetErrorCode(
    mut parser: crate::expat_h::XML_Parser,
) -> crate::expat_h::XML_Error {
    if parser.is_null() {
        return crate::expat_h::XML_ERROR_INVALID_ARGUMENT;
    }
    return (*parser).m_errorCode;
}
#[export_name = "XML_GetErrorCode"]

pub unsafe extern "C" fn XML_GetErrorCode_ffi(
    mut parser: crate::expat_h::XML_Parser,
) -> crate::expat_h::XML_Error {
    XML_GetErrorCode(parser)
}
pub unsafe extern "C" fn XML_GetCurrentByteIndex(
    mut parser: crate::expat_h::XML_Parser,
) -> crate::expat_external_h::XML_Index {
    if parser.is_null() {
        return -1 as crate::expat_external_h::XML_Index;
    }
    if !(*parser).m_eventPtr.is_null() {
        return ((*parser).m_parseEndByteIndex as isize
            - (*parser).m_parseEndPtr.offset_from((*parser).m_eventPtr))
            as crate::expat_external_h::XML_Index;
    }
    return -1 as crate::expat_external_h::XML_Index;
}
#[export_name = "XML_GetCurrentByteIndex"]

pub unsafe extern "C" fn XML_GetCurrentByteIndex_ffi(
    mut parser: crate::expat_h::XML_Parser,
) -> crate::expat_external_h::XML_Index {
    XML_GetCurrentByteIndex(parser)
}
pub unsafe extern "C" fn XML_GetCurrentByteCount(
    mut parser: crate::expat_h::XML_Parser,
) -> ::core::ffi::c_int {
    if parser.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if !(*parser).m_eventEndPtr.is_null() && !(*parser).m_eventPtr.is_null() {
        return (*parser).m_eventEndPtr.offset_from((*parser).m_eventPtr) as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[export_name = "XML_GetCurrentByteCount"]

pub unsafe extern "C" fn XML_GetCurrentByteCount_ffi(
    mut parser: crate::expat_h::XML_Parser,
) -> ::core::ffi::c_int {
    XML_GetCurrentByteCount(parser)
}
pub unsafe extern "C" fn XML_GetInputContext(
    mut parser: crate::expat_h::XML_Parser,
    mut offset: *mut ::core::ffi::c_int,
    mut size: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    if parser.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    if !(*parser).m_eventPtr.is_null() && !(*parser).m_buffer.is_null() {
        if !offset.is_null() {
            *offset = (*parser).m_eventPtr.offset_from((*parser).m_buffer) as ::core::ffi::c_int;
        }
        if !size.is_null() {
            *size = (*parser).m_bufferEnd.offset_from((*parser).m_buffer) as ::core::ffi::c_int;
        }
        return (*parser).m_buffer;
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[export_name = "XML_GetInputContext"]

pub unsafe extern "C" fn XML_GetInputContext_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut offset: *mut ::core::ffi::c_int,
    mut size: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    XML_GetInputContext(parser, offset, size)
}
pub unsafe extern "C" fn XML_GetCurrentLineNumber(
    mut parser: crate::expat_h::XML_Parser,
) -> crate::expat_external_h::XML_Size {
    if parser.is_null() {
        return 0 as crate::expat_external_h::XML_Size;
    }
    if !(*parser).m_eventPtr.is_null() && (*parser).m_eventPtr >= (*parser).m_positionPtr {
        crate::src::xmltok::initUpdatePosition(
            (*parser_encoding(parser)).updatePosition,
            parser_encoding(parser),
            (*parser).m_positionPtr,
            (*parser).m_eventPtr,
            &raw mut (*parser).m_position,
        );
        (*parser).m_positionPtr = (*parser).m_eventPtr;
    }
    return (*parser)
        .m_position
        .lineNumber
        .wrapping_add(1 as crate::expat_external_h::XML_Size);
}
#[export_name = "XML_GetCurrentLineNumber"]

pub unsafe extern "C" fn XML_GetCurrentLineNumber_ffi(
    mut parser: crate::expat_h::XML_Parser,
) -> crate::expat_external_h::XML_Size {
    XML_GetCurrentLineNumber(parser)
}
pub unsafe extern "C" fn XML_GetCurrentColumnNumber(
    mut parser: crate::expat_h::XML_Parser,
) -> crate::expat_external_h::XML_Size {
    if parser.is_null() {
        return 0 as crate::expat_external_h::XML_Size;
    }
    if !(*parser).m_eventPtr.is_null() && (*parser).m_eventPtr >= (*parser).m_positionPtr {
        crate::src::xmltok::initUpdatePosition(
            (*parser_encoding(parser)).updatePosition,
            parser_encoding(parser),
            (*parser).m_positionPtr,
            (*parser).m_eventPtr,
            &raw mut (*parser).m_position,
        );
        (*parser).m_positionPtr = (*parser).m_eventPtr;
    }
    return (*parser).m_position.columnNumber;
}
#[export_name = "XML_GetCurrentColumnNumber"]

pub unsafe extern "C" fn XML_GetCurrentColumnNumber_ffi(
    mut parser: crate::expat_h::XML_Parser,
) -> crate::expat_external_h::XML_Size {
    XML_GetCurrentColumnNumber(parser)
}
pub unsafe extern "C" fn XML_FreeContentModel(
    mut parser: crate::expat_h::XML_Parser,
    mut model: *mut crate::expat_h::XML_Content,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_mem.free_fcn.expect("non-null function pointer")(model as *mut ::core::ffi::c_void);
}
#[export_name = "XML_FreeContentModel"]

pub unsafe extern "C" fn XML_FreeContentModel_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut model: *mut crate::expat_h::XML_Content,
) {
    XML_FreeContentModel(parser, model)
}
pub unsafe extern "C" fn XML_MemMalloc(
    mut parser: crate::expat_h::XML_Parser,
    mut size: crate::__stddef_size_t_h::size_t,
) -> *mut ::core::ffi::c_void {
    if parser.is_null() {
        return crate::__stddef_null_h::NULL;
    }
    return (*parser)
        .m_mem
        .malloc_fcn
        .expect("non-null function pointer")(size);
}
#[export_name = "XML_MemMalloc"]

pub unsafe extern "C" fn XML_MemMalloc_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut size: crate::__stddef_size_t_h::size_t,
) -> *mut ::core::ffi::c_void {
    XML_MemMalloc(parser, size)
}
pub unsafe extern "C" fn XML_MemRealloc(
    mut parser: crate::expat_h::XML_Parser,
    mut ptr: *mut ::core::ffi::c_void,
    mut size: crate::__stddef_size_t_h::size_t,
) -> *mut ::core::ffi::c_void {
    if parser.is_null() {
        return crate::__stddef_null_h::NULL;
    }
    return (*parser)
        .m_mem
        .realloc_fcn
        .expect("non-null function pointer")(ptr, size);
}
#[export_name = "XML_MemRealloc"]

pub unsafe extern "C" fn XML_MemRealloc_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut ptr: *mut ::core::ffi::c_void,
    mut size: crate::__stddef_size_t_h::size_t,
) -> *mut ::core::ffi::c_void {
    XML_MemRealloc(parser, ptr, size)
}
pub unsafe extern "C" fn XML_MemFree(
    mut parser: crate::expat_h::XML_Parser,
    mut ptr: *mut ::core::ffi::c_void,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_mem.free_fcn.expect("non-null function pointer")(ptr);
}
#[export_name = "XML_MemFree"]

pub unsafe extern "C" fn XML_MemFree_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut ptr: *mut ::core::ffi::c_void,
) {
    XML_MemFree(parser, ptr)
}
pub unsafe extern "C" fn XML_DefaultCurrent(mut parser: crate::expat_h::XML_Parser) {
    if parser.is_null() {
        return;
    }
    if (*parser).m_defaultHandler {
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
                parser_encoding(parser),
                (*parser).m_eventPtr,
                (*parser).m_eventEndPtr,
            );
        }
    }
}
#[export_name = "XML_DefaultCurrent"]

pub unsafe extern "C" fn XML_DefaultCurrent_ffi(mut parser: crate::expat_h::XML_Parser) {
    XML_DefaultCurrent(parser)
}
pub unsafe extern "C" fn XML_ErrorString(
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
pub unsafe extern "C" fn XML_ExpatVersion() -> *const crate::expat_external_h::XML_LChar {
    return b"expat_2.7.4\0".as_ptr() as *const crate::expat_external_h::XML_LChar;
}
#[export_name = "XML_ExpatVersion"]

pub unsafe extern "C" fn XML_ExpatVersion_ffi() -> *const crate::expat_external_h::XML_LChar {
    XML_ExpatVersion()
}
pub unsafe extern "C" fn XML_ExpatVersionInfo() -> crate::expat_h::XML_Expat_Version {
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
pub unsafe extern "C" fn XML_GetFeatureList() -> *const crate::expat_h::XML_Feature {
    static mut features: [crate::expat_h::XML_Feature; 11] = [
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
    ];
    return &raw const features as *const crate::expat_h::XML_Feature;
}
#[export_name = "XML_GetFeatureList"]

pub unsafe extern "C" fn XML_GetFeatureList_ffi() -> *const crate::expat_h::XML_Feature {
    XML_GetFeatureList()
}
pub unsafe extern "C" fn XML_SetBillionLaughsAttackProtectionMaximumAmplification(
    mut parser: crate::expat_h::XML_Parser,
    mut maximumAmplificationFactor: ::core::ffi::c_float,
) -> crate::expat_h::XML_Bool {
    if parser.is_null()
        || !(*parser).m_parentParser.is_null()
        || maximumAmplificationFactor.is_nan() as i32 != 0
        || maximumAmplificationFactor < 1.0f32
    {
        return crate::expat_h::XML_FALSE;
    }
    (*parser).m_accounting.maximumAmplificationFactor = maximumAmplificationFactor;
    return crate::expat_h::XML_TRUE;
}
#[export_name = "XML_SetBillionLaughsAttackProtectionMaximumAmplification"]

pub unsafe extern "C" fn XML_SetBillionLaughsAttackProtectionMaximumAmplification_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut maximumAmplificationFactor: ::core::ffi::c_float,
) -> crate::expat_h::XML_Bool {
    XML_SetBillionLaughsAttackProtectionMaximumAmplification(parser, maximumAmplificationFactor)
}
pub unsafe extern "C" fn XML_SetBillionLaughsAttackProtectionActivationThreshold(
    mut parser: crate::expat_h::XML_Parser,
    mut activationThresholdBytes: ::core::ffi::c_ulonglong,
) -> crate::expat_h::XML_Bool {
    if parser.is_null() || !(*parser).m_parentParser.is_null() {
        return crate::expat_h::XML_FALSE;
    }
    (*parser).m_accounting.activationThresholdBytes = activationThresholdBytes;
    return crate::expat_h::XML_TRUE;
}
#[export_name = "XML_SetBillionLaughsAttackProtectionActivationThreshold"]

pub unsafe extern "C" fn XML_SetBillionLaughsAttackProtectionActivationThreshold_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut activationThresholdBytes: ::core::ffi::c_ulonglong,
) -> crate::expat_h::XML_Bool {
    XML_SetBillionLaughsAttackProtectionActivationThreshold(parser, activationThresholdBytes)
}
pub unsafe extern "C" fn XML_SetAllocTrackerMaximumAmplification(
    mut parser: crate::expat_h::XML_Parser,
    mut maximumAmplificationFactor: ::core::ffi::c_float,
) -> crate::expat_h::XML_Bool {
    if parser.is_null()
        || !(*parser).m_parentParser.is_null()
        || maximumAmplificationFactor.is_nan() as i32 != 0
        || maximumAmplificationFactor < 1.0f32
    {
        return crate::expat_h::XML_FALSE;
    }
    (*parser).m_alloc_tracker.maximumAmplificationFactor = maximumAmplificationFactor;
    return crate::expat_h::XML_TRUE;
}
#[export_name = "XML_SetAllocTrackerMaximumAmplification"]

pub unsafe extern "C" fn XML_SetAllocTrackerMaximumAmplification_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut maximumAmplificationFactor: ::core::ffi::c_float,
) -> crate::expat_h::XML_Bool {
    XML_SetAllocTrackerMaximumAmplification(parser, maximumAmplificationFactor)
}
pub unsafe extern "C" fn XML_SetAllocTrackerActivationThreshold(
    mut parser: crate::expat_h::XML_Parser,
    mut activationThresholdBytes: ::core::ffi::c_ulonglong,
) -> crate::expat_h::XML_Bool {
    if parser.is_null() || !(*parser).m_parentParser.is_null() {
        return crate::expat_h::XML_FALSE;
    }
    (*parser).m_alloc_tracker.activationThresholdBytes = activationThresholdBytes as XmlBigCount;
    return crate::expat_h::XML_TRUE;
}
#[export_name = "XML_SetAllocTrackerActivationThreshold"]

pub unsafe extern "C" fn XML_SetAllocTrackerActivationThreshold_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut activationThresholdBytes: ::core::ffi::c_ulonglong,
) -> crate::expat_h::XML_Bool {
    XML_SetAllocTrackerActivationThreshold(parser, activationThresholdBytes)
}
pub unsafe extern "C" fn XML_SetReparseDeferralEnabled(
    mut parser: crate::expat_h::XML_Parser,
    mut enabled: crate::expat_h::XML_Bool,
) -> crate::expat_h::XML_Bool {
    if !parser.is_null()
        && (enabled as ::core::ffi::c_int == crate::expat_h::XML_TRUE as ::core::ffi::c_int
            || enabled as ::core::ffi::c_int == crate::expat_h::XML_FALSE as ::core::ffi::c_int)
    {
        (*parser).m_reparseDeferralEnabled = enabled;
        return crate::expat_h::XML_TRUE;
    }
    return crate::expat_h::XML_FALSE;
}
#[export_name = "XML_SetReparseDeferralEnabled"]

pub unsafe extern "C" fn XML_SetReparseDeferralEnabled_ffi(
    mut parser: crate::expat_h::XML_Parser,
    mut enabled: crate::expat_h::XML_Bool,
) -> crate::expat_h::XML_Bool {
    XML_SetReparseDeferralEnabled(parser, enabled)
}
unsafe extern "C" fn storeRawNames(
    mut parser: crate::expat_h::XML_Parser,
) -> crate::expat_h::XML_Bool {
    let mut tag: *mut TAG = (*parser).m_tagStack;
    while !tag.is_null() {
        let mut bufSize: crate::__stddef_size_t_h::size_t = 0;
        let mut nameLen: crate::__stddef_size_t_h::size_t =
            ::core::mem::size_of::<crate::expat_external_h::XML_Char>().wrapping_mul(
                ((*tag).name.strLen + 1 as ::core::ffi::c_int) as crate::__stddef_size_t_h::size_t,
            );
        let mut rawNameLen: crate::__stddef_size_t_h::size_t = 0;
        let tag_buf = (*tag).bufEnd as *mut crate::expat_external_h::XML_Char;
        let mut rawNameBuf: *mut ::core::ffi::c_char = tag_buf.offset(nameLen as isize);
        if (*tag).rawName == rawNameBuf as *const ::core::ffi::c_char {
            break;
        }
        rawNameLen = (((*tag).rawNameLength as usize).wrapping_add(
            ::core::mem::size_of::<crate::expat_external_h::XML_Char>().wrapping_sub(1 as usize),
        ) & !::core::mem::size_of::<crate::expat_external_h::XML_Char>()
            .wrapping_sub(1 as usize)) as crate::__stddef_size_t_h::size_t;
        if rawNameLen
            > (crate::limits_h::INT_MAX as crate::__stddef_size_t_h::size_t).wrapping_sub(nameLen)
        {
            return crate::expat_h::XML_FALSE;
        }
        bufSize = nameLen.wrapping_add(rawNameLen);
        if bufSize > (*tag).bufSize {
            let mut temp: *mut ::core::ffi::c_char = expat_realloc(
                parser,
                tag_buf as *mut ::core::ffi::c_void,
                bufSize,
                3151 as ::core::ffi::c_int,
            ) as *mut ::core::ffi::c_char;
            if temp.is_null() {
                return crate::expat_h::XML_FALSE;
            }
            (*tag).bufEnd = temp;
            (*tag).bufSize = bufSize;
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
        parser_encoding(parser),
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
    (*parser).m_processor = ProcessorState::ExternalEntityInit2;
    return externalEntityInitProcessor2(parser, start, end, endPtr);
}

/// Returns the tokenizer encoding selected by the parser state.
///
/// `Initial` deliberately consults `INIT_ENCODING.selected_encoding` on each
/// dispatch.  The initial scanner updates that safe index itself, so there is
/// no second pointer-synchronisation step after a BOM or leading byte scan.
unsafe fn parser_encoding(
    mut parser: crate::expat_h::XML_Parser,
) -> *const crate::src::xmltok::ENCODING {
    let parser = &*parser;
    match parser.m_encoding {
        EncodingState::Initial => match parser.m_initEncoding.selected_encoding {
            Some(index) if index < 7 => {
                if parser.m_ns != 0 {
                    crate::src::xmltok::encodingsNS[index]
                } else {
                    crate::src::xmltok::encodings[index]
                }
            }
            _ => &raw const parser.m_initEncoding.initEnc,
        },
        EncodingState::Unknown => parser.m_unknownEncodingMem.cast(),
    }
}

unsafe fn select_known_encoding(
    mut parser: crate::expat_h::XML_Parser,
    encoding: *const crate::src::xmltok::ENCODING,
) -> bool {
    let parser_ref = &mut *parser;
    let table = if parser_ref.m_ns != 0 {
        crate::src::xmltok::encodingsNS
    } else {
        crate::src::xmltok::encodings
    };
    let Some(index) = table.iter().position(|&candidate| candidate == encoding) else {
        return false;
    };
    parser_ref.m_initEncoding.selected_encoding = Some(index);
    parser_ref.m_encoding = EncodingState::Initial;
    true
}

unsafe extern "C" fn externalEntityInitProcessor2(
    mut parser: crate::expat_h::XML_Parser,
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut endPtr: *mut *const ::core::ffi::c_char,
) -> crate::expat_h::XML_Error {
    let mut next: *const ::core::ffi::c_char = start;
    let mut tok: ::core::ffi::c_int = (*parser_encoding(parser)).scanners[1 as usize].scan(
        parser_encoding(parser),
        start,
        end,
        &raw mut next,
    );
    match tok {
        crate::src::xmltok::XML_TOK_BOM => {
            if accountingDiffTolerated(
                parser,
                tok,
                start,
                next,
                3208 as ::core::ffi::c_int,
                XML_ACCOUNT_DIRECT,
            ) == 0
            {
                accountingOnAbort(parser);
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
    (*parser).m_processor = ProcessorState::ExternalEntityInit3;
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
    tok = (*parser_encoding(parser)).scanners[1 as usize].scan(
        parser_encoding(parser),
        start,
        end,
        &raw mut next,
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
    (*parser).m_processor = ProcessorState::ExternalEntityContent;
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
        parser_encoding(parser),
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
    if enc == parser_encoding(parser) {
        eventPP = &raw mut (*parser).m_eventPtr;
        eventEndPP = &raw mut (*parser).m_eventEndPtr;
    } else {
        eventPP = &raw mut (*(*parser).m_openInternalEntities).internalEventPtr;
        eventEndPP = &raw mut (*(*parser).m_openInternalEntities).internalEventEndPtr;
    }
    *eventPP = s;
    loop {
        let mut next: *const ::core::ffi::c_char = s;
        let mut tok: ::core::ffi::c_int =
            (*enc).scanners[1 as usize].scan(enc, s, end, &raw mut next);
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
        if accountingDiffTolerated(
            parser,
            tok,
            s,
            accountAfter,
            3337 as ::core::ffi::c_int,
            account,
        ) == 0
        {
            accountingOnAbort(parser);
            return crate::expat_h::XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
        }
        *eventEndPP = next;
        's_1235: {
            match tok {
                crate::src::xmltok::XML_TOK_TRAILING_CR => {
                    if haveMore != 0 {
                        *nextPtr = s;
                        return crate::expat_h::XML_ERROR_NONE;
                    }
                    *eventEndPP = end;
                    if (*parser).m_characterDataHandler {
                        let mut c: crate::expat_external_h::XML_Char =
                            0xa as crate::expat_external_h::XML_Char;
                        callCharacterDataHandler(parser, &raw const c, 1 as ::core::ffi::c_int);
                    } else if (*parser).m_defaultHandler {
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
                    let mut ch: crate::expat_external_h::XML_Char =
                        crate::src::xmltok::predefined_entity_name(
                            enc,
                            s.offset((*enc).minBytesPerChar as isize),
                            next.offset(-((*enc).minBytesPerChar as isize)),
                        ) as crate::expat_external_h::XML_Char;
                    if ch != 0 {
                        accountingDiffTolerated(
                            parser,
                            tok,
                            &raw mut ch as *mut ::core::ffi::c_char,
                            (&raw mut ch as *mut ::core::ffi::c_char).offset(
                                ::core::mem::size_of::<crate::expat_external_h::XML_Char>()
                                    as isize,
                            ),
                            3403 as ::core::ffi::c_int,
                            XML_ACCOUNT_ENTITY_EXPANSION,
                        );
                        if (*parser).m_characterDataHandler {
                            callCharacterDataHandler(
                                parser,
                                &raw const ch,
                                1 as ::core::ffi::c_int,
                            );
                        } else if (*parser).m_defaultHandler {
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
                        (*dtd).pool.rewind();
                        if (*dtd).hasParamEntityRefs == 0
                            || (*dtd).standalone as ::core::ffi::c_int != 0
                        {
                            if entity.is_null() {
                                return crate::expat_h::XML_ERROR_UNDEFINED_ENTITY;
                            } else if (*entity).is_internal == 0 {
                                return crate::expat_h::XML_ERROR_ENTITY_DECLARED_IN_PE;
                            }
                        } else if entity.is_null() {
                            if (*parser).m_skippedEntityHandler {
                                let callback = SKIPPED_ENTITY_HANDLERS
                                    .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
                                    .lock()
                                    .unwrap_or_else(|poisoned| poisoned.into_inner())
                                    .get(&(parser as usize))
                                    .cloned();
                                if let Some(callback) = callback {
                                    callback.invoke(
                                        (*parser).m_handlerArg,
                                        name,
                                        0 as ::core::ffi::c_int,
                                    );
                                }
                            } else if (*parser).m_defaultHandler {
                                reportDefault(parser, enc, s, next);
                            }
                            break 's_1235;
                        }
                        if (*entity).open != 0 {
                            return crate::expat_h::XML_ERROR_RECURSIVE_ENTITY_REF;
                        }
                        if (*entity).notation.is_some() {
                            return crate::expat_h::XML_ERROR_BINARY_ENTITY_REF;
                        }
                        if (*entity).textPtr.is_some() {
                            let mut result: crate::expat_h::XML_Error =
                                crate::expat_h::XML_ERROR_NONE;
                            if (*parser).m_defaultExpandInternalEntities == 0 {
                                if (*parser).m_skippedEntityHandler {
                                    let callback = SKIPPED_ENTITY_HANDLERS
                                        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
                                        .lock()
                                        .unwrap_or_else(|poisoned| poisoned.into_inner())
                                        .get(&(parser as usize))
                                        .cloned();
                                    if let Some(callback) = callback {
                                        callback.invoke(
                                            (*parser).m_handlerArg,
                                            (*entity).named.name,
                                            0 as ::core::ffi::c_int,
                                        );
                                    }
                                } else if (*parser).m_defaultHandler {
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
                        } else if (*parser).m_externalEntityRefHandler {
                            let mut context: *const crate::expat_external_h::XML_Char =
                                ::core::ptr::null::<crate::expat_external_h::XML_Char>();
                            (*entity).open = crate::expat_h::XML_TRUE;
                            context = getContext(parser);
                            (*entity).open = crate::expat_h::XML_FALSE;
                            if context.is_null() {
                                return crate::expat_h::XML_ERROR_NO_MEMORY;
                            }
                            let handler = EXTERNAL_ENTITY_REF_HANDLERS
                                .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
                                .lock()
                                .unwrap_or_else(|poisoned| poisoned.into_inner())
                                .get(&(parser as usize))
                                .cloned()
                                .expect("installed external entity handler");
                            if invoke_external_entity_ref_handler(
                                handler.as_ref(),
                                (*parser).m_externalEntityRefHandlerArg,
                                context,
                                &raw const (*dtd).pool,
                                entity,
                            ) == 0
                            {
                                return crate::expat_h::XML_ERROR_EXTERNAL_ENTITY_HANDLING;
                            }
                            (*parser).m_tempPool.rewind();
                        } else if (*parser).m_defaultHandler {
                            reportDefault(parser, enc, s, next);
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
                            ::core::mem::size_of::<TAG>(),
                            3477 as ::core::ffi::c_int,
                        ) as *mut TAG;
                        if tag.is_null() {
                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                        }
                        (*tag).bufEnd = expat_malloc(
                            parser,
                            32 as crate::__stddef_size_t_h::size_t,
                            3480 as ::core::ffi::c_int,
                        ) as *mut ::core::ffi::c_char;
                        if (*tag).bufEnd.is_null() {
                            expat_free(
                                parser,
                                tag as *mut ::core::ffi::c_void,
                                3482 as ::core::ffi::c_int,
                            );
                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                        }
                        (*tag).bufSize = INIT_TAG_BUF_SIZE as crate::__stddef_size_t_h::size_t;
                    }
                    (*tag).bindings = ::core::ptr::null_mut::<BINDING>();
                    (*tag).parent = (*parser).m_tagStack as *mut tag;
                    (*parser).m_tagStack = tag;
                    (*tag).name.localPart = None;
                    (*tag).rawName = s.offset((*enc).minBytesPerChar as isize);
                    (*tag).rawNameLength = crate::src::xmltok::name_length(enc, (*tag).rawName);
                    (*parser).m_tagLevel += 1;
                    let mut rawNameEnd: *const ::core::ffi::c_char =
                        (*tag).rawName.offset((*tag).rawNameLength as isize);
                    let mut fromPtr: *const ::core::ffi::c_char = (*tag).rawName;
                    toPtr = (*tag).bufEnd as *mut crate::expat_external_h::XML_Char;
                    loop {
                        let mut convLen: ::core::ffi::c_int = 0;
                        let convert_res: crate::src::xmltok::XML_Convert_Result =
                            crate::src::xmltok::convert_to_utf8(
                                enc,
                                &raw mut fromPtr,
                                rawNameEnd,
                                &raw mut toPtr as *mut *mut ::core::ffi::c_char,
                                ((*tag).bufEnd as *mut ICHAR)
                                    .offset((*tag).bufSize as isize)
                                    .offset(-(1 as ::core::ffi::c_int as isize)),
                            );
                        convLen = toPtr.offset_from(
                            (*tag).bufEnd as *mut crate::expat_external_h::XML_Char,
                        ) as ::core::ffi::c_int;
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
                                < (*tag).bufSize
                            {
                                return crate::expat_h::XML_ERROR_NO_MEMORY;
                            }
                            let bufSize: crate::__stddef_size_t_h::size_t =
                                ((*tag).bufSize)
                                    .wrapping_mul(2 as crate::__stddef_size_t_h::size_t);
                            let mut temp: *mut ::core::ffi::c_char = expat_realloc(
                                parser,
                                (*tag).bufEnd as *mut ::core::ffi::c_void,
                                bufSize,
                                3514 as ::core::ffi::c_int,
                            )
                                as *mut ::core::ffi::c_char;
                            if temp.is_null() {
                                return crate::expat_h::XML_ERROR_NO_MEMORY;
                            }
                            (*tag).bufEnd = temp;
                            (*tag).bufSize = bufSize;
                            toPtr = (temp as *mut crate::expat_external_h::XML_Char)
                                .offset(convLen as isize);
                        }
                    }
                    (*tag).name.str = TagNameStorage::TagBuffer { offset: 0 };
                    *toPtr = '\0' as crate::expat_external_h::XML_Char;
                    result_0 = storeAtts(
                        parser,
                        enc,
                        s,
                        &raw mut (*tag).name,
                        tag,
                        &raw mut (*tag).bindings,
                        account,
                    );
                    if result_0 as u64 != 0 {
                        return result_0;
                    }
                    if (*parser).m_startElementHandler {
                        let callback = START_ELEMENT_HANDLERS
                            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
                            .lock()
                            .unwrap_or_else(|poisoned| poisoned.into_inner())
                            .get(&(parser as usize))
                            .cloned();
                        if let Some(callback) = callback {
                            let name = match (*tag).name.str {
                                TagNameStorage::TagBuffer { offset } => ((*tag).bufEnd
                                    as *const crate::expat_external_h::XML_Char)
                                    .offset(offset as isize),
                                TagNameStorage::NamespaceUri => namespace_name_pointer(
                                    parser,
                                    (*tag).bufEnd as *const crate::expat_external_h::XML_Char,
                                ),
                                _ => return crate::expat_h::XML_ERROR_UNEXPECTED_STATE,
                            };
                            callback.invoke(
                                (*parser).m_handlerArg,
                                name,
                                (*parser).m_atts as *mut *const crate::expat_external_h::XML_Char,
                            );
                        }
                    } else if (*parser).m_defaultHandler {
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
                        str: TagNameStorage::Unset,
                        localPart: None,
                        strLen: 0,
                        uriLen: 0,
                        prefixLen: 0,
                    };
                    let name_string = poolStoreString(
                        &raw mut (*parser).m_tempPool,
                        enc,
                        rawName,
                        rawName.offset(crate::src::xmltok::name_length(enc, rawName) as isize),
                    );
                    if name_string.is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    let Some(name_ref) = (*parser).m_tempPool.start_ref(false) else {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    };
                    name_0.str = TagNameStorage::TempPool(name_ref);
                    let raw_name_pointer = (*parser)
                        .m_tempPool
                        .chars_from(name_ref)
                        .map_or(::core::ptr::null(), |chars| chars.as_ptr());
                    (*parser).m_tempPool.commit();
                    result_1 = storeAtts(
                        parser,
                        enc,
                        s,
                        &raw mut name_0,
                        ::core::ptr::null_mut(),
                        &raw mut bindings,
                        XML_ACCOUNT_NONE,
                    );
                    if result_1 as ::core::ffi::c_uint
                        != crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int
                            as ::core::ffi::c_uint
                    {
                        freeBindings(parser, bindings);
                        return result_1;
                    }
                    let name_pointer = match name_0.str {
                        TagNameStorage::TempPool(name) => (*parser)
                            .m_tempPool
                            .chars_from(name)
                            .map_or(::core::ptr::null(), |chars| chars.as_ptr()),
                        TagNameStorage::NamespaceUri => {
                            namespace_name_pointer(parser, raw_name_pointer)
                        }
                        _ => ::core::ptr::null(),
                    };
                    (*parser).m_tempPool.commit();
                    if (*parser).m_startElementHandler {
                        let callback = START_ELEMENT_HANDLERS
                            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
                            .lock()
                            .unwrap_or_else(|poisoned| poisoned.into_inner())
                            .get(&(parser as usize))
                            .cloned();
                        if let Some(callback) = callback {
                            callback.invoke(
                                (*parser).m_handlerArg,
                                name_pointer,
                                (*parser).m_atts as *mut *const crate::expat_external_h::XML_Char,
                            );
                        }
                        noElmHandlers = crate::expat_h::XML_FALSE;
                    }
                    if (*parser).m_endElementHandler {
                        if (*parser).m_startElementHandler {
                            *eventPP = *eventEndPP;
                        }
                        let callback = END_ELEMENT_HANDLERS
                            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
                            .lock()
                            .unwrap_or_else(|poisoned| poisoned.into_inner())
                            .get(&(parser as usize))
                            .cloned();
                        if let Some(callback) = callback {
                            callback.invoke(
                                (*parser).m_handlerArg,
                                name_pointer,
                            );
                        }
                        noElmHandlers = crate::expat_h::XML_FALSE;
                    }
                    if noElmHandlers as ::core::ffi::c_int != 0 && (*parser).m_defaultHandler {
                        reportDefault(parser, enc, s, next);
                    }
                    poolClear(&raw mut (*parser).m_tempPool);
                    freeBindings(parser, bindings);
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
                            (*parser).m_processor = ProcessorState::Epilog;
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
                        len = crate::src::xmltok::name_length(enc, rawName_0);
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
                        if (*parser).m_endElementHandler {
                            let mut localPart: *const crate::expat_external_h::XML_Char =
                                ::core::ptr::null::<crate::expat_external_h::XML_Char>();
                            let mut prefix: *const crate::expat_external_h::XML_Char =
                                ::core::ptr::null::<crate::expat_external_h::XML_Char>();
                            let mut uri: *mut crate::expat_external_h::XML_Char =
                                ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
                            let name = match (*tag_0).name.str {
                                TagNameStorage::TagBuffer { offset } => ((*tag_0).bufEnd
                                    as *const crate::expat_external_h::XML_Char)
                                    .offset(offset as isize),
                                TagNameStorage::NamespaceUri => {
                                    namespace_name_pointer(
                                        parser,
                                        (*tag_0).bufEnd
                                            as *const crate::expat_external_h::XML_Char,
                                    )
                                }
                                _ => return crate::expat_h::XML_ERROR_UNEXPECTED_STATE,
                            };
                            if let Some(localPartOffset) = (*tag_0).name.localPart {
                                localPart = ((*tag_0).bufEnd
                                    as *const crate::expat_external_h::XML_Char)
                                    .offset(localPartOffset as isize);
                            }
                            if (*parser).m_ns as ::core::ffi::c_int != 0 && !localPart.is_null() {
                                uri = (name as *mut crate::expat_external_h::XML_Char)
                                    .offset((*tag_0).name.uriLen as isize);
                                while *localPart != 0 {
                                    let c2rust_fresh18 = localPart;
                                    localPart = localPart.offset(1);
                                    let c2rust_fresh19 = uri;
                                    uri = uri.offset(1);
                                    *c2rust_fresh19 = *c2rust_fresh18;
                                }
                                if (*parser).m_ns_triplets as ::core::ffi::c_int != 0
                                    && (*tag_0).name.prefixLen != 0
                                {
                                    // The tag buffer remains owned by the parser's configured
                                    // allocator for the tag's complete lifetime.  Its original
                                    // converted name supplies the prefix, so no interior pointer
                                    // has to be retained in TAG_NAME.
                                    prefix = (*tag_0).bufEnd
                                        as *const crate::expat_external_h::XML_Char;
                                    let c2rust_fresh20 = uri;
                                    uri = uri.offset(1);
                                    *c2rust_fresh20 = (*parser).m_namespaceSeparator;
                                    // `prefixLen` includes the terminating NUL.  The source
                                    // buffer also holds the local part after the colon, so use
                                    // the recorded bound rather than searching for a NUL there.
                                    let mut prefix_remaining = (*tag_0).name.prefixLen - 1;
                                    while prefix_remaining != 0 {
                                        let c2rust_fresh21 = prefix;
                                        prefix = prefix.offset(1);
                                        let c2rust_fresh22 = uri;
                                        uri = uri.offset(1);
                                        *c2rust_fresh22 = *c2rust_fresh21;
                                        prefix_remaining -= 1;
                                    }
                                }
                                *uri = '\0' as crate::expat_external_h::XML_Char;
                            }
                            let callback = END_ELEMENT_HANDLERS
                                .get_or_init(|| {
                                    std::sync::Mutex::new(std::collections::HashMap::new())
                                })
                                .lock()
                                .unwrap_or_else(|poisoned| poisoned.into_inner())
                                .get(&(parser as usize))
                                .cloned();
                            if let Some(callback) = callback {
                                callback.invoke(
                                    (*parser).m_handlerArg,
                                    name,
                                );
                            }
                        } else if (*parser).m_defaultHandler {
                            reportDefault(parser, enc, s, next);
                        }
                        while !(*tag_0).bindings.is_null() {
                            let mut b: *mut BINDING = (*tag_0).bindings;
                            if (*parser).m_endNamespaceDeclHandler {
                                let callback = END_NAMESPACE_DECL_HANDLERS
                                    .get_or_init(|| {
                                        std::sync::Mutex::new(std::collections::HashMap::new())
                                    })
                                    .lock()
                                    .unwrap_or_else(|poisoned| poisoned.into_inner())
                                    .get(&(parser as usize))
                                    .cloned();
                                if let Some(callback) = callback {
                                    callback.invoke((*parser).m_handlerArg, (*(*b).prefix).name);
                                }
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
                                (*parser).m_processor = ProcessorState::Epilog;
                            } else {
                                return epilogProcessor(parser, next, end, nextPtr);
                            }
                        }
                    }
                }
                crate::src::xmltok::XML_TOK_CHAR_REF => {
                    let mut n: ::core::ffi::c_int = (*enc).charRefNumber.decode(enc, s);
                    if n < 0 as ::core::ffi::c_int {
                        return crate::expat_h::XML_ERROR_BAD_CHAR_REF;
                    }
                    if (*parser).m_characterDataHandler {
                        let mut buf: [crate::expat_external_h::XML_Char; 4] = [0; 4];
                        callCharacterDataHandler(
                            parser,
                            buf.as_ptr(),
                            crate::src::xmltok::XmlUtf8Encode(
                                n,
                                &raw mut buf as *mut crate::expat_external_h::XML_Char
                                    as *mut ::core::ffi::c_char,
                            ),
                        );
                    } else if (*parser).m_defaultHandler {
                        reportDefault(parser, enc, s, next);
                    }
                }
                crate::src::xmltok::XML_TOK_XML_DECL => {
                    return crate::expat_h::XML_ERROR_MISPLACED_XML_PI
                }
                crate::src::xmltok::XML_TOK_DATA_NEWLINE => {
                    if (*parser).m_characterDataHandler {
                        let mut c_0: crate::expat_external_h::XML_Char =
                            0xa as crate::expat_external_h::XML_Char;
                        callCharacterDataHandler(parser, &raw const c_0, 1 as ::core::ffi::c_int);
                    } else if (*parser).m_defaultHandler {
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
                    } else if false && (*parser).m_characterDataHandler {
                        callCharacterDataHandler(
                            parser,
                            (*parser).m_dataBuf,
                            0 as ::core::ffi::c_int,
                        );
                    } else if (*parser).m_defaultHandler {
                        reportDefault(parser, enc, s, next);
                    }
                    result_2 =
                        doCdataSection(parser, enc, &mut next, end, nextPtr, haveMore, account);
                    if result_2 as ::core::ffi::c_uint
                        != crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int
                            as ::core::ffi::c_uint
                    {
                        return result_2;
                    } else if next.is_null() {
                        (*parser).m_processor = ProcessorState::CdataSection;
                        return result_2;
                    }
                }
                crate::src::xmltok::XML_TOK_TRAILING_RSQB => {
                    if haveMore != 0 {
                        *nextPtr = s;
                        return crate::expat_h::XML_ERROR_NONE;
                    }
                    if (*parser).m_characterDataHandler {
                        if (*enc).isUtf8 == 0 {
                            let mut dataPtr: *mut ICHAR = (*parser).m_dataBuf as *mut ICHAR;
                            crate::src::xmltok::convert_to_utf8(
                                enc,
                                &raw mut s,
                                end,
                                &raw mut dataPtr,
                                (*parser).m_dataBufEnd as *mut ICHAR,
                            );
                            callCharacterDataHandler(
                                parser,
                                (*parser).m_dataBuf,
                                dataPtr.offset_from((*parser).m_dataBuf as *mut ICHAR)
                                    as ::core::ffi::c_int,
                            );
                        } else {
                            callCharacterDataHandler(
                                parser,
                                s as *const crate::expat_external_h::XML_Char,
                                (end as *const crate::expat_external_h::XML_Char)
                                    .offset_from(s as *const crate::expat_external_h::XML_Char)
                                    as ::core::ffi::c_int,
                            );
                        }
                    } else if (*parser).m_defaultHandler {
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
                    let charDataHandler = CHARACTER_DATA_HANDLERS
                        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
                        .lock()
                        .unwrap_or_else(|poisoned| poisoned.into_inner())
                        .get(&(parser as usize))
                        .cloned();
                    if let Some(charDataHandler) = charDataHandler {
                        if (*enc).isUtf8 == 0 {
                            loop {
                                let mut dataPtr_0: *mut ICHAR = (*parser).m_dataBuf as *mut ICHAR;
                                let convert_res_0: crate::src::xmltok::XML_Convert_Result =
                                    crate::src::xmltok::convert_to_utf8(
                                        enc,
                                        &raw mut s,
                                        next,
                                        &raw mut dataPtr_0,
                                        (*parser).m_dataBufEnd as *mut ICHAR,
                                    );
                                *eventEndPP = s;
                                charDataHandler.invoke(
                                    (*parser).m_handlerArg,
                                    (*parser).m_dataBuf,
                                    dataPtr_0.offset_from((*parser).m_dataBuf as *mut ICHAR)
                                        as ::core::ffi::c_int,
                                );
                                if convert_res_0 as ::core::ffi::c_uint
                                    == crate::src::xmltok::XML_CONVERT_COMPLETED
                                        as ::core::ffi::c_int
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
                            charDataHandler.invoke(
                                (*parser).m_handlerArg,
                                s as *const crate::expat_external_h::XML_Char,
                                (next as *const crate::expat_external_h::XML_Char)
                                    .offset_from(s as *const crate::expat_external_h::XML_Char)
                                    as ::core::ffi::c_int,
                            );
                        }
                    } else if (*parser).m_defaultHandler {
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
                    if (*parser).m_defaultHandler {
                        reportDefault(parser, enc, s, next);
                    }
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
        if (*parser).m_endNamespaceDeclHandler {
            let callback = END_NAMESPACE_DECL_HANDLERS
                .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .get(&(parser as usize))
                .cloned();
            if let Some(callback) = callback {
                callback.invoke((*parser).m_handlerArg, (*(*b).prefix).name);
            }
        }
        bindings = (*bindings).nextTagBinding as *mut BINDING;
        (*b).nextTagBinding = (*parser).m_freeBindingList as *mut binding;
        (*parser).m_freeBindingList = b;
        (*(*b).prefix).binding = (*b).prevPrefixBinding as *mut BINDING;
    }
}

unsafe fn namespace_name_pointer(
    parser: crate::expat_h::XML_Parser,
    name: *const crate::expat_external_h::XML_Char,
) -> *const crate::expat_external_h::XML_Char {
    let dtd = (*parser).m_dtd;
    if dtd.is_null() {
        return ::core::ptr::null();
    }
    let dtd_ref = &mut *dtd;
    let element = lookup(
        parser,
        &raw mut dtd_ref.elementTypes,
        name as KEY,
        0,
    ) as *mut ELEMENT_TYPE;
    if element.is_null() {
        return ::core::ptr::null();
    }
    let element_ref = &*element;
    let binding = if element_ref.hasPrefix != 0 {
        let prefix_name = pool_string_pointer(&raw const dtd_ref.pool, element_ref.prefix);
        if prefix_name.is_null() {
            return ::core::ptr::null();
        }
        let prefix = lookup(parser, &raw mut dtd_ref.prefixes, prefix_name as KEY, 0) as *mut PREFIX;
        if prefix.is_null() {
            return ::core::ptr::null();
        }
        (*prefix).binding
    } else {
        dtd_ref.defaultPrefix.binding
    };
    if binding.is_null() {
        ::core::ptr::null()
    } else {
        (*binding).uri
    }
}

unsafe extern "C" fn storeAtts(
    mut parser: crate::expat_h::XML_Parser,
    mut enc: *const crate::src::xmltok::ENCODING,
    mut attStr: *const ::core::ffi::c_char,
    mut tagNamePtr: *mut TAG_NAME,
    mut tagPtr: *mut TAG,
    mut bindingsPtr: *mut *mut BINDING,
    mut account: XML_Account,
) -> crate::expat_h::XML_Error {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut elementType: *mut ELEMENT_TYPE = ::core::ptr::null_mut::<ELEMENT_TYPE>();
    let mut nDefaultAtts: ::core::ffi::c_int = 0;
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
    let mut localPartOffset: usize = 0;
    let tag_name_state = &mut *tagNamePtr;
    let tag_name = match tag_name_state.str {
        TagNameStorage::TagBuffer { offset } if !tagPtr.is_null() => {
            ((*tagPtr).bufEnd as *const crate::expat_external_h::XML_Char).offset(offset as isize)
        }
        TagNameStorage::TempPool(name) => (*parser)
            .m_tempPool
            .chars_from(name)
            .map_or(::core::ptr::null(), |chars| chars.as_ptr()),
        _ => return crate::expat_h::XML_ERROR_NO_MEMORY,
    };
    if tag_name.is_null() {
        return crate::expat_h::XML_ERROR_NO_MEMORY;
    }
    elementType = lookup(
        parser,
        &raw mut (*dtd).elementTypes,
        tag_name as KEY,
        0 as crate::__stddef_size_t_h::size_t,
    ) as *mut ELEMENT_TYPE;
    if elementType.is_null() {
        let mut name: *const crate::expat_external_h::XML_Char =
            poolCopyString(&raw mut (*dtd).pool, tag_name);
        if name.is_null() {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
        elementType = lookup(
            parser,
            &raw mut (*dtd).elementTypes,
            name as KEY,
            ::core::mem::size_of::<ELEMENT_TYPE>(),
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
    let eventEnd = (*parser).m_eventEndPtr;
    n = match (*enc).getAtts {
        crate::src::xmltok::AttributeScanner::Normal => {
            crate::src::xmltok::normal_getAtts(
                enc,
                attStr,
                eventEnd,
                (*parser).m_attsSize,
                (*parser).m_atts,
            )
        }
        crate::src::xmltok::AttributeScanner::Little2 => crate::src::xmltok::little2_getAtts(
            enc,
            attStr,
            eventEnd,
            (*parser).m_attsSize,
            (*parser).m_atts,
        ),
        crate::src::xmltok::AttributeScanner::Big2 => {
            crate::src::xmltok::big2_getAtts(enc, attStr, (*parser).m_attsSize, (*parser).m_atts)
        }
    };
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
                .wrapping_mul(::core::mem::size_of::<crate::src::xmltok::ATTRIBUTE>()),
            3894 as ::core::ffi::c_int,
        ) as *mut crate::src::xmltok::ATTRIBUTE;
        if temp.is_null() {
            (*parser).m_attsSize = oldAttsSize;
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
        (*parser).m_atts = temp;
        if n > oldAttsSize {
            match (*enc).getAtts {
                crate::src::xmltok::AttributeScanner::Normal => {
                    crate::src::xmltok::normal_getAtts(
                        enc,
                        attStr,
                        eventEnd,
                        n,
                        (*parser).m_atts,
                    );
                }
                crate::src::xmltok::AttributeScanner::Little2 => {
                    crate::src::xmltok::little2_getAtts(
                        enc,
                        attStr,
                        eventEnd,
                        n,
                        (*parser).m_atts,
                    );
                }
                crate::src::xmltok::AttributeScanner::Big2 => {
                    crate::src::xmltok::big2_getAtts(enc, attStr, n, (*parser).m_atts);
                }
            }
        }
    }
    // `m_atts` is allocated (and, above, resized) as exactly `m_attsSize`
    // ATTRIBUTE records.  Expat intentionally reuses this allocation as the
    // callback-facing name/value array, so keep it under the configured
    // allocator rather than replacing it with a Vec.  Each ATTRIBUTE occupies
    // an integral number of pointer slots, which bounds every name/value pair
    // written below.
    if (*parser).m_atts.is_null() || (*parser).m_attsSize <= 0 {
        return crate::expat_h::XML_ERROR_NO_MEMORY;
    }
    let appAttsLen = match ((*parser).m_attsSize as usize).checked_mul(
        ::core::mem::size_of::<crate::src::xmltok::ATTRIBUTE>()
            / ::core::mem::size_of::<*const crate::expat_external_h::XML_Char>(),
    ) {
        Some(len) => len,
        None => return crate::expat_h::XML_ERROR_NO_MEMORY,
    };
    let appAtts = ::core::slice::from_raw_parts_mut(
        (*parser).m_atts as *mut *const crate::expat_external_h::XML_Char,
        appAttsLen,
    );
    i = 0 as ::core::ffi::c_int;
    while i < n {
        // The scanner has completed before the name/value view is formed.
        // Copy this record before its storage is reused by `appAtts`.
        let currAtt = ::core::ptr::read((*parser).m_atts.add(i as usize));
        let mut attId: *mut ATTRIBUTE_ID = getAttributeId(
            parser,
            enc,
            currAtt.name,
            currAtt
                .name
                .offset(crate::src::xmltok::name_length(enc, currAtt.name) as isize),
        );
        if attId.is_null() {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
        if *(*attId).name.offset(-1 as isize) != 0 {
            if enc == parser_encoding(parser) {
                (*parser).m_eventPtr = currAtt.name;
            }
            return crate::expat_h::XML_ERROR_DUPLICATE_ATTRIBUTE;
        }
        *(*attId).name.offset(-1 as isize) = 1 as crate::expat_external_h::XML_Char;
        let c2rust_fresh23 = attIndex;
        attIndex = attIndex + 1;
        appAtts[c2rust_fresh23 as usize] = (*attId).name;
        if currAtt.normalized == 0 {
            let mut result: crate::expat_h::XML_Error = crate::expat_h::XML_ERROR_NONE;
            let mut isCdata: crate::expat_h::XML_Bool = crate::expat_h::XML_TRUE;
            if (*attId).maybeTokenized != 0 {
                let mut j: ::core::ffi::c_int = 0;
                j = 0 as ::core::ffi::c_int;
                while j < nDefaultAtts {
                    let default_att = &*(*elementType)
                        .defaultAtts
                        .expect("default attribute storage must exist for a non-empty list")
                        .as_ptr()
                        .offset(j as isize);
                    let default_name = default_att.id.map_or(::core::ptr::null(), |name| {
                        (*dtd)
                            .pool
                            .chars_from(name)
                            .map_or(::core::ptr::null(), |chars| chars.as_ptr())
                    });
                    if default_name == (*attId).name {
                        isCdata = (*(*elementType)
                            .defaultAtts
                            .expect("default attribute storage must exist for a non-empty list")
                            .as_ptr()
                            .offset(j as isize))
                        .isCdata;
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
                currAtt.valuePtr,
                currAtt.valueEnd,
                &raw mut (*parser).m_tempPool,
                account,
            );
            if result as u64 != 0 {
                return result;
            }
            let temp_pool = &mut (*parser).m_tempPool;
            let Some(start) = temp_pool.start_ref(true) else {
                return crate::expat_h::XML_ERROR_NO_MEMORY;
            };
            appAtts[attIndex as usize] = temp_pool
                .chars_from(start)
                .map_or(::core::ptr::null(), |chars| chars.as_ptr());
            (*parser).m_tempPool.commit();
        } else {
            appAtts[attIndex as usize] = poolStoreString(
                &raw mut (*parser).m_tempPool,
                enc,
                currAtt.valuePtr,
                currAtt.valueEnd,
            );
            if appAtts[attIndex as usize].is_null() {
                return crate::expat_h::XML_ERROR_NO_MEMORY;
            }
            (*parser).m_tempPool.commit();
        }
        if !(*attId).prefix.is_null() {
            if (*attId).xmlns != 0 {
                let mut result_0: crate::expat_h::XML_Error = addBinding(
                    parser,
                    (*attId).prefix,
                    attId,
                    appAtts[attIndex as usize],
                    bindingsPtr,
                );
                if result_0 as u64 != 0 {
                    return result_0;
                }
                attIndex -= 1;
            } else {
                attIndex += 1;
                nPrefixes += 1;
                *(*attId).name.offset(-1 as isize) = 2 as crate::expat_external_h::XML_Char;
            }
        } else {
            attIndex += 1;
        }
        i += 1;
    }
    (*parser).m_nSpecifiedAtts = attIndex;
    if !(*elementType).idAtt.is_null()
        && *(*(*elementType).idAtt).name.offset(-1 as isize) as ::core::ffi::c_int != 0
    {
        i = 0 as ::core::ffi::c_int;
        while i < attIndex {
            if appAtts[i as usize]
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
        let mut da: *const DEFAULT_ATTRIBUTE = (*elementType)
            .defaultAtts
            .expect("default attribute storage must exist for a non-empty list")
            .as_ptr()
            .offset(i as isize);
        let Some(id_name_ref) = (*da).id else {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        };
        let id_name = (*dtd)
            .pool
            .chars_from(id_name_ref)
            .map_or(::core::ptr::null(), |chars| chars.as_ptr());
        if id_name.is_null() {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
        let id = lookup(
            parser,
            &raw mut (*dtd).attributeIds,
            id_name as KEY,
            0 as crate::__stddef_size_t_h::size_t,
        ) as *mut ATTRIBUTE_ID;
        if id.is_null() {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
        if *(*id).name.offset(-1 as isize) == 0 && (*da).value.is_some() {
            let value = (*da)
                .value
                .expect("a present default attribute value is non-null")
                .as_ptr() as *const crate::expat_external_h::XML_Char;
            if !(*id).prefix.is_null() {
                if (*id).xmlns != 0 {
                    let mut result_1: crate::expat_h::XML_Error = addBinding(
                        parser,
                        (*id).prefix,
                        id,
                        value,
                        bindingsPtr,
                    );
                    if result_1 as u64 != 0 {
                        return result_1;
                    }
                } else {
                    *(*id).name.offset(-1 as isize) = 2 as crate::expat_external_h::XML_Char;
                    nPrefixes += 1;
                    let c2rust_fresh24 = attIndex;
                    attIndex = attIndex + 1;
                    appAtts[c2rust_fresh24 as usize] = (*id).name;
                    let c2rust_fresh25 = attIndex;
                    attIndex = attIndex + 1;
                    appAtts[c2rust_fresh25 as usize] = value;
                }
            } else {
                *(*id).name.offset(-1 as isize) = 1 as crate::expat_external_h::XML_Char;
                let c2rust_fresh26 = attIndex;
                attIndex = attIndex + 1;
                appAtts[c2rust_fresh26 as usize] = (*id).name;
                let c2rust_fresh27 = attIndex;
                attIndex = attIndex + 1;
                appAtts[c2rust_fresh27 as usize] = value;
            }
        }
        i += 1;
    }
    appAtts[attIndex as usize] = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    i = 0 as ::core::ffi::c_int;
    if nPrefixes != 0 {
        let mut j_0: ::core::ffi::c_uint = 0;
        let mut version: ::core::ffi::c_ulong = (*parser).m_nsAttsVersion;
        if (*parser).m_nsAttsPower as usize
            >= ::core::mem::size_of::<::core::ffi::c_uint>().wrapping_mul(8 as usize)
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
                let c2rust_fresh28 = (*parser).m_nsAttsPower;
                (*parser).m_nsAttsPower = (*parser).m_nsAttsPower.wrapping_add(1);
                if nPrefixes >> c2rust_fresh28 as ::core::ffi::c_int == 0 {
                    break;
                }
            }
            if ((*parser).m_nsAttsPower as ::core::ffi::c_int) < 3 as ::core::ffi::c_int {
                (*parser).m_nsAttsPower = 3 as ::core::ffi::c_uchar;
            }
            if (*parser).m_nsAttsPower as usize
                >= ::core::mem::size_of::<::core::ffi::c_uint>().wrapping_mul(8 as usize)
            {
                (*parser).m_nsAttsPower = oldNsAttsPower;
                return crate::expat_h::XML_ERROR_NO_MEMORY;
            }
            nsAttsSize =
                (1 as ::core::ffi::c_uint) << (*parser).m_nsAttsPower as ::core::ffi::c_int;
            temp_0 = expat_realloc(
                parser,
                (*parser).m_nsAtts as *mut ::core::ffi::c_void,
                (nsAttsSize as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(::core::mem::size_of::<NS_ATT>()),
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
        let parser_ref = &mut *parser;
        while i < attIndex {
            let mut s: *const crate::expat_external_h::XML_Char = appAtts[i as usize];
            if *s.offset(-1 as isize) as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
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
                let mut sip_key: crate::siphash_h::sipkey = crate::siphash_h::sipkey {
                    k: [0, get_hash_secret_salt(parser) as crate::stdlib::uint64_t],
                };
                sip24_init(&raw mut sip_state, &raw mut sip_key);
                *(s as *mut crate::expat_external_h::XML_Char).offset(-1 as isize) =
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
                    if if (*parser).m_tempPool.is_full()
                        && poolGrow(&raw mut (*parser).m_tempPool) == 0
                    {
                        0 as ::core::ffi::c_int
                    } else {
                        if (*parser).m_tempPool.write_cursor(c) {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        }
                    } == 0
                    {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    j_0 = j_0.wrapping_add(1);
                }
                sip24_update(
                    &raw mut sip_state,
                    (*b).uri as *const ::core::ffi::c_void,
                    ((*b).uriLen as crate::__stddef_size_t_h::size_t)
                        .wrapping_mul(::core::mem::size_of::<crate::expat_external_h::XML_Char>()),
                );
                loop {
                    let c2rust_fresh30 = s;
                    s = s.offset(1);
                    if *c2rust_fresh30 as ::core::ffi::c_int == 0x3a as ::core::ffi::c_int {
                        break;
                    }
                }
                sip24_update(
                    &raw mut sip_state,
                    s as *const ::core::ffi::c_void,
                    keylen(s as KEY)
                        .wrapping_mul(::core::mem::size_of::<crate::expat_external_h::XML_Char>()),
                );
                loop {
                    if if (*parser).m_tempPool.is_full()
                        && poolGrow(&raw mut (*parser).m_tempPool) == 0
                    {
                        0 as ::core::ffi::c_int
                    } else {
                        if (*parser).m_tempPool.write_cursor(*s) {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        }
                    } == 0
                    {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    let c2rust_fresh32 = s;
                    s = s.offset(1);
                    if *c2rust_fresh32 == 0 {
                        break;
                    }
                }
                uriHash = sip24_final(&raw mut sip_state) as ::core::ffi::c_ulong;
                let mut step: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                let mut mask: ::core::ffi::c_ulong =
                    nsAttsSize.wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_ulong;
                j_0 = (uriHash & mask) as ::core::ffi::c_uint;
                while (*parser_ref.m_nsAtts.offset(j_0 as isize)).version == version {
                    if uriHash == (*parser_ref.m_nsAtts.offset(j_0 as isize)).hash {
                        let Some(start) = parser_ref.m_tempPool.start_ref(true) else {
                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                        };
                        let mut s1 = parser_ref
                            .m_tempPool
                            .chars_from(start)
                            .map_or(::core::ptr::null(), |chars| chars.as_ptr());
                        if s1.is_null() {
                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                        }
                        let Some(uri_name) = (*parser_ref.m_nsAtts.offset(j_0 as isize)).uriName
                        else {
                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                        };
                        let s2 = pool_string_pointer(&raw const parser_ref.m_tempPool, uri_name);
                        if s2.is_null() {
                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                        }
                        let mut s2 = s2;
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
                            >> parser_ref.m_nsAttsPower as ::core::ffi::c_int
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
                if parser_ref.m_ns_triplets != 0 {
                    if !parser_ref
                        .m_tempPool
                        .replace_last_cursor_char(parser_ref.m_namespaceSeparator)
                    {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    s = (*(*b).prefix).name;
                    loop {
                        if if parser_ref.m_tempPool.is_full()
                            && poolGrow(&raw mut parser_ref.m_tempPool) == 0
                        {
                            0 as ::core::ffi::c_int
                        } else {
                            if parser_ref.m_tempPool.write_cursor(*s) {
                                1 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            }
                        } == 0
                        {
                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                        }
                        let c2rust_fresh34 = s;
                        s = s.offset(1);
                        if *c2rust_fresh34 == 0 {
                            break;
                        }
                    }
                }
                let Some(start) = parser_ref.m_tempPool.start_ref(true) else {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                };
                s = parser_ref
                    .m_tempPool
                    .chars_from(start)
                    .map_or(::core::ptr::null(), |chars| chars.as_ptr());
                if s.is_null() {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
                parser_ref.m_tempPool.commit();
                appAtts[i as usize] = s;
                (*parser_ref.m_nsAtts.offset(j_0 as isize)).version = version;
                (*parser_ref.m_nsAtts.offset(j_0 as isize)).hash = uriHash;
                let Some(uri_name) = pool_string_ref(&raw const parser_ref.m_tempPool, s, false) else {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                };
                (*parser_ref.m_nsAtts.offset(j_0 as isize)).uriName = Some(uri_name);
                nPrefixes -= 1;
                if nPrefixes == 0 {
                    i += 2 as ::core::ffi::c_int;
                    break;
                }
            } else {
                *(s as *mut crate::expat_external_h::XML_Char).offset(-1 as isize) =
                    0 as crate::expat_external_h::XML_Char;
            }
            i += 2 as ::core::ffi::c_int;
        }
    }
    while i < attIndex {
        *(appAtts[i as usize] as *mut crate::expat_external_h::XML_Char)
            .offset(-1 as isize) = 0 as crate::expat_external_h::XML_Char;
        i += 2 as ::core::ffi::c_int;
    }
    binding = *bindingsPtr;
    while !binding.is_null() {
        let binding_ref = &*binding;
        *(*binding_ref.attId).name.offset(-1 as isize) = 0 as crate::expat_external_h::XML_Char;
        binding = binding_ref.nextTagBinding as *mut BINDING;
    }
    if (*parser).m_ns == 0 {
        return crate::expat_h::XML_ERROR_NONE;
    }
    if (*elementType).hasPrefix != 0 {
        let prefix_name = pool_string_pointer(&raw const (*dtd).pool, (*elementType).prefix);
        if prefix_name.is_null() {
            return crate::expat_h::XML_ERROR_UNBOUND_PREFIX;
        }
        let prefix = lookup(
            parser,
            &raw mut (*dtd).prefixes,
            prefix_name as KEY,
            0 as crate::__stddef_size_t_h::size_t,
        ) as *mut PREFIX;
        if prefix.is_null() {
            return crate::expat_h::XML_ERROR_UNBOUND_PREFIX;
        }
        binding = (*prefix).binding;
        if binding.is_null() {
            return crate::expat_h::XML_ERROR_UNBOUND_PREFIX;
        }
        localPart = tag_name;
        loop {
            let c2rust_fresh35 = localPart;
            localPart = localPart.offset(1);
            localPartOffset += 1;
            if *c2rust_fresh35 as ::core::ffi::c_int == 0x3a as ::core::ffi::c_int {
                break;
            }
        }
    } else if !(*dtd).defaultPrefix.binding.is_null() {
        binding = (*dtd).defaultPrefix.binding;
        localPart = tag_name;
    } else {
        return crate::expat_h::XML_ERROR_NONE;
    }
    let binding_ref = &mut *binding;
    prefixLen = 0 as ::core::ffi::c_int;
    if (*parser).m_ns_triplets as ::core::ffi::c_int != 0 && !(*binding_ref.prefix).name.is_null() {
        loop {
            let c2rust_fresh36 = prefixLen;
            prefixLen = prefixLen + 1;
            if *(*binding_ref.prefix).name.offset(c2rust_fresh36 as isize) == 0 {
                break;
            }
        }
    }
    tag_name_state.localPart = Some(localPartOffset);
    tag_name_state.uriLen = binding_ref.uriLen;
    tag_name_state.prefixLen = prefixLen;
    i = 0 as ::core::ffi::c_int;
    loop {
        let c2rust_fresh37 = i;
        i = i + 1;
        if *localPart.offset(c2rust_fresh37 as isize) == 0 {
            break;
        }
    }
    if binding_ref.uriLen > crate::limits_h::INT_MAX - prefixLen
        || i > crate::limits_h::INT_MAX - (binding_ref.uriLen + prefixLen)
    {
        return crate::expat_h::XML_ERROR_NO_MEMORY;
    }
    n = i + binding_ref.uriLen + prefixLen;
    if n > binding_ref.uriAlloc {
        if n > crate::limits_h::INT_MAX - EXPAND_SPARE {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
        uri = expat_malloc(
            parser,
            ((n + 24 as ::core::ffi::c_int) as crate::__stddef_size_t_h::size_t)
                .wrapping_mul(::core::mem::size_of::<crate::expat_external_h::XML_Char>()),
            4270 as ::core::ffi::c_int,
        ) as *mut crate::expat_external_h::XML_Char;
        if uri.is_null() {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
        binding_ref.uriAlloc = n + EXPAND_SPARE;
        crate::stdlib::memcpy(
            uri as *mut ::core::ffi::c_void,
            binding_ref.uri as *const ::core::ffi::c_void,
            (binding_ref.uriLen as crate::__stddef_size_t_h::size_t)
                .wrapping_mul(::core::mem::size_of::<crate::expat_external_h::XML_Char>()),
        );
        expat_free(
            parser,
            binding_ref.uri as *mut ::core::ffi::c_void,
            4278 as ::core::ffi::c_int,
        );
        binding_ref.uri = uri;
    }
    uri = binding_ref.uri.offset(binding_ref.uriLen as isize);
    crate::stdlib::memcpy(
        uri as *mut ::core::ffi::c_void,
        localPart as *const ::core::ffi::c_void,
        (i as crate::__stddef_size_t_h::size_t)
            .wrapping_mul(::core::mem::size_of::<crate::expat_external_h::XML_Char>()),
    );
    if prefixLen != 0 {
        uri = uri.offset((i - 1 as ::core::ffi::c_int) as isize);
        *uri = (*parser).m_namespaceSeparator;
        crate::stdlib::memcpy(
            uri.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            (*binding_ref.prefix).name as *const ::core::ffi::c_void,
            (prefixLen as crate::__stddef_size_t_h::size_t)
                .wrapping_mul(::core::mem::size_of::<crate::expat_external_h::XML_Char>()),
        );
    }
    tag_name_state.str = TagNameStorage::NamespaceUri;
    return crate::expat_h::XML_ERROR_NONE;
}

fn is_rfc3986_uri_char(
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
    const XML_NAMESPACE: [crate::expat_external_h::XML_Char; 37] = [
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
        '\0' as crate::expat_external_h::XML_Char,
    ];
    const XMLNS_NAMESPACE: [crate::expat_external_h::XML_Char; 30] = [
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
        '\0' as crate::expat_external_h::XML_Char,
    ];
    let mut mustBeXML: crate::expat_h::XML_Bool = crate::expat_h::XML_FALSE;
    let mut isXML: crate::expat_h::XML_Bool = crate::expat_h::XML_TRUE;
    let mut isXMLNS: crate::expat_h::XML_Bool = crate::expat_h::XML_TRUE;
    let mut b: *mut BINDING = ::core::ptr::null_mut::<BINDING>();
    let mut len: ::core::ffi::c_int = 0;
    if *uri as ::core::ffi::c_int == '\0' as ::core::ffi::c_int && !(*prefix).name.is_null() {
        return crate::expat_h::XML_ERROR_UNDECLARING_PREFIX;
    }
    if !(*prefix).name.is_null()
        && *(*prefix).name.offset(0 as isize) as ::core::ffi::c_int == 0x78 as ::core::ffi::c_int
        && *(*prefix).name.offset(1 as isize) as ::core::ffi::c_int == 0x6d as ::core::ffi::c_int
        && *(*prefix).name.offset(2 as isize) as ::core::ffi::c_int == 0x6c as ::core::ffi::c_int
    {
        if *(*prefix).name.offset(3 as isize) as ::core::ffi::c_int == 0x6e as ::core::ffi::c_int
            && *(*prefix).name.offset(4 as isize) as ::core::ffi::c_int
                == 0x73 as ::core::ffi::c_int
            && *(*prefix).name.offset(5 as isize) as ::core::ffi::c_int
                == '\0' as ::core::ffi::c_int
        {
            return crate::expat_h::XML_ERROR_RESERVED_PREFIX_XMLNS;
        }
        if *(*prefix).name.offset(3 as isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            mustBeXML = crate::expat_h::XML_TRUE;
        }
    }
    len = 0 as ::core::ffi::c_int;
    while *uri.offset(len as isize) != 0 {
        if isXML as ::core::ffi::c_int != 0
            && (len > XML_NAMESPACE_LEN
                || *uri.offset(len as isize) as ::core::ffi::c_int
                    != XML_NAMESPACE[len as usize] as ::core::ffi::c_int)
        {
            isXML = crate::expat_h::XML_FALSE;
        }
        if mustBeXML == 0
            && isXMLNS as ::core::ffi::c_int != 0
            && (len > XMLNS_NAMESPACE_LEN
                || *uri.offset(len as isize) as ::core::ffi::c_int
                    != XMLNS_NAMESPACE[len as usize] as ::core::ffi::c_int)
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
    isXML = (isXML as ::core::ffi::c_int != 0 && len == XML_NAMESPACE_LEN) as ::core::ffi::c_int
        as crate::expat_h::XML_Bool;
    isXMLNS = (isXMLNS as ::core::ffi::c_int != 0 && len == XMLNS_NAMESPACE_LEN) as ::core::ffi::c_int
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
                ::core::mem::size_of::<crate::expat_external_h::XML_Char>().wrapping_mul(
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
            ::core::mem::size_of::<BINDING>(),
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
            ::core::mem::size_of::<crate::expat_external_h::XML_Char>()
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
            .wrapping_mul(::core::mem::size_of::<crate::expat_external_h::XML_Char>()),
    );
    if (*parser).m_namespaceSeparator != 0 {
        *(*b).uri.offset((len - 1 as ::core::ffi::c_int) as isize) = (*parser).m_namespaceSeparator;
    }
    (*b).prefix = prefix as *mut prefix;
    (*b).attId = attId as *const attribute_id;
    (*b).prevPrefixBinding = (*prefix).binding as *mut binding;
    if *uri as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        && prefix == &raw mut (*(*parser).m_dtd).defaultPrefix
    {
        (*prefix).binding = ::core::ptr::null_mut::<BINDING>();
    } else {
        (*prefix).binding = b;
    }
    (*b).nextTagBinding = *bindingsPtr as *mut binding;
    *bindingsPtr = b;
    if !attId.is_null() && (*parser).m_startNamespaceDeclHandler {
        let callback = START_NAMESPACE_DECL_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .get(&(parser as usize))
            .cloned();
        if let Some(callback) = callback {
            callback.invoke(
                (*parser).m_handlerArg,
                (*prefix).name,
                if !(*prefix).binding.is_null() {
                    uri
                } else {
                    ::core::ptr::null::<crate::expat_external_h::XML_Char>()
                },
            );
        }
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
        parser_encoding(parser),
        &mut start,
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
            (*parser).m_processor = ProcessorState::ExternalEntityContent;
            return externalEntityContentProcessor(parser, start, end, endPtr);
        } else {
            (*parser).m_processor = ProcessorState::Content;
            return contentProcessor(parser, start, end, endPtr);
        }
    }
    return result;
}

unsafe extern "C" fn doCdataSection(
    mut parser: crate::expat_h::XML_Parser,
    mut enc: *const crate::src::xmltok::ENCODING,
    startPtr: &mut *const ::core::ffi::c_char,
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
    if enc == parser_encoding(parser) {
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
        let mut tok: ::core::ffi::c_int =
            (*enc).scanners[2 as usize].scan(enc, s, end, &raw mut next);
        if accountingDiffTolerated(parser, tok, s, next, 4619 as ::core::ffi::c_int, account) == 0 {
            accountingOnAbort(parser);
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
                } else if false && (*parser).m_characterDataHandler {
                    callCharacterDataHandler(parser, (*parser).m_dataBuf, 0 as ::core::ffi::c_int);
                } else if (*parser).m_defaultHandler {
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
                if (*parser).m_characterDataHandler {
                    let mut c: crate::expat_external_h::XML_Char =
                        0xa as crate::expat_external_h::XML_Char;
                    callCharacterDataHandler(parser, &raw const c, 1 as ::core::ffi::c_int);
                } else if (*parser).m_defaultHandler {
                    reportDefault(parser, enc, s, next);
                }
            }
            crate::src::xmltok::XML_TOK_DATA_CHARS => {
                let charDataHandler = CHARACTER_DATA_HANDLERS
                    .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner())
                    .get(&(parser as usize))
                    .cloned();
                if let Some(charDataHandler) = charDataHandler {
                    if (*enc).isUtf8 == 0 {
                        loop {
                            let mut dataPtr: *mut ICHAR = (*parser).m_dataBuf as *mut ICHAR;
                            let convert_res: crate::src::xmltok::XML_Convert_Result =
                                crate::src::xmltok::convert_to_utf8(
                                    enc,
                                    &raw mut s,
                                    next,
                                    &raw mut dataPtr,
                                    (*parser).m_dataBufEnd as *mut ICHAR,
                                );
                            *eventEndPP = next;
                            charDataHandler.invoke(
                                (*parser).m_handlerArg,
                                (*parser).m_dataBuf,
                                dataPtr.offset_from((*parser).m_dataBuf as *mut ICHAR)
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
                        charDataHandler.invoke(
                            (*parser).m_handlerArg,
                            s as *const crate::expat_external_h::XML_Char,
                            (next as *const crate::expat_external_h::XML_Char)
                                .offset_from(s as *const crate::expat_external_h::XML_Char)
                                as ::core::ffi::c_int,
                        );
                    }
                } else if (*parser).m_defaultHandler {
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
        parser_encoding(parser),
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
        (*parser).m_processor = ProcessorState::Prolog;
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
    if enc == parser_encoding(parser) {
        eventPP = &raw mut (*parser).m_eventPtr;
        *eventPP = s;
        eventEndPP = &raw mut (*parser).m_eventEndPtr;
    } else {
        eventPP = &raw mut (*(*parser).m_openInternalEntities).internalEventPtr;
        eventEndPP = &raw mut (*(*parser).m_openInternalEntities).internalEventEndPtr;
    }
    *eventPP = s;
    *startPtr = ::core::ptr::null::<::core::ffi::c_char>();
    tok = (*enc).scanners[3 as usize].scan(enc, s, end, &raw mut next);
    if accountingDiffTolerated(
        parser,
        tok,
        s,
        next,
        4778 as ::core::ffi::c_int,
        XML_ACCOUNT_DIRECT,
    ) == 0
    {
        accountingOnAbort(parser);
        return crate::expat_h::XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
    }
    *eventEndPP = next;
    match tok {
        crate::src::xmltok::XML_TOK_IGNORE_SECT => {
            if (*parser).m_defaultHandler {
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
    let mut initialized_encoding = ::core::ptr::null::<crate::src::xmltok::ENCODING>();
    if if (*parser).m_ns as ::core::ffi::c_int != 0 {
        Some(
            crate::src::xmltok::xmltok_ns_c::XmlInitEncodingNS
                as unsafe extern "C" fn(
                    *mut crate::src::xmltok::INIT_ENCODING,
                    *mut *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        )
    } else {
        Some(
            crate::src::xmltok::xmltok_ns_c::XmlInitEncoding
                as unsafe extern "C" fn(
                    *mut crate::src::xmltok::INIT_ENCODING,
                    *mut *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        )
    }
    .expect("non-null function pointer")(
        &raw mut (*parser).m_initEncoding,
        &raw mut initialized_encoding,
        s,
    ) != 0
    {
        (*parser).m_encoding = EncodingState::Initial;
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
    let encoding = parser_encoding(parser);
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
    if accountingDiffTolerated(
        parser,
        crate::src::xmltok::XML_TOK_XML_DECL,
        s,
        next,
        4870 as ::core::ffi::c_int,
        XML_ACCOUNT_DIRECT,
    ) == 0
    {
        accountingOnAbort(parser);
        return crate::expat_h::XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
    }
    let parsed = {
        // Parsing a declaration updates only the parser's event cursor.  Keep
        // this borrow confined to the tokenizer call: user callbacks below
        // may re-enter the parser and must not overlap a Rust state borrow.
        let parser_state = &mut *parser;
        if parser_state.m_ns as ::core::ffi::c_int != 0 {
        Some(
            crate::src::xmltok::xmltok_ns_c::XmlParseXmlDeclNS
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                    *mut *const crate::src::xmltok::ENCODING,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
    } else {
        Some(
            crate::src::xmltok::xmltok_ns_c::XmlParseXmlDecl
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                    *mut *const crate::src::xmltok::ENCODING,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
    }
        .expect("non-null function pointer")(
        isGeneralTextEntity,
        encoding,
        s,
        next,
        &raw mut parser_state.m_eventPtr,
        &raw mut version,
        &raw mut versionend,
        &raw mut encodingName,
        &raw mut newEncoding,
        &raw mut standalone,
        ) != 0
    };
    if !parsed {
        if isGeneralTextEntity != 0 {
            return crate::expat_h::XML_ERROR_TEXT_DECL;
        } else {
            return crate::expat_h::XML_ERROR_XML_DECL;
        }
    }
    if isGeneralTextEntity == 0 && standalone == 1 as ::core::ffi::c_int {
        let parser_state = &mut *parser;
        (*parser_state.m_dtd).standalone = crate::expat_h::XML_TRUE;
        if parser_state.m_paramEntityParsing as ::core::ffi::c_uint
            == crate::expat_h::XML_PARAM_ENTITY_PARSING_UNLESS_STANDALONE as ::core::ffi::c_int
                as ::core::ffi::c_uint
        {
            parser_state.m_paramEntityParsing = crate::expat_h::XML_PARAM_ENTITY_PARSING_NEVER;
        }
    }
    let (xml_decl_handler, handler_arg, callback, default_handler) = {
        // Store callback arguments while holding the parser, then release the
        // borrow before invoking user code.  The pool owns these strings until
        // the declaration-processing tail clears it below.
        let parser_state = &mut *parser;
        let xml_decl_handler = parser_state.m_xmlDeclHandler;
        if xml_decl_handler {
            if !encodingName.is_null() {
                storedEncName = poolStoreString(
                    &raw mut parser_state.m_temp2Pool,
                    encoding,
                    encodingName,
                    encodingName.offset(
                        crate::src::xmltok::name_length(encoding, encodingName) as isize,
                    ),
                );
                if storedEncName.is_null() {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
                parser_state.m_temp2Pool.commit();
            }
            if !version.is_null() {
                storedversion = poolStoreString(
                    &raw mut parser_state.m_temp2Pool,
                    encoding,
                    version,
                    versionend.offset(-((*encoding).minBytesPerChar as isize)),
                );
                if storedversion.is_null() {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
            }
        }
        let callback = if xml_decl_handler {
            XML_DECL_HANDLERS
                .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .get(&(parser as usize))
                .cloned()
        } else {
            None
        };
        (
            xml_decl_handler,
            parser_state.m_handlerArg,
            callback,
            parser_state.m_defaultHandler,
        )
    };
    if xml_decl_handler {
        if let Some(callback) = callback {
            callback.invoke(
                handler_arg,
                storedversion,
                storedEncName,
                standalone,
            );
        }
    } else if default_handler {
        reportDefault(parser, encoding, s, next);
    }
    let has_no_protocol_encoding = {
        let parser_state = &mut *parser;
        parser_state.m_protocolEncodingName.is_null()
    };
    if has_no_protocol_encoding {
        if !newEncoding.is_null() {
            if (*newEncoding).minBytesPerChar != (*encoding).minBytesPerChar
                || (*newEncoding).minBytesPerChar == 2 as ::core::ffi::c_int
                    && newEncoding != encoding
            {
                let parser_state = &mut *parser;
                parser_state.m_eventPtr = encodingName;
                return crate::expat_h::XML_ERROR_INCORRECT_ENCODING;
            }
            if !select_known_encoding(parser, newEncoding) {
                return crate::expat_h::XML_ERROR_INCORRECT_ENCODING;
            }
        } else if !encodingName.is_null() {
            if storedEncName.is_null() {
                let parser_state = &mut *parser;
                storedEncName = poolStoreString(
                    &raw mut parser_state.m_temp2Pool,
                    encoding,
                    encodingName,
                    encodingName
                        .offset(crate::src::xmltok::name_length(encoding, encodingName) as isize),
                );
                if storedEncName.is_null() {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
            }
            // The unknown-encoding hook is user code, so the pool borrow used
            // to stage its name must end before the hook can re-enter.
            let result = handleUnknownEncoding(parser, storedEncName);
            let parser_state = &mut *parser;
            poolClear(&raw mut parser_state.m_temp2Pool);
            if result as ::core::ffi::c_uint
                == crate::expat_h::XML_ERROR_UNKNOWN_ENCODING as ::core::ffi::c_int
                    as ::core::ffi::c_uint
            {
                parser_state.m_eventPtr = encodingName;
            }
            return result;
        }
    }
    if !storedEncName.is_null() || !storedversion.is_null() {
        let parser_state = &mut *parser;
        poolClear(&raw mut parser_state.m_temp2Pool);
    }
    return crate::expat_h::XML_ERROR_NONE;
}

unsafe extern "C" fn handleUnknownEncoding(
    mut parser: crate::expat_h::XML_Parser,
    mut encodingName: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Error {
    if (*parser).m_unknownEncodingHandler {
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
        let callback = UNKNOWN_ENCODING_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .get(&(parser as usize))
            .cloned();
        if callback.is_some_and(|callback| {
            callback.invoke(
                (*parser).m_unknownEncodingHandlerData,
                encodingName,
                &raw mut info,
            ) != 0
        }) {
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
                    crate::src::xmltok::XmlInitUnknownEncodingNS
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
                    crate::src::xmltok::XmlInitUnknownEncoding
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
                (*parser).m_encoding = EncodingState::Unknown;
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
    (*parser).m_processor = ProcessorState::Prolog;
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
        (*parser).m_processor = ProcessorState::EntityValueInit;
        return entityValueInitProcessor(parser, s, end, nextPtr);
    } else {
        (*parser).m_processor = ProcessorState::ExternalParEnt;
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
        tok = (*parser_encoding(parser)).scanners[0 as usize].scan(
            parser_encoding(parser),
            start,
            end,
            &raw mut next,
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
                parser_encoding(parser),
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
            (*parser).m_processor = ProcessorState::EntityValue;
            return entityValueProcessor(parser, next, end, nextPtr);
        } else if tok == crate::src::xmltok::XML_TOK_BOM {
            if accountingDiffTolerated(
                parser,
                tok,
                s,
                next,
                5077 as ::core::ffi::c_int,
                XML_ACCOUNT_DIRECT,
            ) == 0
            {
                accountingOnAbort(parser);
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
    tok = (*parser_encoding(parser)).scanners[0 as usize].scan(
        parser_encoding(parser),
        s,
        end,
        &raw mut next,
    );
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
        if accountingDiffTolerated(
            parser,
            tok,
            s,
            next,
            5130 as ::core::ffi::c_int,
            XML_ACCOUNT_DIRECT,
        ) == 0
        {
            accountingOnAbort(parser);
            return crate::expat_h::XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
        }
        s = next;
        tok = (*parser_encoding(parser)).scanners[0 as usize].scan(
            parser_encoding(parser),
            s,
            end,
            &raw mut next,
        );
    }
    (*parser).m_processor = ProcessorState::Prolog;
    return doProlog(
        parser,
        parser_encoding(parser),
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
    let mut enc: *const crate::src::xmltok::ENCODING = parser_encoding(parser);
    let mut tok: ::core::ffi::c_int = 0;
    loop {
        tok = (*enc).scanners[0 as usize].scan(enc, start, end, &raw mut next);
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
    let mut tok: ::core::ffi::c_int = (*parser_encoding(parser)).scanners[0 as usize].scan(
        parser_encoding(parser),
        s,
        end,
        &raw mut next,
    );
    return doProlog(
        parser,
        parser_encoding(parser),
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
    static externalSubsetName: [crate::expat_external_h::XML_Char; 2] = [
        crate::ascii_h::ASCII_HASH as crate::expat_external_h::XML_Char,
        '\0' as crate::expat_external_h::XML_Char,
    ];
    static atypeCDATA: [crate::expat_external_h::XML_Char; 6] = [
        crate::ascii_h::ASCII_C as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_D as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_A as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_T as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_A as crate::expat_external_h::XML_Char,
        '\0' as crate::expat_external_h::XML_Char,
    ];
    static atypeID: [crate::expat_external_h::XML_Char; 3] = [
        crate::ascii_h::ASCII_I as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_D as crate::expat_external_h::XML_Char,
        '\0' as crate::expat_external_h::XML_Char,
    ];
    static atypeIDREF: [crate::expat_external_h::XML_Char; 6] = [
        crate::ascii_h::ASCII_I as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_D as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_R as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_E as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_F as crate::expat_external_h::XML_Char,
        '\0' as crate::expat_external_h::XML_Char,
    ];
    static atypeIDREFS: [crate::expat_external_h::XML_Char; 7] = [
        crate::ascii_h::ASCII_I as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_D as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_R as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_E as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_F as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_S as crate::expat_external_h::XML_Char,
        '\0' as crate::expat_external_h::XML_Char,
    ];
    static atypeENTITY: [crate::expat_external_h::XML_Char; 7] = [
        crate::ascii_h::ASCII_E as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_N as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_T as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_I as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_T as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_Y as crate::expat_external_h::XML_Char,
        '\0' as crate::expat_external_h::XML_Char,
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
        '\0' as crate::expat_external_h::XML_Char,
    ];
    static atypeNMTOKEN: [crate::expat_external_h::XML_Char; 8] = [
        crate::ascii_h::ASCII_N as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_M as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_T as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_O as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_K as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_E as crate::expat_external_h::XML_Char,
        crate::ascii_h::ASCII_N as crate::expat_external_h::XML_Char,
        '\0' as crate::expat_external_h::XML_Char,
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
        '\0' as crate::expat_external_h::XML_Char,
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
        '\0' as crate::expat_external_h::XML_Char,
    ];
    static enumValueSep: [crate::expat_external_h::XML_Char; 2] = [
        crate::ascii_h::ASCII_PIPE as crate::expat_external_h::XML_Char,
        '\0' as crate::expat_external_h::XML_Char,
    ];
    static enumValueStart: [crate::expat_external_h::XML_Char; 2] = [
        crate::ascii_h::ASCII_LPAREN as crate::expat_external_h::XML_Char,
        '\0' as crate::expat_external_h::XML_Char,
    ];
    let dtd: *mut DTD = (*parser).m_dtd;
    let dtd_pool: *mut STRING_POOL = &raw mut (*dtd).pool;
    let mut eventPP: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    let mut eventEndPP: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    let mut quant: crate::expat_h::XML_Content_Quant = crate::expat_h::XML_CQUANT_NONE;
    if enc == parser_encoding(parser) {
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
                    if enc != parser_encoding(parser)
                        && (*(*parser).m_openInternalEntities).betweenDecl == 0
                    {
                        *nextPtr = s;
                        return crate::expat_h::XML_ERROR_NONE;
                    }
                    if (*parser).m_isParamEntity as ::core::ffi::c_int != 0
                        || enc != parser_encoding(parser)
                    {
                        let prolog_state = &mut (*parser).m_prologState;
                        let token: &[::core::ffi::c_char] = &[];
                        let min_bytes_per_char = (*enc).minBytesPerChar;
                        if crate::src::xmlrole::prolog_handler_dispatch(
                            prolog_state
                                .handler
                                .expect("prolog state must have a handler"),
                            prolog_state,
                            -4 as ::core::ffi::c_int,
                            token,
                            min_bytes_per_char,
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
        let prolog_state = &mut (*parser).m_prologState;
        let token = ::core::slice::from_raw_parts(s, next.offset_from(s) as usize);
        let min_bytes_per_char = (*enc).minBytesPerChar;
        role = crate::src::xmlrole::prolog_handler_dispatch(
            prolog_state
                .handler
                .expect("prolog state must have a handler"),
            prolog_state,
            tok,
            token,
            min_bytes_per_char,
        );
        match role {
            2 | 1 | 57 => {}
            _ => {
                if accountingDiffTolerated(
                    parser,
                    tok,
                    s,
                    next,
                    5301 as ::core::ffi::c_int,
                    account,
                ) == 0
                {
                    accountingOnAbort(parser);
                    return crate::expat_h::XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
                }
            }
        }
        's_2375: {
            '_alreadyChecked: {
                '_closeGroup: {
                    '_elementContent: {
                        '_checkAttListDeclHandler: {
                            'c_12793: {
                                match role {
                                    1 => {
                                        let mut result: crate::expat_h::XML_Error = processXmlDecl(
                                            parser,
                                            0 as ::core::ffi::c_int,
                                            s,
                                            next,
                                        );
                                        if result as ::core::ffi::c_uint
                                            != crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int
                                                as ::core::ffi::c_uint
                                        {
                                            return result;
                                        }
                                        enc = parser_encoding(parser);
                                        handleDefault = crate::expat_h::XML_FALSE;
                                        break 's_2375;
                                    }
                                    4 => {
                                        if (*parser).m_startDoctypeDeclHandler {
                                            (*parser).m_doctypeName = poolStoreString(
                                                &raw mut (*parser).m_tempPool,
                                                enc,
                                                s,
                                                next,
                                            );
                                            if (*parser).m_doctypeName.is_null() {
                                                return crate::expat_h::XML_ERROR_NO_MEMORY;
                                            }
                                            (*parser).m_tempPool.commit();
                                            (*parser).m_doctypePubid = ::core::ptr::null::<
                                                crate::expat_external_h::XML_Char,
                                            >(
                                            );
                                            handleDefault = crate::expat_h::XML_FALSE;
                                        }
                                        (*parser).m_doctypeSysid =
                                            ::core::ptr::null::<crate::expat_external_h::XML_Char>(
                                            );
                                        break 's_2375;
                                    }
                                    7 => {
                                        if (*parser).m_startDoctypeDeclHandler {
                                            let callback = START_DOCTYPE_DECL_HANDLERS
                                                .get_or_init(|| {
                                                    std::sync::Mutex::new(
                                                        std::collections::HashMap::new(),
                                                    )
                                                })
                                                .lock()
                                                .unwrap_or_else(|poisoned| poisoned.into_inner())
                                                .get(&(parser as usize))
                                                .cloned();
                                            if let Some(callback) = callback {
                                                callback.invoke(
                                                    (*parser).m_handlerArg,
                                                    (*parser).m_doctypeName,
                                                    (*parser).m_doctypeSysid,
                                                    (*parser).m_doctypePubid,
                                                    1 as ::core::ffi::c_int,
                                                );
                                            }
                                            (*parser).m_doctypeName = ::core::ptr::null::<
                                                crate::expat_external_h::XML_Char,
                                            >(
                                            );
                                            poolClear(&raw mut (*parser).m_tempPool);
                                            handleDefault = crate::expat_h::XML_FALSE;
                                        }
                                        break 's_2375;
                                    }
                                    57 => {
                                        let mut result_0: crate::expat_h::XML_Error =
                                            processXmlDecl(
                                                parser,
                                                1 as ::core::ffi::c_int,
                                                s,
                                                next,
                                            );
                                        if result_0 as ::core::ffi::c_uint
                                            != crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int
                                                as ::core::ffi::c_uint
                                        {
                                            return result_0;
                                        }
                                        enc = parser_encoding(parser);
                                        handleDefault = crate::expat_h::XML_FALSE;
                                        break 's_2375;
                                    }
                                    6 => {
                                        (*parser).m_useForeignDTD = crate::expat_h::XML_FALSE;
                                        (*parser).m_declEntity = lookup(
                                            parser,
                                            &raw mut (*dtd).paramEntities,
                                            &raw const externalSubsetName as KEY,
                                            ::core::mem::size_of::<ENTITY>(),
                                        )
                                            as *mut ENTITY;
                                        if (*parser).m_declEntity.is_null() {
                                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                                        }
                                        (*dtd).hasParamEntityRefs = crate::expat_h::XML_TRUE;
                                        if (*parser).m_startDoctypeDeclHandler {
                                            let mut pubId: *mut crate::expat_external_h::XML_Char =
                                                ::core::ptr::null_mut::<
                                                    crate::expat_external_h::XML_Char,
                                                >();
                                            if crate::src::xmltok::check_public_id(
                                                (*enc).isPublicId,
                                                enc,
                                                s,
                                                next,
                                                eventPP,
                                            ) == 0
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
                                            normalizePublicId(pubId);
                                            (*parser).m_tempPool.commit();
                                            (*parser).m_doctypePubid = pubId;
                                            handleDefault = crate::expat_h::XML_FALSE;
                                            break '_alreadyChecked;
                                        }
                                    }
                                    14 => {}
                                    8 => {
                                        if allowClosingDoctype as ::core::ffi::c_int
                                            != crate::expat_h::XML_TRUE as ::core::ffi::c_int
                                        {
                                            return crate::expat_h::XML_ERROR_INVALID_TOKEN;
                                        }
                                        if !(*parser).m_doctypeName.is_null() {
                                            let callback = START_DOCTYPE_DECL_HANDLERS
                                                .get_or_init(|| {
                                                    std::sync::Mutex::new(
                                                        std::collections::HashMap::new(),
                                                    )
                                                })
                                                .lock()
                                                .unwrap_or_else(|poisoned| poisoned.into_inner())
                                                .get(&(parser as usize))
                                                .cloned();
                                            if let Some(callback) = callback {
                                                callback.invoke(
                                                    (*parser).m_handlerArg,
                                                    (*parser).m_doctypeName,
                                                    (*parser).m_doctypeSysid,
                                                    (*parser).m_doctypePubid,
                                                    0 as ::core::ffi::c_int,
                                                );
                                            }
                                            poolClear(&raw mut (*parser).m_tempPool);
                                            handleDefault = crate::expat_h::XML_FALSE;
                                        }
                                        if !(*parser).m_doctypeSysid.is_null()
                                            || (*parser).m_useForeignDTD as ::core::ffi::c_int != 0
                                        {
                                            let mut hadParamEntityRefs: crate::expat_h::XML_Bool =
                                                (*dtd).hasParamEntityRefs;
                                            (*dtd).hasParamEntityRefs = crate::expat_h::XML_TRUE;
                                            if (*parser).m_paramEntityParsing as ::core::ffi::c_uint
                                                != 0
                                                && (*parser).m_externalEntityRefHandler
                                            {
                                                let mut entity: *mut ENTITY = lookup(
                                                    parser,
                                                    &raw mut (*dtd).paramEntities,
                                                    &raw const externalSubsetName as KEY,
                                                    ::core::mem::size_of::<ENTITY>(),
                                                )
                                                    as *mut ENTITY;
                                                if entity.is_null() {
                                                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                                                }
                                                if (*parser).m_useForeignDTD != 0 {
                                                    (*entity).base = pool_string_ref(
                                                        dtd_pool as *const STRING_POOL,
                                                        (*parser).m_curBase,
                                                        false,
                                                    );
                                                }
                                                (*dtd).paramEntityRead = crate::expat_h::XML_FALSE;
                                                let handler = EXTERNAL_ENTITY_REF_HANDLERS
                                                    .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
                                                    .lock()
                                                    .unwrap_or_else(|poisoned| poisoned.into_inner())
                                                    .get(&(parser as usize))
                                                    .cloned()
                                                    .expect("installed external entity handler");
                                                if invoke_external_entity_ref_handler(
                                                    handler.as_ref(),
                                                    (*parser).m_externalEntityRefHandlerArg,
                                                    ::core::ptr::null(),
                                                    dtd_pool as *const STRING_POOL,
                                                    entity,
                                                ) == 0
                                                {
                                                    return crate::expat_h::XML_ERROR_EXTERNAL_ENTITY_HANDLING;
                                                }
                                                if (*dtd).paramEntityRead != 0 {
                                                    if (*dtd).standalone == 0
                                                        && (*parser)
                                                            .m_notStandaloneHandler
                                                            .is_some()
                                                        && (*parser)
                                                            .m_notStandaloneHandler
                                                            .expect("non-null function pointer")(
                                                            (*parser).m_handlerArg,
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
                                                (*parser).m_handlerArg,
                                            );
                                            handleDefault = crate::expat_h::XML_FALSE;
                                        }
                                        break 's_2375;
                                    }
                                    2 => {
                                        if (*parser).m_useForeignDTD != 0 {
                                            let mut hadParamEntityRefs_0: crate::expat_h::XML_Bool =
                                                (*dtd).hasParamEntityRefs;
                                            (*dtd).hasParamEntityRefs = crate::expat_h::XML_TRUE;
                                            if (*parser).m_paramEntityParsing as ::core::ffi::c_uint
                                                != 0
                                                && (*parser).m_externalEntityRefHandler
                                            {
                                                let mut entity_0: *mut ENTITY = lookup(
                                                    parser,
                                                    &raw mut (*dtd).paramEntities,
                                                    &raw const externalSubsetName as KEY,
                                                    ::core::mem::size_of::<ENTITY>(),
                                                )
                                                    as *mut ENTITY;
                                                if entity_0.is_null() {
                                                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                                                }
                                                (*entity_0).base = pool_string_ref(
                                                    dtd_pool as *const STRING_POOL,
                                                    (*parser).m_curBase,
                                                    false,
                                                );
                                                (*dtd).paramEntityRead = crate::expat_h::XML_FALSE;
                                                let handler = EXTERNAL_ENTITY_REF_HANDLERS
                                                    .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
                                                    .lock()
                                                    .unwrap_or_else(|poisoned| poisoned.into_inner())
                                                    .get(&(parser as usize))
                                                    .cloned()
                                                    .expect("installed external entity handler");
                                                if invoke_external_entity_ref_handler(
                                                    handler.as_ref(),
                                                    (*parser).m_externalEntityRefHandlerArg,
                                                    ::core::ptr::null(),
                                                    dtd_pool as *const STRING_POOL,
                                                    entity_0,
                                                ) == 0
                                                {
                                                    return crate::expat_h::XML_ERROR_EXTERNAL_ENTITY_HANDLING;
                                                }
                                                if (*dtd).paramEntityRead != 0 {
                                                    if (*dtd).standalone == 0
                                                        && (*parser)
                                                            .m_notStandaloneHandler
                                                            .is_some()
                                                        && (*parser)
                                                            .m_notStandaloneHandler
                                                            .expect("non-null function pointer")(
                                                            (*parser).m_handlerArg,
                                                        ) == 0
                                                    {
                                                        return crate::expat_h::XML_ERROR_NOT_STANDALONE;
                                                    }
                                                } else {
                                                    (*dtd).hasParamEntityRefs =
                                                        hadParamEntityRefs_0;
                                                }
                                            }
                                        }
                                        (*parser).m_processor = ProcessorState::Content;
                                        return contentProcessor(parser, s, end, nextPtr);
                                    }
                                    34 => {
                                        (*parser).m_declElementType =
                                            getElementType(parser, enc, s, next);
                                        if (*parser).m_declElementType.is_null() {
                                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                                        }
                                        break '_checkAttListDeclHandler;
                                    }
                                    22 => {
                                        (*parser).m_declAttributeId =
                                            getAttributeId(parser, enc, s, next);
                                        if (*parser).m_declAttributeId.is_null() {
                                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                                        }
                                        (*parser).m_declAttributeIsCdata =
                                            crate::expat_h::XML_FALSE;
                                        (*parser).m_declAttributeType =
                                            ::core::ptr::null::<crate::expat_external_h::XML_Char>(
                                            );
                                        (*parser).m_declAttributeIsId = crate::expat_h::XML_FALSE;
                                        break '_checkAttListDeclHandler;
                                    }
                                    23 => {
                                        (*parser).m_declAttributeIsCdata = crate::expat_h::XML_TRUE;
                                        (*parser).m_declAttributeType = &raw const atypeCDATA
                                            as *const crate::expat_external_h::XML_Char;
                                        break '_checkAttListDeclHandler;
                                    }
                                    24 => {
                                        (*parser).m_declAttributeIsId = crate::expat_h::XML_TRUE;
                                        (*parser).m_declAttributeType = &raw const atypeID
                                            as *const crate::expat_external_h::XML_Char;
                                        break '_checkAttListDeclHandler;
                                    }
                                    25 => {
                                        (*parser).m_declAttributeType = &raw const atypeIDREF
                                            as *const crate::expat_external_h::XML_Char;
                                        break '_checkAttListDeclHandler;
                                    }
                                    26 => {
                                        (*parser).m_declAttributeType = &raw const atypeIDREFS
                                            as *const crate::expat_external_h::XML_Char;
                                        break '_checkAttListDeclHandler;
                                    }
                                    27 => {
                                        (*parser).m_declAttributeType = &raw const atypeENTITY
                                            as *const crate::expat_external_h::XML_Char;
                                        break '_checkAttListDeclHandler;
                                    }
                                    28 => {
                                        (*parser).m_declAttributeType = &raw const atypeENTITIES
                                            as *const crate::expat_external_h::XML_Char;
                                        break '_checkAttListDeclHandler;
                                    }
                                    29 => {
                                        (*parser).m_declAttributeType = &raw const atypeNMTOKEN
                                            as *const crate::expat_external_h::XML_Char;
                                        break '_checkAttListDeclHandler;
                                    }
                                    30 => {
                                        (*parser).m_declAttributeType = &raw const atypeNMTOKENS
                                            as *const crate::expat_external_h::XML_Char;
                                        break '_checkAttListDeclHandler;
                                    }
                                    31 | 32 => {
                                        if (*dtd).keepProcessing as ::core::ffi::c_int != 0
                                            && (*parser).m_attlistDeclHandler
                                        {
                                            let mut prefix: *const crate::expat_external_h::XML_Char =
                                                ::core::ptr::null:: <crate::expat_external_h::XML_Char>();
                                            if !(*parser).m_declAttributeType.is_null() {
                                                prefix = &raw const enumValueSep
                                                    as *const crate::expat_external_h::XML_Char;
                                            } else {
                                                prefix = if role
                                                    == crate::src::xmlrole::XML_ROLE_ATTRIBUTE_NOTATION_VALUE
                                                        as ::core::ffi::c_int
                                                {
                                                    &raw const notationPrefix as *const crate::expat_external_h::XML_Char
                                                } else {
                                                    &raw const enumValueStart as *const crate::expat_external_h::XML_Char
                                                };
                                            }
                                            if poolAppendString(
                                                &raw mut (*parser).m_tempPool,
                                                prefix,
                                            )
                                            .is_null()
                                            {
                                                return crate::expat_h::XML_ERROR_NO_MEMORY;
                                            }
                                            if poolAppend(
                                                &raw mut (*parser).m_tempPool,
                                                enc,
                                                s,
                                                next,
                                            )
                                            .is_null()
                                            {
                                                return crate::expat_h::XML_ERROR_NO_MEMORY;
                                            }
                                            let parser_ref = &mut *parser;
                                            let Some(start) = parser_ref.m_tempPool.start_ref(true) else {
                                                return crate::expat_h::XML_ERROR_NO_MEMORY;
                                            };
                                            parser_ref.m_declAttributeType = parser_ref
                                                .m_tempPool
                                                .chars_from(start)
                                                .map_or(::core::ptr::null(), |chars| chars.as_ptr());
                                            handleDefault = crate::expat_h::XML_FALSE;
                                        }
                                        break 's_2375;
                                    }
                                    35 | 36 => {
                                        if (*dtd).keepProcessing != 0 {
                                            if defineAttribute(
                                                (*parser).m_declElementType,
                                                (*parser).m_declAttributeId,
                                                (*parser).m_declAttributeIsCdata,
                                                (*parser).m_declAttributeIsId,
                                                ::core::ptr::null::<
                                                    crate::expat_external_h::XML_Char,
                                                >(),
                                                parser,
                                            ) == 0
                                            {
                                                return crate::expat_h::XML_ERROR_NO_MEMORY;
                                            }
                                            if (*parser).m_attlistDeclHandler
                                                && !(*parser).m_declAttributeType.is_null()
                                            {
                                                if *(*parser).m_declAttributeType
                                                    as ::core::ffi::c_int
                                                    == 0x28 as ::core::ffi::c_int
                                                    || *(*parser).m_declAttributeType
                                                        as ::core::ffi::c_int
                                                        == 0x4e as ::core::ffi::c_int
                                                        && *(*parser)
                                                            .m_declAttributeType
                                                            .offset(1 as isize)
                                                            as ::core::ffi::c_int
                                                            == 0x4f as ::core::ffi::c_int
                                                {
                                                    if (if (*parser).m_tempPool.is_full()
                                                        && poolGrow(&raw mut (*parser).m_tempPool)
                                                            == 0
                                                    {
                                                        0 as ::core::ffi::c_int
                                                    } else {
                                                        if (*parser).m_tempPool.write_cursor(
                                                            0x29 as crate::expat_external_h::XML_Char,
                                                        ) {
                                                            1 as ::core::ffi::c_int
                                                        } else {
                                                            0 as ::core::ffi::c_int
                                                        }
                                                    }) == 0
                                                        || (if (*parser).m_tempPool.is_full()
                                                            && poolGrow(
                                                                &raw mut (*parser).m_tempPool,
                                                            ) == 0
                                                        {
                                                            0 as ::core::ffi::c_int
                                                        } else {
                                                            if (*parser).m_tempPool.write_cursor(
                                                                '\0' as crate::expat_external_h::XML_Char,
                                                            ) {
                                                                1 as ::core::ffi::c_int
                                                            } else {
                                                                0 as ::core::ffi::c_int
                                                            }
                                                        }) == 0
                                                    {
                                                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                                                    }
                                                    let parser_ref = &mut *parser;
                                                    let Some(start) = parser_ref.m_tempPool.start_ref(true) else {
                                                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                                                    };
                                                    parser_ref.m_declAttributeType = parser_ref
                                                        .m_tempPool
                                                        .chars_from(start)
                                                        .map_or(::core::ptr::null(), |chars| chars.as_ptr());
                                                    parser_ref.m_tempPool.commit();
                                                }
                                                *eventEndPP = s;
                                                if let Some(callback) = attlist_decl_handler(parser as usize) {
                                                    callback.invoke(
                                                        (*parser).m_handlerArg,
                                                        (*(*parser).m_declElementType).named.name,
                                                        (*(*parser).m_declAttributeId).name,
                                                        (*parser).m_declAttributeType,
                                                        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
                                                        (role
                                                            == crate::src::xmlrole::XML_ROLE_REQUIRED_ATTRIBUTE_VALUE
                                                                as ::core::ffi::c_int)
                                                            as ::core::ffi::c_int,
                                                    );
                                                }
                                                handleDefault = crate::expat_h::XML_FALSE;
                                            }
                                        }
                                        poolClear(&raw mut (*parser).m_tempPool);
                                        break 's_2375;
                                    }
                                    37 | 38 => {
                                        if (*dtd).keepProcessing != 0 {
                                            let mut attVal: *const crate::expat_external_h::XML_Char =
                                                ::core::ptr::null:: <crate::expat_external_h::XML_Char>();
                                            let mut result_1: crate::expat_h::XML_Error =
                                                storeAttributeValue(
                                                    parser,
                                                    enc,
                                                    (*parser).m_declAttributeIsCdata,
                                                    s.offset((*enc).minBytesPerChar as isize),
                                                    next.offset(-((*enc).minBytesPerChar as isize)),
                                                    dtd_pool,
                                                    XML_ACCOUNT_NONE,
                                                );
                                            if result_1 as u64 != 0 {
                                                return result_1;
                                            }
                                            let dtd_ref = &mut *dtd;
                                            let Some(start) = dtd_ref.pool.start_ref(true) else {
                                                return crate::expat_h::XML_ERROR_NO_MEMORY;
                                            };
                                            attVal = dtd_ref
                                                .pool
                                                .chars_from(start)
                                                .map_or(::core::ptr::null(), |chars| chars.as_ptr());
                                            dtd_ref.pool.commit();
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
                                            if (*parser).m_attlistDeclHandler
                                                && !(*parser).m_declAttributeType.is_null()
                                            {
                                                if *(*parser).m_declAttributeType
                                                    as ::core::ffi::c_int
                                                    == 0x28 as ::core::ffi::c_int
                                                    || *(*parser).m_declAttributeType
                                                        as ::core::ffi::c_int
                                                        == 0x4e as ::core::ffi::c_int
                                                        && *(*parser)
                                                            .m_declAttributeType
                                                            .offset(1 as isize)
                                                            as ::core::ffi::c_int
                                                            == 0x4f as ::core::ffi::c_int
                                                {
                                                    if (if (*parser).m_tempPool.is_full()
                                                        && poolGrow(&raw mut (*parser).m_tempPool)
                                                            == 0
                                                    {
                                                        0 as ::core::ffi::c_int
                                                    } else {
                                                        if (*parser).m_tempPool.write_cursor(
                                                            0x29 as crate::expat_external_h::XML_Char,
                                                        ) {
                                                            1 as ::core::ffi::c_int
                                                        } else {
                                                            0 as ::core::ffi::c_int
                                                        }
                                                    }) == 0
                                                        || (if (*parser).m_tempPool.is_full()
                                                            && poolGrow(
                                                                &raw mut (*parser).m_tempPool,
                                                            ) == 0
                                                        {
                                                            0 as ::core::ffi::c_int
                                                        } else {
                                                            if (*parser).m_tempPool.write_cursor(
                                                                '\0' as crate::expat_external_h::XML_Char,
                                                            ) {
                                                                1 as ::core::ffi::c_int
                                                            } else {
                                                                0 as ::core::ffi::c_int
                                                            }
                                                        }) == 0
                                                    {
                                                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                                                    }
                                                    let parser_ref = &mut *parser;
                                                    let Some(start) = parser_ref.m_tempPool.start_ref(true) else {
                                                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                                                    };
                                                    parser_ref.m_declAttributeType = parser_ref
                                                        .m_tempPool
                                                        .chars_from(start)
                                                        .map_or(::core::ptr::null(), |chars| chars.as_ptr());
                                                    parser_ref.m_tempPool.commit();
                                                }
                                                *eventEndPP = s;
                                                if let Some(callback) = attlist_decl_handler(parser as usize) {
                                                    callback.invoke(
                                                        (*parser).m_handlerArg,
                                                        (*(*parser).m_declElementType).named.name,
                                                        (*(*parser).m_declAttributeId).name,
                                                        (*parser).m_declAttributeType,
                                                        attVal,
                                                        (role
                                                            == crate::src::xmlrole::XML_ROLE_FIXED_ATTRIBUTE_VALUE
                                                                as ::core::ffi::c_int)
                                                            as ::core::ffi::c_int,
                                                    );
                                                }
                                                poolClear(&raw mut (*parser).m_tempPool);
                                                handleDefault = crate::expat_h::XML_FALSE;
                                            }
                                        }
                                        break 's_2375;
                                    }
                                    12 => {
                                        if (*dtd).keepProcessing != 0 {
                                            let mut result_2: crate::expat_h::XML_Error =
                                                callStoreEntityValue(
                                                    parser,
                                                    enc,
                                                    s.offset((*enc).minBytesPerChar as isize),
                                                    next.offset(-((*enc).minBytesPerChar as isize)),
                                                    XML_ACCOUNT_NONE,
                                            );
                                            if !(*parser).m_declEntity.is_null() {
                                                let dtd_ref = &mut *dtd;
                                                let Some(entity_text_ref) = dtd_ref.entityValuePool.start_ref(true) else {
                                                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                                                };
                                                let entity_text = dtd_ref.entityValuePool
                                                    .chars_from(entity_text_ref)
                                                    .map_or(::core::ptr::null(), |chars| chars.as_ptr());
                                                (*(*parser).m_declEntity).textPtr = EntityTextRef {
                                                    pool: EntityTextPool::EntityValue,
                                                    string: Some(entity_text_ref),
                                                };
                                                let Ok(text_len) = ::core::ffi::c_int::try_from(
                                                    dtd_ref.entityValuePool.ptr_offset,
                                                ) else {
                                                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                                                };
                                                (*(*parser).m_declEntity).textLen = text_len;
                                                dtd_ref.entityValuePool.commit();
                                                if (*parser).m_entityDeclHandler {
                                                    *eventEndPP = s;
                                                    let callback = ENTITY_DECL_HANDLERS
                                                        .get_or_init(|| {
                                                            std::sync::Mutex::new(
                                                                std::collections::HashMap::new(),
                                                            )
                                                    })
                                                        .lock()
                                                        .unwrap_or_else(|poisoned| {
                                                            poisoned.into_inner()
                                                        })
                                                        .get(&(parser as usize))
                                                        .cloned();
                                                    if let Some(callback) = callback {
                                                        callback.invoke(
                                                            (*parser).m_handlerArg,
                                                            (*(*parser).m_declEntity).named.name,
                                                            (*(*parser).m_declEntity).is_param
                                                                as ::core::ffi::c_int,
                                                            entity_text,
                                                            (*(*parser).m_declEntity).textLen,
                                                            (*parser).m_curBase,
                                                            ::core::ptr::null::<
                                                                crate::expat_external_h::XML_Char,
                                                            >(
                                                            ),
                                                            ::core::ptr::null::<
                                                                crate::expat_external_h::XML_Char,
                                                            >(
                                                            ),
                                                            ::core::ptr::null::<
                                                                crate::expat_external_h::XML_Char,
                                                            >(
                                                            ),
                                                        );
                                                    }
                                                    handleDefault = crate::expat_h::XML_FALSE;
                                                }
                                            } else {
                                                (*dtd).entityValuePool.rewind();
                                            }
                                            if result_2 as ::core::ffi::c_uint
                                                != crate::expat_h::XML_ERROR_NONE
                                                    as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint
                                            {
                                                return result_2;
                                            }
                                        }
                                        break 's_2375;
                                    }
                                    5 => {
                                        (*parser).m_useForeignDTD = crate::expat_h::XML_FALSE;
                                        (*dtd).hasParamEntityRefs = crate::expat_h::XML_TRUE;
                                        if (*parser).m_startDoctypeDeclHandler {
                                            (*parser).m_doctypeSysid = poolStoreString(
                                                &raw mut (*parser).m_tempPool,
                                                enc,
                                                s.offset((*enc).minBytesPerChar as isize),
                                                next.offset(-((*enc).minBytesPerChar as isize)),
                                            );
                                            if (*parser).m_doctypeSysid.is_null() {
                                                return crate::expat_h::XML_ERROR_NO_MEMORY;
                                            }
                                            (*parser).m_tempPool.commit();
                                            handleDefault = crate::expat_h::XML_FALSE;
                                        } else {
                                            (*parser).m_doctypeSysid = &raw const externalSubsetName
                                                as *const crate::expat_external_h::XML_Char;
                                        }
                                        if (*dtd).standalone == 0
                                            && (*parser).m_paramEntityParsing as u64 == 0
                                            && (*parser).m_notStandaloneHandler.is_some()
                                            && (*parser)
                                                .m_notStandaloneHandler
                                                .expect("non-null function pointer")(
                                                (*parser).m_handlerArg,
                                            ) == 0
                                        {
                                            return crate::expat_h::XML_ERROR_NOT_STANDALONE;
                                        }
                                        if (*parser).m_declEntity.is_null() {
                                            (*parser).m_declEntity = lookup(
                                                parser,
                                                &raw mut (*dtd).paramEntities,
                                                &raw const externalSubsetName as KEY,
                                                ::core::mem::size_of::<ENTITY>(),
                                            )
                                                as *mut ENTITY;
                                            if (*parser).m_declEntity.is_null() {
                                                return crate::expat_h::XML_ERROR_NO_MEMORY;
                                            }
                                            (*(*parser).m_declEntity).publicId = None;
                                        }
                                        break 'c_12793;
                                    }
                                    13 => {
                                        break 'c_12793;
                                    }
                                    15 => {
                                        if (*dtd).keepProcessing as ::core::ffi::c_int != 0
                                            && !(*parser).m_declEntity.is_null()
                                            && (*parser).m_entityDeclHandler
                                        {
                                            *eventEndPP = s;
                                            let callback = ENTITY_DECL_HANDLERS
                                                .get_or_init(|| {
                                                    std::sync::Mutex::new(
                                                        std::collections::HashMap::new(),
                                                    )
                                                })
                                                .lock()
                                                .unwrap_or_else(|poisoned| poisoned.into_inner())
                                                .get(&(parser as usize))
                                                .cloned();
                                            if let Some(callback) = callback {
                                                let (
                                                    handler_arg,
                                                    entity_name,
                                                    entity_is_param,
                                                    entity_base,
                                                    entity_system_id,
                                                    entity_public_id,
                                                ) = {
                                                    let entity = &*(*parser).m_declEntity;
                                                    (
                                                        (*parser).m_handlerArg,
                                                        entity.named.name,
                                                        entity.is_param as ::core::ffi::c_int,
                                                        entity.base,
                                                        entity.systemId,
                                                        entity.publicId,
                                                    )
                                                };
                                                callback.invoke(
                                                    handler_arg,
                                                    entity_name,
                                                    entity_is_param,
                                                    ::core::ptr::null::<
                                                        crate::expat_external_h::XML_Char,
                                                    >(
                                                    ),
                                                    0 as ::core::ffi::c_int,
                                                    entity_base.map_or(
                                                        ::core::ptr::null(),
                                                        |base| pool_string_pointer(
                                                            dtd_pool as *const STRING_POOL,
                                                            base,
                                                        ),
                                                    ),
                                                    entity_system_id.map_or(
                                                        ::core::ptr::null(),
                                                        |system_id| pool_string_pointer(
                                                            dtd_pool as *const STRING_POOL,
                                                            system_id,
                                                        ),
                                                    ),
                                                    entity_public_id.map_or(
                                                        ::core::ptr::null(),
                                                        |public_id| pool_string_pointer(
                                                            dtd_pool as *const STRING_POOL,
                                                            public_id,
                                                        ),
                                                    ),
                                                    ::core::ptr::null::<
                                                        crate::expat_external_h::XML_Char,
                                                    >(
                                                    ),
                                                );
                                            }
                                            handleDefault = crate::expat_h::XML_FALSE;
                                        }
                                        break 's_2375;
                                    }
                                    16 => {
                                        if (*dtd).keepProcessing as ::core::ffi::c_int != 0
                                            && !(*parser).m_declEntity.is_null()
                                        {
                                            let notation_pointer = poolStoreString(dtd_pool, enc, s, next);
                                            let Some(notation) = pool_string_ref(
                                                dtd_pool as *const STRING_POOL,
                                                notation_pointer,
                                                false,
                                            ) else {
                                                return crate::expat_h::XML_ERROR_NO_MEMORY;
                                            };
                                            (*(*parser).m_declEntity).notation = Some(notation);
                                            (*dtd).pool.commit();
                                            let callback = UNPARSED_ENTITY_DECL_HANDLERS
                                                .get_or_init(|| {
                                                    std::sync::Mutex::new(
                                                        std::collections::HashMap::new(),
                                                    )
                                                })
                                                .lock()
                                                .unwrap_or_else(|poisoned| {
                                                    poisoned.into_inner()
                                                })
                                                .get(&(parser as usize))
                                                .cloned();
                                            let (
                                                handler_arg,
                                                entity_name,
                                                entity_base,
                                                entity_system_id,
                                                entity_public_id,
                                                entity_notation,
                                            ) = {
                                                let entity = &*(*parser).m_declEntity;
                                                (
                                                    (*parser).m_handlerArg,
                                                    entity.named.name,
                                                    entity.base,
                                                    entity.systemId,
                                                    entity.publicId,
                                                    notation_pointer,
                                                )
                                            };
                                            let entity_base = entity_base.map_or(
                                                ::core::ptr::null(),
                                                |base| pool_string_pointer(
                                                    dtd_pool as *const STRING_POOL,
                                                    base,
                                                ),
                                            );
                                            let entity_system_id = entity_system_id.map_or(
                                                ::core::ptr::null(),
                                                |system_id| pool_string_pointer(
                                                    dtd_pool as *const STRING_POOL,
                                                    system_id,
                                                ),
                                            );
                                            let entity_public_id = entity_public_id.map_or(
                                                ::core::ptr::null(),
                                                |public_id| pool_string_pointer(
                                                    dtd_pool as *const STRING_POOL,
                                                    public_id,
                                                ),
                                            );
                                            if let Some(callback) = callback {
                                                *eventEndPP = s;
                                                callback.invoke(
                                                    handler_arg,
                                                    entity_name,
                                                    entity_base,
                                                    entity_system_id,
                                                    entity_public_id,
                                                    entity_notation,
                                                );
                                                handleDefault = crate::expat_h::XML_FALSE;
                                            } else if (*parser).m_entityDeclHandler {
                                                *eventEndPP = s;
                                                let callback = ENTITY_DECL_HANDLERS
                                                    .get_or_init(|| {
                                                        std::sync::Mutex::new(
                                                            std::collections::HashMap::new(),
                                                        )
                                                    })
                                                    .lock()
                                                    .unwrap_or_else(|poisoned| {
                                                        poisoned.into_inner()
                                                    })
                                                    .get(&(parser as usize))
                                                    .cloned();
                                                if let Some(callback) = callback {
                                                    callback.invoke(
                                                        handler_arg,
                                                        entity_name,
                                                        0 as ::core::ffi::c_int,
                                                        ::core::ptr::null::<
                                                            crate::expat_external_h::XML_Char,
                                                        >(
                                                        ),
                                                        0 as ::core::ffi::c_int,
                                                        entity_base,
                                                        entity_system_id,
                                                        entity_public_id,
                                                        entity_notation,
                                                    );
                                                }
                                                handleDefault = crate::expat_h::XML_FALSE;
                                            }
                                        }
                                        break 's_2375;
                                    }
                                    9 => {
                                        if crate::src::xmltok::predefined_entity_name(enc, s, next)
                                            != 0
                                        {
                                            (*parser).m_declEntity =
                                                ::core::ptr::null_mut::<ENTITY>();
                                            break 's_2375;
                                        } else {
                                            if (*dtd).keepProcessing != 0 {
                                                let mut name: *const crate::expat_external_h::XML_Char = poolStoreString(
                                                    dtd_pool,
                                                    enc,
                                                    s,
                                                    next,
                                                );
                                                if name.is_null() {
                                                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                                                }
                                                (*parser).m_declEntity = lookup(
                                                    parser,
                                                    &raw mut (*dtd).generalEntities,
                                                    name as KEY,
                                                    ::core::mem::size_of::<ENTITY>(),
                                                )
                                                    as *mut ENTITY;
                                                if (*parser).m_declEntity.is_null() {
                                                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                                                }
                                                if (*(*parser).m_declEntity).named.name != name
                                                {
                                                    (*dtd).pool.rewind();
                                                    (*parser).m_declEntity =
                                                        ::core::ptr::null_mut::<ENTITY>();
                                                } else {
                                                    (*dtd).pool.commit();
                                                    (*(*parser).m_declEntity).publicId = None;
                                                    (*(*parser).m_declEntity).is_param =
                                                        crate::expat_h::XML_FALSE;
                                                    (*(*parser).m_declEntity).is_internal =
                                                        !(!(*parser).m_parentParser.is_null()
                                                            || !(*parser)
                                                                .m_openInternalEntities
                                                                .is_null())
                                                            as ::core::ffi::c_int
                                                            as crate::expat_h::XML_Bool;
                                                    if (*parser).m_entityDeclHandler {
                                                        handleDefault = crate::expat_h::XML_FALSE;
                                                    }
                                                }
                                            } else {
                                                (*dtd).pool.rewind();
                                                (*parser).m_declEntity =
                                                    ::core::ptr::null_mut::<ENTITY>();
                                            }
                                            break 's_2375;
                                        }
                                    }
                                    10 => {
                                        if (*dtd).keepProcessing != 0 {
                                            let mut name_0: *const crate::expat_external_h::XML_Char =
                                                poolStoreString(dtd_pool, enc, s, next);
                                            if name_0.is_null() {
                                                return crate::expat_h::XML_ERROR_NO_MEMORY;
                                            }
                                            (*parser).m_declEntity = lookup(
                                                parser,
                                                &raw mut (*dtd).paramEntities,
                                                name_0 as KEY,
                                                ::core::mem::size_of::<ENTITY>(),
                                            )
                                                as *mut ENTITY;
                                            if (*parser).m_declEntity.is_null() {
                                                return crate::expat_h::XML_ERROR_NO_MEMORY;
                                            }
                                            if (*(*parser).m_declEntity).named.name != name_0
                                            {
                                                (*dtd).pool.rewind();
                                                (*parser).m_declEntity =
                                                    ::core::ptr::null_mut::<ENTITY>();
                                            } else {
                                                (*dtd).pool.commit();
                                                (*(*parser).m_declEntity).publicId = None;
                                                (*(*parser).m_declEntity).is_param =
                                                    crate::expat_h::XML_TRUE;
                                                (*(*parser).m_declEntity).is_internal = !(!(*parser)
                                                    .m_parentParser
                                                    .is_null()
                                                    || !(*parser).m_openInternalEntities.is_null())
                                                    as ::core::ffi::c_int
                                                    as crate::expat_h::XML_Bool;
                                                if (*parser).m_entityDeclHandler {
                                                    handleDefault = crate::expat_h::XML_FALSE;
                                                }
                                            }
                                        } else {
                                            (*dtd).pool.rewind();
                                            (*parser).m_declEntity =
                                                ::core::ptr::null_mut::<ENTITY>();
                                        }
                                        break 's_2375;
                                    }
                                    18 => {
                                        (*parser).m_declNotationPublicId =
                                            ::core::ptr::null::<crate::expat_external_h::XML_Char>(
                                            );
                                        (*parser).m_declNotationName =
                                            ::core::ptr::null::<crate::expat_external_h::XML_Char>(
                                            );
                                        if (*parser).m_notationDeclHandler {
                                            (*parser).m_declNotationName = poolStoreString(
                                                &raw mut (*parser).m_tempPool,
                                                enc,
                                                s,
                                                next,
                                            );
                                            if (*parser).m_declNotationName.is_null() {
                                                return crate::expat_h::XML_ERROR_NO_MEMORY;
                                            }
                                            (*parser).m_tempPool.commit();
                                            handleDefault = crate::expat_h::XML_FALSE;
                                        }
                                        break 's_2375;
                                    }
                                    21 => {
                                        if crate::src::xmltok::check_public_id(
                                            (*enc).isPublicId,
                                            enc,
                                            s,
                                            next,
                                            eventPP,
                                        ) == 0
                                        {
                                            return crate::expat_h::XML_ERROR_PUBLICID;
                                        }
                                        if !(*parser).m_declNotationName.is_null() {
                                            let mut tem_0: *mut crate::expat_external_h::XML_Char =
                                                poolStoreString(
                                                    &raw mut (*parser).m_tempPool,
                                                    enc,
                                                    s.offset((*enc).minBytesPerChar as isize),
                                                    next.offset(-((*enc).minBytesPerChar as isize)),
                                                );
                                            if tem_0.is_null() {
                                                return crate::expat_h::XML_ERROR_NO_MEMORY;
                                            }
                                            normalizePublicId(tem_0);
                                            (*parser).m_declNotationPublicId = tem_0;
                                            (*parser).m_tempPool.commit();
                                            handleDefault = crate::expat_h::XML_FALSE;
                                        }
                                        break 's_2375;
                                    }
                                    19 => {
                                        if !(*parser).m_declNotationName.is_null()
                                            && (*parser).m_notationDeclHandler
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
                                            let callback = NOTATION_DECL_HANDLERS
                                                .get_or_init(|| {
                                                    std::sync::Mutex::new(
                                                        std::collections::HashMap::new(),
                                                    )
                                                })
                                                .lock()
                                                .unwrap_or_else(|poisoned| {
                                                    poisoned.into_inner()
                                                })
                                                .get(&(parser as usize))
                                                .cloned();
                                            if let Some(callback) = callback {
                                                callback.invoke(
                                                    (*parser).m_handlerArg,
                                                    (*parser).m_declNotationName,
                                                    (*parser).m_curBase,
                                                    systemId,
                                                    (*parser).m_declNotationPublicId,
                                                );
                                                handleDefault = crate::expat_h::XML_FALSE;
                                            }
                                        }
                                        poolClear(&raw mut (*parser).m_tempPool);
                                        break 's_2375;
                                    }
                                    20 => {
                                        if !(*parser).m_declNotationPublicId.is_null()
                                            && (*parser).m_notationDeclHandler
                                        {
                                            *eventEndPP = s;
                                            let callback = NOTATION_DECL_HANDLERS
                                                .get_or_init(|| {
                                                    std::sync::Mutex::new(
                                                        std::collections::HashMap::new(),
                                                    )
                                                })
                                                .lock()
                                                .unwrap_or_else(|poisoned| {
                                                    poisoned.into_inner()
                                                })
                                                .get(&(parser as usize))
                                                .cloned();
                                            if let Some(callback) = callback {
                                                callback.invoke(
                                                    (*parser).m_handlerArg,
                                                    (*parser).m_declNotationName,
                                                    (*parser).m_curBase,
                                                    ::core::ptr::null::<
                                                        crate::expat_external_h::XML_Char,
                                                    >(),
                                                    (*parser).m_declNotationPublicId,
                                                );
                                                handleDefault = crate::expat_h::XML_FALSE;
                                            }
                                        }
                                        poolClear(&raw mut (*parser).m_tempPool);
                                        break 's_2375;
                                    }
                                    -1 => match tok {
                                        crate::src::xmltok::XML_TOK_PARAM_ENTITY_REF => {
                                            return crate::expat_h::XML_ERROR_PARAM_ENTITY_REF;
                                        }
                                        crate::src::xmltok::XML_TOK_XML_DECL => {
                                            return crate::expat_h::XML_ERROR_MISPLACED_XML_PI
                                        }
                                        _ => return crate::expat_h::XML_ERROR_SYNTAX,
                                    },
                                    58 => {
                                        let mut result_3: crate::expat_h::XML_Error =
                                            crate::expat_h::XML_ERROR_NONE;
                                        if (*parser).m_defaultHandler {
                                            reportDefault(parser, enc, s, next);
                                        }
                                        handleDefault = crate::expat_h::XML_FALSE;
                                        result_3 = doIgnoreSection(
                                            parser,
                                            enc,
                                            &raw mut next,
                                            end,
                                            nextPtr,
                                            haveMore,
                                        );
                                        if result_3 as ::core::ffi::c_uint
                                            != crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int
                                                as ::core::ffi::c_uint
                                        {
                                            return result_3;
                                        } else if next.is_null() {
                                            (*parser).m_processor = ProcessorState::IgnoreSection;
                                            return result_3;
                                        }
                                        break 's_2375;
                                    }
                                    44 => {
                                        if (*parser).m_prologState.level >= (*parser).m_groupSize {
                                            if (*parser).m_groupSize != 0 {
                                                if (*parser).m_groupSize
                                                    > (-1 as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint)
                                                        .wrapping_div(2 as ::core::ffi::c_uint)
                                                {
                                                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                                                }
                                                (*parser).m_groupSize = (*parser)
                                                    .m_groupSize
                                                    .wrapping_mul(2 as ::core::ffi::c_uint);
                                                let new_connector: *mut ::core::ffi::c_char =
                                                    expat_realloc(
                                                        parser,
                                                        (*parser).m_groupConnector
                                                            as *mut ::core::ffi::c_void,
                                                        (*parser).m_groupSize
                                                            as crate::__stddef_size_t_h::size_t,
                                                        5915 as ::core::ffi::c_int,
                                                    )
                                                        as *mut ::core::ffi::c_char;
                                                if new_connector.is_null() {
                                                    (*parser).m_groupSize = (*parser)
                                                        .m_groupSize
                                                        .wrapping_div(2 as ::core::ffi::c_uint);
                                                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                                                }
                                                (*parser).m_groupConnector = new_connector;
                                                let mut scaff_index = (*dtd)
                                                    .scaffIndex
                                                    .lock()
                                                    .unwrap_or_else(|poisoned| poisoned.into_inner());
                                                let additional = ((*parser).m_groupSize as usize)
                                                    .saturating_sub(scaff_index.len());
                                                if !scaff_index.is_empty()
                                                    && scaff_index
                                                        .try_reserve_exact(additional)
                                                        .is_err()
                                                {
                                                    (*parser).m_groupSize = (*parser)
                                                        .m_groupSize
                                                        .wrapping_div(2 as ::core::ffi::c_uint);
                                                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                                                }
                                            } else {
                                                (*parser).m_groupSize = 32 as ::core::ffi::c_uint;
                                                (*parser).m_groupConnector = expat_malloc(
                                                    parser,
                                                    (*parser).m_groupSize
                                                        as crate::__stddef_size_t_h::size_t,
                                                    5944 as ::core::ffi::c_int,
                                                )
                                                    as *mut ::core::ffi::c_char;
                                                if (*parser).m_groupConnector.is_null() {
                                                    (*parser).m_groupSize =
                                                        0 as ::core::ffi::c_uint;
                                                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                                                }
                                            }
                                        }
                                        *(*parser)
                                            .m_groupConnector
                                            .offset((*parser).m_prologState.level as isize) =
                                            0 as ::core::ffi::c_char;
                                        if (*dtd).in_eldecl != 0 {
                                            let mut myindex: ::core::ffi::c_int =
                                                nextScaffoldPart(parser);
                                            if myindex < 0 as ::core::ffi::c_int {
                                                return crate::expat_h::XML_ERROR_NO_MEMORY;
                                            }
                                            let mut scaff_index = (*dtd)
                                                .scaffIndex
                                                .lock()
                                                .unwrap_or_else(|poisoned| poisoned.into_inner());
                                            let level = (*dtd).scaffLevel as usize;
                                            if scaff_index.len() <= level {
                                                let additional = level + 1 - scaff_index.len();
                                                if scaff_index
                                                    .try_reserve_exact(additional)
                                                    .is_err()
                                                {
                                                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                                                }
                                                scaff_index.resize(level + 1, 0);
                                            }
                                            scaff_index[level] = myindex;
                                            (*dtd).scaffLevel += 1;
                                            let mut scaffold = (*dtd)
                                                .scaffold
                                                .lock()
                                                .unwrap_or_else(|poisoned| poisoned.into_inner());
                                            let Some(node) = scaffold.nodes.get_mut(myindex as usize) else {
                                                return crate::expat_h::XML_ERROR_SYNTAX;
                                            };
                                            node.type_0 = crate::expat_h::XML_CTYPE_SEQ;
                                            if (*parser).m_elementDeclHandler {
                                                handleDefault = crate::expat_h::XML_FALSE;
                                            }
                                        }
                                        break 's_2375;
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
                                            && (*parser).m_elementDeclHandler
                                        {
                                            handleDefault = crate::expat_h::XML_FALSE;
                                        }
                                        break 's_2375;
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
                                        {
                                            let parent_index = {
                                                let scaff_index = (*dtd)
                                                    .scaffIndex
                                                    .lock()
                                                    .unwrap_or_else(|poisoned| poisoned.into_inner());
                                                match scaff_index
                                                    .get(((*dtd).scaffLevel - 1) as usize)
                                                    .copied()
                                                {
                                                    Some(index) => index,
                                                    None => return crate::expat_h::XML_ERROR_SYNTAX,
                                                }
                                            };
                                            let mut scaffold = (*dtd)
                                                .scaffold
                                                .lock()
                                                .unwrap_or_else(|poisoned| poisoned.into_inner());
                                            let Some(parent) = scaffold.nodes.get_mut(parent_index as usize) else {
                                                return crate::expat_h::XML_ERROR_SYNTAX;
                                            };
                                            if parent.type_0 as ::core::ffi::c_uint
                                                != crate::expat_h::XML_CTYPE_MIXED
                                                    as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint
                                            {
                                                parent.type_0 = crate::expat_h::XML_CTYPE_CHOICE;
                                                if (*parser).m_elementDeclHandler {
                                                    handleDefault = crate::expat_h::XML_FALSE;
                                                }
                                            }
                                        }
                                        *(*parser)
                                            .m_groupConnector
                                            .offset((*parser).m_prologState.level as isize) =
                                            crate::ascii_h::ASCII_PIPE as ::core::ffi::c_char;
                                        break 's_2375;
                                    }
                                    60 | 59 => {
                                        (*dtd).hasParamEntityRefs = crate::expat_h::XML_TRUE;
                                        if (*parser).m_paramEntityParsing as u64 == 0 {
                                            (*dtd).keepProcessing = (*dtd).standalone;
                                        } else {
                                            let mut name_1: *const crate::expat_external_h::XML_Char =
                                                ::core::ptr::null:: <crate::expat_external_h::XML_Char>();
                                            let mut entity_1: *mut ENTITY =
                                                ::core::ptr::null_mut::<ENTITY>();
                                            name_1 = poolStoreString(
                                                dtd_pool,
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
                                            )
                                                as *mut ENTITY;
                                            (*dtd).pool.rewind();
                                            if (*parser).m_prologState.documentEntity != 0
                                                && (if (*dtd).standalone as ::core::ffi::c_int != 0
                                                {
                                                    (*parser).m_openInternalEntities.is_null()
                                                        as ::core::ffi::c_int
                                                } else {
                                                    ((*dtd).hasParamEntityRefs == 0)
                                                        as ::core::ffi::c_int
                                                }) != 0
                                            {
                                                if entity_1.is_null() {
                                                    return crate::expat_h::XML_ERROR_UNDEFINED_ENTITY;
                                                } else if (*entity_1).is_internal == 0 {
                                                    return crate::expat_h::XML_ERROR_ENTITY_DECLARED_IN_PE;
                                                }
                                            } else if entity_1.is_null() {
                                                (*dtd).keepProcessing = (*dtd).standalone;
                                                if role
                                                    == crate::src::xmlrole::XML_ROLE_PARAM_ENTITY_REF
                                                        as ::core::ffi::c_int
                                                    && (*parser).m_skippedEntityHandler
                                                {
                                                    let callback = SKIPPED_ENTITY_HANDLERS
                                                        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
                                                        .lock()
                                                        .unwrap_or_else(|poisoned| poisoned.into_inner())
                                                        .get(&(parser as usize))
                                                        .cloned();
                                                    if let Some(callback) = callback {
                                                        callback.invoke(
                                                            (*parser).m_handlerArg,
                                                            name_1,
                                                            1 as ::core::ffi::c_int,
                                                        );
                                                    }
                                                    handleDefault = crate::expat_h::XML_FALSE;
                                                }
                                                break 's_2375;
                                            }
                                            if (*entity_1).open != 0 {
                                                return crate::expat_h::XML_ERROR_RECURSIVE_ENTITY_REF;
                                            }
                                            if (*entity_1).textPtr.is_some() {
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
                                                result_4 = processEntity(
                                                    parser,
                                                    entity_1,
                                                    betweenDecl,
                                                    ENTITY_INTERNAL,
                                                );
                                                if result_4 as ::core::ffi::c_uint
                                                    != crate::expat_h::XML_ERROR_NONE
                                                        as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint
                                                {
                                                    return result_4;
                                                }
                                                handleDefault = crate::expat_h::XML_FALSE;
                                                break 's_2375;
                                            } else if (*parser).m_externalEntityRefHandler
                                            {
                                                (*dtd).paramEntityRead = crate::expat_h::XML_FALSE;
                                                (*entity_1).open = crate::expat_h::XML_TRUE;
                                                entityTrackingOnOpen(
                                                    parser,
                                                    entity_1,
                                                    6057 as ::core::ffi::c_int,
                                                );
                                                let handler = EXTERNAL_ENTITY_REF_HANDLERS
                                                    .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
                                                    .lock()
                                                    .unwrap_or_else(|poisoned| poisoned.into_inner())
                                                    .get(&(parser as usize))
                                                    .cloned()
                                                    .expect("installed external entity handler");
                                                if invoke_external_entity_ref_handler(
                                                    handler.as_ref(),
                                                    (*parser).m_externalEntityRefHandlerArg,
                                                    ::core::ptr::null(),
                                                    dtd_pool as *const STRING_POOL,
                                                    entity_1,
                                                ) == 0
                                                {
                                                    entityTrackingOnClose(
                                                        parser,
                                                        entity_1,
                                                        6061 as ::core::ffi::c_int,
                                                    );
                                                    (*entity_1).open = crate::expat_h::XML_FALSE;
                                                    return crate::expat_h::XML_ERROR_EXTERNAL_ENTITY_HANDLING;
                                                }
                                                entityTrackingOnClose(
                                                    parser,
                                                    entity_1,
                                                    6065 as ::core::ffi::c_int,
                                                );
                                                (*entity_1).open = crate::expat_h::XML_FALSE;
                                                handleDefault = crate::expat_h::XML_FALSE;
                                                if (*dtd).paramEntityRead == 0 {
                                                    (*dtd).keepProcessing = (*dtd).standalone;
                                                    break 's_2375;
                                                }
                                            } else {
                                                (*dtd).keepProcessing = (*dtd).standalone;
                                                break 's_2375;
                                            }
                                        }
                                        if (*dtd).standalone == 0
                                            && (*parser).m_notStandaloneHandler.is_some()
                                            && (*parser)
                                                .m_notStandaloneHandler
                                                .expect("non-null function pointer")(
                                                (*parser).m_handlerArg,
                                            ) == 0
                                        {
                                            return crate::expat_h::XML_ERROR_NOT_STANDALONE;
                                        }
                                        break 's_2375;
                                    }
                                    40 => {
                                        if (*parser).m_elementDeclHandler {
                                            (*parser).m_declElementType =
                                                getElementType(parser, enc, s, next);
                                            if (*parser).m_declElementType.is_null() {
                                                return crate::expat_h::XML_ERROR_NO_MEMORY;
                                            }
                                            (*dtd).scaffLevel = 0 as ::core::ffi::c_int;
                                            (*dtd).scaffCount = 0 as ::core::ffi::c_uint;
                                            (*dtd).in_eldecl = crate::expat_h::XML_TRUE;
                                            handleDefault = crate::expat_h::XML_FALSE;
                                        }
                                        break 's_2375;
                                    }
                                    41 | 42 => {
                                        if (*dtd).in_eldecl != 0 {
                                            if (*parser).m_elementDeclHandler {
                                                let mut content: *mut crate::expat_h::XML_Content =
                                                    (*parser)
                                                        .m_mem
                                                        .malloc_fcn
                                                        .expect("non-null function pointer")(
                                                        ::core::mem::size_of::<
                                                            crate::expat_h::XML_Content,
                                                        >(
                                                        ),
                                                    )
                                                        as *mut crate::expat_h::XML_Content;
                                                if content.is_null() {
                                                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                                                }
                                                (*content).quant = crate::expat_h::XML_CQUANT_NONE;
                                                (*content).name = ::core::ptr::null_mut::<
                                                    crate::expat_external_h::XML_Char,
                                                >(
                                                );
                                                (*content).numchildren = 0 as ::core::ffi::c_uint;
                                                (*content).children = ::core::ptr::null_mut::<
                                                    crate::expat_h::XML_Content,
                                                >(
                                                );
                                                (*content).type_0 = (if role
                                                    == crate::src::xmlrole::XML_ROLE_CONTENT_ANY
                                                        as ::core::ffi::c_int
                                                {
                                                    crate::expat_h::XML_CTYPE_ANY
                                                        as ::core::ffi::c_int
                                                } else {
                                                    crate::expat_h::XML_CTYPE_EMPTY
                                                        as ::core::ffi::c_int
                                                })
                                                    as crate::expat_h::XML_Content_Type;
                                                *eventEndPP = s;
                                                callElementDeclHandler(
                                                    parser,
                                                    (*(*parser).m_declElementType).named.name,
                                                    content,
                                                );
                                                handleDefault = crate::expat_h::XML_FALSE;
                                            }
                                            (*dtd).in_eldecl = crate::expat_h::XML_FALSE;
                                        }
                                        break 's_2375;
                                    }
                                    43 => {
                                        if (*dtd).in_eldecl != 0 {
                                            let parent_index = {
                                                let scaff_index = (*dtd)
                                                    .scaffIndex
                                                    .lock()
                                                    .unwrap_or_else(|poisoned| poisoned.into_inner());
                                                match scaff_index
                                                    .get(((*dtd).scaffLevel - 1) as usize)
                                                    .copied()
                                                {
                                                    Some(index) => index,
                                                    None => return crate::expat_h::XML_ERROR_SYNTAX,
                                                }
                                            };
                                            let mut scaffold = (*dtd)
                                                .scaffold
                                                .lock()
                                                .unwrap_or_else(|poisoned| poisoned.into_inner());
                                            let Some(parent) = scaffold.nodes.get_mut(parent_index as usize) else {
                                                return crate::expat_h::XML_ERROR_SYNTAX;
                                            };
                                            parent.type_0 = crate::expat_h::XML_CTYPE_MIXED;
                                            if (*parser).m_elementDeclHandler {
                                                handleDefault = crate::expat_h::XML_FALSE;
                                            }
                                        }
                                        break 's_2375;
                                    }
                                    51 => {
                                        quant = crate::expat_h::XML_CQUANT_NONE;
                                        break '_elementContent;
                                    }
                                    53 => {
                                        quant = crate::expat_h::XML_CQUANT_OPT;
                                        break '_elementContent;
                                    }
                                    52 => {
                                        quant = crate::expat_h::XML_CQUANT_REP;
                                        break '_elementContent;
                                    }
                                    54 => {
                                        quant = crate::expat_h::XML_CQUANT_PLUS;
                                        break '_elementContent;
                                    }
                                    45 => {
                                        quant = crate::expat_h::XML_CQUANT_NONE;
                                        break '_closeGroup;
                                    }
                                    47 => {
                                        quant = crate::expat_h::XML_CQUANT_OPT;
                                        break '_closeGroup;
                                    }
                                    46 => {
                                        quant = crate::expat_h::XML_CQUANT_REP;
                                        break '_closeGroup;
                                    }
                                    48 => {
                                        quant = crate::expat_h::XML_CQUANT_PLUS;
                                        break '_closeGroup;
                                    }
                                    55 => {
                                        if reportProcessingInstruction(parser, enc, s, next) == 0 {
                                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                                        }
                                        handleDefault = crate::expat_h::XML_FALSE;
                                        break 's_2375;
                                    }
                                    56 => {
                                        if reportComment(parser, enc, s, next) == 0 {
                                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                                        }
                                        handleDefault = crate::expat_h::XML_FALSE;
                                        break 's_2375;
                                    }
                                    0 => {
                                        match tok {
                                            crate::src::xmltok::XML_TOK_BOM => {
                                                handleDefault = crate::expat_h::XML_FALSE;
                                            }
                                            _ => {}
                                        }
                                        break 's_2375;
                                    }
                                    3 => {
                                        if (*parser).m_startDoctypeDeclHandler {
                                            handleDefault = crate::expat_h::XML_FALSE;
                                        }
                                        break 's_2375;
                                    }
                                    11 => {
                                        if (*dtd).keepProcessing as ::core::ffi::c_int != 0
                                            && (*parser).m_entityDeclHandler
                                        {
                                            handleDefault = crate::expat_h::XML_FALSE;
                                        }
                                        break 's_2375;
                                    }
                                    17 => {
                                        if (*parser).m_notationDeclHandler {
                                            handleDefault = crate::expat_h::XML_FALSE;
                                        }
                                        break 's_2375;
                                    }
                                    33 => {
                                        if (*dtd).keepProcessing as ::core::ffi::c_int != 0
                                            && (*parser).m_attlistDeclHandler
                                        {
                                            handleDefault = crate::expat_h::XML_FALSE;
                                        }
                                        break 's_2375;
                                    }
                                    39 => {
                                        if (*parser).m_elementDeclHandler {
                                            handleDefault = crate::expat_h::XML_FALSE;
                                        }
                                        break 's_2375;
                                    }
                                    _ => {
                                        break 's_2375;
                                    }
                                }
                                if crate::src::xmltok::check_public_id(
                                    (*enc).isPublicId,
                                    enc,
                                    s,
                                    next,
                                    eventPP,
                                ) == 0
                                {
                                    return crate::expat_h::XML_ERROR_PUBLICID;
                                }
                                break '_alreadyChecked;
                            }
                            if (*dtd).keepProcessing as ::core::ffi::c_int != 0
                                && !(*parser).m_declEntity.is_null()
                            {
                                let system_id = poolStoreString(
                                    dtd_pool,
                                    enc,
                                    s.offset((*enc).minBytesPerChar as isize),
                                    next.offset(-((*enc).minBytesPerChar as isize)),
                                );
                                let Some(system_id) = pool_string_ref(
                                    dtd_pool as *const STRING_POOL,
                                    system_id,
                                    false,
                                ) else {
                                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                                };
                                (*(*parser).m_declEntity).systemId = Some(system_id);
                                (*(*parser).m_declEntity).base = pool_string_ref(
                                    dtd_pool as *const STRING_POOL,
                                    (*parser).m_curBase,
                                    false,
                                );
                                (*dtd).pool.commit();
                                if (*parser).m_entityDeclHandler
                                    && role
                                        == crate::src::xmlrole::XML_ROLE_ENTITY_SYSTEM_ID
                                            as ::core::ffi::c_int
                                {
                                    handleDefault = crate::expat_h::XML_FALSE;
                                }
                            }
                            break 's_2375;
                        }
                        if (*dtd).keepProcessing as ::core::ffi::c_int != 0
                            && (*parser).m_attlistDeclHandler
                        {
                            handleDefault = crate::expat_h::XML_FALSE;
                        }
                        break 's_2375;
                    }
                    if (*dtd).in_eldecl != 0 {
                        let mut el: *mut ELEMENT_TYPE = ::core::ptr::null_mut::<ELEMENT_TYPE>();
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
                        {
                            let mut scaffold = (*dtd)
                                .scaffold
                                .lock()
                                .unwrap_or_else(|poisoned| poisoned.into_inner());
                            let Some(node) = scaffold.nodes.get_mut(myindex_0 as usize) else {
                                return crate::expat_h::XML_ERROR_SYNTAX;
                            };
                            node.type_0 = crate::expat_h::XML_CTYPE_NAME;
                            node.quant = quant;
                        }
                        el = getElementType(parser, enc, s, nxt);
                        if el.is_null() {
                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                        }
                        let name_2 = (*el).named.name;
                        let name_ref = pool_string_ref(&raw const (*dtd).pool, name_2, false)
                            .expect("element type table entries always have a DTD pool name");
                        {
                            let mut scaffold = (*dtd)
                                .scaffold
                                .lock()
                                .unwrap_or_else(|poisoned| poisoned.into_inner());
                            let Some(node) = scaffold.nodes.get_mut(myindex_0 as usize) else {
                                return crate::expat_h::XML_ERROR_SYNTAX;
                            };
                            node.name = Some(name_ref);
                        }
                        nameLen = 0 as crate::__stddef_size_t_h::size_t;
                        loop {
                            let c2rust_fresh5 = nameLen;
                            nameLen = nameLen.wrapping_add(1);
                            if *name_2.offset(c2rust_fresh5 as isize) == 0 {
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
                        if (*parser).m_elementDeclHandler {
                            handleDefault = crate::expat_h::XML_FALSE;
                        }
                    }
                    break 's_2375;
                }
                if (*dtd).in_eldecl != 0 {
                    if (*parser).m_elementDeclHandler {
                        handleDefault = crate::expat_h::XML_FALSE;
                    }
                    (*dtd).scaffLevel -= 1;
                    let parent_index = {
                        let scaff_index = (*dtd)
                            .scaffIndex
                            .lock()
                            .unwrap_or_else(|poisoned| poisoned.into_inner());
                        match scaff_index.get((*dtd).scaffLevel as usize).copied() {
                            Some(index) => index,
                            None => return crate::expat_h::XML_ERROR_SYNTAX,
                        }
                    };
                    {
                        let mut scaffold = (*dtd)
                            .scaffold
                            .lock()
                            .unwrap_or_else(|poisoned| poisoned.into_inner());
                        let Some(parent) = scaffold.nodes.get_mut(parent_index as usize) else {
                            return crate::expat_h::XML_ERROR_SYNTAX;
                        };
                        parent.quant = quant;
                    }
                    if (*dtd).scaffLevel == 0 as ::core::ffi::c_int {
                        if handleDefault == 0 {
                            let mut model: *mut crate::expat_h::XML_Content = build_model(parser);
                            if model.is_null() {
                                return crate::expat_h::XML_ERROR_NO_MEMORY;
                            }
                            *eventEndPP = s;
                            callElementDeclHandler(
                                parser,
                                (*(*parser).m_declElementType).named.name,
                                model,
                            );
                        }
                        (*dtd).in_eldecl = crate::expat_h::XML_FALSE;
                        (*dtd).contentStringLen = 0 as ::core::ffi::c_uint;
                    }
                }
                break 's_2375;
            }
            if (*dtd).keepProcessing as ::core::ffi::c_int != 0 && !(*parser).m_declEntity.is_null()
            {
                let mut tem: *mut crate::expat_external_h::XML_Char = poolStoreString(
                    dtd_pool,
                    enc,
                    s.offset((*enc).minBytesPerChar as isize),
                    next.offset(-((*enc).minBytesPerChar as isize)),
                );
                if tem.is_null() {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
                normalizePublicId(tem);
                (*(*parser).m_declEntity).publicId = pool_string_ref(dtd_pool, tem, false);
                if (*(*parser).m_declEntity).publicId.is_none() {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
                (*dtd).pool.commit();
                if (*parser).m_entityDeclHandler
                    && role == crate::src::xmlrole::XML_ROLE_ENTITY_PUBLIC_ID as ::core::ffi::c_int
                {
                    handleDefault = crate::expat_h::XML_FALSE;
                }
            }
        }
        if handleDefault as ::core::ffi::c_int != 0 && (*parser).m_defaultHandler {
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
        tok = (*enc).scanners[0 as usize].scan(enc, s, end, &raw mut next);
    }
}

unsafe extern "C" fn epilogProcessor(
    mut parser: crate::expat_h::XML_Parser,
    mut s: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextPtr: *mut *const ::core::ffi::c_char,
) -> crate::expat_h::XML_Error {
    (*parser).m_processor = ProcessorState::Epilog;
    (*parser).m_eventPtr = s;
    loop {
        let mut next: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut tok: ::core::ffi::c_int = (*parser_encoding(parser)).scanners[0 as usize].scan(
            parser_encoding(parser),
            s,
            end,
            &raw mut next,
        );
        if accountingDiffTolerated(
            parser,
            tok,
            s,
            next,
            6279 as ::core::ffi::c_int,
            XML_ACCOUNT_DIRECT,
        ) == 0
        {
            accountingOnAbort(parser);
            return crate::expat_h::XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
        }
        (*parser).m_eventEndPtr = next;
        match tok {
            -15 => {
                if (*parser).m_defaultHandler {
                    reportDefault(parser, parser_encoding(parser), s, next);
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
                if (*parser).m_defaultHandler {
                    reportDefault(parser, parser_encoding(parser), s, next);
                }
            }
            crate::src::xmltok::XML_TOK_PI => {
                if reportProcessingInstruction(parser, parser_encoding(parser), s, next) == 0 {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
            }
            crate::src::xmltok::XML_TOK_COMMENT => {
                if reportComment(parser, parser_encoding(parser), s, next) == 0 {
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
            (*parser).m_processor = ProcessorState::InternalEntity;
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
            ::core::mem::size_of::<OPEN_INTERNAL_ENTITY>(),
            6382 as ::core::ffi::c_int,
        ) as *mut OPEN_INTERNAL_ENTITY;
        if openEntity.is_null() {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
    }
    (*entity).open = crate::expat_h::XML_TRUE;
    (*entity).hasMore = crate::expat_h::XML_TRUE;
    entityTrackingOnOpen(parser, entity, 6389 as ::core::ffi::c_int);
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
        triggerReenter(parser);
    }
    return crate::expat_h::XML_ERROR_NONE;
}

unsafe extern "C" fn internalEntityProcessor(
    mut parser: crate::expat_h::XML_Parser,
    _s: *const ::core::ffi::c_char,
    _end: *const ::core::ffi::c_char,
    _nextPtr: *mut *const ::core::ffi::c_char,
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
        let Some(text) = (*entity).textPtr.present() else {
            return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
        };
        let dtd = (*parser).m_dtd;
        if dtd.is_null() {
            return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
        }
        let Some(text) = entity_text_chars(&*dtd, text, (*entity).textLen) else {
            return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
        };
        let Ok(processed) = usize::try_from((*entity).processed) else {
            return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
        };
        let Some(unprocessed) = text.get(processed..) else {
            return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
        };
        textStart = unprocessed.as_ptr().cast::<::core::ffi::c_char>();
        textEnd = text
            .as_ptr()
            .wrapping_add(text.len())
            .cast::<::core::ffi::c_char>();
        next = textStart;
        if (*entity).is_param != 0 {
            let mut tok: ::core::ffi::c_int = (*(*parser).m_internalEncoding).scanners[0 as usize]
                .scan(
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
            let Some(processed) = next.addr().checked_sub(textStart.addr()) else {
                return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
            };
            let Ok(processed) = ::core::ffi::c_int::try_from(processed) else {
                return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
            };
            (*entity).processed = (*entity).processed.saturating_add(processed);
            return result;
        }
        (*entity).hasMore = crate::expat_h::XML_FALSE;
        if (*entity).is_param == 0 && (*openEntity).startTagLevel != (*parser).m_tagLevel {
            return crate::expat_h::XML_ERROR_ASYNC_ENTITY;
        }
        triggerReenter(parser);
        return result;
    }
    entityTrackingOnClose(parser, entity, 6470 as ::core::ffi::c_int);
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
            ProcessorState::Prolog
        } else {
            ProcessorState::Content
        };
    }
    triggerReenter(parser);
    return crate::expat_h::XML_ERROR_NONE;
}

unsafe extern "C" fn errorProcessor(
    mut parser: crate::expat_h::XML_Parser,
    _s: *const ::core::ffi::c_char,
    _end: *const ::core::ffi::c_char,
    _nextPtr: *mut *const ::core::ffi::c_char,
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
            let Some(text) = (*entity).textPtr.present() else {
                return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
            };
            let dtd = (*parser).m_dtd;
            if dtd.is_null() {
                return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
            }
            let Some(text) = entity_text_chars(&*dtd, text, (*entity).textLen) else {
                return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
            };
            let Ok(processed) = usize::try_from((*entity).processed) else {
                return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
            };
            let Some(unprocessed) = text.get(processed..) else {
                return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
            };
            let textStart = unprocessed.as_ptr().cast::<::core::ffi::c_char>();
            let textEnd = text
                .as_ptr()
                .wrapping_add(text.len())
                .cast::<::core::ffi::c_char>();
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
                    let Some(processed) = nextInEntity.addr().checked_sub(textStart.addr()) else {
                        return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
                    };
                    let Ok(processed) = ::core::ffi::c_int::try_from(processed) else {
                        return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
                    };
                    (*entity).processed = (*entity).processed.saturating_add(processed);
                    continue;
                } else {
                    (*entity).hasMore = crate::expat_h::XML_FALSE;
                    continue;
                }
            } else {
                entityTrackingOnClose(parser, entity, 6547 as ::core::ffi::c_int);
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
        && (*pool).ptr_offset != 0
        && (*pool).last_cursor_char() == Some(0x20 as crate::expat_external_h::XML_Char)
    {
        (*pool).discard_last_cursor_char();
    }
    if if (*pool).is_full()
        && poolGrow(pool) == 0
    {
        0 as ::core::ffi::c_int
    } else {
        if (&mut *pool).write_cursor('\0' as crate::expat_external_h::XML_Char) {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }
    } == 0
    {
        return crate::expat_h::XML_ERROR_NO_MEMORY;
    }
    return crate::expat_h::XML_ERROR_NONE;
}

/// Encodes a validated XML character reference without exposing the temporary
/// output buffer as a raw pointer.
fn encode_xml_char_ref(
    c: ::core::ffi::c_int,
) -> ([crate::expat_external_h::XML_Char; 4], ::core::ffi::c_int) {
    let mut buf = [0; 4];
    if c < 0 {
        return (buf, 0);
    }
    if c < 0x80 {
        buf[0] = c as crate::expat_external_h::XML_Char;
        return (buf, 1);
    }
    if c < 0x800 {
        buf[0] = (c >> 6 | 0xc0) as crate::expat_external_h::XML_Char;
        buf[1] = (c & 0x3f | 0x80) as crate::expat_external_h::XML_Char;
        return (buf, 2);
    }
    if c < 0x10000 {
        buf[0] = (c >> 12 | 0xe0) as crate::expat_external_h::XML_Char;
        buf[1] = (c >> 6 & 0x3f | 0x80) as crate::expat_external_h::XML_Char;
        buf[2] = (c & 0x3f | 0x80) as crate::expat_external_h::XML_Char;
        return (buf, 3);
    }
    if c < 0x110000 {
        buf[0] = (c >> 18 | 0xf0) as crate::expat_external_h::XML_Char;
        buf[1] = (c >> 12 & 0x3f | 0x80) as crate::expat_external_h::XML_Char;
        buf[2] = (c >> 6 & 0x3f | 0x80) as crate::expat_external_h::XML_Char;
        buf[3] = (c & 0x3f | 0x80) as crate::expat_external_h::XML_Char;
        return (buf, 4);
    }
    (buf, 0)
}

/// Appends one character to a valid string pool.  Growth happens before the
/// temporary mutable borrow, so no borrow is held across an allocator callback.
unsafe fn pool_append_char(
    pool: *mut STRING_POOL,
    value: crate::expat_external_h::XML_Char,
) -> bool {
    if (*pool).is_full() && poolGrow(pool) == 0 {
        return false;
    }

    (&mut *pool).write_cursor(value)
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
        let scanner = match (*enc).literalScanners[0] {
            crate::src::xmltok::LiteralScanner::NormalAttributeValue => {
                crate::src::xmltok::xmltok_impl_c::normal_attributeValueTok
            }
            crate::src::xmltok::LiteralScanner::Little2AttributeValue => {
                crate::src::xmltok::xmltok_impl_c::little2_attributeValueTok
            }
            crate::src::xmltok::LiteralScanner::Big2AttributeValue => {
                crate::src::xmltok::xmltok_impl_c::big2_attributeValueTok
            }
            _ => unreachable!("attribute literal scanner must match its table slot"),
        };
        let mut tok: ::core::ffi::c_int = scanner(enc, ptr, end, &raw mut next);
        if accountingDiffTolerated(parser, tok, ptr, next, 6591 as ::core::ffi::c_int, account) == 0
        {
            accountingOnAbort(parser);
            return crate::expat_h::XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
        }
        's_350: {
            match tok {
                crate::src::xmltok::XML_TOK_NONE => {
                    if !nextPtr.is_null() {
                        *nextPtr = next;
                    }
                    return crate::expat_h::XML_ERROR_NONE;
                }
                crate::src::xmltok::XML_TOK_INVALID => {
                    if enc == parser_encoding(parser) {
                        (*parser).m_eventPtr = next;
                    }
                    return crate::expat_h::XML_ERROR_INVALID_TOKEN;
                }
                crate::src::xmltok::XML_TOK_PARTIAL => {
                    if enc == parser_encoding(parser) {
                        (*parser).m_eventPtr = ptr;
                    }
                    return crate::expat_h::XML_ERROR_INVALID_TOKEN;
                }
                crate::src::xmltok::XML_TOK_CHAR_REF => {
                    let mut buf: [crate::expat_external_h::XML_Char; 4] = [0; 4];
                    let mut i: ::core::ffi::c_int = 0;
                    let mut n: ::core::ffi::c_int = (*enc).charRefNumber.decode(enc, ptr);
                    if n < 0 as ::core::ffi::c_int {
                        if enc == parser_encoding(parser) {
                            (*parser).m_eventPtr = ptr;
                        }
                        return crate::expat_h::XML_ERROR_BAD_CHAR_REF;
                    }
                    if isCdata == 0
                        && n == 0x20 as ::core::ffi::c_int
                        && ((*pool).ptr_offset == 0
                            || (*pool).last_cursor_char()
                                == Some(0x20 as crate::expat_external_h::XML_Char))
                    {
                        break 's_350;
                    } else {
                        (buf, n) = encode_xml_char_ref(n);
                        i = 0 as ::core::ffi::c_int;
                        while i < n {
                            if !pool_append_char(pool, buf[i as usize]) {
                                return crate::expat_h::XML_ERROR_NO_MEMORY;
                            }
                            i += 1;
                        }
                        break 's_350;
                    }
                }
                crate::src::xmltok::XML_TOK_DATA_CHARS => {
                    if poolAppend(pool, enc, ptr, next).is_null() {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                    break 's_350;
                }
                crate::src::xmltok::XML_TOK_TRAILING_CR => {
                    next = ptr.offset((*enc).minBytesPerChar as isize);
                }
                crate::src::xmltok::XML_TOK_ATTRIBUTE_VALUE_S
                | crate::src::xmltok::XML_TOK_DATA_NEWLINE => {}
                crate::src::xmltok::XML_TOK_ENTITY_REF => {
                    let mut name: *const crate::expat_external_h::XML_Char =
                        ::core::ptr::null::<crate::expat_external_h::XML_Char>();
                    let mut entity: *mut ENTITY = ::core::ptr::null_mut::<ENTITY>();
                    let mut checkEntityDecl: bool = false;
                    let mut ch: crate::expat_external_h::XML_Char =
                        crate::src::xmltok::predefined_entity_name(
                            enc,
                            ptr.offset((*enc).minBytesPerChar as isize),
                            next.offset(-((*enc).minBytesPerChar as isize)),
                        ) as crate::expat_external_h::XML_Char;
                    if ch != 0 {
                        accountingDiffTolerated(
                            parser,
                            tok,
                            &raw mut ch as *mut ::core::ffi::c_char,
                            (&raw mut ch as *mut ::core::ffi::c_char).offset(
                                ::core::mem::size_of::<crate::expat_external_h::XML_Char>()
                                    as isize,
                            ),
                            6663 as ::core::ffi::c_int,
                            XML_ACCOUNT_ENTITY_EXPANSION,
                        );
                        if !pool_append_char(pool, ch) {
                            return crate::expat_h::XML_ERROR_NO_MEMORY;
                        }
                        break 's_350;
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
                        (*parser).m_temp2Pool.rewind();
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
                        } else if entity.is_null() {
                            break 's_350;
                        }
                        if (*entity).open != 0 {
                            if enc == parser_encoding(parser) {
                                (*parser).m_eventPtr = ptr;
                            }
                            return crate::expat_h::XML_ERROR_RECURSIVE_ENTITY_REF;
                        }
                        if (*entity).notation.is_some() {
                            if enc == parser_encoding(parser) {
                                (*parser).m_eventPtr = ptr;
                            }
                            return crate::expat_h::XML_ERROR_BINARY_ENTITY_REF;
                        }
                        if (*entity).textPtr.is_none() {
                            if enc == parser_encoding(parser) {
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
                _ => {
                    if enc == parser_encoding(parser) {
                        (*parser).m_eventPtr = ptr;
                    }
                    return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
                }
            }
            if !(isCdata == 0
                && ((*pool).ptr_offset == 0
                    || (*pool).last_cursor_char()
                        == Some(0x20 as crate::expat_external_h::XML_Char)))
            {
                if !pool_append_char(pool, 0x20 as crate::expat_external_h::XML_Char) {
                    return crate::expat_h::XML_ERROR_NO_MEMORY;
                }
            }
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
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut pool: *mut STRING_POOL = &raw mut (*dtd).entityValuePool;
    let mut result: crate::expat_h::XML_Error = crate::expat_h::XML_ERROR_NONE;
    let mut oldInEntityValue: ::core::ffi::c_int = (*parser).m_prologState.inEntityValue;
    (*parser).m_prologState.inEntityValue = 1 as ::core::ffi::c_int;
    if (*pool).storage.active.is_empty() {
        if poolGrow(pool) == 0 {
            return crate::expat_h::XML_ERROR_NO_MEMORY;
        }
    }
    let mut next: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    '_endEntityValue: loop {
        next = entityTextPtr;
        let scanner = match (*enc).literalScanners[1] {
            crate::src::xmltok::LiteralScanner::NormalEntityValue => {
                crate::src::xmltok::xmltok_impl_c::normal_entityValueTok
            }
            crate::src::xmltok::LiteralScanner::Little2EntityValue => {
                crate::src::xmltok::xmltok_impl_c::little2_entityValueTok
            }
            crate::src::xmltok::LiteralScanner::Big2EntityValue => {
                crate::src::xmltok::xmltok_impl_c::big2_entityValueTok
            }
            _ => unreachable!("entity literal scanner must match its table slot"),
        };
        let mut tok: ::core::ffi::c_int = scanner(enc, entityTextPtr, entityTextEnd, &raw mut next);
        if accountingDiffTolerated(
            parser,
            tok,
            entityTextPtr,
            next,
            6798 as ::core::ffi::c_int,
            account,
        ) == 0
        {
            accountingOnAbort(parser);
            result = crate::expat_h::XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
            break;
        } else {
            's_340: {
                match tok {
                    crate::src::xmltok::XML_TOK_PARAM_ENTITY_REF => {
                        if (*parser).m_isParamEntity as ::core::ffi::c_int != 0
                            || enc != parser_encoding(parser)
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
                                break '_endEntityValue;
                            } else {
                                entity = lookup(
                                    parser,
                                    &raw mut (*dtd).paramEntities,
                                    name as KEY,
                                    0 as crate::__stddef_size_t_h::size_t,
                                ) as *mut ENTITY;
                                (*parser).m_tempPool.rewind();
                                if entity.is_null() {
                                    (*dtd).keepProcessing = (*dtd).standalone;
                                    break '_endEntityValue;
                                } else if (*entity).open as ::core::ffi::c_int != 0
                                    || entity == (*parser).m_declEntity
                                {
                                    if enc == parser_encoding(parser) {
                                        (*parser).m_eventPtr = entityTextPtr;
                                    }
                                    result = crate::expat_h::XML_ERROR_RECURSIVE_ENTITY_REF;
                                    break '_endEntityValue;
                                } else if (*entity).systemId.is_some() {
                                    if (*parser).m_externalEntityRefHandler {
                                        (*dtd).paramEntityRead = crate::expat_h::XML_FALSE;
                                        (*entity).open = crate::expat_h::XML_TRUE;
                                        entityTrackingOnOpen(
                                            parser,
                                            entity,
                                            6840 as ::core::ffi::c_int,
                                        );
                                        let handler = EXTERNAL_ENTITY_REF_HANDLERS
                                            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
                                            .lock()
                                            .unwrap_or_else(|poisoned| poisoned.into_inner())
                                            .get(&(parser as usize))
                                            .cloned()
                                            .expect("installed external entity handler");
                                        if invoke_external_entity_ref_handler(
                                            handler.as_ref(),
                                            (*parser).m_externalEntityRefHandlerArg,
                                            ::core::ptr::null(),
                                            &raw const (*dtd).pool,
                                            entity,
                                        ) == 0
                                        {
                                            entityTrackingOnClose(
                                                parser,
                                                entity,
                                                6844 as ::core::ffi::c_int,
                                            );
                                            (*entity).open = crate::expat_h::XML_FALSE;
                                            result =
                                                crate::expat_h::XML_ERROR_EXTERNAL_ENTITY_HANDLING;
                                            break '_endEntityValue;
                                        } else {
                                            entityTrackingOnClose(
                                                parser,
                                                entity,
                                                6849 as ::core::ffi::c_int,
                                            );
                                            (*entity).open = crate::expat_h::XML_FALSE;
                                            if (*dtd).paramEntityRead == 0 {
                                                (*dtd).keepProcessing = (*dtd).standalone;
                                            }
                                            break 's_340;
                                        }
                                    } else {
                                        (*dtd).keepProcessing = (*dtd).standalone;
                                        break 's_340;
                                    }
                                } else {
                                    result = processEntity(
                                        parser,
                                        entity,
                                        crate::expat_h::XML_FALSE,
                                        ENTITY_VALUE,
                                    );
                                    break '_endEntityValue;
                                }
                            }
                        } else {
                            (*parser).m_eventPtr = entityTextPtr;
                            result = crate::expat_h::XML_ERROR_PARAM_ENTITY_REF;
                            break '_endEntityValue;
                        }
                    }
                    crate::src::xmltok::XML_TOK_NONE => {
                        result = crate::expat_h::XML_ERROR_NONE;
                        break '_endEntityValue;
                    }
                    crate::src::xmltok::XML_TOK_ENTITY_REF
                    | crate::src::xmltok::XML_TOK_DATA_CHARS => {
                        if poolAppend(pool, enc, entityTextPtr, next).is_null() {
                            result = crate::expat_h::XML_ERROR_NO_MEMORY;
                            break '_endEntityValue;
                        } else {
                            break 's_340;
                        }
                    }
                    crate::src::xmltok::XML_TOK_TRAILING_CR => {
                        next = entityTextPtr.offset((*enc).minBytesPerChar as isize);
                    }
                    crate::src::xmltok::XML_TOK_DATA_NEWLINE => {}
                    crate::src::xmltok::XML_TOK_CHAR_REF => {
                        let mut buf: [crate::expat_external_h::XML_Char; 4] = [0; 4];
                        let mut i: ::core::ffi::c_int = 0;
                        let mut n: ::core::ffi::c_int =
                            (*enc).charRefNumber.decode(enc, entityTextPtr);
                        if n < 0 as ::core::ffi::c_int {
                            if enc == parser_encoding(parser) {
                                (*parser).m_eventPtr = entityTextPtr;
                            }
                            result = crate::expat_h::XML_ERROR_BAD_CHAR_REF;
                            break '_endEntityValue;
                        } else {
                            n = crate::src::xmltok::XmlUtf8Encode(
                                n,
                                &raw mut buf as *mut crate::expat_external_h::XML_Char
                                    as *mut ::core::ffi::c_char,
                            );
                            i = 0 as ::core::ffi::c_int;
                            loop {
                                if i >= n {
                                    break 's_340;
                                }
                                if (*pool).is_full()
                                    && poolGrow(pool) == 0
                                {
                                    result = crate::expat_h::XML_ERROR_NO_MEMORY;
                                    break '_endEntityValue;
                                } else {
                                    if !(&mut *pool).write_cursor(buf[i as usize]) {
                                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                                    }
                                    i += 1;
                                }
                            }
                        }
                    }
                    crate::src::xmltok::XML_TOK_PARTIAL => {
                        if enc == parser_encoding(parser) {
                            (*parser).m_eventPtr = entityTextPtr;
                        }
                        result = crate::expat_h::XML_ERROR_INVALID_TOKEN;
                        break '_endEntityValue;
                    }
                    crate::src::xmltok::XML_TOK_INVALID => {
                        if enc == parser_encoding(parser) {
                            (*parser).m_eventPtr = next;
                        }
                        result = crate::expat_h::XML_ERROR_INVALID_TOKEN;
                        break '_endEntityValue;
                    }
                    _ => {
                        if enc == parser_encoding(parser) {
                            (*parser).m_eventPtr = entityTextPtr;
                        }
                        result = crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
                        break '_endEntityValue;
                    }
                }
                if (*pool).is_full()
                    && poolGrow(pool) == 0
                {
                    result = crate::expat_h::XML_ERROR_NO_MEMORY;
                    break '_endEntityValue;
                } else {
                    if !(&mut *pool).write_cursor(0xa as crate::expat_external_h::XML_Char) {
                        return crate::expat_h::XML_ERROR_NO_MEMORY;
                    }
                }
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
            let Some(text) = (*entity).textPtr.present() else {
                return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
            };
            let dtd = (*parser).m_dtd;
            if dtd.is_null() {
                return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
            }
            let Some(text) = entity_text_chars(&*dtd, text, (*entity).textLen) else {
                return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
            };
            let Ok(processed) = usize::try_from((*entity).processed) else {
                return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
            };
            let Some(unprocessed) = text.get(processed..) else {
                return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
            };
            let textStart = unprocessed.as_ptr().cast::<::core::ffi::c_char>();
            let textEnd = text
                .as_ptr()
                .wrapping_add(text.len())
                .cast::<::core::ffi::c_char>();
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
                    let Some(processed) = nextInEntity.addr().checked_sub(textStart.addr()) else {
                        return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
                    };
                    let Ok(processed) = ::core::ffi::c_int::try_from(processed) else {
                        return crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
                    };
                    (*entity).processed = (*entity).processed.saturating_add(processed);
                    continue;
                } else {
                    (*entity).hasMore = crate::expat_h::XML_FALSE;
                    continue;
                }
            } else {
                entityTrackingOnClose(parser, entity, 6998 as ::core::ffi::c_int);
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

unsafe extern "C" fn normalizeLines(mut s: *mut crate::expat_external_h::XML_Char) {
    let mut p: *mut crate::expat_external_h::XML_Char =
        ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
    loop {
        if *s as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            return;
        }
        if *s as ::core::ffi::c_int == 0xd as ::core::ffi::c_int {
            break;
        }
        s = s.offset(1);
    }
    p = s;
    loop {
        if *s as ::core::ffi::c_int == 0xd as ::core::ffi::c_int {
            let c2rust_fresh6 = p;
            p = p.offset(1);
            *c2rust_fresh6 = 0xa as crate::expat_external_h::XML_Char;
            s = s.offset(1);
            if *s as ::core::ffi::c_int == 0xa as ::core::ffi::c_int {
                s = s.offset(1);
            }
        } else {
            let c2rust_fresh7 = s;
            s = s.offset(1);
            let c2rust_fresh8 = p;
            p = p.offset(1);
            *c2rust_fresh8 = *c2rust_fresh7;
        }
        if *s == 0 {
            break;
        }
    }
    *p = '\0' as crate::expat_external_h::XML_Char;
}

unsafe extern "C" fn reportProcessingInstruction(
    mut parser: crate::expat_h::XML_Parser,
    mut enc: *const crate::src::xmltok::ENCODING,
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut tem: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let (has_processing_instruction_handler, has_default_handler) = {
        let parser_state = &*parser;
        (
            parser_state.m_processingInstructionHandler,
            parser_state.m_defaultHandler,
        )
    };
    if !has_processing_instruction_handler {
        if has_default_handler {
            reportDefault(parser, enc, start, end);
        }
        return 1 as ::core::ffi::c_int;
    }
    start = start.offset(((*enc).minBytesPerChar * 2 as ::core::ffi::c_int) as isize);
    tem = start.offset(crate::src::xmltok::name_length(enc, start) as isize);
    let (target, data, handler_arg) = {
        let parser_state = &mut *parser;
        let target = poolStoreString(&raw mut parser_state.m_tempPool, enc, start, tem);
        if target.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        parser_state.m_tempPool.commit();
        let data = poolStoreString(
            &raw mut parser_state.m_tempPool,
            enc,
            crate::src::xmltok::skip_s(enc, tem, (*enc).skipS),
            end.offset(-(((*enc).minBytesPerChar * 2 as ::core::ffi::c_int) as isize)),
        );
        if data.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        normalizeLines(data);
        (target, data, parser_state.m_handlerArg)
    };
    let callback = PROCESSING_INSTRUCTION_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&(parser as usize))
        .cloned();
    if let Some(callback) = callback {
        callback.invoke(handler_arg, target, data);
    }
    {
        let parser_state = &mut *parser;
        poolClear(&raw mut parser_state.m_tempPool);
    }
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
    let (has_comment_handler, has_default_handler) = {
        let parser_state = &*parser;
        (parser_state.m_commentHandler, parser_state.m_defaultHandler)
    };
    if !has_comment_handler {
        if has_default_handler {
            reportDefault(parser, enc, start, end);
        }
        return 1 as ::core::ffi::c_int;
    }
    let temp_pool = &raw mut (*parser).m_tempPool;
    data = poolStoreString(
        temp_pool,
        enc,
        start.offset(((*enc).minBytesPerChar * 4 as ::core::ffi::c_int) as isize),
        end.offset(-(((*enc).minBytesPerChar * 3 as ::core::ffi::c_int) as isize)),
    );
    if data.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    normalizeLines(data);
    let callback = COMMENT_HANDLERS
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&(parser as usize))
        .cloned();
    if let Some(callback) = callback {
        callback.invoke((*parser).m_handlerArg, data);
    }
    poolClear(temp_pool);
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
        if enc == parser_encoding(parser) {
            eventPP = &raw mut (*parser).m_eventPtr;
            eventEndPP = &raw mut (*parser).m_eventEndPtr;
        } else {
            eventPP = &raw mut (*(*parser).m_openInternalEntities).internalEventPtr;
            eventEndPP = &raw mut (*(*parser).m_openInternalEntities).internalEventEndPtr;
        }
        loop {
            let mut dataPtr: *mut ICHAR = (*parser).m_dataBuf as *mut ICHAR;
            convert_res = crate::src::xmltok::convert_to_utf8(
                enc,
                &raw mut s,
                end,
                &raw mut dataPtr,
                (*parser).m_dataBufEnd as *mut ICHAR,
            );
            *eventEndPP = s;
            let callback = DEFAULT_HANDLERS
                .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .get(&(parser as usize))
                .cloned()
                .expect("default callback must be registered when installed");
            callback.invoke(
                (*parser).m_handlerArg,
                (*parser).m_dataBuf,
                dataPtr.offset_from((*parser).m_dataBuf as *mut ICHAR) as ::core::ffi::c_int,
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
        let callback = DEFAULT_HANDLERS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .get(&(parser as usize))
            .cloned()
            .expect("default callback must be registered when installed");
        callback.invoke(
            (*parser).m_handlerArg,
            s as *const crate::expat_external_h::XML_Char,
            (end as *const crate::expat_external_h::XML_Char)
                .offset_from(s as *const crate::expat_external_h::XML_Char)
                as ::core::ffi::c_int,
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
    if type_0.is_null() || attId.is_null() || parser.is_null() || (*parser).m_dtd.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let type_0 = &mut *type_0;
    let Some(att_name) = pool_string_ref(&raw const (*(*parser).m_dtd).pool, (*attId).name, false) else {
        return 0 as ::core::ffi::c_int;
    };
    if !value.is_null() || isId as ::core::ffi::c_int != 0 {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < type_0.nDefaultAtts {
            if (*type_0
                .defaultAtts
                .expect("default attribute storage must exist for a non-empty list")
                .as_ptr()
                .offset(i as isize))
                .id
                == Some(att_name)
            {
                return 1 as ::core::ffi::c_int;
            }
            i += 1;
        }
        if isId as ::core::ffi::c_int != 0 && type_0.idAtt.is_null() && (*attId).xmlns == 0 {
            type_0.idAtt = attId;
        }
    }
    if type_0.nDefaultAtts == type_0.allocDefaultAtts {
        if type_0.allocDefaultAtts == 0 as ::core::ffi::c_int {
            type_0.allocDefaultAtts = 8 as ::core::ffi::c_int;
            type_0.defaultAtts = std::ptr::NonNull::new(
                expat_malloc(
                    parser,
                    (type_0.allocDefaultAtts as crate::__stddef_size_t_h::size_t)
                        .wrapping_mul(::core::mem::size_of::<DEFAULT_ATTRIBUTE>()),
                    7182 as ::core::ffi::c_int,
                ) as *mut DEFAULT_ATTRIBUTE,
            );
            if type_0.defaultAtts.is_none() {
                type_0.allocDefaultAtts = 0 as ::core::ffi::c_int;
                return 0 as ::core::ffi::c_int;
            }
        } else {
            let mut temp: *mut DEFAULT_ATTRIBUTE = ::core::ptr::null_mut::<DEFAULT_ATTRIBUTE>();
            if type_0.allocDefaultAtts > crate::limits_h::INT_MAX / 2 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            }
            let mut count: ::core::ffi::c_int =
                type_0.allocDefaultAtts * 2 as ::core::ffi::c_int;
            temp = expat_realloc(
                parser,
                type_0
                    .defaultAtts
                    .expect("default attribute storage must exist before growing")
                    .as_ptr() as *mut ::core::ffi::c_void,
                (count as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(::core::mem::size_of::<DEFAULT_ATTRIBUTE>()),
                7208 as ::core::ffi::c_int,
            ) as *mut DEFAULT_ATTRIBUTE;
            if temp.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            type_0.allocDefaultAtts = count;
            type_0.defaultAtts = Some(
                std::ptr::NonNull::new(temp)
                    .expect("a successful realloc must return a non-null allocation"),
            );
        }
    }
    att = type_0
        .defaultAtts
        .expect("default attribute storage must exist after allocation")
        .as_ptr()
        .offset(type_0.nDefaultAtts as isize);
    (*att).id = Some(att_name);
    (*att).value = std::ptr::NonNull::new(value as *mut crate::expat_external_h::XML_Char);
    (*att).isCdata = isCdata;
    if isCdata == 0 {
        (*attId).maybeTokenized = crate::expat_h::XML_TRUE;
    }
    type_0.nDefaultAtts += 1 as ::core::ffi::c_int;
    return 1 as ::core::ffi::c_int;
}

unsafe extern "C" fn setElementTypePrefix(
    mut parser: crate::expat_h::XML_Parser,
    mut elementType: *mut ELEMENT_TYPE,
) -> ::core::ffi::c_int {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut name: *const crate::expat_external_h::XML_Char =
        ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    name = (*elementType).named.name;
    while *name != 0 {
        if *name as ::core::ffi::c_int == 0x3a as ::core::ffi::c_int {
            let mut prefix: *mut PREFIX = ::core::ptr::null_mut::<PREFIX>();
            let mut s: *const crate::expat_external_h::XML_Char =
                ::core::ptr::null::<crate::expat_external_h::XML_Char>();
            s = (*elementType).named.name;
            while s != name {
                if if (*dtd).pool.is_full()
                    && poolGrow(&raw mut (*dtd).pool) == 0
                {
                    0 as ::core::ffi::c_int
                } else {
                if (*dtd).pool.write_cursor(*s) {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }
                } == 0
                {
                    return 0 as ::core::ffi::c_int;
                }
                s = s.offset(1);
            }
            if if (*dtd).pool.is_full()
                && poolGrow(&raw mut (*dtd).pool) == 0
            {
                0 as ::core::ffi::c_int
            } else {
                if (*dtd).pool.write_cursor('\0' as crate::expat_external_h::XML_Char) {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }
            } == 0
            {
                return 0 as ::core::ffi::c_int;
            }
            let Some(pool_start) = (*dtd).pool.start_ref(true) else {
                return 0 as ::core::ffi::c_int;
            };
            let pool_start = (*dtd)
                .pool
                .chars_from(pool_start)
                .map_or(::core::ptr::null(), |chars| chars.as_ptr());
            if pool_start.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            prefix = lookup(
                parser,
                &raw mut (*dtd).prefixes,
                pool_start as KEY,
                ::core::mem::size_of::<PREFIX>(),
            ) as *mut PREFIX;
            if prefix.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            if (*prefix).name == pool_start {
                (*dtd).pool.commit();
            } else {
                (*dtd).pool.rewind();
            }
            let prefix_ref = pool_string_ref(&raw const (*dtd).pool, (*prefix).name, false);
            let Some(prefix_ref) = prefix_ref else {
                return 0 as ::core::ffi::c_int;
            };
            (*elementType).prefix = prefix_ref;
            (*elementType).hasPrefix = crate::expat_h::XML_TRUE;
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
    let parser_ptr = parser;
    let parser = &mut *parser;
    let dtd = &mut *parser.m_dtd;
    if !pool_append_char(
        &raw mut dtd.pool,
        '\0' as crate::expat_external_h::XML_Char,
    ) {
        return ::core::ptr::null_mut::<ATTRIBUTE_ID>();
    }
    let mut name = poolStoreString(&raw mut dtd.pool, enc, start, end);
    if name.is_null() {
        return ::core::ptr::null_mut::<ATTRIBUTE_ID>();
    }
    name = name.wrapping_add(1);
    let name_bytes = ::std::ffi::CStr::from_ptr(name).to_bytes();
    let id = lookup(
        parser_ptr,
        &raw mut dtd.attributeIds,
        name as KEY,
        ::core::mem::size_of::<ATTRIBUTE_ID>(),
    ) as *mut ATTRIBUTE_ID;
    if id.is_null() {
        return ::core::ptr::null_mut::<ATTRIBUTE_ID>();
    }
    let id = &mut *id;
    if id.name != name as *mut crate::expat_external_h::XML_Char {
        dtd.pool.rewind();
    } else {
        dtd.pool.commit();
        if parser.m_ns != 0 {
            if name_bytes.starts_with(b"xmlns")
                && matches!(name_bytes.get(5), None | Some(b':'))
            {
                if name_bytes.len() == 5 {
                    id.prefix = &raw mut dtd.defaultPrefix;
                } else {
                    id.prefix = lookup(
                        parser_ptr,
                        &raw mut dtd.prefixes,
                        name.wrapping_add(6),
                        ::core::mem::size_of::<PREFIX>(),
                    ) as *mut PREFIX;
                }
                id.xmlns = crate::expat_h::XML_TRUE;
            } else {
                if let Some(prefix_len) = name_bytes.iter().position(|&ch| ch == b':') {
                    for &ch in &name_bytes[..prefix_len] {
                        if !pool_append_char(
                            &raw mut dtd.pool,
                            ch as crate::expat_external_h::XML_Char,
                        ) {
                            return ::core::ptr::null_mut::<ATTRIBUTE_ID>();
                        }
                    }
                    if !pool_append_char(
                        &raw mut dtd.pool,
                        '\0' as crate::expat_external_h::XML_Char,
                    ) {
                        return ::core::ptr::null_mut::<ATTRIBUTE_ID>();
                    }
                    let Some(pool_start) = dtd.pool.start_ref(true) else {
                        return ::core::ptr::null_mut::<ATTRIBUTE_ID>();
                    };
                    let pool_start = dtd
                        .pool
                        .chars_from(pool_start)
                        .map_or(::core::ptr::null(), |chars| chars.as_ptr());
                    if pool_start.is_null() {
                        return ::core::ptr::null_mut::<ATTRIBUTE_ID>();
                    }
                    id.prefix = lookup(
                            parser_ptr,
                            &raw mut dtd.prefixes,
                            pool_start as KEY,
                            ::core::mem::size_of::<PREFIX>(),
                        ) as *mut PREFIX;
                    if id.prefix.is_null() {
                        return ::core::ptr::null_mut::<ATTRIBUTE_ID>();
                    }
                    if (*id.prefix).name == pool_start {
                        dtd.pool.commit();
                    } else {
                        dtd.pool.rewind();
                    }
                }
            }
        }
    }
    id
}

// The string pool owns the resulting context.  Keep the raw-pointer work for
// filling that pool here, where the pool's `ptr..end` invariant is checked
// before every write.
unsafe fn pool_append_context_char(
    pool: &mut STRING_POOL,
    ch: crate::expat_external_h::XML_Char,
) -> bool {
    if pool.is_full() && poolGrow(pool) == 0 {
        return false;
    }
    pool.write_cursor(ch)
}

unsafe fn pool_append_context_chars(
    pool: &mut STRING_POOL,
    source: *const crate::expat_external_h::XML_Char,
    len: ::core::ffi::c_int,
) -> bool {
    if len <= 0 {
        return true;
    }
    for index in 0..len as usize {
        if !pool_append_context_char(pool, *source.add(index)) {
            return false;
        }
    }
    true
}

unsafe fn pool_append_context_c_string(
    pool: &mut STRING_POOL,
    source: *const crate::expat_external_h::XML_Char,
) -> bool {
    let mut source = source;
    while *source != 0 {
        if !pool_append_context_char(pool, *source) {
            return false;
        }
        source = source.add(1);
    }
    true
}

unsafe extern "C" fn getContext(
    mut parser: crate::expat_h::XML_Parser,
) -> *const crate::expat_external_h::XML_Char {
    let parser = &mut *parser;
    let dtd = &mut *parser.m_dtd;
    let mut iter: HASH_TABLE_ITER = HASH_TABLE_ITER {
        table: None,
        next: 0 as crate::__stddef_size_t_h::size_t,
    };
    let mut needSep: crate::expat_h::XML_Bool = crate::expat_h::XML_FALSE;
    if !dtd.defaultPrefix.binding.is_null() {
        if !pool_append_context_char(
            &mut parser.m_tempPool,
            0x3d as crate::expat_external_h::XML_Char,
        ) {
            return ::core::ptr::null::<crate::expat_external_h::XML_Char>();
        }
        let binding = &*dtd.defaultPrefix.binding;
        let mut len = binding.uriLen;
        if (*parser).m_namespaceSeparator != 0 {
            len -= 1;
        }
        if !pool_append_context_chars(&mut parser.m_tempPool, binding.uri, len) {
            return ::core::ptr::null::<crate::expat_external_h::XML_Char>();
        }
        needSep = crate::expat_h::XML_TRUE;
    }
    let table = &dtd.prefixes;
    hashTableIterInit(&raw mut iter, table);
    loop {
        let prefix = hashTableIterNext(&raw mut iter) as *mut PREFIX;
        if prefix.is_null() {
            break;
        }
        let prefix = &*prefix;
        if prefix.binding.is_null() {
            continue;
        }
        if needSep as ::core::ffi::c_int != 0
            && !pool_append_context_char(
                &mut parser.m_tempPool,
                0xc as crate::expat_external_h::XML_Char,
            )
        {
            return ::core::ptr::null::<crate::expat_external_h::XML_Char>();
        }
        if !pool_append_context_c_string(&mut parser.m_tempPool, prefix.name) {
            return ::core::ptr::null::<crate::expat_external_h::XML_Char>();
        }
        if !pool_append_context_char(
            &mut parser.m_tempPool,
            0x3d as crate::expat_external_h::XML_Char,
        ) {
            return ::core::ptr::null::<crate::expat_external_h::XML_Char>();
        }
        let binding = &*prefix.binding;
        let mut len = binding.uriLen;
        if parser.m_namespaceSeparator != 0 {
            len -= 1;
        }
        if !pool_append_context_chars(&mut parser.m_tempPool, binding.uri, len) {
            return ::core::ptr::null::<crate::expat_external_h::XML_Char>();
        }
        needSep = crate::expat_h::XML_TRUE;
    }
    let table = &dtd.generalEntities;
    hashTableIterInit(&raw mut iter, table);
    loop {
        let e = hashTableIterNext(&raw mut iter) as *mut ENTITY;
        if e.is_null() {
            break;
        }
        let e = &*e;
        if e.open == 0 {
            continue;
        }
        if needSep as ::core::ffi::c_int != 0
            && !pool_append_context_char(
                &mut parser.m_tempPool,
                0xc as crate::expat_external_h::XML_Char,
            )
        {
            return ::core::ptr::null::<crate::expat_external_h::XML_Char>();
        }
        if !pool_append_context_c_string(
            &mut parser.m_tempPool,
            e.named.name,
        ) {
            return ::core::ptr::null::<crate::expat_external_h::XML_Char>();
        }
        needSep = crate::expat_h::XML_TRUE;
    }
    if !pool_append_context_char(
        &mut parser.m_tempPool,
        '\0' as crate::expat_external_h::XML_Char,
    ) {
        return ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    }
    let Some(start) = parser.m_tempPool.start_ref(true) else {
        return ::core::ptr::null();
    };
    return parser
        .m_tempPool
        .chars_from(start)
        .map_or(::core::ptr::null(), |chars| chars.as_ptr());
}

unsafe extern "C" fn setContext(
    mut parser: crate::expat_h::XML_Parser,
    context: *const crate::expat_external_h::XML_Char,
) -> crate::expat_h::XML_Bool {
    if context.is_null() {
        return crate::expat_h::XML_FALSE;
    }
    // Context values are produced by getContext or accepted by the public C API
    // as NUL-terminated XML_Char data.  Read the terminator once, then keep all
    // delimiter handling inside this bounded byte slice.
    let context = ::core::ffi::CStr::from_ptr(context).to_bytes();
    let parser = &mut *parser;
    let dtd = &mut *parser.m_dtd;
    let mut position = 0usize;

    while position < context.len() {
        match context[position] {
            0x0c => {
                if !pool_append_context_char(
                    &mut parser.m_tempPool,
                    '\0' as crate::expat_external_h::XML_Char,
                ) {
                    return crate::expat_h::XML_FALSE;
                }
                let Some(pool_start) = parser.m_tempPool.start_ref(true) else {
                    return crate::expat_h::XML_FALSE;
                };
                let pool_start = parser
                    .m_tempPool
                    .chars_from(pool_start)
                    .map_or(::core::ptr::null(), |chars| chars.as_ptr());
                if pool_start.is_null() {
                    return crate::expat_h::XML_FALSE;
                }
                let entity = lookup(
                    parser as *mut XML_ParserStruct,
                    &raw mut dtd.generalEntities,
                    pool_start as KEY,
                    0 as crate::__stddef_size_t_h::size_t,
                ) as *mut ENTITY;
                if !entity.is_null() {
                    (*entity).open = crate::expat_h::XML_TRUE;
                }
                parser.m_tempPool.rewind();
                position += 1;
            }
            0x3d => {
                let prefix = if parser.m_tempPool.ptr_offset == 0 {
                    &raw mut dtd.defaultPrefix
                } else {
                    if !pool_append_context_char(
                        &mut parser.m_tempPool,
                        '\0' as crate::expat_external_h::XML_Char,
                    ) {
                        return crate::expat_h::XML_FALSE;
                    }
                    let Some(pool_start) = parser.m_tempPool.start_ref(true) else {
                        return crate::expat_h::XML_FALSE;
                    };
                    let pool_start = parser
                        .m_tempPool
                        .chars_from(pool_start)
                        .map_or(::core::ptr::null(), |chars| chars.as_ptr());
                    if pool_start.is_null() {
                        return crate::expat_h::XML_FALSE;
                    }
                    let prefix = lookup(
                        parser as *mut XML_ParserStruct,
                        &raw mut dtd.prefixes,
                        pool_start as KEY,
                        ::core::mem::size_of::<PREFIX>(),
                    ) as *mut PREFIX;
                    if prefix.is_null() {
                        return crate::expat_h::XML_FALSE;
                    }
                    if (*prefix).name
                        == pool_start
                    {
                        (*prefix).name = poolCopyString(&raw mut dtd.pool, (*prefix).name);
                        if (*prefix).name.is_null() {
                            return crate::expat_h::XML_FALSE;
                        }
                    }
                    parser.m_tempPool.rewind();
                    prefix
                };

                position += 1;
                while position < context.len() && context[position] != 0x0c {
                    if !pool_append_context_char(
                        &mut parser.m_tempPool,
                        context[position] as crate::expat_external_h::XML_Char,
                    ) {
                        return crate::expat_h::XML_FALSE;
                    }
                    position += 1;
                }
                if !pool_append_context_char(
                    &mut parser.m_tempPool,
                    '\0' as crate::expat_external_h::XML_Char,
                ) {
                    return crate::expat_h::XML_FALSE;
                }
                let Some(pool_start) = parser.m_tempPool.start_ref(true) else {
                    return crate::expat_h::XML_FALSE;
                };
                let pool_start = parser
                    .m_tempPool
                    .chars_from(pool_start)
                    .map_or(::core::ptr::null(), |chars| chars.as_ptr());
                if pool_start.is_null() {
                    return crate::expat_h::XML_FALSE;
                }
                if addBinding(
                    parser as *mut XML_ParserStruct,
                    prefix,
                    ::core::ptr::null::<ATTRIBUTE_ID>(),
                    pool_start,
                    &raw mut parser.m_inheritedBindings,
                ) as ::core::ffi::c_uint
                    != crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    return crate::expat_h::XML_FALSE;
                }
                parser.m_tempPool.rewind();
                if position < context.len() {
                    position += 1;
                }
            }
            ch => {
                if !pool_append_context_char(
                    &mut parser.m_tempPool,
                    ch as crate::expat_external_h::XML_Char,
                ) {
                    return crate::expat_h::XML_FALSE;
                }
                position += 1;
            }
        }
    }

    // The original NUL-terminated walk treats its final terminator as an entity
    // separator.  An assignment consumes its own terminator above, so only a
    // remaining name in the temporary pool needs this final close-out.
    if parser.m_tempPool.ptr_offset != 0 {
        if !pool_append_context_char(
            &mut parser.m_tempPool,
            '\0' as crate::expat_external_h::XML_Char,
        ) {
            return crate::expat_h::XML_FALSE;
        }
        let Some(pool_start) = parser.m_tempPool.start_ref(true) else {
            return crate::expat_h::XML_FALSE;
        };
        let pool_start = parser
            .m_tempPool
            .chars_from(pool_start)
            .map_or(::core::ptr::null(), |chars| chars.as_ptr());
        if pool_start.is_null() {
            return crate::expat_h::XML_FALSE;
        }
        let entity = lookup(
            parser as *mut XML_ParserStruct,
            &raw mut dtd.generalEntities,
            pool_start as KEY,
            0 as crate::__stddef_size_t_h::size_t,
        ) as *mut ENTITY;
        if !entity.is_null() {
            (*entity).open = crate::expat_h::XML_TRUE;
        }
        parser.m_tempPool.rewind();
    }
    crate::expat_h::XML_TRUE
}

unsafe extern "C" fn normalizePublicId(mut publicId: *mut crate::expat_external_h::XML_Char) {
    let mut p: *mut crate::expat_external_h::XML_Char = publicId;
    let mut s: *mut crate::expat_external_h::XML_Char =
        ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
    s = publicId;
    while *s != 0 {
        match *s as ::core::ffi::c_int {
            32 | 13 | 10 => {
                if p != publicId
                    && *p.offset(-1 as isize) as ::core::ffi::c_int != 0x20 as ::core::ffi::c_int
                {
                    let c2rust_fresh56 = p;
                    p = p.offset(1);
                    *c2rust_fresh56 = 0x20 as crate::expat_external_h::XML_Char;
                }
            }
            _ => {
                let c2rust_fresh57 = p;
                p = p.offset(1);
                *c2rust_fresh57 = *s;
            }
        }
        s = s.offset(1);
    }
    if p != publicId && *p.offset(-1 as isize) as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int {
        p = p.offset(-1);
    }
    *p = '\0' as crate::expat_external_h::XML_Char;
}

unsafe extern "C" fn dtdCreate(mut parser: crate::expat_h::XML_Parser) -> *mut DTD {
    let mut p: *mut DTD = expat_malloc(
        parser,
        ::core::mem::size_of::<DTD>(),
        7500 as ::core::ffi::c_int,
    ) as *mut DTD;
    if p.is_null() {
        return p;
    }
    poolInit(&raw mut (*p).pool, parser);
    poolInit(&raw mut (*p).entityValuePool, parser);
    hashTableInit(&raw mut (*p).generalEntities, parser);
    hashTableInit(&raw mut (*p).elementTypes, parser);
    hashTableInit(&raw mut (*p).attributeIds, parser);
    hashTableInit(&raw mut (*p).prefixes, parser);
    (*p).paramEntityRead = crate::expat_h::XML_FALSE;
    hashTableInit(&raw mut (*p).paramEntities, parser);
    (*p).defaultPrefix.name = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    (*p).defaultPrefix.binding = ::core::ptr::null_mut::<BINDING>();
    (*p).in_eldecl = crate::expat_h::XML_FALSE;
    ::core::ptr::write(
        &raw mut (*p).scaffIndex,
        std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
    );
    ::core::ptr::write(&raw mut (*p).scaffold, empty_scaffold());
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
        table: None,
        next: 0 as crate::__stddef_size_t_h::size_t,
    };
    let table = &(*p).elementTypes;
    hashTableIterInit(&raw mut iter, table);
    loop {
        let mut e: *mut ELEMENT_TYPE = hashTableIterNext(&raw mut iter) as *mut ELEMENT_TYPE;
        if e.is_null() {
            break;
        }
        if (*e).allocDefaultAtts != 0 as ::core::ffi::c_int {
            let defaultAtts = (*e)
                .defaultAtts
                .expect("default attribute storage must exist for a non-zero capacity");
            expat_free(
                parser,
                defaultAtts.as_ptr() as *mut ::core::ffi::c_void,
                7539 as ::core::ffi::c_int,
            );
        }
    }
    hashTableClear(&raw mut (*p).generalEntities);
    (*p).paramEntityRead = crate::expat_h::XML_FALSE;
    hashTableClear(&raw mut (*p).paramEntities);
    hashTableClear(&raw mut (*p).elementTypes);
    hashTableClear(&raw mut (*p).attributeIds);
    hashTableClear(&raw mut (*p).prefixes);
    poolClear(&raw mut (*p).pool);
    poolClear(&raw mut (*p).entityValuePool);
    (*p).defaultPrefix.name = ::core::ptr::null::<crate::expat_external_h::XML_Char>();
    (*p).defaultPrefix.binding = ::core::ptr::null_mut::<BINDING>();
    (*p).in_eldecl = crate::expat_h::XML_FALSE;
    (*p).scaffIndex = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let backing = {
        let mut scaffold = (*p)
            .scaffold
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        scaffold.nodes.clear();
        scaffold.backing.take()
    };
    if let Some(mut backing) = backing {
        backing(ScaffoldAllocationAction::Free(7558 as ::core::ffi::c_int));
    }
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
        table: None,
        next: 0 as crate::__stddef_size_t_h::size_t,
    };
    let table = &(*p).elementTypes;
    hashTableIterInit(&raw mut iter, table);
    loop {
        let mut e: *mut ELEMENT_TYPE = hashTableIterNext(&raw mut iter) as *mut ELEMENT_TYPE;
        if e.is_null() {
            break;
        }
        if (*e).allocDefaultAtts != 0 as ::core::ffi::c_int {
            let defaultAtts = (*e)
                .defaultAtts
                .expect("default attribute storage must exist for a non-zero capacity");
            expat_free(
                parser,
                defaultAtts.as_ptr() as *mut ::core::ffi::c_void,
                7580 as ::core::ffi::c_int,
            );
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
        let backing = {
            let mut scaffold = (*p)
                .scaffold
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            scaffold.backing.take()
        };
        if let Some(mut backing) = backing {
            backing(ScaffoldAllocationAction::Free(7593 as ::core::ffi::c_int));
        }
    }
    ::core::ptr::drop_in_place(&raw mut (*p).scaffIndex);
    ::core::ptr::drop_in_place(&raw mut (*p).scaffold);
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
    // These DTDs remain allocated for the whole copy operation: `oldDtd` belongs to
    // the parent parser and `newDtd` to the parser being constructed.  Keeping
    // references to them makes the field-level copy below ordinary Rust access.
    let old_dtd = &*oldDtd;
    let new_dtd = &mut *newDtd;
    let mut iter: HASH_TABLE_ITER = HASH_TABLE_ITER {
        table: None,
        next: 0 as crate::__stddef_size_t_h::size_t,
    };
    let table = &old_dtd.prefixes;
    hashTableIterInit(&raw mut iter, table);
    loop {
        let old_p = hashTableIterNext(&raw mut iter) as *const PREFIX;
        if old_p.is_null() {
            break;
        }
        let old_p = &*old_p;
        let name = poolCopyString(&raw mut new_dtd.pool, old_p.name);
        if name.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if lookup(
            oldParser,
            &raw mut new_dtd.prefixes,
            name as KEY,
            ::core::mem::size_of::<PREFIX>(),
        )
        .is_null()
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    let table = &old_dtd.attributeIds;
    hashTableIterInit(&raw mut iter, table);
    loop {
        let old_a = hashTableIterNext(&raw mut iter) as *const ATTRIBUTE_ID;
        if old_a.is_null() {
            break;
        }
        let old_a = &*old_a;
        if if new_dtd.pool.is_full()
            && poolGrow(&raw mut new_dtd.pool) == 0
        {
            0 as ::core::ffi::c_int
        } else {
            if new_dtd.pool.write_cursor('\0' as crate::expat_external_h::XML_Char) {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }
        } == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        let name_0 = poolCopyString(&raw mut new_dtd.pool, old_a.name);
        if name_0.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        let name_0 = name_0.offset(1);
        let new_a = lookup(
            oldParser,
            &raw mut new_dtd.attributeIds,
            name_0 as KEY,
            ::core::mem::size_of::<ATTRIBUTE_ID>(),
        ) as *mut ATTRIBUTE_ID;
        if new_a.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        let new_a = &mut *new_a;
        new_a.maybeTokenized = old_a.maybeTokenized;
        if !old_a.prefix.is_null() {
            new_a.xmlns = old_a.xmlns;
            if old_a.prefix == &raw const old_dtd.defaultPrefix as *mut PREFIX {
                new_a.prefix = &raw mut new_dtd.defaultPrefix;
            } else {
                let old_prefix = &*old_a.prefix;
                new_a.prefix = lookup(
                    oldParser,
                    &raw mut new_dtd.prefixes,
                    old_prefix.name as KEY,
                    0 as crate::__stddef_size_t_h::size_t,
                ) as *mut PREFIX;
            }
        }
    }
    let table = &old_dtd.elementTypes;
    hashTableIterInit(&raw mut iter, table);
    loop {
        let old_e = hashTableIterNext(&raw mut iter) as *const ELEMENT_TYPE;
        if old_e.is_null() {
            break;
        }
        let old_e = &*old_e;
        let name_1 = poolCopyString(&raw mut new_dtd.pool, old_e.named.name);
        if name_1.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        let new_e = lookup(
            oldParser,
            &raw mut new_dtd.elementTypes,
            name_1 as KEY,
            ::core::mem::size_of::<ELEMENT_TYPE>(),
        ) as *mut ELEMENT_TYPE;
        if new_e.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        let new_e = &mut *new_e;
        if old_e.nDefaultAtts != 0 {
            new_e.defaultAtts = std::ptr::NonNull::new(
                expat_malloc(
                    parser,
                    (old_e.nDefaultAtts as crate::__stddef_size_t_h::size_t)
                        .wrapping_mul(::core::mem::size_of::<DEFAULT_ATTRIBUTE>()),
                    7683 as ::core::ffi::c_int,
                ) as *mut DEFAULT_ATTRIBUTE,
            );
            if new_e.defaultAtts.is_none() {
                return 0 as ::core::ffi::c_int;
            }
        }
        if !old_e.idAtt.is_null() {
            let old_id_att = &*old_e.idAtt;
            new_e.idAtt = lookup(
                oldParser,
                &raw mut new_dtd.attributeIds,
                old_id_att.name as KEY,
                0 as crate::__stddef_size_t_h::size_t,
            ) as *mut ATTRIBUTE_ID;
        }
        new_e.nDefaultAtts = old_e.nDefaultAtts;
        new_e.allocDefaultAtts = new_e.nDefaultAtts;
        if old_e.hasPrefix != 0 {
            let old_prefix_name = pool_string_pointer(&raw const old_dtd.pool, old_e.prefix);
            if old_prefix_name.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            let new_prefix = lookup(
                oldParser,
                &raw mut new_dtd.prefixes,
                old_prefix_name as KEY,
                0 as crate::__stddef_size_t_h::size_t,
            ) as *mut PREFIX;
            if new_prefix.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            let prefix_ref = pool_string_ref(&raw const new_dtd.pool, (*new_prefix).name, false);
            let Some(prefix_ref) = prefix_ref else {
                return 0 as ::core::ffi::c_int;
            };
            new_e.prefix = prefix_ref;
            new_e.hasPrefix = crate::expat_h::XML_TRUE;
        }
        let mut i = 0 as ::core::ffi::c_int;
        while i < new_e.nDefaultAtts {
            let old_att = &*old_e
                .defaultAtts
                .expect("default attribute storage must exist for a non-empty list")
                .as_ptr()
                .offset(i as isize);
            let new_att = &mut *new_e
                .defaultAtts
                .expect("default attribute storage must exist for a non-empty list")
                .as_ptr()
                .offset(i as isize);
            let Some(old_id_name) = old_att.id else {
                return 0 as ::core::ffi::c_int;
            };
            let old_id_name = old_dtd
                .pool
                .chars_from(old_id_name)
                .map_or(::core::ptr::null(), |chars| chars.as_ptr());
            if old_id_name.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            let new_id = lookup(
                oldParser,
                &raw mut new_dtd.attributeIds,
                old_id_name as KEY,
                0 as crate::__stddef_size_t_h::size_t,
            ) as *mut ATTRIBUTE_ID;
            if new_id.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            let Some(new_id_name) = pool_string_ref(&raw const new_dtd.pool, (*new_id).name, false) else {
                return 0 as ::core::ffi::c_int;
            };
            new_att.id = Some(new_id_name);
            new_att.isCdata = old_att.isCdata;
            if let Some(value) = old_att.value {
                new_att.value = std::ptr::NonNull::new(
                    poolCopyString(
                        &raw mut new_dtd.pool,
                        value.as_ptr() as *const crate::expat_external_h::XML_Char,
                    ) as *mut crate::expat_external_h::XML_Char,
                );
                if new_att.value.is_none() {
                    return 0 as ::core::ffi::c_int;
                }
            } else {
                new_att.value = None;
            }
            i += 1;
        }
    }
    if copyEntityTable(
        oldParser,
        &raw mut new_dtd.generalEntities,
        &raw mut new_dtd.pool,
        &raw const old_dtd.generalEntities,
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if copyEntityTable(
        oldParser,
        &raw mut new_dtd.paramEntities,
        &raw mut new_dtd.pool,
        &raw const old_dtd.paramEntities,
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    new_dtd.paramEntityRead = old_dtd.paramEntityRead;
    new_dtd.keepProcessing = old_dtd.keepProcessing;
    new_dtd.hasParamEntityRefs = old_dtd.hasParamEntityRefs;
    new_dtd.standalone = old_dtd.standalone;
    new_dtd.in_eldecl = old_dtd.in_eldecl;
    new_dtd.scaffold = old_dtd.scaffold.clone();
    new_dtd.contentStringLen = old_dtd.contentStringLen;
    new_dtd.scaffSize = old_dtd.scaffSize;
    new_dtd.scaffLevel = old_dtd.scaffLevel;
    new_dtd.scaffIndex = old_dtd.scaffIndex.clone();
    return 1 as ::core::ffi::c_int;
}

unsafe extern "C" fn copyEntityTable(
    mut oldParser: crate::expat_h::XML_Parser,
    mut newTable: *mut HASH_TABLE,
    mut newPool: *mut STRING_POOL,
    mut oldTable: *const HASH_TABLE,
) -> ::core::ffi::c_int {
    let mut iter: HASH_TABLE_ITER = HASH_TABLE_ITER {
        table: None,
        next: 0 as crate::__stddef_size_t_h::size_t,
    };
    let mut cachedOldBase: Option<PoolStringRef> = None;
    let mut cachedNewBase: Option<PoolStringRef> = None;
    let table = &*oldTable;
    let old_dtd = &*(*oldParser).m_dtd;
    hashTableIterInit(&raw mut iter, table);
    loop {
        let mut newE: *mut ENTITY = ::core::ptr::null_mut::<ENTITY>();
        let mut name: *const crate::expat_external_h::XML_Char =
            ::core::ptr::null::<crate::expat_external_h::XML_Char>();
        let mut oldE: *const ENTITY = hashTableIterNext(&raw mut iter) as *mut ENTITY;
        if oldE.is_null() {
            break;
        }
        let old_e = &*oldE;
        name = poolCopyString(
            newPool,
            old_e.named.name,
        );
        if name.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        newE = lookup(
            oldParser,
            newTable,
            name as KEY,
            ::core::mem::size_of::<ENTITY>(),
        ) as *mut ENTITY;
        if newE.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        let new_e = &mut *newE;
        if let Some(old_system_id) = old_e.systemId {
            let old_system_id = pool_string_pointer(
                &raw const old_dtd.pool,
                old_system_id,
            );
            if old_system_id.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            let mut tem: *const crate::expat_external_h::XML_Char =
                poolCopyString(newPool, old_system_id);
            if tem.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            new_e.systemId = pool_string_ref(newPool, tem, false);
            if new_e.systemId.is_none() {
                return 0 as ::core::ffi::c_int;
            }
            if old_e.base.is_some() {
                if old_e.base == cachedOldBase {
                    new_e.base = cachedNewBase;
                } else {
                    cachedOldBase = old_e.base;
                    tem = poolCopyString(
                        newPool,
                        pool_string_pointer(
                            &raw const old_dtd.pool,
                            cachedOldBase.expect("base is present after the non-null check"),
                        ),
                    );
                    if tem.is_null() {
                        return 0 as ::core::ffi::c_int;
                    }
                    new_e.base = pool_string_ref(newPool, tem, false);
                    if new_e.base.is_none() {
                        return 0 as ::core::ffi::c_int;
                    }
                    cachedNewBase = new_e.base;
                }
            }
            if let Some(old_public_id) = old_e.publicId {
                let old_public_id = pool_string_pointer(
                    &raw const old_dtd.pool,
                    old_public_id,
                );
                if old_public_id.is_null() {
                    return 0 as ::core::ffi::c_int;
                }
                tem = poolCopyString(newPool, old_public_id);
                if tem.is_null() {
                    return 0 as ::core::ffi::c_int;
                }
                new_e.publicId = pool_string_ref(newPool, tem, false);
                if new_e.publicId.is_none() {
                    return 0 as ::core::ffi::c_int;
                }
            }
        } else {
            let Some(old_text) = old_e.textPtr.present() else {
                return 0 as ::core::ffi::c_int;
            };
            let Some(old_text) = entity_text_chars(old_dtd, old_text, old_e.textLen) else {
                return 0 as ::core::ffi::c_int;
            };
            let Some(new_text) = poolCopyStringN(newPool, old_text.as_ptr(), old_e.textLen)
            else {
                return 0 as ::core::ffi::c_int;
            };
            new_e.textPtr = EntityTextRef {
                pool: EntityTextPool::Dtd,
                string: Some(new_text),
            };
            new_e.textLen = old_e.textLen;
        }
        if let Some(old_notation) = old_e.notation {
            let old_notation = pool_string_pointer(
                &raw const old_dtd.pool,
                old_notation,
            );
            if old_notation.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            let mut tem_1: *const crate::expat_external_h::XML_Char =
                poolCopyString(newPool, old_notation);
            if tem_1.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            new_e.notation = pool_string_ref(newPool, tem_1, false);
            if new_e.notation.is_none() {
                return 0 as ::core::ffi::c_int;
            }
        }
        new_e.is_param = old_e.is_param;
        new_e.is_internal = old_e.is_internal;
    }
    return 1 as ::core::ffi::c_int;
}

pub const INIT_POWER: ::core::ffi::c_int = 6 as ::core::ffi::c_int;

unsafe extern "C" fn keyeq(mut s1: KEY, mut s2: KEY) -> crate::expat_h::XML_Bool {
    while *s1 as ::core::ffi::c_int == *s2 as ::core::ffi::c_int {
        if *s1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return crate::expat_h::XML_TRUE;
        }
        s1 = s1.offset(1);
        s2 = s2.offset(1);
    }
    return crate::expat_h::XML_FALSE;
}

unsafe extern "C" fn keylen(mut s: KEY) -> crate::__stddef_size_t_h::size_t {
    let mut len: crate::__stddef_size_t_h::size_t = 0 as crate::__stddef_size_t_h::size_t;
    while *s != 0 {
        s = s.offset(1);
        len = len.wrapping_add(1);
    }
    return len;
}

unsafe extern "C" fn hash(
    mut parser: crate::expat_h::XML_Parser,
    mut s: KEY,
) -> ::core::ffi::c_ulong {
    let mut state: crate::siphash_h::siphash = crate::siphash_h::siphash {
        v0: 0,
        v1: 0,
        v2: 0,
        v3: 0,
        buf: [0; 8],
        p: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        c: 0,
    };
    let mut key: crate::siphash_h::sipkey = crate::siphash_h::sipkey {
        k: [0, get_hash_secret_salt(parser) as crate::stdlib::uint64_t],
    };
    sip24_init(&raw mut state, &raw mut key);
    sip24_update(
        &raw mut state,
        s as *const ::core::ffi::c_void,
        keylen(s).wrapping_mul(::core::mem::size_of::<crate::expat_external_h::XML_Char>()),
    );
    return sip24_final(&raw mut state) as ::core::ffi::c_ulong;
}

unsafe fn allocation_backing(
    parser: crate::expat_h::XML_Parser,
    size: crate::__stddef_size_t_h::size_t,
    source_line: ::core::ffi::c_int,
) -> Option<Box<dyn FnMut(::core::ffi::c_int)>> {
    let allocation = expat_malloc(parser, size, source_line);
    if allocation.is_null() {
        return None;
    }
    Some(Box::new(move |free_source_line| {
        expat_free(parser, allocation, free_source_line);
    }))
}

fn hash_table_allocation_backing(
    table: &mut HASH_TABLE,
    size: crate::__stddef_size_t_h::size_t,
    source_line: ::core::ffi::c_int,
) -> Option<Box<dyn FnMut(::core::ffi::c_int)>> {
    table
        .allocator
        .as_mut()
        .expect("initialized hash table allocator")
        .allocate(size, source_line)
}

unsafe extern "C" fn lookup(
    mut parser: crate::expat_h::XML_Parser,
    mut table: *mut HASH_TABLE,
    mut name: KEY,
    mut createSize: crate::__stddef_size_t_h::size_t,
) -> *mut NAMED {
    let table = &mut *table;
    let mut i: crate::__stddef_size_t_h::size_t = 0;
    if table.size == 0 as crate::__stddef_size_t_h::size_t {
        if createSize == 0 {
            return ::core::ptr::null_mut::<NAMED>();
        }
        table.power = INIT_POWER as ::core::ffi::c_uchar;
        table.size = (1 as ::core::ffi::c_int as crate::__stddef_size_t_h::size_t) << INIT_POWER;
        let allocation_size = table
            .size
            .wrapping_mul(::core::mem::size_of::<*mut NAMED>());
        let Some(mut backing) = hash_table_allocation_backing(table, allocation_size, 7839 as ::core::ffi::c_int) else {
            table.size = 0 as crate::__stddef_size_t_h::size_t;
            return ::core::ptr::null_mut::<NAMED>();
        };
        let mut slots = Vec::new();
        if slots.try_reserve_exact(table.size).is_err() {
            backing(7841 as ::core::ffi::c_int);
            table.size = 0 as crate::__stddef_size_t_h::size_t;
            return ::core::ptr::null_mut::<NAMED>();
        }
        slots.resize_with(table.size, || None);
        table.v = Some(HashTableSlots {
            entries: slots,
            backing,
        });
        i = (hash(parser, name)
            & (table.size as ::core::ffi::c_ulong).wrapping_sub(1 as ::core::ffi::c_ulong))
            as crate::__stddef_size_t_h::size_t;
    } else {
        let mut h: ::core::ffi::c_ulong = hash(parser, name);
        let mut mask: ::core::ffi::c_ulong =
            (table.size as ::core::ffi::c_ulong).wrapping_sub(1 as ::core::ffi::c_ulong);
        let mut step: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
        i = (h & mask) as crate::__stddef_size_t_h::size_t;
        while table.v.as_ref().expect("initialized hash table").entries[i].is_some() {
            let entry = table.v.as_ref().expect("initialized hash table").entries[i]
                .as_ref()
                .expect("occupied hash table slot")
                .bytes
                .as_ptr() as *mut NAMED;
            if keyeq(name, (*entry).name) != 0 {
                return entry;
            }
            if step == 0 {
                step = ((h & !mask)
                    >> table.power as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    & mask >> 2 as ::core::ffi::c_int
                    | 1 as ::core::ffi::c_ulong) as ::core::ffi::c_uchar;
            }
            if i < step as crate::__stddef_size_t_h::size_t {
                i = i.wrapping_add(
                    table
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
        if table.used >> table.power as ::core::ffi::c_int - 1 as ::core::ffi::c_int != 0 {
            let mut newPower: ::core::ffi::c_uchar = (table.power as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
            if newPower as usize
                >= ::core::mem::size_of::<::core::ffi::c_ulong>().wrapping_mul(8 as usize)
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
                    .wrapping_div(::core::mem::size_of::<*mut NAMED>())
            {
                return ::core::ptr::null_mut::<NAMED>();
            }
            let allocation_size = newSize.wrapping_mul(::core::mem::size_of::<*mut NAMED>());
            let Some(mut backing) = hash_table_allocation_backing(table, allocation_size, 7887 as ::core::ffi::c_int) else {
                return ::core::ptr::null_mut::<NAMED>();
            };
            let mut new_slots = Vec::new();
            if new_slots.try_reserve_exact(newSize).is_err() {
                backing(7889 as ::core::ffi::c_int);
                return ::core::ptr::null_mut::<NAMED>();
            }
            new_slots.resize_with(newSize, || None);
            i = 0 as crate::__stddef_size_t_h::size_t;
            while i < table.size {
                let entry = table
                    .v
                    .as_mut()
                    .expect("initialized hash table")
                    .entries[i]
                    .take();
                if let Some(entry) = entry {
                    let named = entry.bytes.as_ptr() as *mut NAMED;
                    let mut newHash: ::core::ffi::c_ulong =
                        hash(parser, (*named).name);
                    let mut j: crate::__stddef_size_t_h::size_t = newHash
                        as crate::__stddef_size_t_h::size_t
                        & newMask as crate::__stddef_size_t_h::size_t;
                    step = 0 as ::core::ffi::c_uchar;
                    while new_slots[j].is_some() {
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
                    new_slots[j] = Some(entry);
                }
                i = i.wrapping_add(1);
            }
            let old_slots = table
                .v
                .replace(HashTableSlots {
                    entries: new_slots,
                    backing,
                })
                .expect("initialized hash table");
            let mut old_slots = old_slots;
            (old_slots.backing)(7900 as ::core::ffi::c_int);
            table.power = newPower;
            table.size = newSize;
            i = (h & newMask) as crate::__stddef_size_t_h::size_t;
            step = 0 as ::core::ffi::c_uchar;
            while table.v.as_ref().expect("initialized hash table").entries[i].is_some() {
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
    let Some(mut backing) = hash_table_allocation_backing(table, createSize, 7914 as ::core::ffi::c_int) else {
        return ::core::ptr::null_mut::<NAMED>();
    };
    let word_size = ::core::mem::size_of::<usize>();
    let Some(words) = createSize
        .checked_add(word_size.wrapping_sub(1))
        .map(|size| size / word_size)
    else {
        backing(7915 as ::core::ffi::c_int);
        return ::core::ptr::null_mut::<NAMED>();
    };
    let mut bytes = Vec::new();
    if bytes.try_reserve_exact(words).is_err() {
        backing(7915 as ::core::ffi::c_int);
        return ::core::ptr::null_mut::<NAMED>();
    }
    bytes.resize(words, 0);
    let entry = bytes.as_mut_ptr() as *mut NAMED;
    (*entry).name = name;
    table.v.as_mut().expect("initialized hash table").entries[i] =
        Some(NamedAllocation { bytes, backing });
    table.used = table.used.wrapping_add(1);
    return entry;
}

unsafe extern "C" fn hashTableClear(mut table: *mut HASH_TABLE) {
    let table = &mut *table;
    if let Some(slots) = table.v.as_mut() {
        for entry in &mut slots.entries {
            if let Some(mut entry) = entry.take() {
                (entry.backing)(7927 as ::core::ffi::c_int);
            }
        }
    }
    table.used = 0 as crate::__stddef_size_t_h::size_t;
}

unsafe extern "C" fn hashTableDestroy(mut table: *mut HASH_TABLE) {
    let table = &mut *table;
    if let Some(mut slots) = table.v.take() {
        for entry in slots.entries {
            if let Some(mut entry) = entry {
                (entry.backing)(7937 as ::core::ffi::c_int);
            }
        }
        (slots.backing)(7938 as ::core::ffi::c_int);
    }
}

unsafe extern "C" fn hashTableInit(mut p: *mut HASH_TABLE, mut parser: crate::expat_h::XML_Parser) {
    (*p).power = 0 as ::core::ffi::c_uchar;
    (*p).size = 0 as crate::__stddef_size_t_h::size_t;
    (*p).used = 0 as crate::__stddef_size_t_h::size_t;
    ::core::ptr::write(&raw mut (*p).v, None);
    ::core::ptr::write(
        &raw mut (*p).allocator,
        Some(HashTableAllocator {
            allocate: Box::new(move |size, source_line| {
                allocation_backing(parser, size, source_line)
            }),
        }),
    );
}

unsafe extern "C" fn hashTableIterInit<'a>(
    mut iter: *mut HASH_TABLE_ITER<'a>,
    table: &'a HASH_TABLE,
) {
    let iter = &mut *iter;
    iter.table = Some(table);
    iter.next = 0 as crate::__stddef_size_t_h::size_t;
}

unsafe extern "C" fn hashTableIterNext(mut iter: *mut HASH_TABLE_ITER) -> *mut NAMED {
    let iter = &mut *iter;
    let table = match iter.table {
        Some(table) => table,
        None => return ::core::ptr::null_mut::<NAMED>(),
    };
    while iter.next < table.size {
        let index = iter.next;
        iter.next += 1;
        let tem = table
            .v
            .as_ref()
            .and_then(|slots| slots.entries.get(index))
            .and_then(|entry| entry.as_ref());
        if let Some(tem) = tem {
            return tem.bytes.as_ptr() as *mut NAMED;
        }
    }
    return ::core::ptr::null_mut::<NAMED>();
}

// Pool strings live in Rust slabs whose allocator backing remains observable
// through the configured Expat memory suite.  This conversion records only a
// checked tail-relative block ordinal and character offset; it never transfers
// ownership or turns an integer back into an address.
unsafe fn pool_string_ref(
    pool: *const STRING_POOL,
    string: *const crate::expat_external_h::XML_Char,
    allow_block_end: bool,
) -> Option<PoolStringRef> {
    if string.is_null() {
        return None;
    }
    let pool = &*pool;
    let char_size = ::core::mem::size_of::<crate::expat_external_h::XML_Char>();
    for (block_index, block) in pool.storage.active.iter().enumerate() {
        let start = block.chars.as_ptr();
        let start_address = start.addr();
        let byte_offset = string.addr().wrapping_sub(start_address);
        let capacity_bytes = block.chars.len().wrapping_mul(char_size);
        // A committed empty entity may start directly after a preceding value
        // that filled its slab.  Its caller explicitly permits that one-past
        // location; terminated identifiers always require an in-slab element.
        if string.addr() >= start_address
            && (byte_offset < capacity_bytes || allow_block_end && byte_offset == capacity_bytes)
            && byte_offset % char_size == 0
        {
            return Some(PoolStringRef {
                block_from_tail: std::num::NonZeroUsize::new(block_index.checked_add(1)?)?,
                offset: byte_offset / char_size,
            });
        }
    }
    None
}

unsafe fn pool_string_pointer(
    pool: *const STRING_POOL,
    string: PoolStringRef,
) -> *const crate::expat_external_h::XML_Char {
    let pool = &*pool;
    let Some(block_index) = string.block_from_tail.get().checked_sub(1) else {
        return ::core::ptr::null();
    };
    let Some(block) = pool.storage.active.get(block_index) else {
        return ::core::ptr::null();
    };
    // `offset == len` is the valid zero-length string retained above; callers
    // use it only with the entity's explicit zero length.
    if string.offset > block.chars.len() {
        return ::core::ptr::null();
    }
    block.chars.as_ptr().wrapping_add(string.offset)
}

// The callback boundary is the only point where pool-backed entity
// identifiers become C pointers.  Keeping the conversion with the callback
// invocation avoids retaining allocator-owned addresses in `ENTITY`.
unsafe fn invoke_external_entity_ref_handler(
    handler: &dyn ExternalEntityRefCallback,
    parser: crate::expat_h::XML_Parser,
    context: *const crate::expat_external_h::XML_Char,
    pool: *const STRING_POOL,
    entity: *const ENTITY,
) -> ::core::ffi::c_int {
    let entity = &*entity;
    handler.invoke(
        parser,
        context,
        entity.base.map_or(::core::ptr::null(), |base| {
            pool_string_pointer(pool, base)
        }),
        entity.systemId.map_or(::core::ptr::null(), |system_id| {
            pool_string_pointer(pool, system_id)
        }),
        entity.publicId.map_or(::core::ptr::null(), |public_id| {
            pool_string_pointer(pool, public_id)
        }),
    )
}

unsafe extern "C" fn poolInit(mut pool: *mut STRING_POOL, mut parser: crate::expat_h::XML_Parser) {
    let pool = &mut *pool;
    // DTD pools are initialized in memory returned directly by the configured
    // allocator, so this field may not have been constructed yet.
    ::core::ptr::write(
        &raw mut pool.storage,
        StringPoolStorage {
            active: Vec::new(),
            free: Vec::new(),
        },
    );
    pool.start = None;
    pool.ptr_offset = 0;
    pool.parser = parser;
    pool.blockCount = 0;
}

unsafe extern "C" fn poolClear(mut pool: *mut STRING_POOL) {
    let pool = &mut *pool;
    pool.storage.free.append(&mut pool.storage.active);
    pool.blockCount = 0;
    pool.start = None;
    pool.ptr_offset = 0;
}

unsafe extern "C" fn poolDestroy(mut pool: *mut STRING_POOL) {
    let pool = &mut *pool;
    while let Some(mut block) = pool.storage.active.pop() {
        (block.backing)(StringPoolAllocationAction::Free(8000));
    }
    while let Some(mut block) = pool.storage.free.pop() {
        (block.backing)(StringPoolAllocationAction::Free(8006));
    }
}

unsafe extern "C" fn poolAppend(
    mut pool: *mut STRING_POOL,
    mut enc: *const crate::src::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> *mut crate::expat_external_h::XML_Char {
    let pool = &mut *pool;
    if pool.start.is_none() && poolGrow(pool as *mut STRING_POOL) == 0 {
        return ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
    }
    loop {
        let Some(start) = pool.start_ref(true) else {
            return ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
        };
        let Some(capacity) = pool.remaining_capacity() else {
            return ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
        };
        let Some(output_chars) = pool
            .chars_from(start)
            .and_then(|chars| chars.get(..capacity))
        else {
            return ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
        };
        let Some(output_tail) = output_chars.get(pool.ptr_offset..) else {
            return ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
        };
        let output_end = output_chars.as_ptr().wrapping_add(output_chars.len());
        let output_start = output_tail.as_ptr() as *mut crate::expat_external_h::XML_Char;
        let mut output = output_start.cast::<::core::ffi::c_char>();
        let convert_res: crate::src::xmltok::XML_Convert_Result =
            crate::src::xmltok::convert_to_utf8(
                enc,
                &raw mut ptr,
                end,
                &raw mut output,
                output_end as *const ::core::ffi::c_char,
            );
        let Some(written_bytes) = output.addr().checked_sub(output_start.addr()) else {
            return ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
        };
        let char_size = ::core::mem::size_of::<crate::expat_external_h::XML_Char>();
        if written_bytes % char_size != 0 {
            return ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
        }
        let Some(cursor) = pool.ptr_offset.checked_add(written_bytes / char_size) else {
            return ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
        };
        pool.ptr_offset = cursor;
        if convert_res as ::core::ffi::c_uint
            == crate::src::xmltok::XML_CONVERT_COMPLETED as ::core::ffi::c_int
                as ::core::ffi::c_uint
            || convert_res as ::core::ffi::c_uint
                == crate::src::xmltok::XML_CONVERT_INPUT_INCOMPLETE as ::core::ffi::c_int
                    as ::core::ffi::c_uint
        {
            break;
        }
        if poolGrow(pool as *mut STRING_POOL) == 0 {
            return ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
        }
    }
    let Some(start) = pool.start_ref(true) else {
        return ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
    };
    pool.chars_from(start)
        .map_or(::core::ptr::null_mut(), |chars| chars.as_ptr() as *mut _)
}

unsafe extern "C" fn poolCopyString(
    mut pool: *mut STRING_POOL,
    mut s: *const crate::expat_external_h::XML_Char,
) -> *const crate::expat_external_h::XML_Char {
    loop {
        if if (*pool).is_full()
            && poolGrow(pool) == 0
        {
            0 as ::core::ffi::c_int
        } else {
            if (&mut *pool).write_cursor(*s) {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }
        } == 0
        {
            return ::core::ptr::null::<crate::expat_external_h::XML_Char>();
        }
        let c2rust_fresh46 = s;
        s = s.offset(1);
        if *c2rust_fresh46 == 0 {
            break;
        }
    }
    let pool = &mut *pool;
    let Some(start) = pool.start_ref(true) else {
        return ::core::ptr::null();
    };
    s = pool
        .chars_from(start)
        .map_or(::core::ptr::null(), |chars| chars.as_ptr());
    pool.commit();
    return s;
}

unsafe fn poolCopyStringN(
    mut pool: *mut STRING_POOL,
    mut s: *const crate::expat_external_h::XML_Char,
    mut n: ::core::ffi::c_int,
) -> Option<PoolStringRef> {
    if (*pool).start.is_none() && poolGrow(pool) == 0 {
        return None;
    }
    while n > 0 as ::core::ffi::c_int {
        if if (*pool).is_full()
            && poolGrow(pool) == 0
        {
            0 as ::core::ffi::c_int
        } else {
            if (&mut *pool).write_cursor(*s) {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }
        } == 0
        {
            return None;
        }
        n -= 1;
        s = s.offset(1);
    }
    let pool = &mut *pool;
    let string = pool.start_ref(true);
    pool.commit();
    string
}

unsafe extern "C" fn poolAppendString(
    mut pool: *mut STRING_POOL,
    mut s: *const crate::expat_external_h::XML_Char,
) -> *const crate::expat_external_h::XML_Char {
    while *s != 0 {
        if if (*pool).is_full()
            && poolGrow(pool) == 0
        {
            0 as ::core::ffi::c_int
        } else {
            if (&mut *pool).write_cursor(*s) {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }
        } == 0
        {
            return ::core::ptr::null::<crate::expat_external_h::XML_Char>();
        }
        s = s.offset(1);
    }
    let pool = &*pool;
    let Some(start) = pool.start_ref(true) else {
        return ::core::ptr::null();
    };
    pool
        .chars_from(start)
        .map_or(::core::ptr::null(), |chars| chars.as_ptr())
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
    if (*pool).is_full() && poolGrow(pool) == 0 {
        return ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
    }
    if !(&mut *pool).write_cursor(0 as crate::expat_external_h::XML_Char) {
        return ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
    }
    let pool = &*pool;
    let Some(start) = pool.start_ref(true) else {
        return ::core::ptr::null_mut();
    };
    pool
        .chars_from(start)
        .map_or(::core::ptr::null_mut(), |chars| chars.as_ptr() as *mut _)
}

unsafe extern "C" fn poolBytesToAllocateFor(
    mut blockSize: ::core::ffi::c_int,
) -> crate::__stddef_size_t_h::size_t {
    let stretch: crate::__stddef_size_t_h::size_t =
        ::core::mem::size_of::<crate::expat_external_h::XML_Char>();
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
    // The raw cursors are still the C-facing view of the current slab.  Slab
    // ownership and free-list order are ordinary Rust collections, while each
    // slab's backing token preserves the configured allocator lifecycle.
    let pool = &mut *pool;
    if let Some(mut free_block) = pool.storage.free.pop() {
        if pool.start.is_none() {
            pool.storage.active.push(free_block);
            pool.blockCount = pool.storage.active.len();
            pool.start = Some(PoolStringRef {
                block_from_tail: std::num::NonZeroUsize::new(pool.blockCount)
                    .expect("new active block has a non-zero ordinal"),
                offset: 0,
            });
            pool.ptr_offset = 0;
            return crate::expat_h::XML_TRUE;
        }

        let Some(active_capacity) = pool.remaining_capacity() else {
            pool.storage.free.push(free_block);
            return crate::expat_h::XML_FALSE;
        };
        if active_capacity < free_block.chars.len() {
            let Some(source) = pool.start_ref(true) else {
                pool.storage.free.push(free_block);
                return crate::expat_h::XML_FALSE;
            };
            let pointer_offset = pool.ptr_offset;
            let Some(source_index) = source.block_from_tail.get().checked_sub(1) else {
                pool.storage.free.push(free_block);
                return crate::expat_h::XML_FALSE;
            };
            let source_offset = source.offset;
            pool.storage.active.push(free_block);
            let destination_index = pool.storage.active.len() - 1;
            let (older_blocks, destination) = pool.storage.active.split_at_mut(destination_index);
            destination[0].chars[..active_capacity].copy_from_slice(
                &older_blocks[source_index].chars[source_offset..source_offset + active_capacity],
            );
            pool.blockCount = pool.storage.active.len();
            pool.start = Some(PoolStringRef {
                block_from_tail: std::num::NonZeroUsize::new(pool.blockCount)
                    .expect("new active block has a non-zero ordinal"),
                offset: 0,
            });
            pool.ptr_offset = pointer_offset;
            return crate::expat_h::XML_TRUE;
        }
        pool.storage.free.push(free_block);
    }

    let active_block_count = pool.storage.active.len();
    if let Some(block) = pool.storage.active.last_mut() {
        if pool.start == Some(PoolStringRef {
            block_from_tail: std::num::NonZeroUsize::new(active_block_count)
                .expect("non-empty active storage has a non-zero ordinal"),
            offset: 0,
        }) {
            let pointer_offset = pool.ptr_offset;
            let Some(block_size) = block.chars.len().checked_mul(2) else {
                return crate::expat_h::XML_FALSE;
            };
            let Ok(block_size_c) = ::core::ffi::c_int::try_from(block_size) else {
                return crate::expat_h::XML_FALSE;
            };
            let bytes_to_allocate = poolBytesToAllocateFor(block_size_c);
            if bytes_to_allocate == 0 as crate::__stddef_size_t_h::size_t {
                return crate::expat_h::XML_FALSE;
            }
            if block
                .chars
                .try_reserve_exact(block_size - block.chars.len())
                .is_err()
            {
                return crate::expat_h::XML_FALSE;
            }
            if !(block.backing)(StringPoolAllocationAction::Grow(bytes_to_allocate)) {
                pool.ptr_offset = pointer_offset;
                return crate::expat_h::XML_FALSE;
            }
            block.chars.resize(block_size, 0);
            pool.ptr_offset = pointer_offset;
            return crate::expat_h::XML_TRUE;
        }
    }

    let Some(capacity) = pool.remaining_capacity() else {
        return crate::expat_h::XML_FALSE;
    };
    let pointer_offset = pool.ptr_offset;
    let mut block_size = match ::core::ffi::c_int::try_from(capacity) {
        Ok(capacity) => capacity,
        Err(_) => return crate::expat_h::XML_FALSE,
    };
    if block_size < INIT_BLOCK_SIZE {
        block_size = INIT_BLOCK_SIZE;
    } else {
        let Some(doubled) = block_size.checked_mul(2) else {
            return crate::expat_h::XML_FALSE;
        };
        block_size = doubled;
    }
    let bytes_to_allocate = poolBytesToAllocateFor(block_size);
    if bytes_to_allocate == 0 as crate::__stddef_size_t_h::size_t {
        return crate::expat_h::XML_FALSE;
    }
    let allocation = expat_malloc(pool.parser, bytes_to_allocate, 8201 as ::core::ffi::c_int);
    if allocation.is_null() {
        return crate::expat_h::XML_FALSE;
    }
    let mut chars = Vec::new();
    if chars.try_reserve_exact(block_size as usize).is_err() {
        expat_free(pool.parser, allocation, 8201 as ::core::ffi::c_int);
        return crate::expat_h::XML_FALSE;
    }
    chars.resize(block_size as usize, 0);
    let parser = pool.parser;
    let mut allocation = allocation;
    let backing = Box::new(move |action| match action {
        StringPoolAllocationAction::Grow(size) => {
            let reallocated = expat_realloc(parser, allocation, size, 8161);
            if reallocated.is_null() {
                false
            } else {
                allocation = reallocated;
                true
            }
        }
        StringPoolAllocationAction::Free(source_line) => {
            expat_free(parser, allocation, source_line);
            true
        }
    });
    let source = pool.start_ref(true);
    let source_index = if pool.ptr_offset != 0 {
        source.and_then(|source| source.block_from_tail.get().checked_sub(1))
    } else { None };
    let source_offset = source.map(|source| source.offset);
    pool.storage.active.push(StringPoolBlock { chars, backing });
    let destination_index = pool.storage.active.len() - 1;
    if pool.ptr_offset != 0 {
        let Some(source_index) = source_index else {
            return crate::expat_h::XML_FALSE;
        };
        let source_offset = source_offset.expect("pool source offset must accompany its slab");
        let (older_blocks, destination) = pool.storage.active.split_at_mut(destination_index);
        destination[0].chars[..pointer_offset].copy_from_slice(
            &older_blocks[source_index].chars[source_offset..source_offset + pointer_offset],
        );
    }
    pool.blockCount = pool.storage.active.len();
    pool.start = Some(PoolStringRef {
        block_from_tail: std::num::NonZeroUsize::new(pool.blockCount)
            .expect("new active block has a non-zero ordinal"),
        offset: 0,
    });
    pool.ptr_offset = pointer_offset;
    crate::expat_h::XML_TRUE
}

unsafe extern "C" fn nextScaffoldPart(
    mut parser: crate::expat_h::XML_Parser,
) -> ::core::ffi::c_int {
    let dtd: *mut DTD = (*parser).m_dtd;
    {
        let mut scaff_index = (*dtd)
            .scaffIndex
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if scaff_index.is_empty() {
            if scaff_index
                .try_reserve_exact((*parser).m_groupSize as usize)
                .is_err()
            {
                return -1 as ::core::ffi::c_int;
            }
            scaff_index.push(0);
        }
    }
    if (*dtd).scaffCount > crate::limits_h::INT_MAX as ::core::ffi::c_uint {
        return -1 as ::core::ffi::c_int;
    }
    let mut scaffold = (*dtd)
        .scaffold
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if (*dtd).scaffCount >= (*dtd).scaffSize {
        let new_size = if scaffold.backing.is_some() {
            if (*dtd).scaffSize > crate::limits_h::UINT_MAX.wrapping_div(2 as ::core::ffi::c_uint) {
                return -1 as ::core::ffi::c_int;
            }
            (*dtd).scaffSize.wrapping_mul(2 as ::core::ffi::c_uint)
        } else {
            INIT_SCAFFOLD_ELEMENTS as ::core::ffi::c_uint
        };
        let additional = (new_size as usize).saturating_sub(scaffold.nodes.len());
        if scaffold.nodes.try_reserve_exact(additional).is_err() {
            return -1 as ::core::ffi::c_int;
        }
        let allocation_size = (new_size as crate::__stddef_size_t_h::size_t)
            .wrapping_mul(::core::mem::size_of::<CONTENT_SCAFFOLD>());
        if let Some(backing) = scaffold.backing.as_mut() {
            if !backing(ScaffoldAllocationAction::Grow(allocation_size)) {
                return -1 as ::core::ffi::c_int;
            }
        } else {
            let allocation = expat_malloc(
                parser,
                (32 as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(::core::mem::size_of::<CONTENT_SCAFFOLD>()),
                8266 as ::core::ffi::c_int,
            );
            if allocation.is_null() {
                return -1 as ::core::ffi::c_int;
            }
            let mut allocation = allocation;
            scaffold.backing = Some(Box::new(move |action| match action {
                ScaffoldAllocationAction::Grow(size) => {
                    let reallocated = expat_realloc(
                        parser,
                        allocation,
                        size,
                        8261 as ::core::ffi::c_int,
                    );
                    if reallocated.is_null() {
                        false
                    } else {
                        allocation = reallocated;
                        true
                    }
                }
                ScaffoldAllocationAction::Free(source_line) => {
                    expat_free(parser, allocation, source_line);
                    true
                }
            }));
        }
        (*dtd).scaffSize = new_size;
    }
    let next = (*dtd).scaffCount as ::core::ffi::c_int;
    (*dtd).scaffCount = (*dtd).scaffCount.wrapping_add(1);
    if (*dtd).scaffLevel != 0 {
        let parent_index = {
            let scaff_index = (*dtd)
                .scaffIndex
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            match scaff_index.get(((*dtd).scaffLevel - 1) as usize).copied() {
                Some(index) => index,
                None => return -1 as ::core::ffi::c_int,
            }
        };
        let Some(parent) = scaffold.nodes.get(parent_index as usize) else {
            return -1 as ::core::ffi::c_int;
        };
        let last_child = parent.lastchild;
        if last_child != 0 {
            let Some(last_child) = scaffold.nodes.get_mut(last_child as usize) else {
                return -1 as ::core::ffi::c_int;
            };
            last_child.nextsib = next;
        }
        let Some(parent) = scaffold.nodes.get_mut(parent_index as usize) else {
            return -1 as ::core::ffi::c_int;
        };
        if parent.childcnt == 0 {
            parent.firstchild = next;
        }
        parent.lastchild = next;
        parent.childcnt += 1;
    }
    let node = CONTENT_SCAFFOLD {
        type_0: crate::expat_h::XML_CTYPE_EMPTY,
        quant: crate::expat_h::XML_CQUANT_NONE,
        name: None,
        firstchild: 0,
        lastchild: 0,
        childcnt: 0,
        nextsib: 0,
    };
    if let Some(slot) = scaffold.nodes.get_mut(next as usize) {
        *slot = node;
    } else {
        scaffold.nodes.push(node);
    }
    return next;
}

unsafe extern "C" fn build_model(
    mut parser: crate::expat_h::XML_Parser,
) -> *mut crate::expat_h::XML_Content {
    let parser_ref = &mut *parser;
    let dtd = &mut *parser_ref.m_dtd;
    let content_count = dtd.scaffCount as usize;
    let string_count = dtd.contentStringLen as usize;
    let Some(content_bytes) = content_count
        .checked_mul(::core::mem::size_of::<crate::expat_h::XML_Content>())
    else {
        return ::core::ptr::null_mut::<crate::expat_h::XML_Content>();
    };
    let Some(string_bytes) = string_count
        .checked_mul(::core::mem::size_of::<crate::expat_external_h::XML_Char>())
    else {
        return ::core::ptr::null_mut::<crate::expat_h::XML_Content>();
    };
    let Some(allocsize) = content_bytes.checked_add(string_bytes) else {
        return ::core::ptr::null_mut::<crate::expat_h::XML_Content>();
    };
    if content_count == 0 {
        return ::core::ptr::null_mut::<crate::expat_h::XML_Content>();
    }

    // Stage the model in owned Rust storage first.  The final allocation must
    // still use Expat's configured allocator because XML_FreeContentModel
    // returns that ABI-owned block through the same allocator.
    let empty_content = crate::expat_h::XML_Content {
        type_0: crate::expat_h::XML_CTYPE_EMPTY,
        quant: crate::expat_h::XML_CQUANT_NONE,
        name: ::core::ptr::null_mut(),
        numchildren: 0,
        children: ::core::ptr::null_mut(),
    };
    let mut contents = Vec::new();
    let mut child_starts = Vec::new();
    let mut name_offsets = Vec::new();
    let mut names = Vec::new();
    if contents.try_reserve_exact(content_count).is_err()
        || child_starts.try_reserve_exact(content_count).is_err()
        || name_offsets.try_reserve_exact(content_count).is_err()
        || names.try_reserve_exact(string_count).is_err()
    {
        return ::core::ptr::null_mut::<crate::expat_h::XML_Content>();
    }
    contents.resize(content_count, empty_content);
    child_starts.resize(content_count, None::<usize>);
    name_offsets.resize(content_count, None::<usize>);

    let scaffold = dtd
        .scaffold
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let mut job_dest = 1usize;
    contents[0].numchildren = 0;
    for dest_index in 0..content_count {
        let source_index = contents[dest_index].numchildren as usize;
        let Some(source) = scaffold.nodes.get(source_index) else {
            return ::core::ptr::null_mut::<crate::expat_h::XML_Content>();
        };
        contents[dest_index].type_0 = source.type_0;
        contents[dest_index].quant = source.quant;
        if source.type_0 == crate::expat_h::XML_CTYPE_NAME {
            let name_ref = source
                .name
                .expect("name content scaffold must have a pool name");
            let Some(block_index) = name_ref.block_from_tail.get().checked_sub(1) else {
                return ::core::ptr::null_mut::<crate::expat_h::XML_Content>();
            };
            let Some(block) = dtd.pool.storage.active.get(block_index) else {
                return ::core::ptr::null_mut::<crate::expat_h::XML_Content>();
            };
            let Some(name) = block.chars.get(name_ref.offset..) else {
                return ::core::ptr::null_mut::<crate::expat_h::XML_Content>();
            };
            let Some(nul_offset) = name.iter().position(|&ch| ch == 0) else {
                return ::core::ptr::null_mut::<crate::expat_h::XML_Content>();
            };
            let Some(name_len) = nul_offset.checked_add(1) else {
                return ::core::ptr::null_mut::<crate::expat_h::XML_Content>();
            };
            let Some(name_end) = names.len().checked_add(name_len) else {
                return ::core::ptr::null_mut::<crate::expat_h::XML_Content>();
            };
            if name_end > string_count {
                return ::core::ptr::null_mut::<crate::expat_h::XML_Content>();
            }
            name_offsets[dest_index] = Some(names.len());
            names.extend_from_slice(&name[..name_len]);
            contents[dest_index].numchildren = 0;
        } else {
            let Ok(child_count) = usize::try_from(source.childcnt) else {
                return ::core::ptr::null_mut::<crate::expat_h::XML_Content>();
            };
            let Some(next_job_dest) = job_dest.checked_add(child_count) else {
                return ::core::ptr::null_mut::<crate::expat_h::XML_Content>();
            };
            if next_job_dest > content_count {
                return ::core::ptr::null_mut::<crate::expat_h::XML_Content>();
            }
            contents[dest_index].numchildren = child_count as ::core::ffi::c_uint;
            if child_count != 0 {
                child_starts[dest_index] = Some(job_dest);
            }
            let mut child_index = source.firstchild;
            for child_dest in job_dest..next_job_dest {
                let Ok(child_index_usize) = usize::try_from(child_index) else {
                    return ::core::ptr::null_mut::<crate::expat_h::XML_Content>();
                };
                let Some(child) = scaffold.nodes.get(child_index_usize) else {
                    return ::core::ptr::null_mut::<crate::expat_h::XML_Content>();
                };
                contents[child_dest].numchildren = child_index as ::core::ffi::c_uint;
                child_index = child.nextsib;
            }
            job_dest = next_job_dest;
        }
    }
    if job_dest != content_count || names.len() != string_count {
        return ::core::ptr::null_mut::<crate::expat_h::XML_Content>();
    }
    drop(scaffold);

    let ret = parser_ref
        .m_mem
        .malloc_fcn
        .expect("non-null function pointer")(allocsize)
        as *mut crate::expat_h::XML_Content;
    if ret.is_null() {
        return ::core::ptr::null_mut::<crate::expat_h::XML_Content>();
    }
    ::core::ptr::copy_nonoverlapping(contents.as_ptr(), ret, content_count);
    let string_start = ret
        .cast::<u8>()
        .wrapping_add(content_bytes)
        .cast::<crate::expat_external_h::XML_Char>();
    ::core::ptr::copy_nonoverlapping(names.as_ptr(), string_start, string_count);
    let output = ::core::slice::from_raw_parts_mut(ret, content_count);
    for index in 0..content_count {
        if let Some(name_offset) = name_offsets[index] {
            output[index].name = string_start.wrapping_add(name_offset);
        }
        if let Some(child_start) = child_starts[index] {
            output[index].children = ret.wrapping_add(child_start);
        }
    }
    ret
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
        ::core::mem::size_of::<ELEMENT_TYPE>(),
    ) as *mut ELEMENT_TYPE;
    if ret.is_null() {
        return ::core::ptr::null_mut::<ELEMENT_TYPE>();
    }
    if (*ret).named.name != name {
        (*dtd).pool.rewind();
    } else {
        (*dtd).pool.commit();
        if setElementTypePrefix(parser, ret) == 0 {
            return ::core::ptr::null_mut::<ELEMENT_TYPE>();
        }
    }
    return ret;
}

unsafe extern "C" fn copyString(
    mut s: *const crate::expat_external_h::XML_Char,
    mut parser: crate::expat_h::XML_Parser,
) -> *mut crate::expat_external_h::XML_Char {
    let mut charsRequired: crate::__stddef_size_t_h::size_t = 0 as crate::__stddef_size_t_h::size_t;
    let mut result: *mut crate::expat_external_h::XML_Char =
        ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
    while *s.offset(charsRequired as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        charsRequired = charsRequired.wrapping_add(1);
    }
    charsRequired = charsRequired.wrapping_add(1);
    result = expat_malloc(
        parser,
        charsRequired.wrapping_mul(::core::mem::size_of::<crate::expat_external_h::XML_Char>()),
        8456 as ::core::ffi::c_int,
    ) as *mut crate::expat_external_h::XML_Char;
    if result.is_null() {
        return ::core::ptr::null_mut::<crate::expat_external_h::XML_Char>();
    }
    crate::stdlib::memcpy(
        result as *mut ::core::ffi::c_void,
        s as *const ::core::ffi::c_void,
        charsRequired.wrapping_mul(::core::mem::size_of::<crate::expat_external_h::XML_Char>()),
    );
    return result;
}

unsafe extern "C" fn accountingGetCurrentAmplification(
    mut rootParser: crate::expat_h::XML_Parser,
) -> ::core::ffi::c_float {
    let lenOfShortestInclude: crate::__stddef_size_t_h::size_t =
        ::core::mem::size_of::<[::core::ffi::c_char; 23]>()
            .wrapping_sub(1 as crate::__stddef_size_t_h::size_t);
    let countBytesOutput: XmlBigCount = (*rootParser)
        .m_accounting
        .countBytesDirect
        .wrapping_add((*rootParser).m_accounting.countBytesIndirect);
    let amplificationFactor: ::core::ffi::c_float =
        if (*rootParser).m_accounting.countBytesDirect != 0 {
            countBytesOutput as ::core::ffi::c_float
                / (*rootParser).m_accounting.countBytesDirect as ::core::ffi::c_float
        } else {
            (lenOfShortestInclude as XmlBigCount)
                .wrapping_add((*rootParser).m_accounting.countBytesIndirect)
                as ::core::ffi::c_float
                / lenOfShortestInclude as ::core::ffi::c_float
        };
    '_c2rust_label: {
        if (*rootParser).m_parentParser.is_null() {
        } else {
            crate::stdlib::__assert_fail(
                b"! rootParser->m_parentParser\0".as_ptr() as *const ::core::ffi::c_char,
                b"../../expat/lib/xmlparse.c\0".as_ptr() as *const ::core::ffi::c_char,
                8480 as ::core::ffi::c_uint,
                b"float accountingGetCurrentAmplification(XML_Parser)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    return amplificationFactor;
}

unsafe extern "C" fn accountingReportStats(
    mut originParser: crate::expat_h::XML_Parser,
    mut epilog: *const ::core::ffi::c_char,
) {
    let rootParser: crate::expat_h::XML_Parser =
        getRootParserOf(originParser, ::core::ptr::null_mut::<::core::ffi::c_uint>());
    '_c2rust_label: {
        if (*rootParser).m_parentParser.is_null() {
        } else {
            crate::stdlib::__assert_fail(
                b"! rootParser->m_parentParser\0".as_ptr() as *const ::core::ffi::c_char,
                b"../../expat/lib/xmlparse.c\0".as_ptr() as *const ::core::ffi::c_char,
                8487 as ::core::ffi::c_uint,
                b"void accountingReportStats(XML_Parser, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*rootParser).m_accounting.debugLevel == 0 as ::core::ffi::c_ulong {
        return;
    }
    let amplificationFactor: ::core::ffi::c_float = accountingGetCurrentAmplification(rootParser);
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"expat: Accounting(%p): Direct %10llu, indirect %10llu, amplification %8.2f%s\0".as_ptr()
            as *const ::core::ffi::c_char,
        rootParser as *mut ::core::ffi::c_void,
        (*rootParser).m_accounting.countBytesDirect,
        (*rootParser).m_accounting.countBytesIndirect,
        amplificationFactor as ::core::ffi::c_double,
        epilog,
    );
}

unsafe extern "C" fn accountingOnAbort(mut originParser: crate::expat_h::XML_Parser) {
    accountingReportStats(
        originParser,
        b" ABORTING\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
}

fn append_printable_byte(output: &mut Vec<u8>, byte: u8) {
    match byte {
        0 => output.extend_from_slice(b"\\0"),
        b'\t' => output.extend_from_slice(b"\\t"),
        b'\n' => output.extend_from_slice(b"\\n"),
        b'\r' => output.extend_from_slice(b"\\r"),
        b'"' => output.extend_from_slice(b"\\\""),
        b'\\' => output.extend_from_slice(b"\\\\"),
        0x20..=0x7e => output.push(byte),
        _ => {
            const HEX: &[u8; 16] = b"0123456789ABCDEF";
            output.extend_from_slice(b"\\x");
            if byte >= 16 {
                output.push(HEX[(byte >> 4) as usize]);
            }
            output.push(HEX[(byte & 0x0f) as usize]);
        }
    }
}

unsafe extern "C" fn accountingReportDiff(
    mut rootParser: crate::expat_h::XML_Parser,
    mut levelsAwayFromRootParser: ::core::ffi::c_uint,
    mut before: *const ::core::ffi::c_char,
    mut after: *const ::core::ffi::c_char,
    mut bytesMore: crate::__stddef_ptrdiff_t_h::ptrdiff_t,
    mut source_line: ::core::ffi::c_int,
    mut account: XML_Account,
) {
    '_c2rust_label: {
        if (*rootParser).m_parentParser.is_null() {
        } else {
            crate::stdlib::__assert_fail(
                b"! rootParser->m_parentParser\0".as_ptr() as *const ::core::ffi::c_char,
                b"../../expat/lib/xmlparse.c\0".as_ptr() as *const ::core::ffi::c_char,
                8513 as ::core::ffi::c_uint,
                b"void accountingReportDiff(XML_Parser, unsigned int, const char *, const char *, ptrdiff_t, int, enum XML_Account)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    use std::io::Write;

    let mut stderr = std::io::stderr().lock();
    let account_kind = if account as ::core::ffi::c_uint
        == XML_ACCOUNT_DIRECT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        "DIR"
    } else {
        "EXP"
    };
    let _ = write!(
        stderr,
        " (+{:>6} bytes {}|{}, xmlparse.c:{}) {:>10}\"",
        bytesMore,
        account_kind,
        levelsAwayFromRootParser,
        source_line,
        "",
    );
    let ellipsisLength: crate::__stddef_size_t_h::size_t =
        b"[..]".len() as crate::__stddef_size_t_h::size_t;
    let contextLength: ::core::ffi::c_uint = 10 as ::core::ffi::c_uint;
    let mut walker: *const ::core::ffi::c_char = before;
    let mut rendered = Vec::new();
    if (*rootParser).m_accounting.debugLevel >= 3 as ::core::ffi::c_ulong
        || after.offset_from(before)
            <= (contextLength as crate::__stddef_size_t_h::size_t)
                .wrapping_add(ellipsisLength)
                .wrapping_add(contextLength as crate::__stddef_size_t_h::size_t)
                as crate::__stddef_ptrdiff_t_h::ptrdiff_t
    {
        while walker < after {
            append_printable_byte(&mut rendered, *walker as u8);
            walker = walker.offset(1);
        }
    } else {
        while walker < before.offset(contextLength as isize) {
            append_printable_byte(&mut rendered, *walker as u8);
            walker = walker.offset(1);
        }
        rendered.extend_from_slice(b"[..]");
        walker = after.offset(-(contextLength as isize));
        while walker < after {
            append_printable_byte(&mut rendered, *walker as u8);
            walker = walker.offset(1);
        }
    }
    let _ = stderr.write_all(&rendered);
    let _ = stderr.write_all(b"\"\n");
}

unsafe extern "C" fn accountingDiffTolerated(
    mut originParser: crate::expat_h::XML_Parser,
    mut tok: ::core::ffi::c_int,
    mut before: *const ::core::ffi::c_char,
    mut after: *const ::core::ffi::c_char,
    mut source_line: ::core::ffi::c_int,
    mut account: XML_Account,
) -> crate::expat_h::XML_Bool {
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
    let mut levelsAwayFromRootParser: ::core::ffi::c_uint = 0;
    let rootParser: crate::expat_h::XML_Parser =
        getRootParserOf(originParser, &raw mut levelsAwayFromRootParser);
    '_c2rust_label: {
        if (*rootParser).m_parentParser.is_null() {
        } else {
            crate::stdlib::__assert_fail(
                b"! rootParser->m_parentParser\0".as_ptr() as *const ::core::ffi::c_char,
                b"../../expat/lib/xmlparse.c\0".as_ptr() as *const ::core::ffi::c_char,
                8566 as ::core::ffi::c_uint,
                b"XML_Bool accountingDiffTolerated(XML_Parser, int, const char *, const char *, int, enum XML_Account)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    let isDirect: ::core::ffi::c_int = (account as ::core::ffi::c_uint
        == XML_ACCOUNT_DIRECT as ::core::ffi::c_int as ::core::ffi::c_uint
        && originParser == rootParser) as ::core::ffi::c_int;
    let bytesMore: crate::__stddef_ptrdiff_t_h::ptrdiff_t = after.offset_from(before);
    let additionTarget: *mut XmlBigCount = if isDirect != 0 {
        &raw mut (*rootParser).m_accounting.countBytesDirect
    } else {
        &raw mut (*rootParser).m_accounting.countBytesIndirect
    };
    if *additionTarget
        > (-1 as ::core::ffi::c_int as XmlBigCount).wrapping_sub(bytesMore as XmlBigCount)
    {
        return crate::expat_h::XML_FALSE;
    }
    *additionTarget = (*additionTarget).wrapping_add(bytesMore as XmlBigCount);
    let countBytesOutput: XmlBigCount = (*rootParser)
        .m_accounting
        .countBytesDirect
        .wrapping_add((*rootParser).m_accounting.countBytesIndirect);
    let amplificationFactor: ::core::ffi::c_float = accountingGetCurrentAmplification(rootParser);
    let tolerated: crate::expat_h::XML_Bool =
        (countBytesOutput < (*rootParser).m_accounting.activationThresholdBytes
            || amplificationFactor <= (*rootParser).m_accounting.maximumAmplificationFactor)
            as ::core::ffi::c_int as crate::expat_h::XML_Bool;
    if (*rootParser).m_accounting.debugLevel >= 2 as ::core::ffi::c_ulong {
        accountingReportStats(rootParser, b"\0".as_ptr() as *const ::core::ffi::c_char);
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
pub unsafe extern "C" fn testingAccountingGetCountBytesDirect(
    mut parser: crate::expat_h::XML_Parser,
) -> ::core::ffi::c_ulonglong {
    if parser.is_null() {
        return 0 as ::core::ffi::c_ulonglong;
    }
    return (*parser).m_accounting.countBytesDirect as ::core::ffi::c_ulonglong;
}
#[export_name = "testingAccountingGetCountBytesDirect"]

pub unsafe extern "C" fn testingAccountingGetCountBytesDirect_ffi(
    mut parser: crate::expat_h::XML_Parser,
) -> ::core::ffi::c_ulonglong {
    testingAccountingGetCountBytesDirect(parser)
}
pub unsafe extern "C" fn testingAccountingGetCountBytesIndirect(
    mut parser: crate::expat_h::XML_Parser,
) -> ::core::ffi::c_ulonglong {
    if parser.is_null() {
        return 0 as ::core::ffi::c_ulonglong;
    }
    return (*parser).m_accounting.countBytesIndirect as ::core::ffi::c_ulonglong;
}
#[export_name = "testingAccountingGetCountBytesIndirect"]

pub unsafe extern "C" fn testingAccountingGetCountBytesIndirect_ffi(
    mut parser: crate::expat_h::XML_Parser,
) -> ::core::ffi::c_ulonglong {
    testingAccountingGetCountBytesIndirect(parser)
}
unsafe extern "C" fn entityTrackingReportStats(
    mut rootParser: crate::expat_h::XML_Parser,
    mut entity: *mut ENTITY,
    mut action: *const ::core::ffi::c_char,
    mut sourceLine: ::core::ffi::c_int,
) {
    '_c2rust_label: {
        if (*rootParser).m_parentParser.is_null() {
        } else {
            crate::stdlib::__assert_fail(
                b"! rootParser->m_parentParser\0".as_ptr() as *const ::core::ffi::c_char,
                b"../../expat/lib/xmlparse.c\0".as_ptr() as *const ::core::ffi::c_char,
                8617 as ::core::ffi::c_uint,
                b"void entityTrackingReportStats(XML_Parser, ENTITY *, const char *, int)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    if (*rootParser).m_entity_stats.debugLevel == 0 as ::core::ffi::c_ulong {
        return;
    }
    let entityName: *const ::core::ffi::c_char = (*entity).named.name.cast();
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"expat: Entities(%p): Count %9u, depth %2u/%2u %*s%s%s; %s length %d (xmlparse.c:%d)\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        rootParser as *mut ::core::ffi::c_void,
        (*rootParser).m_entity_stats.countEverOpened,
        (*rootParser).m_entity_stats.currentDepth,
        (*rootParser).m_entity_stats.maximumDepthSeen,
        ((*rootParser).m_entity_stats.currentDepth as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            * 2 as ::core::ffi::c_int,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        if (*entity).is_param as ::core::ffi::c_int != 0 {
            b"%\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"&\0".as_ptr() as *const ::core::ffi::c_char
        },
        entityName,
        action,
        (*entity).textLen,
        sourceLine,
    );
}

unsafe extern "C" fn entityTrackingOnOpen(
    mut originParser: crate::expat_h::XML_Parser,
    mut entity: *mut ENTITY,
    mut sourceLine: ::core::ffi::c_int,
) {
    let rootParser: crate::expat_h::XML_Parser =
        getRootParserOf(originParser, ::core::ptr::null_mut::<::core::ffi::c_uint>());
    '_c2rust_label: {
        if (*rootParser).m_parentParser.is_null() {
        } else {
            crate::stdlib::__assert_fail(
                b"! rootParser->m_parentParser\0".as_ptr() as *const ::core::ffi::c_char,
                b"../../expat/lib/xmlparse.c\0".as_ptr() as *const ::core::ffi::c_char,
                8641 as ::core::ffi::c_uint,
                b"void entityTrackingOnOpen(XML_Parser, ENTITY *, int)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
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
        b"OPEN \0".as_ptr() as *const ::core::ffi::c_char,
        sourceLine,
    );
}

unsafe extern "C" fn entityTrackingOnClose(
    mut originParser: crate::expat_h::XML_Parser,
    mut entity: *mut ENTITY,
    mut sourceLine: ::core::ffi::c_int,
) {
    let rootParser: crate::expat_h::XML_Parser =
        getRootParserOf(originParser, ::core::ptr::null_mut::<::core::ffi::c_uint>());
    '_c2rust_label: {
        if (*rootParser).m_parentParser.is_null() {
        } else {
            crate::stdlib::__assert_fail(
                b"! rootParser->m_parentParser\0".as_ptr() as *const ::core::ffi::c_char,
                b"../../expat/lib/xmlparse.c\0".as_ptr() as *const ::core::ffi::c_char,
                8656 as ::core::ffi::c_uint,
                b"void entityTrackingOnClose(XML_Parser, ENTITY *, int)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    entityTrackingReportStats(
        rootParser,
        entity,
        b"CLOSE\0".as_ptr() as *const ::core::ffi::c_char,
        sourceLine,
    );
    (*rootParser).m_entity_stats.currentDepth =
        (*rootParser).m_entity_stats.currentDepth.wrapping_sub(1);
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
pub unsafe extern "C" fn unsignedCharToPrintable(
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
        _ => {
            '_c2rust_label: {
                crate::stdlib::__assert_fail(
                    b"0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../../expat/lib/xmlparse.c\0".as_ptr() as *const ::core::ffi::c_char,
                    9198 as ::core::ffi::c_uint,
                    b"const char *unsignedCharToPrintable(unsigned char)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            return b"dead code\0".as_ptr() as *const ::core::ffi::c_char;
        }
    };
}
#[export_name = "unsignedCharToPrintable"]

pub unsafe extern "C" fn unsignedCharToPrintable_ffi(
    mut c: ::core::ffi::c_uchar,
) -> *const ::core::ffi::c_char {
    unsignedCharToPrintable(c)
}
unsafe extern "C" fn getDebugLevel(
    mut variableName: *const ::core::ffi::c_char,
    mut defaultDebugLevel: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    let valueOrNull: *const ::core::ffi::c_char = crate::stdlib::getenv(variableName);
    if valueOrNull.is_null() {
        return defaultDebugLevel;
    }
    let value: *const ::core::ffi::c_char = valueOrNull;
    *crate::stdlib::__errno_location() = 0 as ::core::ffi::c_int;
    let mut afterValue: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut debugLevel: ::core::ffi::c_ulong =
        crate::stdlib::strtoul(value, &raw mut afterValue, 10 as ::core::ffi::c_int);
    if *crate::stdlib::__errno_location() != 0 as ::core::ffi::c_int
        || afterValue == value as *mut ::core::ffi::c_char
        || *afterValue.offset(0 as isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        *crate::stdlib::__errno_location() = 0 as ::core::ffi::c_int;
        return defaultDebugLevel;
    }
    return debugLevel;
}
unsafe extern "C" fn c2rust_run_static_initializers() {
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];
