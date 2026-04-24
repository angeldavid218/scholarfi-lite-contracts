use anchor_lang::prelude::*;
use crate::DeleteAchievement;


pub fn handler(ctx: Context<DeleteAchievement>, _achievement_id: u64) -> Result<()> {
    let achievement = &mut ctx.accounts.achievement;

    msg!("Achievement {} deleted", achievement.title);
    Ok(())
}
