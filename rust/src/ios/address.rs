use super::data::DataPtr;
use super::result::CResult;
use super::string::{CharPtr, IntoCString, IntoStr};
// use crate::address::{AddressDiscrimination, AddressKind};
use crate::panic::{handle_exception_result, ToResult, Zip};
use crate::ptr::{RPtr, RPtrRepresentable};
use cardano_wallet::Address;


#[no_mangle]
pub unsafe extern "C" fn address_from_base58(
  string: CharPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    Address::from_base58(string.into_str()).map(|addr| addr.rptr()).into_result()
  })
  .response(result, error)
}
