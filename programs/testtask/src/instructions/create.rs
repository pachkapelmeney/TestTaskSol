use crate::state::UserInfo;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateUserInfo<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + UserInfo::INIT_SPACE,
    )]
    user_info: Account<'info, UserInfo>,
    system_program: Program<'info, System>,
}

pub fn create_user_info(
    ctx: Context<CreateUserInfo>,
    first_name: String,
    last_name: String,
) -> Result<()> {
    *ctx.accounts.user_info = UserInfo {
        first_name,
        last_name,
    };
    Ok(())
}