//! Program entrypoint

use {
    crate::{error::TokenError, processor::Processor},
    solana_program::{
        account_info::AccountInfo, entrypoint, entrypoint::ProgramResult,
        program_error::PrintProgramError, pubkey::Pubkey,
    },
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

//entrypoint!(process_instruction);
entrypoint!(cvt_entrypoint_process_empty_account);
//entrypoint!(cvt_entrypoint_process_withdraw_account);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = Processor::process(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        error.print::<TokenError>();
        return Err(error);
    }
    Ok(())
}


fn cvt_entrypoint_process_empty_account(
    program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {

    // Create non-deterministic inputs for process_empty_account
    let token_account_info = cvt::CVT_nondet_account_info();
    let instructions_sysvar_info = cvt::CVT_nondet_account_info();
    let authority_info = cvt::CVT_nondet_account_info();
    let acc_infos = [token_account_info.clone(), instructions_sysvar_info.clone(), authority_info.clone()];
    let proof_instruction_offset = cvt::CVT_nondet_i64();

    process_empty_account(program_id, &acc_infos, proof_instruction_offset).unwrap();

    // Check the property
    let token_account_data = &acc_infos[0].data.borrow();
    let token_account = StateWithExtensions::<Account>::unpack(token_account_data).unwrap();
    let confidential_transfer_account =
        token_account.get_extension::<ConfidentialTransferAccount>().unwrap();
    cvt::CVT_assert(confidential_transfer_account.encryption_pubkey == cvt_confidential::get_proof_close_account().pubkey);

    Ok(())
}

fn cvt_entrypoint_process_withdraw_account(
    program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let token_account_info = cvt::CVT_nondet_account_info();
    let mint_account_info = cvt::CVT_nondet_account_info();
    let instructions_sysvar_info = cvt::CVT_nondet_account_info();
    let authority_info = cvt::CVT_nondet_account_info();
    let acc_infos = [token_account_info.clone(), mint_account_info.clone(), instructions_sysvar_info.clone(), authority_info.clone()];
    let amount = cvt::CVT_nondet_u64();
    let expected_decimals = cvt::CVT_nondet_u8();
    let new_decryptable_available_balance: DecryptableBalance = cvt_confidential::CVT_mk_decryptable_balance();
    let proof_instruction_offset = cvt::CVT_nondet_i64();
    process_withdraw(program_id, &acc_infos, amount, expected_decimals, new_decryptable_available_balance, proof_instruction_offset).unwrap();

    // Check the property
    let token_account_data = &acc_infos[0].data.borrow();
    let token_account = StateWithExtensions::<Account>::unpack(token_account_data).unwrap();
    let confidential_transfer_account =
        token_account.get_extension::<ConfidentialTransferAccount>().unwrap();
    cvt::CVT_assert(confidential_transfer_account.encryption_pubkey == cvt_confidential::get_proof_withdraw_account().pubkey);

    Ok(())
}