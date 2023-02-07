//! Certora specs

use {
    crate::{processor::Processor},
    solana_program::{account_info::AccountInfo},
    crate::{extension::confidential_transfer::processor::process_empty_account,
            extension::confidential_transfer::processor::process_withdraw,
            extension::confidential_transfer::ConfidentialTransferAccount,
            extension::confidential_transfer::processor::cvt_confidential},
    crate::extension::confidential_transfer::DecryptableBalance,
    crate::{extension::StateWithExtensions},
    crate::state::{Account},
    cvt
};
use crate::extension::BaseStateWithExtensions;
use crate::extension::confidential_transfer::processor::cvt_confidential::NondetDecryptableBalance;
use crate::pod::EncryptionPubkey;

/** Rules
- [proper_use_of_encryption_key_process_empty_account] expected true
- [proper_use_of_encryption_key_process_withdraw_account] expected true
- [integrity_of_process_mint_to] expected true
- [integrity_of_process_mint_to_false] expected false
 **/

fn get_account_amount(account_info: &AccountInfo) -> u64 {
    Processor::get_account_amount(account_info).unwrap()
}

fn get_mint_supply(mint_info: &AccountInfo) -> u64 {
    Processor::get_mint_supply(mint_info).unwrap()
}

fn get_encryption_key_from_confidential_extension(token_account_info: &AccountInfo) -> EncryptionPubkey {
    let token_account_data = &token_account_info.data.borrow();
    let token_account = StateWithExtensions::<Account>::unpack(token_account_data).unwrap();
    let confidential_transfer_account =
        token_account.get_extension::<ConfidentialTransferAccount>().unwrap();
    confidential_transfer_account.encryption_pubkey
}

#[no_mangle]
/// Rule to prove that the encryption key is used correctly during process_empty_account
pub fn proper_use_of_encryption_key_process_empty_account() {
    // Create environment for process_empty_account
    let program_id = cvt::nondet();
    let acc_infos = [cvt::nondet::<AccountInfo>(), cvt::nondet::<AccountInfo>(), cvt::nondet::<AccountInfo>()];

    process_empty_account(&program_id, &acc_infos, cvt::nondet()).unwrap();

    // Check the property
    cvt::CVT_assert(get_encryption_key_from_confidential_extension(&acc_infos[0]) == cvt_confidential::get_proof_close_account().pubkey);
}

#[no_mangle]
/// Rule to prove that the encryption key is used correctly during process_withdraw_account
pub fn proper_use_of_encryption_key_process_withdraw_account() {
    // Create environment for process_withdraw
    let program_id = cvt::nondet();
    let acc_infos = [cvt::nondet::<AccountInfo>(), cvt::nondet::<AccountInfo>(), cvt::nondet::<AccountInfo>(),cvt::nondet::<AccountInfo>()];
    let new_decryptable_available_balance: NondetDecryptableBalance = cvt::nondet();

    process_withdraw(&program_id, &acc_infos, cvt::nondet(), cvt::nondet() ,
                     new_decryptable_available_balance.into(), cvt::nondet()).unwrap();

    // Check the property
    cvt::CVT_assert(get_encryption_key_from_confidential_extension(&acc_infos[0]) == cvt_confidential::get_proof_withdraw_account().pubkey);
}

#[no_mangle]
/// Rule to check functional correctness of process_mint_to
pub fn integrity_of_process_mint_to() {
    // Create environment for process_mint_to
    let program_id = cvt::nondet();
    let acc_infos = [cvt::nondet::<AccountInfo>(), cvt::nondet::<AccountInfo>(), cvt::nondet::<AccountInfo>()];
    let amount = cvt::nondet();
    cvt::CVT_assume(amount > 0 );
    let mint_info = &acc_infos[0];
    let destination_info = &acc_infos[1];
    let destination_amount_before = get_account_amount(destination_info);
    let supply_amount_before = get_mint_supply(mint_info);

    Processor::process_mint_to(&program_id, &acc_infos, amount, None).unwrap();

    // Check properties
    let destination_amount_after = get_account_amount(destination_info);
    let supply_amount_after = get_mint_supply(mint_info);

    cvt::CVT_assert(destination_amount_after == destination_amount_before + amount);
    cvt::CVT_assert(destination_amount_after > destination_amount_before);
    cvt::CVT_assert(supply_amount_after == supply_amount_before + amount);
    cvt::CVT_assert(supply_amount_after > supply_amount_before);
}

#[no_mangle]
/// Buggy spec
pub fn integrity_of_process_mint_to_false() {
    // Create environment for process_mint_to
    let program_id = cvt::nondet();
    let acc_infos = [cvt::nondet::<AccountInfo>(), cvt::nondet::<AccountInfo>(), cvt::nondet::<AccountInfo>()];
    let amount = cvt::nondet();
    let mint_info = &acc_infos[0];
    let destination_info = &acc_infos[1];
    let destination_amount_before = get_account_amount(destination_info);
    let supply_amount_before = get_mint_supply(mint_info);

    Processor::process_mint_to(&program_id, &acc_infos, amount, None).unwrap();

    // Check properties
    let destination_amount_after = get_account_amount(destination_info);
    let supply_amount_after = get_mint_supply(mint_info);

    cvt::CVT_assert(destination_amount_after == destination_amount_before + amount);
    cvt::CVT_assert(destination_amount_after > destination_amount_before);
    cvt::CVT_assert(supply_amount_after == supply_amount_before + amount);
    cvt::CVT_assert(supply_amount_after > supply_amount_before);
}

