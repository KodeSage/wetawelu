use anchor_lang::prelude::*;

use crate::errors::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateProject<'info> {
    #[account(init, 
        seeds = [
            name.as_ref(),
            user_account.key().as_ref(),
            beneficiary.key().as_ref()], 
            bump, 
        payer = beneficiary, 
        space = Project::LEN,
        )]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub user_account: Account<'info, User>,
    #[account(mut)]
    pub beneficiary: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

pub fn create_project(
    ctx: Context<CreateProject>,
    name: String,
    description: String,
    image_link: String,
    twitter: String,
    discord: String,
    github: String,
    goal: String,
) -> Result<()> {
    let beneficiary = &mut ctx.accounts.beneficiary;
    let project = &mut ctx.accounts.project;
    let user_account = &mut ctx.accounts.user_account;

    if name.len() > 35 {
        return Err(Errors::W_Name_TooLong.into());
    }

    if description.len() > 480 {
        return Err(Errors::W_Description_TooLong.into());
    }

    project.raised = 0;
    project.beneficiary = beneficiary.key();
    project.creator = user_account.username.to_string();
    project.name = name;
    project.description = description;
    project.image_link = image_link;
    project.twitter = twitter;
    project.github = github;
    project.discord = discord;
    project.goal = goal;
    project.contributions = 0;
    project.balance = 0;
    project.last_updated = 0;
    project.bump = ctx.bumps.project;
    project.time_stamp = Clock::get()?.unix_timestamp;
    project.category = "project".to_string();

    user_account.contributors += 1;

    Ok(())
}