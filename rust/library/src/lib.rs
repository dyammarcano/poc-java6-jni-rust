mod core;
mod tests;

use crate::core::encoding::*;
use crate::core::helper::*;
use jni::objects::*;
use jni::sys::*;
use jni::JNIEnv;

/**
 * Class:     org_example_exports_MyClass
 * Method:    httpGet
 * Signature: (Ljava/lang/String;)Ljava/lang/String;
 *
 * JNIEXPORT jstring JNICALL Java_com_rustlang_jni_HttpsClientReqwest_httpGet(JNIEnv *, jobject, jstring);
 */

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_rustlang_jni_HttpsClientReqwest_httpGet(
    env: JNIEnv,
    _: JClass,
    url: jstring,
) -> jstring {
    let url = jstring_to_string(&env, url);
    let response: Result<String, reqwest::Error> = reqwest::blocking::get(url).unwrap().text();
    let response = response.unwrap();
    let response = env.new_string(response).unwrap();
    response.into_inner()
}

/*
 * Class:     org_example_exports_MyClass
 * Method:    httpPost
 * Signature: (Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
 *
 * JNIEXPORT jstring JNICALL Java_com_rustlang_jni_HttpsClientReqwest_httpPost(JNIEnv *, jobject, jstring, jstring);
 */

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_rustlang_jni_HttpsClientReqwest_httpPost(
    env: JNIEnv,
    _: JClass,
    url: jstring,
    body: jstring,
) -> jstring {
    let url = jstring_to_string(&env, url);
    let body = jstring_to_string(&env, body);
    let body = reqwest::blocking::Body::from(body);
    let url = reqwest::Url::parse(&url).unwrap();
    let response = reqwest::blocking::Client::new()
        .post(url)
        .body(body)
        .send()
        .unwrap()
        .text();
    let response = response.unwrap();
    let response = env.new_string(response).unwrap();
    response.into_inner()
}

/**
 * Class:     rust_encoding_Base64Native
 * Method:    encode
 * Signature: ([B)Ljava/lang/String;
 * JNIEXPORT jstring JNICALL Java_rust_encoding_Base64Native_encode(JNIEnv *, jclass, jbyteArray);
 */

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_rust_encoding_Base64Native_encode(
    env: JNIEnv,
    _: JClass,
    jbytes: jbyteArray,
) -> jstring {
    let bytes = jbyte_array_to_vec(&env, jbytes);
    let str = base64::base64_encode(&bytes);
    let jstr = env.new_string(str).unwrap();
    jstr.into_inner()
}

/**
 * Class:     rust_encoding_Base64Native
 * Method:    decode
 * Signature: (Ljava/lang/String;)[B
 * JNIEXPORT jbyteArray JNICALL Java_rust_encoding_Base64Native_decode(JNIEnv *, jclass, jstring);
 */

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_rust_encoding_Base64Native_decode(
    env: JNIEnv,
    _: JClass,
    jstr: jstring,
) -> jbyteArray {
    let str = jstring_to_string(&env, jstr);
    let bytes = base64::base64_decode(&str);
    vec_to_jbyte_array(&env, bytes)
}

/**
 * Class:     rust_encoding_Base58Native
 * Method:    encode
 * Signature: ([B)Ljava/lang/String;
 * JNIEXPORT jstring JNICALL Java_rust_encoding_Base58Native_encode(JNIEnv *, jclass, jbyteArray);
 */

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_rust_encoding_Base58Native_encode(
    env: JNIEnv,
    _: JClass,
    jbytes: jbyteArray,
) -> jstring {
    let bytes = jbyte_array_to_vec(&env, jbytes);
    let str = base58::base58_encode(&bytes);
    let jstr = env.new_string(str).unwrap();
    jstr.into_inner()
}

/**
 * Class:     rust_encoding_Base58Native
 * Method:    decode
 * Signature: (Ljava/lang/String;)[B
 * JNIEXPORT jbyteArray JNICALL Java_rust_encoding_Base58Native_decode(JNIEnv *, jclass, jstring);
 */

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_rust_encoding_Base58Native_decode(
    env: JNIEnv,
    _: JClass,
    jstr: jstring,
) -> jbyteArray {
    let str = jstring_to_string(&env, jstr);
    let bytes = base58::base58_decode(str.as_str());
    vec_to_jbyte_array(&env, bytes)
}
