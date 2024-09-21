#![allow(clippy::result_large_err)]
use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
pub mod state;


declare_id!("2rGs8642zNSqWUwoSksL2LYFQUAuM12Si75drmsEDq63");

#[program]
pub mod test_task {
    use super::*;

    pub fn create_user_info(
        ctx: Context<CreateUserInfo>,
        first_name: String,
        last_name: String,
    ) -> Result<()> {
        create::create_user_info(ctx, first_name, last_name)
    }
}