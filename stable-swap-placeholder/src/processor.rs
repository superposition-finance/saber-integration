// instructions on 
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    pubkey::Pubkey,
    program_pack::{Pack, IsInitialized},
};

use stable_swap_client::state::SwapInfo;

pub struct Processor;
impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
        // add an instruction check here, right now we just want to process the account info
        Self::process_swap_account(accounts, program_id)
    }


    fn process_swap_account(
        accounts: &[AccountInfo],
        program_id: &Pubkey
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let initializer = next_account_info(account_info_iter)?;

        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let stable_swap_account = next_account_info(account_info_iter)?;
        let swap_info = SwapInfo::unpack(&stable_swap_account.data.borrow())?;

        msg!("Logging swap information");
        
        msg!("Is Initialized: {}", swap_info.is_initialized);
        msg!("Is Paused: {}", swap_info.is_paused);
        msg!("Nonce: {}", swap_info.nonce);
        
        msg!("Initial Amp Factor: {}", swap_info.initial_amp_factor);
        msg!("Target Amp Factor: {}", swap_info.target_amp_factor);
        msg!("Start Ramp ts: {}", swap_info.start_ramp_ts);
        msg!("Stop Ramp ts: {}", swap_info.stop_ramp_ts);

        msg!("Future Admin Deadline: {}", swap_info.future_admin_deadline);
        msg!("Future Admin Key: {}", swap_info.future_admin_key);
        msg!("Admin Key: {}", swap_info.admin_key);

        msg!("Token A: {:?}", swap_info.token_a);
        msg!("Token B: {:?}", swap_info.token_b);

        msg!("Pool Mint: {}", swap_info.pool_mint);
        msg!("Fees: {:?}", swap_info.fees);

        Ok(())
    }
}