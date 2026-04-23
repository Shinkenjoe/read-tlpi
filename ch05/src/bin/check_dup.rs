use std::ffi::c_int;

use libc::{F_GETFL, F_SETFL, O_APPEND, O_RDONLY, SEEK_CUR, SEEK_SET, close, dup, fcntl, lseek, off_t, open};

// plan: open a file, duplicate fd, lseek and change Open Flags,  check status of other . 
// which open flags can be changed? O_APPEND, O_ASYNC, O_NOATTIME

fn main(){
    unsafe{
        let fd = open(c"files/50digits".as_ptr(), O_RDONLY);
        if fd == -1 { panic!("close"); }
        


        let dupped = dup(fd);
        if dupped == -1 as c_int {
            panic!("dup");
        }
        
        let mut flags = fcntl(dupped, F_GETFL);
        if flags == -1 as c_int { panic!("get flags"); }
        let mut append = flags & O_APPEND == O_APPEND;
        if append { panic!("append beginning"); }

        let beginning = lseek(dupped, 0, SEEK_CUR);
        if beginning != 0 as off_t { panic!("offset beginning"); }
                             

        if lseek(fd, 1, SEEK_SET) == -1 as off_t {
            panic!("seek"); 
        }

        if fcntl(fd, F_SETFL, O_APPEND) == -1 {
            panic!("set flags"); 
        }
        
        let end = lseek(dupped, 0, SEEK_CUR);
        if end != 1 as off_t { panic!("offset end"); }

        flags = fcntl(dupped, F_GETFL);
        if flags == -1 as c_int { panic!("get flags"); }
        append = flags & O_APPEND == O_APPEND;
        if !append { panic!("append end"); }        
        
        println!("Duplicated FDs share Open Flags and file offset!");

        if close(fd) == -1 as c_int {
            panic!("close");
        }
    }
}
