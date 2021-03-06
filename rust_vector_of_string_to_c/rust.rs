// use to create a .so/.dll
#![crate_type = "dylib"]

use std::c_str::CString;

/// create a new vector with some values inside for test
///
/// note: we can return directly a Box<Vec<>> and use it
/// as a pointer from C, because as Vec is not a dynamically sized
/// type, we're garantueed that Box is a single pointer
///
/// note2: after using vector_create from C, we will need to destroy
/// the value ourselves using vector_free function
#[no_mangle]
pub extern fn vector_create() -> Box<Vec<String>> {

    let vector : Box<Vec<String>> = box vec![
        "你好".to_string(),
        "好".to_string()
    ];

    vector
}

/// get the size of the vector given in parameter
///
///
#[no_mangle]
pub extern fn vector_size(vector: &Vec<String>) -> u32 {
    
    vector.len() as u32
}

/// permit to do vector[index] from C
///
/// note: to be 100% correct we should returned
/// *const libc::c_char, but that would required to use the
/// crate libc
///
/// note2: we need to free the pointer returned by this function
/// ourselves, using the function vector_value_free
#[no_mangle]
pub extern fn vector_value_get(
    vector: &Vec<String> ,
    index: u32
) -> *const i8 {

    // (*vector) should not be necessary, there's a bug in current
    // rust (as of end of september 2014) which force us to do that
    unsafe { (*vector)[index as uint].to_c_str().unwrap() }

}

/// retake ownership, so that we can let rust automatically
/// free the variable at the end of this function scope
///
#[no_mangle]
pub extern fn vector_value_free (buffer: *const i8) {
    unsafe { CString::new(buffer, true); }
}


/// simple debug function to print the content of a vector
///
///
#[no_mangle]
pub extern fn vector_print(
    vector: &Vec<String>
) {

    for value in vector.iter() {
        println!("from rust {}", value);
    }
}


/// Give back the ownership of the vector to rust
/// so that rust can free it
///
#[no_mangle]
pub extern fn vector_free(
    vector: Box<Vec<String>>
) {
    let _ = vector;
}
