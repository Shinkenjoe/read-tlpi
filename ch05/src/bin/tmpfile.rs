use libc::{close, mkstemp, pread64, unlink, write, off64_t};
use tlpi::{into_c_char_vec, read_cstr};
use std::ffi::{CString, c_char, c_int, c_void};

fn main() {

    unsafe {
        let mut template : Vec<c_char> =  into_c_char_vec(b"/tmp/someStringXXXXXX");
        let fd : c_int = mkstemp(template.as_mut_ptr());
        if fd == -1 {
            panic!("mkstemp");
        }
        println!("generated filename was {}", 
                 read_cstr(template.as_ptr())
                 .expect("tmpflile name no valid cstr"));
        unlink(template.as_ptr());
        let message = c"Hello from the tmpfile";
        write(fd, message.as_ptr() as *const c_void,  message.count_bytes());        

        let mut buf_holder: Vec<c_char> = Vec::new();
        buf_holder.try_reserve(message.count_bytes() + 1).expect("no more memory");
        buf_holder.resize(message.count_bytes() + 1, b'\0' as c_char);
        let buf : *mut c_char = buf_holder.as_mut_ptr();

        pread64(fd, buf as *mut c_void, message.count_bytes(), 0 as off64_t);
        let message = CString::from_vec_unchecked(buf_holder
                                        .into_iter()
                                        .map(|c| -> u8 { c as u8})
                                        .collect::<Vec<u8>>())
            .into_string().expect("no valid utf-8");
        println!("{message}");        
        close(fd);
    }
    
    
}
