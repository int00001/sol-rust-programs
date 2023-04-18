use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

// export program's entrypoint
entrypoint!(process_instruction);

// entrypoint implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
    ) -> ProgramResult {
    msg!("hi");

    Ok(())
}
