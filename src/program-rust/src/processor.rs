//use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    pubkey::Pubkey,
    program_pack::{IsInitialized, Pack, Sealed},
//    sysvar::{rent::Rent, Sysvar},
//    program::invoke
};

use crate::instruction::ImgOperation;

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};


//#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ImgData{
    pub is_initialized:bool,
    pub owner: String,
    pub cid: String,
    pub parent: String,
    pub child: u8,
    pub diff: u8,
    pub encrypted: u8,
    pub public: u8,
    pub editable: u8,
}

impl Sealed for ImgData {}

impl IsInitialized for ImgData {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for ImgData {
    const LEN: usize = 180;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, ImgData::LEN];
        let (
            is_initialized,
            _key_size, 
            owner, 
            _cid_size, 
            cid, 
            _parent_size, 
            parent, 
            child, 
            diff, 
            encrypted, 
            public, 
            editable
        ) = array_refs![src, 1, 4, 44, 4, 59, 4, 59, 1, 1, 1, 1, 1];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };
        Ok(ImgData {
            is_initialized,
            owner: String::from_utf8(owner.to_vec()).unwrap(),
            cid: String::from_utf8(cid.to_vec()).unwrap(),
            parent: String::from_utf8(parent.to_vec()).unwrap(),
            child: child[0],
            diff: diff[0],
            encrypted: encrypted[0],
            public: public[0],
            editable: editable[0],
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, ImgData::LEN];
        let (
            is_initialized_dst,
            key_size_dst, 
            owner_dst, 
            cid_size_dst, 
            cid_dst, 
            parent_size_dst, 
            parent_dst, 
            child_dst, 
            diff_dst, 
            encrypted_dst, 
            public_dst, 
            editable_dst
        ) = mut_array_refs![dst, 1, 4, 44, 4, 59, 4, 59, 1, 1, 1, 1, 1];
        let ImgData {
            is_initialized,
            owner, 
            cid, 
            parent, 
            child, 
            diff, 
            encrypted, 
            public, 
            editable
        } = self;
        is_initialized_dst[0] = *is_initialized as u8;
        let tmp = owner_dst.len() as u32;
        *key_size_dst = tmp.to_le_bytes();
        owner_dst.copy_from_slice(owner.as_ref());
        let tmp = cid_dst.len() as u32;
        *cid_size_dst = tmp.to_le_bytes();
        cid_dst.copy_from_slice(cid.as_ref());
        let tmp = parent_dst.len() as u32;
        *parent_size_dst = tmp.to_le_bytes();
        parent_dst.copy_from_slice(parent.as_ref());
        *child_dst = child.to_le_bytes();
        *diff_dst = diff.to_le_bytes();
        *encrypted_dst = encrypted.to_le_bytes();
        *public_dst = public.to_le_bytes();
        *editable_dst = editable.to_le_bytes();
    }
}

pub struct Processor;
impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
        let instruction = ImgOperation::unpack(instruction_data)?;


        match instruction {
            ImgOperation::UploadImg {cid, parent, child, diff, encrypted, public, editable} => {
                msg!("Instruction: UploadImg"); 
                Self::process_upload_img_data(accounts, cid, parent, child, diff, encrypted, public, editable, program_id)
            },
        }
    }

    fn process_upload_img_data(
        accounts: &[AccountInfo],
        cid: String,
        parent: String,
        child: u8,
        diff: u8,
        encrypted: u8,
        public: u8,
        editable: u8,
        program_id: &Pubkey
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let owner = next_account_info(accounts_iter)?;

        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        } 

        let img_data_account = next_account_info(accounts_iter)?;

        if img_data_account.owner != program_id {
            msg!("Image data account does not have the correct program_id");
            return Err(ProgramError::IncorrectProgramId);
        }
        //msg!("{}, {}, {}, {}, {}, {}, {}",cid,parent,child,diff,encrypted,public,editable);
        
        //msg!("{:?}", img_data);
        //let encoded_a = img_data.try_to_vec().unwrap();
        let mut img_data = ImgData::unpack_unchecked(&img_data_account.try_borrow_data()?)?;
        if img_data.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        //let mut img_data = ImgData::unpack(wiwa);
        //msg!("{:?}", img_data);
        //msg!("{:?}",encoded_a);
        //let chi = Pubkey::create_with_seed(owner.key, "image", program_id)?; 
        //let mut img_data = ImgData::try_from_slice(&img_data_account.data.borrow())?;
        img_data.is_initialized = true;
        img_data.owner = owner.key.to_string();
        img_data.cid = cid;
        img_data.parent = parent;
        img_data.child = child;
        img_data.diff = diff;
        img_data.encrypted = encrypted;
        img_data.public = public;
        img_data.editable = editable;

        msg!("chi");
        //msg!("{:}",img_data);
        msg!("Image info uploaded!\n owner {} uploaded image with cid {}",
             img_data.owner,
             img_data.cid,);
        ImgData::pack(img_data, &mut img_data_account.try_borrow_mut_data()?)?;
        msg!("chi");

        Ok(())
    }
}

