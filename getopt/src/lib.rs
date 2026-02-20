/// just reproducing the successful call, further calls and errors is now too much work for
/// me. 
/// Here i struggeled with: Dealing with C `char*` as the ffi types only handle const char*
/// I wrote some methods that read a char* and return an Option<String> 
/// also a method to create a Vec<u8> from byte strings as a buffer holder for a mut char*
/// i should adjust the method and give a size, such that the buffer is really a
/// directly C-like indexable buffer of a certain size, zeroing out the tail, if not used at
/// first. These buffers act only as memory holders for C code to fuss around. If i want to
/// do anything with them in Rust I'd turn them into String first. 
/// integrating the mut statics correctly, the fails here was trying to: 
/// - linking with the unistd header
/// - creating an own library that does nothing else but load the header and give the 
///   address for the mut static 
/// - not noting that to link against 'libc' i have to supply the name 'c'
/// 
/// i failed to copy the getopt pattern string correctly, the colon follows the option that
/// takes an argument, then i had a bug such that the option

/// next i will have to deal with the custom tlpi error library from the book. I will maybe
/// do it in steps. First copy paste the c library and create bindings in utils


#[cfg(test)]
mod tests {
    use std::ffi::{c_int, c_char};


    /*
    #[link(name="globals")]
    unsafe extern "C" {
        fn get_optopt_location() -> *const c_int;        
        */
    #[link(name="c")]
    unsafe extern "C" {
        static mut optind: c_int;
        static mut optarg: *mut c_char;
        /*
        static mut optopt: c_int;
        static mut opterr: c_int;
        */
    }
    
    #[test]
    fn test_getopt_full_call() {
        let mut program_vec: Vec<c_char> = tlpi::into_c_char_vec(b"t_getopt");
        let mut arg1_vec: Vec<c_char> = tlpi::into_c_char_vec(b"-x");
        let mut arg2_vec: Vec<c_char> = tlpi::into_c_char_vec(b"-p");
        let mut arg3_vec: Vec<c_char> = tlpi::into_c_char_vec(b"hello");
        let argv = [program_vec.as_mut_ptr(), 
                    arg1_vec.as_mut_ptr(), 
                    arg2_vec.as_mut_ptr(), 
                    arg3_vec.as_mut_ptr()];
        let argc = argv.len() as c_int;
        let pattern_str = c":p:x";
        unsafe {
            let arg_read : c_int =  libc::getopt(argc, argv.as_ptr(), 
                                                   pattern_str.as_ptr());
          
            assert_eq!(arg_read as u8, b'x');
            assert_eq!(optind as i32, 2); 
                        
            let arg_read : c_int =  libc::getopt(argc, argv.as_ptr(),
                                                 pattern_str.as_ptr());
            assert_eq!(arg_read as u8, b'p');            
            assert_eq!(optind as i32, 4);
          
            assert_eq!(tlpi::read_cstr(optarg).unwrap(), "hello");                        
        }        
         
    }
}
