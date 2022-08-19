mod util;

use util::struct_to_json;
use util::json_to_struct;
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::os::raw::c_void;
use std::ffi::CStr;
use json::parse;
use json::object::Object;

#[repr(C)]
#[allow(non_camel_case_types)]
pub(crate) enum json_type {
  JSON_TYPE_NULL,
  JSON_TYPE_STRING,
  JSON_TYPE_NUMBER,
  JSON_TYPE_BOOL,
  JSON_TYPE_OBJECT,
  JSON_TYPE_ARRAY
}

#[repr(C)]
pub(crate) union json_value {
  string: *mut c_char,
  number: u64,
  bool: c_int,
  object: *mut c_void,
  array: *mut *mut json_t,
}

#[repr(C)]
pub(crate) struct json_t {
  buffer_type: json_type,
  buffer: json_value,
}


#[no_mangle]
unsafe extern "C" fn json_parse(source: *const c_char) -> *mut json_t {
  let source_rs = match CStr::from_ptr(source).to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
  };
  let json = match parse(source_rs) {
    Ok(v) => v,
    Err(_) => return std::ptr::null_mut()
  };
  json_to_struct(json)
}

#[no_mangle]
unsafe extern "C" fn json_object_get(object: *mut c_void, key: *const c_char) -> *mut json_t {
  let key_rs = match CStr::from_ptr(key).to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
  };
  let value = match (*(object as *mut Object)).get(key_rs) {
    Some(v) => v,
    None => return std::ptr::null_mut()
  };
  json_to_struct(value.to_owned())
}

/*
#[no_mangle]
unsafe extern "C" fn json_object_insert(object: *mut c_void, key: *const c_char, value: *const c_char) -> *mut json_t { ... }
*/

#[no_mangle]
unsafe extern "C" fn json_object_new() -> *mut c_void {
  Box::into_raw(Box::new(Object::new())) as *mut c_void
}

#[no_mangle]
unsafe extern "C" fn json_object_new_with_capacity(capacity: usize) -> *mut c_void {
  Box::into_raw(Box::new(Object::with_capacity(capacity))) as *mut c_void
}

#[no_mangle]
unsafe extern "C" fn json_object_is_empty(object: *mut c_void) -> c_int {
  if (*(object as *mut Object)).is_empty() {
    return 1;
  }
    0
}

#[no_mangle]
unsafe extern "C" fn json_object_len(object: *mut c_void) -> usize {
  (*(object as *mut Object)).len()
}

#[no_mangle]
unsafe extern "C" fn json_object_clear(object: *mut c_void) {
  (*(object as *mut Object)).clear()
}