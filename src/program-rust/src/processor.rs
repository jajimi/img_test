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
    pub is_initialized: bool,
    pub owner: String,
    pub cid: String,
    pub parent: String,
    pub child: u8,
    pub diff: u8,
    //pub encrypted: u8,
    pub public: u8,
    pub editable: u8,
    pub views: u32,
}

pub struct DwnldLog {
    pub is_initialized: bool,
    pub downloader: String,
    pub cid: String,
}

impl Sealed for ImgData {}

impl IsInitialized for ImgData {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for ImgData {
    const LEN: usize = 183;
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
            //encrypted, 
            public, 
            editable, 
            views,
        ) = array_refs![src, 1, 4, 44, 4, 59, 4, 59, 1, 1, 1, 1, 4];
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
            //encrypted: encrypted[0],
            public: public[0],
            editable: editable[0],
            views: u32::from_le_bytes(*views),
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
            //encrypted_dst, 
            public_dst, 
            editable_dst,
            views_dst
        ) = mut_array_refs![dst, 1, 4, 44, 4, 59, 4, 59, 1, 1, 1, 1, 4];
        let ImgData {
            is_initialized,
            owner, 
            cid, 
            parent, 
            child, 
            diff, 
            //encrypted, 
            public, 
            editable,
            views
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
        //*encrypted_dst = encrypted.to_le_bytes();
        *public_dst = public.to_le_bytes();
        *editable_dst = editable.to_le_bytes();
        *views_dst = views.to_le_bytes();
    }
}

impl Sealed for DwnldLog {}

impl IsInitialized for DwnldLog {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for DwnldLog {
    const LEN: usize = 112;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, DwnldLog::LEN];
        let(
            is_initialized,
            _key_size,
            downloader,
            _cid_size,
            cid,
        ) = array_refs![src, 1, 4, 44, 4, 59];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };
        Ok(DwnldLog {
            is_initialized,
            downloader: String::from_utf8(downloader.to_vec()).unwrap(),
            cid : String::from_utf8(cid.to_vec()).unwrap(),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, DwnldLog::LEN];
        let(
            is_initialized_dst,
            key_size_dst,
            downloader_dst,
            cid_size_dst,
            cid_dst,
        ) = mut_array_refs![dst, 1, 4, 44, 4, 59];
        let DwnldLog { 
            is_initialized,
            downloader, 
            cid 
        } = self;
        is_initialized_dst[0] = *is_initialized as u8;
        msg!("D: D: D: 1");
        let tmp = downloader_dst.len() as u32;
        msg!("D: D: D: 2");
        *key_size_dst = tmp.to_le_bytes();
        msg!("D: D: D: 3");
        downloader_dst.copy_from_slice(downloader.as_ref());
        msg!("D: D: D: 4");
        let tmp = cid_dst.len() as u32;
        msg!("D: D: D: 5");
        *cid_size_dst = tmp.to_le_bytes();
        msg!("D: D: D: 6");
        msg!("{:?}", cid_dst.len());
        msg!("{:?}",cid);
        msg!("{:?}",cid.len());
        cid_dst.copy_from_slice(cid.as_ref());
        msg!("D: D: D: 7");
    }
}


pub struct Processor;
impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
        let instruction = ImgOperation::unpack(instruction_data)?;


        match instruction {
            //ImgOperation::UploadImg {cid, parent, child, diff, encrypted, public, editable} => {
            ImgOperation::UploadImg { cid,  parent, child, diff, public, editable } => {
                msg!("Instruction: Upload Image");
                Self::process_upload_img_data(accounts, cid, parent, child, diff, public, editable, program_id)
                //Self::process_upload_img_data(accounts, cid, parent, child, diff, encrypted, public, editable, program_id)
            },
            ImgOperation::GivePerms {cid, public, editable} => {
                msg!("Instruction: Give Permissions");
                Self::process_give_permissions(accounts, cid, public, editable, program_id)
            },
            ImgOperation::DwnldImg { cid } => {
                msg!("Instruction: Download Image");
                Self::process_download_image(accounts, cid, program_id)
            }
        }
    }

    fn process_upload_img_data(
        accounts: &[AccountInfo],
        cid: String,
        parent: String,
        child: u8,
        diff: u8,
        //encrypted: u8,
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

        let mut img_data = ImgData::unpack_unchecked(&img_data_account.try_borrow_data()?)?;
        if img_data.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        img_data.is_initialized = true;
        img_data.owner = owner.key.to_string();
        img_data.cid = cid;
        img_data.parent = parent;
        img_data.child = child;
        img_data.diff = diff;
        //img_data.encrypted = encrypted;
        img_data.public = public;
        img_data.editable = editable;
        img_data.views = 0;

        msg!("chi");
        //msg!("{:}",img_data);
        msg!("Image info uploaded!\n owner {} uploaded image with cid {}",
             img_data.owner,
             img_data.cid,);
        ImgData::pack(img_data, &mut img_data_account.try_borrow_mut_data()?)?;
        msg!("chi");

        Ok(())
    }

    fn process_give_permissions(
        accounts: &[AccountInfo],
        cid: String,
        public: u8,
        editable: u8,
        program_id: &Pubkey
    ) -> ProgramResult {
        let accounts_iter = & mut accounts.iter();

        let owner = next_account_info(accounts_iter)?;

        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let img_data_account = next_account_info(accounts_iter)?;

        if img_data_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        let mut img_data = ImgData::unpack_unchecked(&img_data_account.try_borrow_data()?)?;
        if !img_data.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        img_data.public = public;
        img_data.editable = editable;

        if public == 0 {
            img_data.cid = cid;
        }

        msg!("Image permissions modified! Image is now {} and {}",
             img_data.public,
             img_data.editable,);

        ImgData::pack(img_data, &mut img_data_account.try_borrow_mut_data()?)?;

        Ok(())
    }

    fn process_download_image(
        accounts: &[AccountInfo], 
        cid: String, 
        program_id: &Pubkey
    ) -> ProgramResult {
        msg!("chi");
        let accounts_iter = & mut accounts.iter();

        let downloader = next_account_info(accounts_iter)?;
        msg!("chi2");
        if !downloader.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let download_log_account = next_account_info(accounts_iter)?;
        msg!("chi3");
        if download_log_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }
        msg!("chi4");
        msg!("{:?}",download_log_account.data);
        msg!("{:?}",download_log_account.data_len());
        let mut dwnld_log = DwnldLog::unpack_unchecked(&download_log_account.try_borrow_data()?)?;
        msg!("chi5");
        if dwnld_log.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }
        //msg!("chi2");
        dwnld_log.is_initialized = true;
        dwnld_log.downloader = downloader.key.to_string();
        dwnld_log.cid = cid;

        msg!("Image {} downloaded by {}", dwnld_log.cid, dwnld_log.downloader);

        DwnldLog::pack(dwnld_log, &mut download_log_account.try_borrow_mut_data()?)?;

        Ok(())
    }
}

