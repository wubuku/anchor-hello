use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
    init,
    payer = authority,
    space = 8 + HelloWorld::INIT_SPACE,
    seeds = [
    b"hello-world",
    authority.key().as_ref(),
    ],
    bump
    )]
    pub hello_world: Account<'info, HelloWorld>,
    //
    //
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
