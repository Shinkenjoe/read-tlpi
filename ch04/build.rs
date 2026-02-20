use std::env;
use std::path::PathBuf;

fn main() {
    
    let util_path = PathBuf::from("../tlpi-dist/lib/")
        .canonicalize()
        .expect("cannot canonicalize path");    
    let build_path = util_path.join("build/");
    let build_path_str = build_path.to_str()
       .expect("Couldn't create build bath");
    println!("cargo:rustc-link-search={}", build_path_str);
    println!("cargo-rustc-link-lib=tlpi");

    

    // where the .so or the .a reside, needed by cargo for linking
    let libdir_path = PathBuf::from("/usr/lib/x86_64-linux-gnu/");
    let so_path_str = libdir_path.to_str().expect("Couldn't create build bath");

    println!("cargo:rustc-link-search={}", so_path_str);
    println!("cargo-rustc-link-lib=c");

    let wrapper_path = PathBuf::from("src")
        .canonicalize()
        .expect("could not canocialize local src path")
        .join("wrapper.h");
    let wrapper_path_str = wrapper_path
        .to_str()
        .expect("wrapper path is not a valid string");

    let bindings = bindgen::Builder::default()
        .header(wrapper_path_str)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_function("errMsg")
        .allowlist_function("errExit")
        .allowlist_function("err_exit")
        .allowlist_function("errExitEn")
        .allowlist_function("fatal")
        .allowlist_function("usageErr")
        .allowlist_function("cmdLineErr")
        .allowlist_function("getLong")
        .allowlist_function("getInt")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").expect("No OUT_DIR present"));
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
