use crate::json_t;
use crate::json_type;
use crate::json_value;
use std::os::raw::c_void;
use std::ffi::CStr;
use std::ffi::CString;
use std::slice;
use json::JsonValue;
use json::short::Short;

pub(crate) unsafe fn struct_to_json(struct_json: *mut json_t) -> JsonValue {
    match (*struct_json).buffer_type {
      json_type::JSON_TYPE_NULL => {
        JsonValue::Null
      },
      json_type::JSON_TYPE_STRING => {
        let str_rs = match CStr::from_ptr((*struct_json).buffer.string).to_str() {
          Ok(v) => v,
          Err(_) => ""
        };
        if str_rs.len() > 30 {
          return JsonValue::Short(Short::from_slice(str_rs))
        }
        JsonValue::String(str_rs.to_string())
      },
      json_type::JSON_TYPE_NUMBER => {JsonValue::Boolean(true)}, // TODO
      json_type::JSON_TYPE_BOOL => {
        if (*struct_json).buffer.bool == 0 {
          return JsonValue::Boolean(false)
        }
        JsonValue::Boolean(true)
      },
      json_type::JSON_TYPE_OBJECT => {JsonValue::Boolean(true)}, // TODO
      json_type::JSON_TYPE_ARRAY => {
        let mut vec = vec![];
        let slice = slice::from_raw_parts((*struct_json).buffer.array, 0); // TODO: Add length for the array in 'json_t'
        for i in slice {
          vec.push(struct_to_json(*i));
        }
        JsonValue::Array(vec)
      }
    }
  }
  
  pub(crate) unsafe fn json_to_struct(json: JsonValue) -> *mut json_t {
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