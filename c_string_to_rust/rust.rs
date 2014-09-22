// use to create a .so/.dll
#![crate_type = "dylib"]

extern crate libc;
use std::string::raw::from_buf;

// used to have this use function callable from C
// 1. no_mangle garanty the name will stay rust_print
// 2. pub extern garanty the name is exported
#[no_mangle]
pub extern fn rust_print(value: *const libc::c_char) {

    // unsafe block because we're not sure if the pointer
    // given by this library is legit (we could receive a null
    // pointer, or pointer on value that can't be converted to
    // a UTF-8 string etc.

    // note we need to cast "as *const u8" because for a reason that
    // I can't explain,  libc::c_char is i8 ...
    let string = unsafe { from_buf(value as *const u8) };
    
    println!("haha im from rust {}", string)
}

