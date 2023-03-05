mod cvt_confidential_stubs;
use crate::extension::confidential_transfer::instruction::CloseAccountData;

#[inline(never)]
#[allow(non_snake_case)]
/// Make a new non-deterministic CloseAccountData object, and return it
pub fn decode_proof_close_account() -> &'static CloseAccountData {
    cvt_confidential_stubs::decode_proof_close_account_impl()
}

#[inline(never)]
#[allow(non_snake_case)]
/// Return the CloseAccountData object created by the last call to decode_proof_close_account.
pub fn get_proof_close_account() -> &'static CloseAccountData {
    cvt_confidential_stubs::get_proof_close_account_impl()
}
