
use std::ffi::{CStr};
use std::error::Error;
use std::fs::File;

pub fn read_libc_version() -> Result<(), Box<dyn Error>>{
    unsafe {
        let version =  CStr::from_ptr(libc::gnu_get_libc_version()).to_str()?;            
        println!("My GNU libc version is {version}");
        Ok(())
    }        
}

pub fn print_error() -> Result<(), Box<dyn Error>>{
    // produce an error on a syscall
    File::open("/home/shinken/does_not_exist.txt").err();    
    let msg = c"custom message";
    unsafe {
        libc::perror(msg.as_ptr());        
        let error_message = CStr::from_ptr(libc::strerror(libc::ENOSYS)).to_str()?;
        println!("system message for ENOSYS error : {error_message}");
    }
    Ok(())
}


