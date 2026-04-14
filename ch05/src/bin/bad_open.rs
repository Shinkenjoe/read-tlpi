use libc::{__errno_location, ENOENT, O_CREAT, O_WRONLY, S_IRUSR, S_IWUSR, close, getpid, open};
use tlpi::{errExit};
use std::env::args;
use std::ffi::{CString};

fn main() {
    let argv = args()
        .map(|arg| CString::new(arg).expect("invalid argument string"))
        .collect::<Vec<CString>>();
    
    unsafe {
        let fd = open(argv[1].as_ptr(), O_WRONLY); 
        if fd != -1 {
            println!("[PID {}]: File \"{}\" already exists", 
                     getpid(), argv[1].to_str().expect("no utf-8 file name"));
            close(fd);
        } else {            
            if argv.len() > 2 {
                std::thread::sleep(std::time::Duration::from_secs(5));
                println!("[PID {}] Done sleeping.", getpid());
            }
            if *__errno_location() != ENOENT {
                errExit(c"open".as_ptr());
            } else {
                let fd = open(argv[1].as_ptr(), 
                              O_WRONLY | O_CREAT, 
                              S_IRUSR | S_IWUSR);
                if fd == -1 {
                    errExit(c"open".as_ptr());
                }
                

                println!("[PID {}] Created file \"{}\" exclusively.", 
                         getpid(), argv[1].to_str().expect("no utf-8 file name"));
                if close(fd) == -1 {
                panic!("close");
            }

                                                         
            }            
        }
    }
}
