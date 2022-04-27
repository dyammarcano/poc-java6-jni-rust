use jni::objects::*;
use jni::sys::*;
use jni::JNIEnv;
use std::ffi::CStr;

/**
 * Convert a Java String to a Rust String.
 *
 * # Arguments
 *
 * * `env` - The JNIEnv pointer.
 * * `jstr` - The Java String to convert.
 *
 * # Returns
 *
 * The Rust String.
 */
pub fn jstring_to_string(env: &JNIEnv, jstr: jstring) -> String {
    let c_str = env
        .get_string(JString::from(jstr))
        .expect("Couldn't get java string!");
    let c_str = c_str.as_ptr();
    let c_str = unsafe { CStr::from_ptr(c_str) };
    c_str.to_str().unwrap().to_owned()
}

/*pub fn jstring_to_string_2(env: &JNIEnv, jstr: JString) -> String {
    let c_str = env.get_string(jstr).expect("Couldn't get java string!");
    let c_str = c_str.as_ptr();
    let c_str = unsafe { CStr::from_ptr(c_str) };
    c_str.to_str().unwrap().to_owned()
}*/

// vec<u8> -> jbyteArray
pub fn vec_to_jbyte_array(env: &JNIEnv, vec: Vec<u8>) -> jbyteArray {
    env.byte_array_from_slice(&vec).unwrap()
}

// jbyteArray -> vec<u8>
pub fn jbyte_array_to_vec(env: &JNIEnv, arr: jbyteArray) -> Vec<u8> {
    let vec = env.convert_byte_array(arr).unwrap();
    vec
}

// jstring -> String
// fn jstring_to_string(env: &JNIEnv, jstr: JString) -> String {
//     let c_str = env.get_string(jstr).unwrap();
//     let c_str = CString::new(c_str).unwrap();
//     let str = c_str.into_string().unwrap();
//     str
// }
// String -> jstring
// fn string_to_jstring(env: &JNIEnv, str: String) -> jstring {
//     let c_str = CString::new(str).unwrap();
//     let c_str = c_str.as_ptr();
//     let c_str = unsafe { CStr::from_ptr(c_str) };
//     let jstr = env.new_string(c_str).unwrap();
//     jstr.into_inner()
// }
//
// // jsobject to jstring
// fn jobject_to_jstring(env: &JNIEnv, jstr: jobject) -> jstring {
//     let jstr = env.get_object_ref(jstr).unwrap();
//     let jstr = env.get_string(jstr).unwrap();
//     jstr.into_inner()
// }
