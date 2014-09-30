// we need this one to be able to change the default hasher
// in the type declaration of our hashmap
#![feature(default_type_params)]
// use to create a .so/.dll
#![crate_type = "dylib"]

extern crate collections;
use std::collections::HashMap;
use collections::hash::sip::SipHasher;

#[no_mangle]
pub extern fn create_hash() -> Box<HashMap<String, String, SipHasher>> {
    // SipHasher at the opposite of RandomSipHasher does not require
    // rust runtime to work, enabling use to use HashMap without runtime
    // too
    let hasher = SipHasher::new();
    let mut english_to_french = box HashMap::with_hasher(hasher);
    english_to_french.insert("one".to_string(), "un".to_string());
    english_to_french.insert("two".to_string(), "deux".to_string());

    english_to_french
}

#[no_mangle]
pub extern fn print_hash(english_to_french: &mut HashMap<String, String, SipHasher>) {
    for (english, french) in english_to_french.iter() {
        println!(
            "in french '{}' is '{}'",
            *english,
            *french
        );
    }

}

#[no_mangle]
pub extern fn hash_free(english_to_french: Box<HashMap<String, String, SipHasher>>) {
    let _ = english_to_french;
}
