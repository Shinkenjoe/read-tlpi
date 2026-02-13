use std::ffi::{c_char, CStr };


/// create a Vec<c_char> as buffer holder to be able to give a *mut c_char to C space
/// adds a terminating null character to the vector.
/// no safety checks as it uses only safe Rust.
/// the result has to be turned into a `*mut c_char` ptr by its `as_mut_ptr`
/// during usage of the `*mut c_char` the Vec<c_char> is not allowed to be touched, else UB
pub fn into_c_char_vec(bytestring: &[u8]) -> Vec<c_char> {
    let mut char_vec: Vec<c_char> = bytestring.iter().map(|c| *c as c_char).collect();
    char_vec.push(0 as c_char);
    char_vec
}

/// given a C char pointer, read either until terminating `/0` char or max characters
/// returns None if the cstring doesn't hold utf8 or is null
/// # SAFETY
/// strptr must not reference unallocated memory
/// size of cstring must fit into isize::MAX
pub unsafe  fn read_cstr(strptr: *const c_char) -> Option<String>{
    if strptr.is_null() { return None };
    let cstr =     unsafe { CStr::from_ptr(strptr) };
    match cstr.to_str() {
        Ok(str) => Some(str.to_string()),
        Err(_) => None
    }
}
    


#[cfg(test)]
mod tests {
    use std::ffi::{c_char, CStr};
    #[test]
    fn mut_c_string_compares_equal_to_equal_rust_str() {
        let origin = b"Hello world!";
        let mut charvec: Vec<c_char> = crate::into_c_char_vec(origin);
        let charptr: *mut c_char = charvec.as_mut_ptr();

        let from_cstr = unsafe { CStr::from_ptr(charptr) }.to_str().unwrap();
        let from_bytestring = str::from_utf8(origin).unwrap();
        assert!(from_cstr == from_bytestring);
    }

    #[test]
    fn mut_c_string_is_mutable() {
        let origin = b"Geta";
        let mut char_vec: Vec<c_char> = crate::into_c_char_vec(origin);
        let char_ptr: *mut c_char = char_vec.as_mut_ptr();
        unsafe {
            *char_ptr.add(2) = 'v' as c_char;
        }
        let from_cstr = unsafe { CStr::from_ptr(char_ptr) }.to_str().unwrap();
        assert!(from_cstr == "Geva");
    }

    #[test]
    fn test_reading_in_char_ptr() {
        let char_vec = crate::into_c_char_vec(b"Working");
        let r_string = unsafe {crate::read_cstr(char_vec.as_ptr()).unwrap() };
        assert!(r_string == "Working");
    }
}
