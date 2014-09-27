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
pub extern fn vector_size(vector: &Vec<String>) -> u32 {
    
    vector.len() as u32
}

#[no_mangle]
pub extern fn vector_get(
    vector: &Vec<String> ,
    index: u32
) -> *const libc::c_char {

    // (*vector) should not be necessary, there's a bug in current
    // rust (as of end of september 2014) which force us to do that
    unsafe { (*vector)[index as uint].to_c_str().unwrap() }

}

#[no_mangle]
pub extern fn vector_print(
    vector: &Vec<String>
) {

    for value in vector.iter() {
        println!("from rust {}", value);
    }
}
