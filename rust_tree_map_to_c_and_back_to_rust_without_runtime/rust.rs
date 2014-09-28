// use to create a .so/.dll
#![crate_type = "dylib"]

// we use crate collections which contains only
// collections that does not need the runtime to run by default
extern crate collections;
extern crate native;
extern crate libc;

// NOTE: at first I wanted to push the example further and not to
// include the std crate, which is basically what you would want to
// do for system programming
//
// but too much features will not be available without implemenenting
// your own memory allocator
// i.e  "".to_string  or insertion in collection etc.
//
// waiting to write a dedicated example about building without std
// here is basically what you need to append at top of your file
//
//#![no_std]
//#![crate_type = "dylib"]
//#![feature(phase)]
//
//#[phase(plugin,link)]
//extern crate core;


// we need to use TreeMap, and not HashMap
// because HashMap use a RNG running in a local thread
// that is init by rust runtime

// note that I could use HashMap with SipHash and own seeded RNG
use collections::TreeMap;

#[no_mangle]
pub extern fn create_hash() -> Box<TreeMap<String,String>> {
    let mut english_to_french =  box TreeMap::new();
    english_to_french.insert("one".to_string(), "un".to_string());
    english_to_french.insert("two".to_string(), "deux".to_string());

    english_to_french
}

#[no_mangle]
pub extern fn print_hash(english_to_french: Box<TreeMap<String,String>>) {
    for (english, french) in english_to_french.iter() {
        println!(
            "in french '{}' is '{}'",
            *english,
            *french
        );
    }

}
