use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum ImgError {

    #[error("Invalid Instruction")]
    InvalidInstruction,
}

impl From<ImgError> for ProgramError {
    fn from(e: ImgError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
