use anchor_lang::prelude::*;

declare_id!("GkgdVw7gRnR28Q979EQPiriovTrXBmpJSZLRWXQuG2EM");
pub mod errors;
pub mod state;
pub mod instructions;

pub use errors::*;
pub use state::*;
pub use instructions::*;

#[program]
pub mod wetawelu_contract {
    use super::*;

     pub fn create_user(ctx: Context<CreateUser>, username: String, image: String) -> Result<()> {
        instructions::create_user(ctx, username, image).expect("User creation unsuccessful.");
        Ok(())
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
        instructions::create_project(
            ctx,
            name,
            description,
            image_link,
            twitter,
            discord,
            github,
            goal,
        )
        .expect("
                The creation of a project fundraiser was unsuccessful.");

        Ok(())
    }

    pub fn contribute_project(ctx: Context<ContributeProject>, amount: u64) -> Result<()> {
        instructions::contribute_project(ctx, amount).expect("Did not succeed in making a contribution to the project.");
        Ok(())
    }

     pub fn update_project(
        ctx: Context<UpdateProject>,
        description: String,
        twitter: String,
        discord: String,
        github: String,
        image: String,
    ) -> Result<()> {
        instructions::update_project(ctx, description, twitter, discord,  github, image)
            .expect("Failed to update project.");

        Ok(())
    }

     pub fn withdraw_project(ctx: Context<WithdrawProject>, amount: u64) -> Result<()> {
        instructions::withdraw_project(ctx, amount).expect("Failed to withdraw to Raised Funds.");
        Ok(())
    }

}

