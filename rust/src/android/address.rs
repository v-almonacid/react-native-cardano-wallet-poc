use super::primitive::ToPrimitiveObject;
use super::ptr_j::*;
use super::result::ToJniResult;
use super::string::ToJniString;
use super::string::*;
use crate::panic::{handle_exception_result, ToResult, Zip};
use crate::ptr::RPtrRepresentable;
use jni::objects::{JObject, JString};
use jni::sys::{jbyteArray, jint, jobject};
use jni::JNIEnv;
use cardano_wallet::{Address};


#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Java_io_emurgo_rncardanowallet_Native_addressFromString(
  env: JNIEnv, _: JObject, string: JString
) -> jobject {
  handle_exception_result(|| {
    string
      .string(&env)
      .and_then(|rstr| Address::from_base58(&rstr).into_result())
      .and_then(|address| address.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rncardanowallet_Native_addressToString(
  env: JNIEnv, _: JObject, ptr: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let rptr = ptr.rptr(&env)?;
    rptr
      .typed_ref::<Address>()
      // .zip(prefix.string(&env))
      .and_then(|address| address.to_base58().jstring(&env))
  })
  .jresult(&env)
}
