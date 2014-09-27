// use to create a .so/.dll
#![crate_type = "dylib"]

extern crate libc;

use std::mem::transmute;
use libc::types::common::c95::c_void;

// used to have this use function callable from C
// 1. no_mangle garanty the name will stay rust_print
// 2. pub extern garanty the name is exported
#[no_mangle]
pub extern fn vector_create() -> *mut c_void {

    let vector : Box<Vec<String>> = box vec![
        "你好".to_string(),
        "好".to_string()
    ];

    unsafe {transmute(vector)}
}

#[no_mangle]
pub extern fn vector_size(vector_ptr: *mut c_void) -> u32 {
    let vector : Box<Vec<String>> = unsafe {
        transmute(vector_ptr)
    };
    
    vector.len() as u32
}

#[no_mangle]
pub extern fn vector_get(
    vector_ptr: *mut c_void,
    index: u32
) -> *const libc::c_char {

    let vector : Box<Vec<String>> = unsafe {
        transmute(vector_ptr)
    };

    // (*vector) should not be necessary, there's a bug in current
    // rust (as of end of september 2014) which force us to do that
    let value = (*vector)[index as uint].to_c_str();

    value.as_ptr()
}

#[no_mangle]
pub extern fn vector_print(
    vector_ptr: *mut c_void,
) {

    let vector : Box<Vec<String>> = unsafe {
        transmute(vector_ptr)
    };

    for value in vector.iter() {
        println!("from rust {}", value);
    }
}
