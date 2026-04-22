use anchor_lang::prelude::*;

/// How the achievement is represented off-chain or in a follow-up mint flow.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace)]
pub enum AchievementType {
    Token,
    Nft,
}

/// One issued achievement for a student, signed by the teacher.
#[account]
#[derive(InitSpace)]
pub struct Achievement {
    pub student: Pubkey,
    pub teacher: Pubkey,
    #[max_len(128)]
    pub title: String,
    pub achievement_type: AchievementType,
}

/// Teacher-scoped list of student wallets allowed to receive achievements.
#[account]
#[derive(InitSpace)]
pub struct StudentList {
    pub teacher: Pubkey,
    pub bump: u8,
    #[max_len(100)]
    pub students: Vec<Pubkey>,
}
