use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use crate::state::Bank;
use crate::constants::*;


//Typically in lending protocols to adhere to one type of token, they use wrapped Sol because it is an SPL token
//this way instead of dealing anything with Lamports and the system program we make sure all the accounts are interface accounts 

#[derive(Accounts)]
pub struct InitBank<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    
pub mint: InterfaceAccount<'info, Mint>,

#[account(
  init, 
  payer = signer, 
  space = ANCHOR_DESCRIMINATOR + Bank::INIT_SPACE, 
  seeds = [mint.key().as_ref()], 
  bump)]
pub bank: Account<'info, Bank>,

#[account(
  init, 
  token::mint = mint,
  token::authority = bank_token_account,
  payer = signer, 
  seeds = [b"treasury", mint.key().as_ref()],
  bump
 )]
pub bank_token_account: InterfaceAccount<'info, TokenAccount>, //we dont want an ATA we want a Token Account with a PDA so we know this account is specific to the lending protocol bank 

pub token_program: Interface<'info, TokenInterface>,
pub system_program: Program<'info, System>,

}



/*
Why a PDA Token Account for the Lending Protocol?

- Specificity to the Bank: By deriving the bank_token_account using the mint and a PDA, you ensure this token account is unique to the lending protocol's Bank account for a specific token mint.
- Ownership by the Protocol: The PDA makes the bank_token_account belong to the program, not a user. This prevents unauthorized access or misuse.
- Isolation: In a lending protocol, the treasury (holding collateral, loans, or rewards) must be isolated from user accounts. The PDA guarantees this.
- Predictability: A PDA allows the program to deterministically find the treasury account associated with a particular Bank.

Why Avoid the ATA in This Context?

- The ATA would be tied to the user's wallet (signer) rather than the program logic.
- This would break the design principle of the treasury account being program-owned.
- Anyone could potentially create an ATA for the same mint under their wallet, leading to ambiguity.
*/