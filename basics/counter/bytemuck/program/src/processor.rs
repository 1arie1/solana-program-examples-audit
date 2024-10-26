use solana_program::{
    account_info::{next_account_info, AccountInfo},
    program_error::ProgramError,
};
use crate::state::Counter;

#[cfg(not(feature = "certora"))]
use solana_program::msg;

pub fn process_increment_counter(
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> Result<(), ProgramError> {
    let account_info_iter = &mut accounts.iter();

    let counter_info = next_account_info(account_info_iter)?;
    let authority_info = next_account_info(account_info_iter)?;
    #[cfg(not(feature = "certora"))]
    assert!(
        counter_account.is_writable,
        "Counter account must be writable"
    );

    if counter_info.owner != &crate::id() {
        return Err(ProgramError::IllegalOwner);
    }

    if !authority_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut data = counter_info.data.borrow_mut();
    let counter: &mut Counter = bytemuck::from_bytes_mut(&mut data[..]);

    if authority_info.key != &counter.owner {
        return Err(ProgramError::Custom(101));
    }

    let mut count: u64 = counter.count.into();
    count += 1;
    counter.count = count.into();

    #[cfg(not(feature = "certora"))]
    msg!("Counter state incremented to {:?}", counter.count);
    Ok(())
}
