use std::os::raw::c_char;
use std::os::raw::c_int;
use std::os::raw::c_void;
use std::ffi::CStr;
use std::ffi::CString;
use json::parse;
use json::JsonValue;
use json::object::Object;

#[repr(C)]
#[allow(non_camel_case_types)]
enum json_type {
  JSON_TYPE_NULL,
  JSON_TYPE_STRING,
  JSON_TYPE_NUMBER,
  JSON_TYPE_BOOL,
  JSON_TYPE_OBJECT,
  JSON_TYPE_ARRAY
}

#[repr(C)]
union json_value {
  string: *mut c_char,
  number: u64,
  bool: c_int,
  object: *mut c_void,
  array: *mut *mut json_t,
}

#[repr(C)]
struct json_t {
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

unsafe fn json_to_struct(json: JsonValue) -> *mut json_t {
  match json {
    JsonValue::Null => {
      let data = Box::new(json_t{
        buffer_type: json_type::JSON_TYPE_NULL,
        buffer: json_value { string: std::mem::zeroed() },
      });
      Box::into_raw(data)
    },
    JsonValue::Short(s) => {
      let data = Box::new(json_t{
        buffer_type: json_type::JSON_TYPE_STRING,
        buffer: json_value { string: match CString::new(s.as_str()) {
          Ok(v) => v.into_raw(),
          Err(_) => std::ptr::null_mut()
        } },
      });
      Box::into_raw(data)
    },
    JsonValue::String(s) => {
      let data = Box::new(json_t{
        buffer_type: json_type::JSON_TYPE_STRING,
        buffer: json_value { string: match CString::new(s) {
          Ok(v) => v.into_raw(),
          Err(_) => std::ptr::null_mut()
        } },
      });
      Box::into_raw(data)
    },
    JsonValue::Number(_n) => {
      let data = Box::new(json_t{
        buffer_type: json_type::JSON_TYPE_NUMBER,
        buffer: json_value { number: 0 }, // TODO
      });
      Box::into_raw(data)
    },
    JsonValue::Boolean(b) => {
      let data = Box::new(json_t{
        buffer_type: json_type::JSON_TYPE_BOOL,
        buffer: json_value { bool: if b { 1 } else { 0 } },
      });
      Box::into_raw(data)
    },
    JsonValue::Object(o) => {
      let data = Box::new(json_t{
        buffer_type: json_type::JSON_TYPE_OBJECT,
        buffer: json_value { object: Box::into_raw(Box::new(o)) as *mut c_void  },
      });
      Box::into_raw(data)
    },
    JsonValue::Array(v) => {
      let mut vec = vec![];
      for i in v {
        vec.push(json_to_struct(i));
      }
      let data = Box::new(json_t{
        buffer_type: json_type::JSON_TYPE_ARRAY,
        buffer: json_value { array: vec.as_mut_ptr()  },
      });
      std::mem::forget(vec);
      Box::into_raw(data)
    }
  }
}