use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] 
pub struct UserInfo {
    #[max_len(50)]
    pub first_name: String, 
    #[max_len(50)]
    pub last_name: String, 
}