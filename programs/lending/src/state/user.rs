use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct User{
  pub owner: Pubkey,
  pub deposited_sol: u64,
  pub deposited_sol_shares: u64,
  pub borrowed_sol: u64,
  pub borrowed_sol_shares: u64,
  pub deposited_usdc: u64,
  pub deposited_usdc_shares: u64,
  pub borrowed_usdc: u64,
  pub borrowed_usdc_shares: u64,
  pub usdc_address: Pubkey, //you want to store the USDC mint address so we can check in instructions if the user is it USDC or Sol
  pub last_update: i64, //timestamp
  

}

//for a regular lending protocol you would have another struct that would point to the allowed assets
//maybe an array/vector that would point to the allowed assets