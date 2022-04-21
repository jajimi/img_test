
use solana_program::program_error::ProgramError;

use crate::error::ImgError::InvalidInstruction;


pub enum ImgOperation {
    UploadImg {
        cid: String,
        parent: String,
        child: u8,
        diff: u8,
        //encrypted: u8,
        public: u8,
        editable: u8,
    },
    GivePerms {
        cid: String,
        public: u8,
        editable: u8,
    },
    DwnldImg{
        cid: String,
    }
}

impl ImgOperation {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (instruction, info) = input.split_first().ok_or(InvalidInstruction)?;
        Ok(match instruction {
            0 => Self::UploadImg {
                cid: String::from_utf8((info[0..59]).to_vec()).unwrap(),
                parent: String::from_utf8((info[59..118]).to_vec()).unwrap(),
                child: info[118],
                diff: info[119],
                //encrypted: info[120],
                public: info[120],
                editable: info[121],
            },
            1 => Self::GivePerms {
                cid: String::from_utf8((info[0..59]).to_vec()).unwrap(),
                public: info[59], 
                editable: info[60], 
            },
            2 => Self::DwnldImg {
                cid: String::from_utf8((info[..59]).to_vec()).unwrap(),
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }
}
