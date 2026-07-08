
use std::process::Command;

fn main() {
    

    let root_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let install_dir = std::path::Path::new(&out_dir).join("p4est_install");
    let out_dir = std::path::Path::new(&out_dir);

    let p4est_src_dir = std::path::Path::new(&root_dir).join("p4est");

    //println!("cargo::rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}/build.rs", root_dir);

    //println!("cargo::warning=\"p4est installed to {}\"", install_dir.display());

    // update the submodules of p4est
    
    //std::fs::create_dir_all(&install_dir).unwrap();

    if p4est_src_dir.join("Makefile").exists() {
        Command::new("make")
            .current_dir(&p4est_src_dir)
            .arg("clean")
            .output()
            .expect("make clean command failed");
    }

    match std::fs::remove_file(p4est_src_dir.join("Makefile")) {
        Ok(_) => {Ok(())},
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                // do nothing, this is fine
                Ok(())
            } else {
                Err(e)
            }
        }
    }.unwrap();

    match std::fs::remove_file(p4est_src_dir.join("configure")) {
        Ok(_) => {Ok(())},
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                // do nothing, this is fine
                Ok(())
            } else {
                Err(e)
            }
        }
    }.unwrap();

    match std::fs::remove_file(p4est_src_dir.join("Makefile.in")) {
        Ok(_) => {Ok(())},
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                // do nothing, this is fine
                Ok(())
            } else {
                Err(e)
            }
        }
    }.unwrap();

    match std::fs::remove_file(p4est_src_dir.join("libtool")) {
        Ok(_) => {Ok(())},
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                // do nothing, this is fine
                Ok(())
            } else {
                Err(e)
            }
        }
    }.unwrap();

    Command::new("./bootstrap")
        .current_dir(&p4est_src_dir)
        .output()
        .expect("autoconf command failed");

    Command::new("./configure")
        .current_dir(&p4est_src_dir)
        .arg(format!("--prefix={}", install_dir.display()))
        .arg("--enable-mpi")
        .arg("CC=gcc")
        .output()
        .expect("configure command failed");

    Command::new("make")
        .current_dir(&p4est_src_dir)
        .arg("V=0")
        .output()
        .expect("make command failed");
    
    Command::new("make")
        .current_dir(&p4est_src_dir)
        .arg("install")
        .output()
        .expect("make install command failed");

    // now that p4est is installed, create the bindings
    println!("cargo:rustc-link-lib=p4est");
    println!("cargo:rustc-link-search={}/lib", install_dir.display());

    let bindings = bindgen::Builder::default()
        .header("src/include.h")
        .clang_arg(format!("-I{}/include", install_dir.display()))
        
        .blocklist_item("FP_INT.*")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_NORMAL")
        .blocklist_item("FP_NAN")

        // generate the bindings
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings");

}
