#![allow(unused)]
use std::env::args;
use std::ffi::{CString, CStr, c_int, c_long, c_char, c_void};
use tlpi::{usageErr, errExit, getLong, GN_ANY_BASE, cmdLineErr};
use libc::{ open, read, write, close, isprint, lseek,
            ssize_t, off_t, 
            O_RDWR, O_CREAT,
            S_IRUSR, S_IWUSR, S_IRGRP, S_IWGRP, S_IROTH, S_IWOTH, 
            SEEK_SET };

fn main(){


    let argv = args()
        .map(|arg| {
            CString::new(arg)
                .expect("invalid argument string")})
        .collect::<Vec<CString>>();
    

    if argv.len() < 3 || argv[1] == c"--help" {
        unsafe { 
            usageErr(c"%s file {r<length>|R<length>|w<string>|s<offset>}...\n".as_ptr(),
                     argv[0].as_ptr());
        }
    }
    
    let fd : c_int = unsafe { open(argv[1].as_ptr(),
                                   O_RDWR | O_CREAT, 
                                   S_IRUSR| S_IWUSR| S_IRGRP| S_IWGRP| S_IROTH| S_IWOTH) };
    if fd == -1 as c_int {
        unsafe { errExit(c"open".as_ptr()); }
    }
         
    for ap in 2..argv.len() {
        let arg : String = argv[ap].clone().into_string().expect("arg no utf-8");
        let carg = CString::new(arg.chars().skip(1).collect::<String>())
            .expect("CString arg not well formed");
        match arg.as_bytes()[0] {
            b'r' | b'R' => {
                let len = unsafe {  getLong(carg.as_ptr(),
                                       GN_ANY_BASE as c_int,
                                       argv[ap].as_ptr()) };
                let mut buf_holder: Vec<c_char> = Vec::new();
                buf_holder.try_reserve(len as usize).expect("no more memory");
                buf_holder.resize(len as usize, b'\0' as c_char);
                let buf : *mut c_char = buf_holder.as_mut_ptr();
                
                let num_read = unsafe { read(fd, buf as *mut c_void, len as usize) };
                
                
                if num_read == 0 as ssize_t {
                    println!("{}: end-of-file", arg);
                } else {
                    print!("{}: ", arg);
                    for character in buf_holder {
                        if arg.as_bytes()[0] == b'r' {
                            let c = unsafe {
                                if isprint(character as i32) != 0 { 
                                    char::from_u32(character as u32)
                                        .expect("invalid utf8 input")                                                                    } else { '?'}
                            };
                            print!("{}", c);
                        } else {
                            print!("{:02x} ", character as u32);
                        }
                        
                    }
                    println!("");
                }                                                                            
            }
            /* Write string at current offset */
            b'w' => { 
                println!("{}, {}", arg, carg.clone().into_string().unwrap());
                let num_written = unsafe { write(fd, 
                                                 carg.as_ptr() as *const c_void,
                                                 // CString len is origin length (bytes)
                                                 // - 1 for not first char
                                                 // + 1 for null terminator
                                                 // the original program didn't write the 
                                                 // null terminator
                                                 arg.len() as usize)};
                if num_written == -1 as ssize_t { 
                    unsafe { errExit(c"write".as_ptr()) };
                }
                println!("{}: wrote {} bytes", arg, num_written);
            }
            /* Change file offset */
            b's' => {
                let offset =  unsafe {  getLong(carg.as_ptr(),
                                            GN_ANY_BASE as c_int,
                                            argv[ap].as_ptr())
                };
                unsafe { 
                    if lseek(fd,
                             offset as off_t,
                             SEEK_SET) == -1 as off_t {
                        errExit(c"lseek".as_ptr()); 
                    } else {
                        println!("{}: seek succeeded", arg);
                    }                    
                }
            }
            _ => {
                unsafe {
                    cmdLineErr(c"Argument must start with [rRws]: {}\n".as_ptr(),
                               argv[ap].as_ptr())
                }
            }
        };
    }
}
