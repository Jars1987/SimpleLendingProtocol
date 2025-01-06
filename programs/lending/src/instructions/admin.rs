use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use crate::state::{Bank, User};
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

#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer, 
        space = ANCHOR_DESCRIMINATOR + User::INIT_SPACE,
        seeds = [signer.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, User>,
    pub system_program: Program <'info, System>,
}

pub fn process_init_bank(
    ctx: Context<InitBank>,
    liquidation_threshold: u64,
    max_ltv: u64,
) -> Result<()> {
    let bank = &mut ctx.accounts.bank;
    bank.mint_address = ctx.accounts.mint.key();
    bank.authority = ctx.accounts.signer.key();
    bank.liquidation_threshold = liquidation_threshold;
    bank.max_ltv = max_ltv;
    Ok(())
}

pub fn process_init_user(ctx: Context<InitUser>, usdc_address: Pubkey) -> Result<()> {
    let user = &mut ctx.accounts.user_account;
    user.owner = ctx.accounts.signer.key();
    user.usdc_address = usdc_address;
    
    let now = Clock::get()?.unix_timestamp; 
    user.last_updated = now;

    Ok(())
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