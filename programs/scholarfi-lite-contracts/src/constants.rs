use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";

pub const STUDENT_LIST_SEED: &[u8] = b"student_list";
pub const ACHIEVEMENT_SEED: &[u8] = b"achievement";

/// Max UTF-8 byte length for achievement titles.
pub const MAX_TITLE_BYTES: usize = 128;

/// Must match `#[max_len(...)]` on `StudentList::students`.
pub const MAX_STUDENTS_PER_LIST: usize = 100;
