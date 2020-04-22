use super::data::DataPtr;
use super::result::CResult;
use super::string::{CharPtr, IntoCString, IntoStr};
use crate::panic::{handle_exception_result, ToResult, Zip};
use crate::ptr::{RPtr, RPtrRepresentable};
use cardano_wallet::Address;


#[no_mangle]
pub unsafe extern "C" fn address_from_string(
  string: CharPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    Address::from_base58(string.into_str()).map(|addr| addr.rptr()).into_result()
  })
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn address_to_string(
  ptr: RPtr, result: &mut CharPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    ptr.typed_ref::<Address>().map(|addr| addr.to_base58().into_cstr())
  })
  .response(result, error)
}
