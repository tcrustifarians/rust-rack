#![feature(cstr_to_str)]

extern crate libc;

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use libc::{c_char, c_int, c_long, uintptr_t};

pub type RbValue = uintptr_t;
pub type RbId = uintptr_t;

#[link(name = "ruby")]
extern {
    fn rb_define_module(name: *const c_char) -> RbValue;
    fn rb_define_singleton_method(
        object: uintptr_t, name: *const c_char,
        func: extern fn(RbValue, RbValue) -> RbValue,
        argc: c_int);

    fn rb_intern(name: *const c_char) -> RbId;
    fn rb_block_call(obj: RbValue, meth: RbId,
                     argc: c_int, argv: *const RbValue,
                     block: extern fn(RbValue, RbValue, c_int, *const RbValue) -> RbValue,
                     data: RbValue) -> RbValue;

    fn rb_inspect(obj: RbValue) -> RbValue;

    fn rb_str_new_cstr(ptr: *const c_char) -> RbValue;
    fn rb_string_value_cstr(ptr: *const RbValue) -> *const c_char;

    fn rb_ary_new(len: c_int) -> RbValue;
    fn rb_ary_entry(array: RbValue, offset: c_long) -> RbValue;
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

extern fn insert_rb_hash_entry_into_map(_: RbValue, map_ptr: RbValue, _argc: c_int, argv: *const RbValue) -> RbValue {
    let (map, key, val): (&mut HashMap<_,_>, String, RbValue);
    unsafe {
        map = &mut *(map_ptr as *mut HashMap<String, RbValue>);
        key = String::from(CStr::from_ptr(rb_string_value_cstr(&rb_ary_entry(*argv, 0) as *const RbValue)).to_str().ok().unwrap());
        val = rb_ary_entry(*argv, 1);
    }
    map.insert(key, val);
    0
}

unsafe fn rb_hash_to_map(rb_hash: RbValue) -> HashMap<String, RbValue> {
    let map = HashMap::new();
    let map_ptr = &map as *const HashMap<_, _>;
    rb_block_call(rb_hash,
                  rb_intern(CString::new("each_pair").unwrap().as_ptr()),
                  0, 0 as *const RbValue,
                  insert_rb_hash_entry_into_map, map_ptr as uintptr_t);
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

fn format_env(env: HashMap<String, RbValue>) -> String {
    let mut formatted = String::from("{");
    for (k, v) in env {
        formatted.push_str(&format!("{:?}", k));
        formatted.push_str(": ");
        let v_to_s = unsafe {
            CStr::from_ptr(rb_string_value_cstr(&rb_inspect(v)))
        }.to_str().ok().unwrap();
        formatted.push_str(v_to_s);
        formatted.push_str(", ")
    }
    formatted.push_str("}");
    formatted
}

fn endpoint(env: HashMap<String, RbValue>) -> (String, HashMap<String, String>, Vec<String>) {
    let status = String::from("200");
    let mut headers = HashMap::new();
    headers.insert(String::from("Content-Type"), String::from("text/plain"));
    let body = vec![String::from("Hello from Rust!\n"), format_env(env)];
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
