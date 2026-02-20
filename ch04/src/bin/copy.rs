// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
use std::env::args;
use std::ffi::{CString, c_char, c_int, c_void};
use std::mem::MaybeUninit;
use tlpi::{errExit, usageErr, fatal};

use libc::{mode_t, ssize_t, read, write, open, close,
           O_RDONLY, O_CREAT, O_WRONLY , O_TRUNC, 
           S_IRUSR, S_IWUSR, S_IRGRP, S_IWGRP, S_IROTH, S_IWOTH};
const BUF_SIZE : usize = 1024;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_fd: c_int;
    let output_fd : c_int;
    let mut num_read: ssize_t;
    let mut buf = MaybeUninit::<[c_char; BUF_SIZE]>::uninit();
    
    let argv = args()
        .map(|arg| CString::new(arg).expect("invalid argument string"))
        .collect::<Vec<CString>>();
    if argv.len() != 3 {
        unsafe { usageErr(c"%s old-file new-file\n".as_ptr(), argv[0].as_ptr());
        }
    }
    
    // Open target input

    unsafe {
        input_fd = open(argv[1].as_ptr(), O_RDONLY);        
    }

    if input_fd == -1 {
        unsafe { errExit(c"opening file %s".as_ptr(),
                                argv[1].as_ptr());
        }
    } 

    let open_flags =  O_CREAT | O_WRONLY | O_TRUNC;
    // rw-rw-rw
    let file_perms : mode_t = S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH ;
    
    unsafe {
        output_fd = open(argv[2].as_ptr(), open_flags, file_perms);        
    }

    if output_fd == -1 {
        unsafe { errExit(c"opening file %s".as_ptr(), argv[2].as_ptr());
        }
    } 

    /* Transfer data until we cencounter end of input or an error */ 
    
    num_read = unsafe { read(input_fd as std::ffi::c_int, 
                             buf.as_mut_ptr() as *mut c_void,
                             BUF_SIZE) };
    while num_read > 0 { 
        unsafe { 
            if  write(output_fd as std::ffi::c_int,
                      buf.as_mut_ptr() as *mut c_void, 
                      num_read as usize) != num_read {
                fatal(c"couldn't write whole buffer".as_ptr());
            }
            num_read =  read(input_fd as std::ffi::c_int, 
                             buf.as_mut_ptr() as *mut c_void,
                             BUF_SIZE);
        }
    }
    if num_read == -1 {
        unsafe { errExit(c"read".as_ptr()); }
    }
    
    if unsafe { close(input_fd) == -1 } {
        unsafe { errExit(c"close input".as_ptr());}
    }

    if unsafe { close(output_fd) == -1 } {
        unsafe { errExit(c"close output".as_ptr());}
    }

    Ok(())
}
    

