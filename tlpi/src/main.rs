use tlpi::errExit;

fn main() {
    unsafe { errExit(c"ErrExit is known!".as_ptr()); }
}
    

