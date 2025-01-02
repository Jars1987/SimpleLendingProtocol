use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Bank {
  pub authority: Pubkey,
  pub mint_address: Pubkey,
  pub total_deposits: u64,
  pub total_deposit_shares: u64,
  //asset specific constants:
  pub liquidation_threshold: u64, // the loan to value (LTV) at which the loan is defined undercollateralized and that can be liquidated
  pub liquidation_bonus: u64, //% of liquidation that is given to the liquidator
  pub liquidation_close_factor: u64, //% of collateral that can be liquidated
  pub max_ltv: u64, //max % collateral that can be borrowed for a specific asset
  pub last_update: i64,
}