use anchor_lang::prelude::*;

declare_id!("5eKArngzHduTsRLhcszUEifwtRSpYg5d6Ad41Sc1bKx7");

#[program]
pub mod hello {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        arg_1: u128,
        arg_2: String,
        arg_3: Vec<String>,
    ) -> Result<()> {
        msg!("arg1: {}", arg_1);
        msg!("arg2: {}", arg_2);
        msg!("arg3: {}", arg_3.join("."));

        *ctx.accounts.hello_world = HelloWorld {
            authority: *ctx.accounts.authority.key,
            data: arg_2,
            //uint_1: arg_1,
            //str_array_data: arg_3,
        };

        emit!(MyEvent { data: 1 });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
    init,
    payer = authority,
    space = 8 + HelloWorld::INIT_SPACE,
    seeds = [b"hello-world"],
    bump
    )]
    pub hello_world: Account<'info, HelloWorld>,
    //
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct HelloWorld {
    pub authority: Pubkey,
    #[max_len(1000)]
    pub data: String,
    //pub uint_1: u128,
    //#[max_len(1000)]
    //pub str_array_data: Vec<String>,
}

#[event]
pub struct MyEvent {
    pub data: u64,
}
