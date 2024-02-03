use anchor_lang::prelude::*;

#[error_code]
pub enum Errors {
    #[msg("The fundraiser name is excessively lengthy.")]
    W_Name_TooLong,
    #[msg("
The fundraiser description is excessively lengthy; please condense it.")]
    W_Description_TooLong,
    #[msg("Unauthorized to make updates.")]
    W_InvalidAuthority,
    #[msg("Trying to withdraw an amount exceeding the balance of the fundraiser.")]
    W_Amount_TooLarge,
    #[msg("The fundraising objective has not been achieved.")]
    W_GoalNotMet,
    #[msg("The beneficiary provided is not valid.")]
    W_InvalidBeneficiary,
}
