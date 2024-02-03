use anchor_lang::prelude::*;

declare_id!("GkgdVw7gRnR28Q979EQPiriovTrXBmpJSZLRWXQuG2EM");
pub mod errors; //This is the Error File for Wetawelu_contract

#[program]
pub mod wetawelu_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
