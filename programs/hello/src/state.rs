use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct HelloWorld {
    pub signer_address: Pubkey,
    #[max_len(1000)]
    pub data: String,
    pub uint_1: u128,
    #[max_len(100, 5)]
    pub str_array_data: Vec<String>,
}