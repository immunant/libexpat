#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(raw_ref_op)]
#![feature(register_tool)]
#![register_tool(c2rust)]

#[macro_use]
extern crate c2rust_bitfields;

pub mod src {
    pub mod lib {
        pub mod xmlparse;
        pub mod xmlrole;
        pub mod xmltok;
    } // mod lib
    pub mod xmlwf {
        pub mod codepage;
        pub mod readfilemap;
        pub mod xmlfile;
    } // mod xmlwf
} // mod src
