use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ImgData {
    pub owner: Pubkey,
    pub cid: String,
    pub parent: String,
    pub child: bool,
    pub diff: bool,
    pub encrypted: bool,
    pub public: bool,
    pub editable: bool

