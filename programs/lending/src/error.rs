use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Thos is your custom message")]
    CustomError,
}