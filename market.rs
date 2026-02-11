use anchor_lang::prelude::*;

#[account]
pub struct Market {
    pub question: String,
    pub resolved: bool,
}
