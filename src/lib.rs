
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


pub use aliases::*;

#[cfg(feature = "2d")]
pub mod aliases {

}

#[cfg(feature = "3d")]
pub mod aliases {
    pub use crate::*;

    pub type p4est = p8est;
    pub type p4est_connectivity = p8est_connectivity;
    pub type p4est_ghost = p8est_ghost;

    pub use p8est_new as p4est_new;
    pub use p8est_destroy as p4est_destroy;

    pub use p8est_connectivity_new_unitcube as p4est_connectivity_new_unitcube;
    pub use p8est_connectivity_destroy as p4est_connectivity_destroy;

    pub use p8est_ghost_new as p4est_ghost_new;
    pub use p8est_ghost_destroy as p4est_ghost_destroy;

}
