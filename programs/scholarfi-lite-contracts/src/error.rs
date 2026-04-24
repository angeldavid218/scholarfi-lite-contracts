use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid achievement id")]
    InvalidAchievementId,
    #[msg("Custom error message")]
    CustomError,
    #[msg("Student is not on this teacher's student list")]
    StudentNotOnList,
    #[msg("Achievement title exceeds max length")]
    TitleTooLong,
    #[msg("Student list exceeds max allowed students")]
    StudentListTooLarge,
    #[msg("Student list account does not belong to this teacher")]
    UnauthorizedTeacher,
}
