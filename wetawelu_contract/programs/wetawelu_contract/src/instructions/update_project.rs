use anchor_lang::prelude::*;

use crate::errors::*;
use crate::state::*;

#[derive(Accounts)]
pub struct UpdateProject<'info> {
    #[account(mut, 
        has_one = beneficiary, 
        seeds = [
            project.name.as_ref(), 
            user_account.key().as_ref(), 
            beneficiary.key().as_ref()],
        bump = project.bump)]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub user_account: Account<'info, User>,
    #[account(mut)]
    pub beneficiary: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn update_project(
    ctx: Context<UpdateProject>,
    description: String,
    twitter: String,
    discord: String,
    github: String,
    image: String,
) -> Result<()> {
    let beneficiary = &mut ctx.accounts.beneficiary;
    let project = &mut ctx.accounts.project;

    if beneficiary.key() != project.beneficiary {
        return Err(Errors::W_InvalidBeneficiary.into());
    } 
    if !description.is_empty() {
    project.description = description;
    }
    if !twitter.is_empty() {
    project.twitter = twitter;
    }
    if !github.is_empty() {
    project.github = github;
    }
    if !discord.is_empty() {
    project.discord = discord;
    }
    if !image.is_empty() {
    project.image_link = image;
    }

    project.last_updated = Clock::get()?.unix_timestamp;

    Ok(())
}
