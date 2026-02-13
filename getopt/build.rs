fn main() {
    /*
    println!("cargo:rustc-link-lib=globals");        
    println!("cargo:rustc-link-search=/home/shinken/comp/genel/os/linux/read-tlpi/getopt/clib/build");    
    */
    
    println!("cargo:rustc-link-lib=c");        
    println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu");    
    
}
