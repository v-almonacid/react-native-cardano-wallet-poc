mod address;
mod primitive;
mod ptr_j;
mod result;
mod string;

pub use address::*;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Java_io_emurgo_chainlibs_Native_initLibrary(
  _env: jni::JNIEnv, _: jni::objects::JObject
) {
  crate::panic::hide_exceptions();
}
