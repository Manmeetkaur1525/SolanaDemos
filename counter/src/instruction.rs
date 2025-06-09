use borsh::BorshDeserialize;
use borsh_derive::BorshSerialize;
use solana_program::program_error::ProgramError;

#[derive(Debug, BorshDeserialize, BorshSerialize)]

pub struct AmountArgs {
    pub value: u64,
}

pub enum CounterInstructions {
    Increment(AmountArgs),
    Decrement(AmountArgs),
    Update(AmountArgs),
    Reset,
}

//encode this statements

impl CounterInstructions {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            0 => Self::Increment(AmountArgs::try_from_slice(rest).unwrap()),
            1 => Self::Decrement(AmountArgs::try_from_slice(rest).unwrap()),
            2 => Self::Update(AmountArgs::try_from_slice(rest).unwrap()),
            3 => Self::Reset,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}