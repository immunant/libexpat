#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![deny(unsafe_op_in_unsafe_fn)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(raw_ref_op)]

#[macro_use]
extern crate c2rust_bitfields;

pub mod src {
    pub mod lib {
        pub mod xmlparse;
        pub mod xmlrole;
        pub mod xmltok;
    } // mod lib
    pub mod tests {
        pub mod acc_tests;
        pub mod alloc_tests;
        pub mod basic_tests;
        pub mod chardata;
        pub mod common;
        pub mod dummy;
        pub mod handlers;
        pub mod memcheck;
        pub mod minicheck;
        pub mod misc_tests;
        pub mod ns_tests;
        pub mod nsalloc_tests;
        pub mod runtests;
        pub mod structdata;
    } // mod tests
} // mod src
