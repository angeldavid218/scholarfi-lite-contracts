pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use state::*;

declare_id!("6WyK1exZACP5v5RDRJsUkkFvxALDikRhLuiovDyJ9wwQ");

#[program]
pub mod scholarfi_lite_contracts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }

    pub fn create_student_list(
        ctx: Context<CreateStudentList>,
        students: Vec<Pubkey>,
    ) -> Result<()> {
        instructions::create_student_list::handler(ctx, students)
    }

    pub fn register_achievement(
        ctx: Context<RegisterAchievement>,
        achievement_id: u64,
        title: String,
        achievement_type: AchievementType,
    ) -> Result<()> {
        instructions::register_achievement::handler(ctx, achievement_id, title, achievement_type)
    }
    
    pub fn delete_achievement(
        ctx: Context<DeleteAchievement>,
        achievement_id: u64,
    ) -> Result<()> {
        instructions::delete_achievement::handler(ctx, achievement_id)
    }
        
    
}


#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct CreateStudentList<'info> {
    #[account(mut)]
    pub teacher: Signer<'info>,
    #[account(
        init,
        payer = teacher,
        space = 8 + StudentList::INIT_SPACE,
        seeds = [crate::constants::STUDENT_LIST_SEED, teacher.key().as_ref()],
        bump
    )]
    pub student_list: Account<'info, StudentList>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(achievement_id: u64, title: String, achievement_type: AchievementType)]
pub struct RegisterAchievement<'info> {
    #[account(mut)]
    pub teacher: Signer<'info>,
    /// CHECK: only used as a public key; membership is enforced against the student list.
    pub student: UncheckedAccount<'info>,
    #[account(
        seeds = [crate::constants::STUDENT_LIST_SEED, teacher.key().as_ref()],
        bump = student_list.bump,
        constraint = student_list.teacher == teacher.key() @ crate::error::ErrorCode::UnauthorizedTeacher
    )]
    pub student_list: Account<'info, StudentList>,
    #[account(
        init,
        payer = teacher,
        space = 8 + Achievement::INIT_SPACE,
        seeds = [
            crate::constants::ACHIEVEMENT_SEED,
            teacher.key().as_ref(),
            student.key().as_ref(),
            &achievement_id.to_le_bytes(),
        ],
        bump
    )]
    pub achievement: Account<'info, Achievement>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(achievement_id: u64)]
pub struct DeleteAchievement<'info> {
    #[account(mut)]
    pub teacher: Signer<'info>,
    /// CHECK: only used as a public key; membership is enforced against the student list.
    pub student: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [
            crate::constants::ACHIEVEMENT_SEED,
            teacher.key().as_ref(),
            student.key().as_ref(),
            &achievement_id.to_le_bytes(),
        ],
        bump = achievement.bump,
       constraint = achievement.teacher == teacher.key() @ crate::error::ErrorCode::UnauthorizedTeacher,
       close = teacher
    )]
    pub achievement: Account<'info, Achievement>,
}