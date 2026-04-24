use anchor_lang::prelude::*;

use crate::{UpdateAchievement, AchievementType};


pub fn handler(ctx: Context<UpdateAchievement>, achievement_id: u64, title: String, achievement_type: AchievementType) -> Result<()> {
    let achievement = &mut ctx.accounts.achievement;
    achievement.title = title;
    achievement.achievement_type = achievement_type;
    msg!("Achievement {} updated successfully", achievement_id);
    Ok(())
}