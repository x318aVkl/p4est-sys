
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(unsafe_code)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


#[cfg(feature = "2d")]
pub mod consts {
    pub const DIM: usize = 2;
    pub const CELL_CORNERS: usize = 4;
    pub const FACE_CORNERS: usize = 2;
    pub const CELL_FACES: usize = 4;
}
#[cfg(feature = "3d")]
pub mod consts {
    pub const DIM: usize = 3;
    pub const CELL_CORNERS: usize = 8;
    pub const FACE_CORNERS: usize = 4;
    pub const CELL_FACES: usize = 6;
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
    pub type p4est_refine_t = p8est_refine_t;
    pub type p4est_coarsen_t = p8est_coarsen_t;
    pub type p4est_iter_volume_info = p8est_iter_volume_info;
    pub type p4est_iter_face_info = p8est_iter_face_info;

    pub type p4est_quadrant = p8est_quadrant;
    pub type p4est_iter_face_side = p8est_iter_face_side;

    pub type p4est_tree = p8est_tree;

    pub use p8est_new as p4est_new;
    pub use p8est_destroy as p4est_destroy;

    pub use p8est_refine as p4est_refine;
    pub use p8est_coarsen as p4est_coarsen;

    pub use p8est_iterate as p4est_iterate;

    pub use p8est_partition as p4est_partition;

    pub use p8est_quadrant_corner_node as p4est_quadrant_corner_node;

    pub use p8est_connectivity_new as p4est_connectivity_new;
    pub use p8est_connectivity_new_unitcube as p4est_connectivity_new_unitcube;
    pub use p8est_connectivity_destroy as p4est_connectivity_destroy;

    pub use p8est_connectivity_is_valid as p4est_connectivity_is_valid;
    pub use p8est_connectivity_complete as p4est_connectivity_complete;


    pub use p8est_ghost_new as p4est_ghost_new;
    pub use p8est_ghost_destroy as p4est_ghost_destroy;

    pub use p8est_ghost_exchange_data as p4est_ghost_exchange_data;

    pub const P4EST_MAXLEVEL: u32 = P8EST_MAXLEVEL;

    pub const p4est_connect_type_t_P4EST_CONNECT_FACE: u32 = p8est_connect_type_t_P8EST_CONNECT_FACE;

}
