use libc::{open, iovec, O_RDONLY, readv, ssize_t};
use std::ffi::{c_int, c_char, CString, c_void};
use std::env::args;


fn main(){

    const BUF_LEN : usize = 20;
    let argv = args()
        .map(|arg| CString::new(arg).expect("invalid argument string"))
        .collect::<Vec<CString>>();
    
    unsafe {
        let fd : c_int = open(argv[1].as_ptr(), O_RDONLY); 
        if fd == -1 as c_int {
            panic!("open");
        }
        
        let mut buf_holder1: Vec<c_char> = Vec::new();
        buf_holder1.try_reserve(BUF_LEN).expect("no more memory");
        buf_holder1.resize(BUF_LEN, b'\0' as c_char);
        let buf1 : *mut c_char = buf_holder1.as_mut_ptr();
        
        let mut buf_holder2: Vec<c_char> = Vec::new();
        buf_holder2.try_reserve(BUF_LEN).expect("no more memory");
        buf_holder2.resize(BUF_LEN, b'\0' as c_char);
        let buf2 : *mut c_char = buf_holder2.as_mut_ptr();
        
        let mut buf_holder3: Vec<c_char> = Vec::new();
        buf_holder3.try_reserve(BUF_LEN).expect("no more memory");
        buf_holder3.resize(BUF_LEN, b'\0' as c_char);
        let buf3 : *mut c_char = buf_holder3.as_mut_ptr();

        let iov : [iovec; 3] = [
            iovec { iov_base: buf1 as *mut c_void, 
                    iov_len: BUF_LEN }, 
            iovec { iov_base: buf2 as *mut c_void,
                    iov_len: BUF_LEN }, 
            iovec { iov_base: buf3 as *mut c_void, 
                    iov_len: BUF_LEN }];
        
        let tot_required : ssize_t = (3 * BUF_LEN) as ssize_t;
        
        let num_read : ssize_t =  readv(fd, &iov[0] as *const iovec, 3);
        if num_read == -1 {
            panic!("readv");
        }
        
        if num_read < tot_required {
            println!("Read fewer bytes than requested");
        }
        
        println!("total bytes requested: {tot_required}, bytes read: {num_read}");
    }
}
