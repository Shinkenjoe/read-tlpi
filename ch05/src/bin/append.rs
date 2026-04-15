use libc::{O_APPEND, O_RDWR, SEEK_SET, close, lseek, off_t, open, read, write};
use tlpi::{into_c_char_vec, read_cstr};
use std::ffi::{CString, c_char, c_int, c_void};
use std::fs::{read_to_string};
use std::path::Path;

fn main() {
    unsafe {
        let fd = open(c"files/to_append.txt".as_ptr(), O_RDWR| O_APPEND);
        if fd == -1 {
            panic!("open");
        }
        let at_end = c"This was written at the end.\n";
        let after_seek = c"This was written after seeking to start.\n";
        
        write(fd, at_end.as_ptr() as *const c_void, at_end.count_bytes());
        lseek(fd, 0 as off_t, SEEK_SET);
        write(fd, after_seek.as_ptr() as *const c_void, after_seek.count_bytes());
        
        close(fd);
    }

    println!("{}", read_to_string(Path::new("files/to_append.txt"))
             .expect("Rust could not open file"));
}

