#![allow(improper_ctypes)]
#![feature(stmt_expr_attributes)]

extern crate bytes;
extern crate core;
extern crate curl;
extern crate futures;
#[macro_use]
extern crate itertools;
extern crate jni;
extern crate jni_sys;
#[macro_use]
extern crate lazy_static;
extern crate libc;
#[macro_use]
extern crate ndk_log;
extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;
extern crate tokio_threadpool;
extern crate tokio_timer;
#[macro_use]
extern crate v8;
extern crate v8_macros;

use std::mem;

use jni::JNIEnv;
use libc::{c_char, size_t};
use v8::fun::CallbackInfo;
use v8::types::*;
use v8_macros::v8_fn;

#[macro_use]
mod macros;
#[macro_use]
mod ndk_graphics;
#[macro_use]
mod dex;
#[macro_use]
mod runtime;
mod buffer;
mod ndk_util;

#[no_mangle]
pub unsafe extern "C" fn get_android_version(env: &JNIEnv) -> i32 {
    // Android Version
    env.get_static_field("android/os/Build$VERSION", "SDK_INT", "I")
        .unwrap()
        .i()
        .unwrap() as i32
}

type Buf = *mut u8;

#[no_mangle]
pub extern "C" fn worker_send_bytes(
    _buf: Buf,
    _len: size_t,
    _callback: Handle<JsFunction>,
) -> *const c_char {
    let _contents: *mut u8;
    unsafe {
        _callback.call::<JsNumber, _, _>(vec![v8::new_array_buffer(&"💖".as_bytes())]);

        let info = js_object!(
            "name" => "Vampire",
            "gender" => "Male",
            "age" => 28,
            "favorites" => vec![
                "Book",
                "Programming",
                "Traveling"
            ]
        );

        let result = _callback.call::<JsObject, _, _>(vec![info]);
        adb_debug!(result);

        c_str!("💖") as *const i8
    }
}

#[v8_fn]
pub fn test_fn(args: &CallbackInfo) {
    args.set_return_value(JsArrayBuffer::new(&"💖".as_bytes()));
}

#[allow(dead_code)]
fn main() {}
