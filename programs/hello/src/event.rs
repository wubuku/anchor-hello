use anchor_lang::prelude::*;

#[event]
pub struct Initialized {
    //pub data: u64,
    pub signer_address: Pubkey,
}


