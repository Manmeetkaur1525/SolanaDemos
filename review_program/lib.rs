//  The file have the actual bussinedd logic

pub mod  state;
pub mod error;
pub mod instruction;
use borsh::BorshSerialize;
use crate::state::AccountState;
use crate::instruction::ReviewInstruction;
use crate:: error::ReviewError;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh0_10::try_from_slice_unchecked,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::IsInitialized,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use std::convert::TryInto;
/// Define the type of state stored in accounts

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {

    let instruction = ReviewInstruction::unpack(instruction_data)?;
    match instruction{
        ReviewInstruction::AddReview { title, rating, description ,
        } => add_review(program_id,accounts,title,rating,description),
        ReviewInstruction::UpdateReview { title, rating, description ,
        } => update_review(program_id,accounts,title,rating,description),
    }
    Ok(())
}


pub fn add_review(
    program_id: &Pubkey,
    accounts : &[AccountInfo],
    title: String,
    rating: u8,
    description: String,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    if !initializer.is_signer{
        return Err(ProgramError::MissingRequiredSignature);
    }
    let (pda,bump_seed) = Pubkey::find_program_address(&[initializer.key.as_ref(),title.as_bytes().as_ref()], program_id,);

    if pda != *pda_account.key {
        return Err(ProgramError::InvalidArgument);
    }
    if rating > 10 || rating < 1 {
        return Err(ReviewError::InvalidRating.into());
    }
    let account_len:usize = 1000;

    let rent =  Rent::get()?;
    let rent_lampots = rent.minimum_balance(account_len);

    //cpi

    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            pda_account.key,
            rent_lampots,
            account_len.try_into().unwrap(),
            program_id),
        &[initializer.clone(),pda_account.clone() , system_program.clone()], //any account involved in the program
        &[&[
            initializer.key.as_ref(),
            title.as_bytes().as_ref(),
            &[bump_seed],
        ]])?;


    let mut account_data = try_from_slice_unchecked::<AccountState>(&pda_account.data.borrow()).unwrap();

    if account_data.is_initialized(){
        return Err(ProgramError::AccountAlreadyInitialized);
    }
    account_data.title = title;
    account_data.rating = rating;
    account_data.description = description;
    account_data.is_initialized = true;

    //serialise the data and store it in the  blockchain
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    Ok(())
}