use std::env;
use std::path::PathBuf;

fn main() {

    let base_path = PathBuf::from("tlpi")
        // Canonicalize the path as `rustc-link-search` requires an absolute
        // path.
        .canonicalize()
        .expect("cannot canonicalize path");
    let lib_path = base_path.join("build");

        // This is the path to the `c` headers file.
    let headers_path = base_path.join("tlpi_hdr.h");
    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", lib_path.to_str().unwrap());

    // Tell cargo to tell rustc to link our `hello` library. Cargo will
    // automatically know it must look for a `libhello.a` file.
    println!("cargo:rustc-link-lib=tlpi");

        // call cmake to compile the C library
    if !std::process::Command::new("cmake")
        .arg("-S")
        .arg("tlpi")
        .arg("-B")
        .arg("tlpi/build")
        .output()
        .expect("could not spawn `cmake`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not configure build with cmake");
    }

        if !std::process::Command::new("cmake")
        .arg("--build")
        .arg("tlpi/build")
        .output()
        .expect("could not spawn `cmake`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not build with cmake");
    }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(headers_path_str)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // cite the functions you want
        .allowlist_function("errMsg")
        .allowlist_function("errExit")
        .allowlist_function("err_exit")
        .allowlist_function("errExitEn")
        .allowlist_function("fatal")
        .allowlist_function("usageErr")
        .allowlist_function("cmdLineErr")
        .allowlist_function("getLong")
        .allowlist_function("getInt")
        .allowlist_function("errExit")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
    
}
