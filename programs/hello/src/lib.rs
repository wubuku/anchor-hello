use anchor_lang::prelude::*;
pub mod context;
pub mod state;
pub mod event;

use crate::context::*;
use crate::state::*;
use crate::event::*;

declare_id!("5eKArngzHduTsRLhcszUEifwtRSpYg5d6Ad41Sc1bKx7");

#[program]
pub mod hello {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        arg_1: u128,
        arg_3: Vec<String>,
        arg_4: Vec<String>,
        arg_2: String,
    ) -> Result<()> {
        msg!("arg1: {}", arg_1);
        msg!("arg2: {}", arg_2);
        msg!("arg3: {}", arg_3.join("."));
        msg!("arg4: {}", arg_4.join("."));

        *ctx.accounts.hello_world = HelloWorld {
            signer_address: *ctx.accounts.authority.key,
            data: arg_2,
            uint_1: arg_1,
            str_array_data: arg_3,
        };
        msg!("ctx.accounts.authority.key: {}", *ctx.accounts.authority.key);

        emit!(Initialized {
            signer_address: *ctx.accounts.authority.key
        });
        Ok(())
    }
}

