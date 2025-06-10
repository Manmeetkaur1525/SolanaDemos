
use borsh::{BorshDeserialize,BorshSerialize};
use solana_program::program_pack::{IsInitialized,Sealed};
use crate::error::ReviewError;
#[derive(BorschSerialize,BorshDeserialize)]
pub struct AccountState {
    pub is_initalized:bool,
    pub rating:u8,
    pub description: String,
    pub title:String,
}

impl Sealed for AccountState{}

impl  IsInitialized for AccountState{
    fn is_initalized(&self) -> bool {
        self.is_initalized
    }
}


