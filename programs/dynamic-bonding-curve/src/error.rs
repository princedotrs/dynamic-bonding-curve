use anchor_lang::prelude::*;

#[error_code]
pub enum DBCErrorCode {
    #[msg("Bonding curve is already complete")]
    BondingCurveComplete,

    #[msg("Bonding curve is not complete yet")]
    BondingCurveNotComplete,

    #[msg("Not authorized to perform this action")]
    NotAuthorized,

    #[msg("Insufficient tokens in bonding curve reserve")]
    InsufficientTokens,

    #[msg("Insufficient SOL in bonding curve reserve")]
    InsufficientSol,
    #[msg("Overflow")]
    Overflow,

    #[msg("Underflow")]
    Underflow,

    #[msg("Slippage: Too much SOL required")]
    TooMuchSolRequired,

    #[msg("Slippage: Too little SOL received")]
    TooLittleSolReceived,

    #[msg("Invalid calculation")]
    InvalidCalculation,

    #[msg("Invalid Input")]
    InvalidInput,

    #[msg("Invalid Fee Account")]
    InvalidFeeAccount,

    #[msg("Contract is Paused for security Purpose")]
    ContractPaused,
}
