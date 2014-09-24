// use to create a .so/.dll
#![crate_type = "dylib"]

extern crate native;
extern crate libc;
extern crate collections;
use libc::types::common::c95::c_void;
use std::mem::transmute;
use std::collections::HashMap;
use collections::hash::sip::SipHasher;

#[no_mangle]
pub extern fn run(argc: int, argv: *const *const u8, kont: extern fn()) {
    native::start(argc, argv, proc() kont());
}

#[no_mangle]
pub extern fn create_hash() -> *mut c_void {
    // SipHasher at the opposite of RandomSipHasher does not require
    // rust runtime to work, enabling use to use HashMap without runtime
    // too
    let hasher = SipHasher::new();
    let mut english_to_french = box HashMap::with_hasher(hasher);
    english_to_french.insert("one".to_string(), "un".to_string());
    english_to_french.insert("two".to_string(), "deux".to_string());

    return unsafe { transmute(english_to_french) };
}

#[no_mangle]
pub extern fn print_hash(english_to_french_ptr: *mut c_void) {
    let english_to_french : Box<HashMap<String, String>> = unsafe { transmute(english_to_french_ptr) };
    for (english, french) in english_to_french.iter() {
        println!(
            "in french '{}' is '{}'",
            *english,
            *french
        );
    }

}
