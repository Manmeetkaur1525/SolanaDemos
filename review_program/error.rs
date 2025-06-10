use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Debug,Error)]

pub enum ReviewError {
    #[error("Account not initialised")]
    UnitionalizedAccount,
    #[error("Rating less than 1 or greater then 10")]
    InvalidRating,
    #[error("PDA error")]
    InvalidPDA,
}

//wrapping the custom erroe to the actual program error of solana
impl FromReviewError for ProgramError {
    fn from(e:ReviewError) -> Self {
        ProgramError::Custom(e as u32)
    }
}