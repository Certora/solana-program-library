use cvt;
use crate::extension::confidential_transfer::instruction::CloseAccountData;

extern "C" {
    fn CVT_nondet_pointer_close_account_data() -> *mut CloseAccountData;
}
fn mk_close_account_data() -> *mut CloseAccountData {
    unsafe {
        return CVT_nondet_pointer_close_account_data();
    }
}

static mut CVT_PROOF_CLOSE_ACCOUNT: *mut CloseAccountData = std::ptr::null_mut();

#[inline(never)]
#[allow(non_snake_case)]
pub fn decode_proof_close_account_impl() -> &'static CloseAccountData {
    unsafe {
        CVT_PROOF_CLOSE_ACCOUNT = mk_close_account_data();
        cvt::CVT_assume(!CVT_PROOF_CLOSE_ACCOUNT.is_null());
        &*CVT_PROOF_CLOSE_ACCOUNT
    }
}

#[inline(never)]
#[allow(non_snake_case)]
pub fn get_proof_close_account_impl() -> &'static CloseAccountData {
    unsafe {
        cvt::CVT_assume(!CVT_PROOF_CLOSE_ACCOUNT.is_null());
        &*CVT_PROOF_CLOSE_ACCOUNT
    }
}

