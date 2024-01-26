use anchor_lang::prelude::*;

declare_id!("Dtg6jj2BmZTaUY3JSDDZe91Vx7X6EKNCJx8b4ZH8gUdP");

#[program]
pub mod wetawelu_solanacontract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
