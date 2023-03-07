//! Program entrypoint

use {
    crate::{error::TokenError, processor::Processor},
    solana_program::{
        account_info::AccountInfo, entrypoint, entrypoint::ProgramResult,
        program_error::PrintProgramError, pubkey::Pubkey,
    },
    crate::{extension::confidential_transfer::processor::process_empty_account,
            extension::confidential_transfer::processor::process_withdraw,
            extension::confidential_transfer::processor::cvt_confidential},
    crate::extension::confidential_transfer::DecryptableBalance,
    cvt
};

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
    let token_account = cvt::CVT_mk_account_info();
    let instructions_sysvar_info = cvt::CVT_mk_account_info();
    let authority_info = cvt::CVT_mk_account_info();
    let acc_infos = [token_account.clone(), instructions_sysvar_info.clone(), authority_info.clone()];
    let proof_instruction_offset = cvt::CVT_nondet_i64();
    process_empty_account(program_id, &acc_infos, proof_instruction_offset).unwrap();
    Ok(())
}

fn cvt_entrypoint_process_withdraw_account(
    program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let token_account = cvt::CVT_mk_account_info();
    let mint_account = cvt::CVT_mk_account_info();
    let instructions_sysvar_info = cvt::CVT_mk_account_info();
    let authority_info = cvt::CVT_mk_account_info();
    let acc_infos = [token_account.clone(), mint_account.clone(), instructions_sysvar_info.clone(), authority_info.clone()];
    let amount = cvt::CVT_nondet_u64();
    let expected_decimals = cvt::CVT_nondet_u8();
    let new_decryptable_available_balance: DecryptableBalance = cvt_confidential::CVT_mk_decryptable_balance();
    let proof_instruction_offset = cvt::CVT_nondet_i64();
    process_withdraw(program_id, &acc_infos, amount, expected_decimals, new_decryptable_available_balance, proof_instruction_offset).unwrap();
    Ok(())
}