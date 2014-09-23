// use to create a .so/.dll
#![crate_type = "dylib"]

// TODO: we use crate collections
// so that without the println we may remove the automatic
// injection of  std crate
// TODO: we could use writeln and/or puts from libc crate
extern crate collections;
extern crate native;
extern crate libc;

use libc::types::common::c95::c_void;
use std::mem::transmute;
// we need to use TreeMap, and not HashMap
// because HashMap use a RNG running in a local thread
// that is init by rust runtime

// note that I could use HashMap with SipHash and own seeded RNG
use collections::TreeMap;

#[no_mangle]
pub extern fn create_hash() -> *mut c_void {
    let mut english_to_french =  box TreeMap::new();
    english_to_french.insert("one".to_string(), "un".to_string());
    english_to_french.insert("two".to_string(), "deux".to_string());

    return unsafe {transmute(english_to_french)};
}

#[no_mangle]
pub extern fn print_hash(english_to_french_ptr: *mut c_void) {
    let english_to_french : Box<TreeMap<String, String>> = unsafe {
        transmute(english_to_french_ptr)
    };
    for (english, french) in english_to_french.iter() {
        println!(
            "in french '{}' is '{}'",
            *english,
            *french
        );
    }

}
