use clap::{ Parser };
use libc::{O_APPEND, O_CREAT, O_WRONLY, SEEK_END, c_int, close, lseek, open, write};
use std::{ffi::{CString, c_void}, path::Path};

// to be called directly in the /target directory, cause i dunno how to supply 
// paths with a cargo call (which is probably possible) 

#[derive(Parser)]
#[command(version, about,
          long_about = "examine the diffrence between append and seek to end before write")]
struct Cli{
    file: String,    
    
    /// number of bytes to write
    #[arg(default_value_t = 1_000_000usize)]
    num_bytes: usize,
    /// append to file
    #[arg(short, long)]
    non_append: bool,
}

fn main(){
    let cli = Cli::parse();
    let rust_path = Path::new(cli.file.as_str());
    let create = !rust_path.exists();
    let path = CString::new(cli.file).expect("could not read path");
    let mut oflags : c_int = O_WRONLY;
        if create {
            oflags |= O_CREAT;
        }
        if !cli.non_append {
            oflags |= O_APPEND;
        }
    let byte = 0u8;
    let byte_ptr = &raw const byte;

    unsafe {
        let fd = open(path.as_ptr(), oflags);
        if cli.non_append {
            for _ in 0..cli.num_bytes {
                lseek(fd, 0, SEEK_END);
                write(fd, byte_ptr  as *const c_void, 1);
            }
        } else {
            for _ in 0..cli.num_bytes {
                write(fd, byte_ptr  as *const c_void, 1);
            }
        }         
        close(fd);
    }                    
}

// can see no difference. Maybe the rust compiler does stuff
