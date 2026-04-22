use anchor_lang::prelude::*;

use crate::constants::MAX_STUDENTS_PER_LIST;
use crate::error::ErrorCode;
use crate::CreateStudentList;

pub fn handler(ctx: Context<CreateStudentList>, students: Vec<Pubkey>) -> Result<()> {
    require!(
        students.len() <= MAX_STUDENTS_PER_LIST,
        ErrorCode::StudentListTooLarge
    );

    let student_list = &mut ctx.accounts.student_list;
    student_list.teacher = ctx.accounts.teacher.key();
    student_list.bump = ctx.bumps.student_list;
    student_list.students = students;
    Ok(())
}
