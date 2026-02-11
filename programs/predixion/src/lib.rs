use anchor_lang::prelude::*;

declare_id!("Pred1xion1111111111111111111111111111111");

pub mod instructions;
pub mod state;

use instructions::*;

#[program]
pub mod predixion {
    use super::*;

    pub fn create_market(ctx: Context<CreateMarket>, question: String) -> Result<()> {
        let market = &mut ctx.accounts.market;
        market.question = question;
        market.resolved = false;
        Ok(())
    }

    pub fn resolve_market(ctx: Context<ResolveMarket>) -> Result<()> {
        let market = &mut ctx.accounts.market;
        market.resolved = true;
        Ok(())
    }
}
