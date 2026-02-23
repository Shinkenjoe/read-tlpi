use std::error::Error;
use libc::{O_APPEND, O_CREAT, O_RDONLY, O_RDWR, O_TRUNC, O_WRONLY, S_IRUSR, S_IWUSR,  open, close, read, ssize_t};
use std::ffi::{c_char, CStr, c_void};

use tlpi::errExit;

fn main() -> Result<(), Box<dyn Error>> {   

    unsafe {
        /* Open existing file for reading */
        let fd1  = open(c"files/read_this".as_ptr(), O_RDONLY); 
        if fd1 as i32 == -1 {
            errExit(c"open".as_ptr());
        }
    
        /* Open new or existing file for reading and wirting, truncating to zero
           bytes; file permissions read and writ for ownner, nothing for others;
        */
        
        let fd2 =  open(c"files/myfile".as_ptr(),
                        O_RDWR | O_CREAT | O_TRUNC,
                        S_IRUSR | S_IWUSR);
        if  fd2 as i32 == -1 {
            errExit(c"open".as_ptr());
        }
        
        /* Open new or exsting file for writing; writes should awlays append
        to end of file */
        
        let fd3 = open(c"files/w.log".as_ptr(), O_WRONLY | O_CREAT | O_TRUNC | O_APPEND, 
                  S_IRUSR | S_IWUSR);
        if fd3 == -1 { errExit(c"open".as_ptr()); }

        /*  use the fact that a file descriptor is always the lowest empty one */

        if close(libc::STDERR_FILENO) == - 1  { /* Close file descriptor 0 */
            errExit(c"close".as_ptr());
        }
        
        let fd4 = open(c"read_this".as_ptr(), O_RDONLY);
        if fd4 as i32 == -1 { errExit(c"open".as_ptr())}

        /* read */ 

        const MAX_READ : usize = 200;
        // I'm zero filling the buffer instead of using MaybeUninit, 
        // because read will potentiall not fill all the buffer. 
        // thus there will be uninitialized memory at the back
        // if i call `assume_init` on it, im in UB
        // I'm not planing on micro-optimizing Rust to use
        // MaybeUninit in some way
        let mut buf_1 = ['\0' as c_char; MAX_READ + 1];

        
        let num_read_1 = read(libc::STDIN_FILENO, 
                buf_1.as_mut_ptr() as *mut c_void,
                MAX_READ); 
        if num_read_1 == -1 as ssize_t  { 
            errExit(c"read".as_ptr());
        }

        buf_1[num_read_1 as usize] = '\0' as c_char;
        println!("The input data was: {}", 
                 CStr::from_ptr(buf_1.as_ptr() as *const c_char)
                 .to_str().expect("CStr"));
        
        
    }

    Ok(())     
}
