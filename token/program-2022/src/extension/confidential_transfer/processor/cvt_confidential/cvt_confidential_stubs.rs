use solana_zk_token_sdk::instruction::WithdrawData;
use cvt;
use crate::extension::confidential_transfer::instruction::CloseAccountData;
use crate::extension::confidential_transfer::DecryptableBalance;

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


extern "C" {
    fn CVT_nondet_pointer_withdraw_account_data() -> *mut WithdrawData;
}
fn mk_withdraw_account_data() -> *mut WithdrawData {
    unsafe {
        return CVT_nondet_pointer_withdraw_account_data();
    }
}

static mut CVT_PROOF_WITHDRAW_ACCOUNT: *mut WithdrawData = std::ptr::null_mut();

#[inline(never)]
#[allow(non_snake_case)]
pub fn decode_proof_withdraw_account_impl() -> &'static WithdrawData {
    unsafe {
        CVT_PROOF_WITHDRAW_ACCOUNT = mk_withdraw_account_data();
        cvt::CVT_assume(!CVT_PROOF_WITHDRAW_ACCOUNT.is_null());
        &*CVT_PROOF_WITHDRAW_ACCOUNT
    }
}

#[inline(never)]
#[allow(non_snake_case)]
pub fn get_proof_withdraw_account_impl() -> &'static WithdrawData {
    unsafe {
        cvt::CVT_assume(!CVT_PROOF_WITHDRAW_ACCOUNT.is_null());
        &*CVT_PROOF_WITHDRAW_ACCOUNT
    }
}


extern "C" {
    fn CVT_nondet_pointer_decryptable_balance() -> *mut DecryptableBalance;
}

#[inline(never)]
#[allow(non_snake_case)]
pub fn CVT_mk_decryptable_balance_impl() -> DecryptableBalance {
    unsafe {
        return *CVT_nondet_pointer_decryptable_balance();
    }
}