use anchor_lang::error_code;

#[error_code]
pub enum RevealError {
    #[msg("Unable to find identity for revealer")]
    IdentityUnknownError,
}
