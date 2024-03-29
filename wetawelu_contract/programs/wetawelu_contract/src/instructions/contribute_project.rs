use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::state::*;

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct ContributeProject<'info> {
    #[account(mut)]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub contributor: Signer<'info>,
    #[account(mut)]
    pub user_account: Account<'info, User>,
    pub token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn contribute_project(ctx: Context<ContributeProject>, amount: u64) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.contributor.to_account_info(),
                to: ctx.accounts.project.to_account_info(),
            },
        ),
        amount,
    )?;

    ctx.accounts.project.raised += amount as u64;
    ctx.accounts.project.contributions += 1;
    ctx.accounts.project.balance += amount as u64;
    user_account.contributions += 1;

    Ok(())
}
