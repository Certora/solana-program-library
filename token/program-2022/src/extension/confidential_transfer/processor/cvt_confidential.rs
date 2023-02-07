mod cvt_confidential_stubs;
use crate::extension::confidential_transfer::instruction::CloseAccountData;
use crate::extension::confidential_transfer::instruction::WithdrawData;
use crate::extension::confidential_transfer::DecryptableBalance;

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


#[inline(never)]
#[allow(non_snake_case)]
/// Make a new non-deterministic WithdrawData object, and return it
pub fn decode_proof_withdraw_account() -> &'static WithdrawData {
    cvt_confidential_stubs::decode_proof_withdraw_account_impl()
}

#[inline(never)]
#[allow(non_snake_case)]
/// Return the WithdrawData object created by the last call to decode_proof_withdraw_account.
pub fn get_proof_withdraw_account() -> &'static WithdrawData {
    cvt_confidential_stubs::get_proof_withdraw_account_impl()
}

#[inline(never)]
#[allow(non_snake_case)]
/// Return a fresh non-deterministic DecryptableBalance object
pub fn CVT_mk_decryptable_balance() -> DecryptableBalance {
    cvt_confidential_stubs::CVT_mk_decryptable_balance_impl()
}

/// Wrapper needed to define Nondet traits
pub struct NondetDecryptableBalance(pub(crate) DecryptableBalance);

impl Into<DecryptableBalance> for NondetDecryptableBalance  {
    // Required method
    fn into(self) -> DecryptableBalance {
        self.0
    }
}

// To avoid E0117: only traits defined in the current crate can be implemented for types
// defined outside of the crate
impl cvt::Nondet for NondetDecryptableBalance {
    fn nondet() -> Self {
        NondetDecryptableBalance{ 0: CVT_mk_decryptable_balance()}
    }
}
