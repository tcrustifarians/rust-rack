extern crate libc;

use std::collections::HashMap;
use std::ffi::CString;
use libc::{c_char, c_int, uintptr_t};

pub type RbValue = uintptr_t;

#[link(name = "ruby")]
extern {
    fn rb_define_module(name: *const c_char) -> RbValue;
    fn rb_define_singleton_method(
        object: uintptr_t, name: *const c_char,
        func: extern fn(RbValue, RbValue) -> RbValue,
        argc: c_int);


    fn rb_str_new_cstr(ptr: *const c_char) -> RbValue;

    fn rb_ary_new(len: c_int) -> RbValue;
    fn rb_ary_push(array: RbValue, value: RbValue);

    fn rb_hash_new() -> RbValue;
    fn rb_hash_aset(hash: RbValue, key: RbValue, value: RbValue);
}

unsafe fn map_to_rb_hash(map: HashMap<String, String>) -> RbValue {
    let rb_hash = rb_hash_new();
    for (k, v) in map {
        rb_hash_aset(
            rb_hash,
            rb_str_new_cstr(CString::new(k).unwrap().as_ptr()),
            rb_str_new_cstr(CString::new(v).unwrap().as_ptr()));
    }
    rb_hash
}

unsafe fn rb_hash_to_map(_: RbValue) -> HashMap<String, String> {
    let map = HashMap::new();
    // insert key/value pairs from Ruby hash into `map`
    map
}

unsafe fn vec_to_rb_array(vec: Vec<String>) -> RbValue {
    let len = vec.len();
    let rb_array = rb_ary_new(len as c_int);
    for el in vec {
        rb_ary_push(
            rb_array,
            rb_str_new_cstr(CString::new(el).unwrap().as_ptr()));
    }
    rb_array
}

fn endpoint(_: HashMap<String, String>) -> (String, HashMap<String, String>, Vec<String>) {
    let status = String::from("200");
    let mut headers = HashMap::new();
    headers.insert(String::from("Content-Type"), String::from("text/plain"));
    let body = vec![String::from("Hello from Rust!")];
    (status, headers, body)
}

pub extern fn endpoint_call(_: RbValue, env_hash: RbValue) -> RbValue {
    let env = unsafe { rb_hash_to_map(env_hash) };
    let (status, headers, body) = endpoint(env);
    let retval: RbValue;

    unsafe {
        retval = rb_ary_new(3);
        rb_ary_push(retval,
                    rb_str_new_cstr(CString::new(status).unwrap().as_ptr()));
        rb_ary_push(retval, map_to_rb_hash(headers));
        rb_ary_push(retval, vec_to_rb_array(body));
    }

    retval
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn Init_rust_rack() {
    let mod_name = CString::new("RustRack").unwrap();
    unsafe {
        let mRustRack = rb_define_module(mod_name.as_ptr());
        rb_define_singleton_method(
            mRustRack, CString::new("call").unwrap().as_ptr(),
            endpoint_call, 1);
    }
}

#[test]
fn it_works() {
}
