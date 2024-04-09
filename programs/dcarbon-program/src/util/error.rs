use anchor_lang::prelude::*;

#[error_code]
pub enum DcarbonError {
    // 0x1770
    #[msg("M0001")]
    InvalidNonce,

    #[msg("M0009")]
    AlreadyMintToday,

    #[msg("M0010")]
    WrongDeviceOwnerOrDeviceSuspended,

    #[msg("M0020")]
    DeviceNotFound,

    #[msg("M0021")]
    LimitShouldBePositive,

    #[msg("M0023")]
    WrongDeviceOwner,

    #[msg("Signature verification failed")]
    SigVerificationFailed,

    #[msg("Signature header verification failed")]
    SigHeaderFailed,

    #[msg("Signature intruction verification failed")]
    SigInstructionFailed,

    // new add
    #[msg("Wrong Signature")]
    WrongSignature,

    #[msg("Permission Denied")]
    PermissionDenied,
}
