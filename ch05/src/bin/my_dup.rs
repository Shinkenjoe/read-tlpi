use std::ffi::c_int;
use libc::{__errno_location, EBADF, F_DUPFD, F_GETFL, O_CREAT, O_TRUNC, O_WRONLY, S_IRUSR, S_IWUSR, open, close, fcntl};

/// # Safety  
/// 
/// handling linux file descriptors
pub unsafe fn my_dup(oldfd: c_int) -> c_int {
    unsafe {
        fcntl(oldfd, F_DUPFD, 0)
    }
}
/// # Safety
///
/// handling linux file descriptors  
pub unsafe fn my_dup2(oldfd: c_int, newfd: c_int) -> c_int {    
    unsafe {
        let errno_ptr = __errno_location();

        if oldfd == newfd {
            if (fcntl(oldfd, F_GETFL)) == -1 {
                *errno_ptr = EBADF;
                return -1;
            }
            return oldfd;
        }
        
        
        let open_status  = fcntl(newfd, F_GETFL);
        if open_status == -1 {
            if *errno_ptr != EBADF {
                return -1;
            }
        } else {
            close(newfd);
        }
        fcntl(oldfd, F_DUPFD, newfd)                    
    }
}


fn main() {
   unsafe {
       let stdin_copy = my_dup(0) as i32;
       println!("fd of stdin copy {}", stdin_copy);
       let fd = open(c"files/redirect_by_dup.txt".as_ptr(), 
                 O_WRONLY | O_TRUNC | O_CREAT, 
                 S_IWUSR | S_IRUSR);
       if fd == -1 { panic!("open") ; }
       let duplicate = my_dup2(fd, 1);
       if duplicate == -1 { panic!("duplicate"); }               
       println!("This was written on stdout");
   }
}
