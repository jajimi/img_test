use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    pubkey::Pubkey,
    program_pack::{Pack, Sealed},
    sysvar::{rent::Rent, Sysvar},
    program::invoke
};

use std::{cell::Ref, borrow::BorrowMut};

use crate::instruction::ImgOperation;

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};


#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ImgData{
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

impl Pack for ImgData {
    const LEN: usize = 179;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, ImgData::LEN];
        let (
            ctrl_1, 
            owner, 
            ctrl_2, 
            cid, 
            ctrl_3, 
            parent, 
            child, 
            diff, 
            encrypted, 
            public, 
            editable
        ) = array_refs![src, 4, 44, 4, 59, 4, 59, 1, 1, 1, 1, 1];
        Ok(ImgData {
            owner: String::from_utf8(owner.to_vec()).unwrap(),
            cid: String::from_utf8(cid.to_vec()).unwrap(),
            parent: String::from_utf8(cid.to_vec()).unwrap(),
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
            ctrl_1_dst, 
            owner_dst, 
            ctrl_2_dst, 
            cid_dst, 
            ctrl_3_dst, 
            parent_dst, 
            child_dst, 
            diff_dst, 
            encrypted_dst, 
            public_dst, 
            editable_dst
        ) = mut_array_refs![dst, 4, 44, 4, 59, 4, 59, 1, 1, 1, 1, 1];
        let ImgData { 
            owner, 
            cid, 
            parent, 
            child, 
            diff, 
            encrypted, 
            public, 
            editable 
        } = self;
        ctrl_1_dst = array_mut_ref![[44, 0, 0, 0], 0, 4];
        owner_dst = array_mut_ref![owner.as_bytes(), 0, 44];
        ctrl_2_dst = array_mut_ref![[59, 0, 0, 0], 0, 4];
        cid_dst = array_mut_ref![cid.as_bytes(), 0, 59];
        ctrl_3_dst = array_mut_ref![[59, 0, 0, 0], 0, 4];
        parent_dst = array_mut_ref![parent.as_bytes(), 0, 59];
        child_dst = array_mut_ref![[*child], 0, 1];
        diff_dst = array_mut_ref![[*diff], 0, 1];
        encrypted_dst = array_mut_ref![[*encrypted], 0, 1];
        public_dst = array_mut_ref![[*public], 0, 1];
        editable_dst = array_mut_ref![[*editable], 0, 1];
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
        let img_data = ImgData {
            owner: owner.key.to_string(),
            cid,
            parent,
            child,
            diff,
            encrypted,
            public,
            editable,
        };
        //msg!("{:?}", img_data);
        //let encoded_a = img_data.try_to_vec().unwrap();
        let wiwa = (&img_data_account.data.borrow()).as_ref();
        //let mut img_data = ImgData::unpack(wiwa);
        msg!("{:?}", img_data);
        //msg!("{:?}",encoded_a);
        //let chi = Pubkey::create_with_seed(owner.key, "image", program_id)?; 
        //let mut img_data = ImgData::try_from_slice(&img_data_account.data.borrow())?;
        img_data.owner = owner.key.to_string();
        img_data.cid = cid;
        img_data.parent = parent;
        img_data.child = child;
        img_data.diff = diff;
        img_data.encrypted = encrypted;
        img_data.public = public;
        img_data.editable = editable;

        msg!("chi");
        //msg!("{:?}",img_data);
        img_data.serialize(&mut &mut img_data_account.data.borrow_mut()[..])?;
        //msg!(
        //    "Image info uploaded!\n owner {} uploaded image with cid {} ", 
        //    img_data.owner, 
        //    img_data.cid);

        Ok(())
    }
}

