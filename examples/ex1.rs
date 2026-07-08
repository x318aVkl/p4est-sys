
fn main() {
    unsafe {

        let connectivity = p4est_sys::ffi::p4est_connectivity_new_unitsquare();

        let grid = p4est_sys::ffi::p4est_new(
            std::ptr::null_mut(),
            connectivity,
            0,
            None,
            std::ptr::null_mut(),
        );

        p4est_sys::ffi::p4est_connectivity_destroy(connectivity);
        p4est_sys::ffi::p4est_destroy(grid);
    }
    
    println!("Hello, world!");
}