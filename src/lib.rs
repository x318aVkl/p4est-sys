
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(unsafe_code)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


#[cfg(feature = "2d")]
pub mod consts {
    pub const DIM: u8 = 2;
}
#[cfg(feature = "3d")]
pub mod consts {
    pub const DIM: u8 = 3;
}
