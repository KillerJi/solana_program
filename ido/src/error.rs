use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum IdoError {
    #[error("Deserialize Error")]
    DeserializeError,
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Not administrator")]
    NotAdministrator,
    #[error("Timestamp Error")]
    TimestampError,
    #[error("Status Is Not Open")]
    StatusIsNotOpen,
    #[error("Status Is Not Ended")]
    StatusIsNotEnded,
    #[error("Status Has Ended")]
    StatusHasEnded,
    #[error("The number of submissions cannot be 0")]
    TheNumberOfSubmissionsCannotBe0,
    #[error("The fundraising is full")]
    TheFundraisingIsFull,
    #[error("Exceed the raising limit")]
    ExceedTheRaisingLimit,
    #[error("User Exceed the raising limit")]
    UserExceedTheRaisingLimit,
    #[error("Insufficient balance")]
    InsufficientBalance,
    #[error("Non project party public key")]
    NonProjectPartyPublicKey,
    #[error("Not exist User ada")]
    NotExistUserAda,
    #[error("Calculation overflow")]
    CalculationOverflow,
    #[error("Success flag Error")]
    SuccessFlagError,
    #[error("Allocation date has not arrived")]
    AllocationDateHasNotArrived,
    #[error("Withdrawn Zero")]
    WithdrawnZero,
    #[error("Non whitelist")]
    NonWhitelist,
    #[error("Whitelist len too long")]
    WhitelistLenTooLong,
    #[error("Auction Access Error")]
    AuctionAccessError,
    #[error("Pubkey Derived Error")]
    PubkeyDerivedError,
    #[error("Signature Error")]
    SignatureError,
    #[error("Invalid Seeds")]
    InvalidSeeds,
    #[error("Allocation Input Error")]
    AllocationInputError,
    #[error("Min Swap Level Input Error")]
    MinSwapLevelInputError,
}

impl From<IdoError> for ProgramError {
    fn from(e: IdoError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
