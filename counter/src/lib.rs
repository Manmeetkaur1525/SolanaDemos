use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};
use solana_sdk::program_error::ProgramError;

use crate::instruction::CounterInstructions;

// it is stateless so instead of storing a state we make a account that stores the state

pub mod instruction;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct CounterAccount {
    pub count: u64,
}
entrypoint!(process_instruction);
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo], //all the accounts
    instructions_data: &[u8],
) -> ProgramResult {
    msg!("Counter program entry point");

    //unpack the instruction
    let instruction: CounterInstructions = CounterInstructions::unpack(instructions_data)?;

    let account_iter = &mut accounts.iter();
    let account = next_account_info(account_iter)?;

    let mut count_account = CounterAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        CounterInstructions::Increment(args) => {
            count_account.count = count_account.count.checked_add(args.value).ok_or(ProgramError::InvalidInstructionData)?;
        }
        CounterInstructions::Decrement(args) => {
            count_account.count = count_account.count.checked_sub(args.value).ok_or(ProgramError::InvalidInstructionData)?;
        }
        CounterInstructions::Reset => {
            count_account.count = 0;
        }
        CounterInstructions::Update(args) => {
            count_account.count = args.value;
        }
    }

    //to store it in the chain we need to deserialise it again

    count_account.serialize(&mut &mut account.data.borrow_mut()[..]);
    Ok({})
}

//test case writing in rust

#[cfg(test)]
mod test {
    use super::*;
    use solana_program::{clock::Epoch, pubkey};
    use solana_sdk::{account, lamports};
    use std::mem;

    #[test]
    fn test_counter() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u64>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let accounts = vec![account];
        let increment_instruction_data: Vec<u8> = vec![0];
        let decrement_instruction_data: Vec<u8> = vec![1];
        let mut update_instruction_data: Vec<u8> = vec![2];
        let reset_instruction_data: Vec<u8> = vec![3];
        process_instruction(&program_id, &accounts, &increment_instruction_data).unwrap();

        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .count,
            1
        );

        process_instruction(&program_id, &accounts, &decrement_instruction_data).unwrap();

        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .count,
            0
        );

        let update_value = 33u64;
        update_instruction_data.extend_from_slice(&update_value.to_le_bytes());

        process_instruction(&program_id, &accounts, &update_instruction_data).unwrap();

        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .count,
            33
        );

        process_instruction(&program_id, &accounts, &reset_instruction_data).unwrap();

        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .count,
            0
        )
    }
}