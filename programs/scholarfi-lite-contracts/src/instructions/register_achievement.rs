use anchor_lang::prelude::*;

use crate::constants::MAX_TITLE_BYTES;
use crate::error::ErrorCode;
use crate::state::AchievementType;
use crate::RegisterAchievement;

pub fn handler(
    ctx: Context<RegisterAchievement>,
    _achievement_id: u64,
    title: String,
    achievement_type: AchievementType,
) -> Result<()> {
    require!(title.len() <= MAX_TITLE_BYTES, ErrorCode::TitleTooLong);
    require!(
        ctx.accounts
            .student_list
            .students
            .contains(&ctx.accounts.student.key()),
        ErrorCode::StudentNotOnList
    );

    let achievement = &mut ctx.accounts.achievement;
    achievement.student = ctx.accounts.student.key();
    achievement.teacher = ctx.accounts.teacher.key();
    achievement.title = title;
    achievement.achievement_type = achievement_type;
    achievement.bump = ctx.bumps.achievement;
    Ok(())
}
