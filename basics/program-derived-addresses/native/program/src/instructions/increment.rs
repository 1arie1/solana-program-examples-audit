use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use crate::state::PageVisits;

pub fn increment_page_visits(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let page_visits_info = next_account_info(accounts_iter)?;
    let user_info = next_account_info(accounts_iter)?;

    let page_visits = &mut PageVisits::try_from_slice(&page_visits_info.data.borrow())?;

    // -- compute expected PDA
    let expected_pk = Pubkey::create_program_address(
        &[
            PageVisits::SEED_PREFIX.as_bytes(),
            user_info.key.as_ref(),
            &[page_visits.bump],
        ],
        program_id,
    )?;

    // -- validate PDA
    if page_visits_info.key != &expected_pk {
        return Err(solana_program::program_error::ProgramError::Custom(101));
    }

    page_visits.increment();
    page_visits.serialize(&mut &mut page_visits_info.data.borrow_mut()[..])?;
    Ok(())
}
