
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

    // figure out the build probe mpi
    // Try to find an MPI library
    let lib = match build_probe_mpi::probe() {
        Ok(lib) => lib,
        Err(errs) => {
            println!("This library checks multiple methods to find an MPI library on your system. This process has failed to find an MPI library for various reasons:\n");
            for (i, err) in errs.iter().enumerate() {
                println!("Reason #{}:\n{}\n", i + 1, err);
            }
            panic!();
        }
    };


    
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
    println!("cargo:rustc-link-lib=sc");
    println!("cargo:rustc-link-search={}/lib", install_dir.display());

    for dir in &lib.lib_paths {
        println!("cargo:rustc-link-search=native={}", dir.display());
    }
    for lib in &lib.libs {
        println!("cargo:rustc-link-lib={}", lib);
    }

    let build_target = std::env::var("CARGO_CFG_TARGET_ARCH").expect("found CARGO_CFG_TARGET_ARCH variable");


    // first, 2d bindings
    let bindings = bindgen::Builder::default();

    #[cfg(feature = "2d")]
    let bindings = bindings.header("src/include_2d.h");

    #[cfg(feature = "3d")]
    let bindings = bindings.header("src/include_3d.h");

    let mut bindings = bindings
        .clang_arg(format!("-I{}/include", install_dir.display()))

        .clang_arg("-target")
        .clang_arg(format!("{}", build_target))
        
        .blocklist_item("FP_INT.*")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_NORMAL")
        .blocklist_item("FP_NAN");

    for dir in &lib.include_paths {
        bindings = bindings.clang_arg(format!("-I{}", dir.display()));
    }

    let bindings = bindings

        // generate the bindings
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings");

    // all done!
}
