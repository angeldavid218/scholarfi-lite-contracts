# ScholarFi Lite Contracts

Anchor-based Solana program for teacher-issued student achievements.

## Overview

This project stores academic achievements on-chain with a simple authorization model:

- A teacher creates a `StudentList` containing approved student wallets.
- The same teacher can register, update, and delete `Achievement` records for students in that list.
- Each achievement stores:
  - `student: Pubkey`
  - `teacher: Pubkey`
  - `title: String`
  - `achievement_type: AchievementType` (`Token` or `Nft`)

## Program

- **Program name:** `scholarfi_lite_contracts`
- **Program ID (local config):** `6WyK1exZACP5v5RDRJsUkkFvxALDikRhLuiovDyJ9wwQ`

## Accounts

### `StudentList`

Teacher-scoped list of approved student wallets.

- PDA seeds: `["student_list", teacher_pubkey]`
- Fields:
  - `teacher: Pubkey`
  - `bump: u8`
  - `students: Vec<Pubkey>` (max `100`)

### `Achievement`

One on-chain achievement record linked to a student and teacher.

- PDA seeds: `["achievement", teacher_pubkey, student_pubkey, achievement_id_le_bytes]`
- Fields:
  - `student: Pubkey`
  - `teacher: Pubkey`
  - `title: String` (max `128` bytes)
  - `achievement_type: AchievementType`
  - `bump: u8`

## Instructions

- `initialize()`
  - Basic init entrypoint.
- `create_student_list(students: Vec<Pubkey>)`
  - Creates a teacher-owned student list.
  - Enforces max list length.
- `register_achievement(achievement_id: u64, title: String, achievement_type: AchievementType)`
  - Requires teacher signer.
  - Verifies student is present in teacher's `StudentList`.
  - Creates achievement PDA.
- `update_achievement(achievement_id: u64, title: String, achievement_type: AchievementType)`
  - Requires teacher signer.
  - Updates title/type on an existing achievement.
- `delete_achievement(achievement_id: u64)`
  - Requires teacher signer.
  - Closes the achievement account and refunds rent to teacher.

## Error Conditions

- `StudentNotOnList` when teacher tries to issue an achievement to a wallet not in `StudentList`.
- `TitleTooLong` when title exceeds max byte length.
- `StudentListTooLarge` when list exceeds max length.
- `UnauthorizedTeacher` when a teacher tries to mutate an account they do not own.

## Local Development

### Prerequisites

- Rust toolchain
- Solana CLI
- Anchor CLI
- Yarn

### Install dependencies

```bash
yarn install
```

### Build

```bash
anchor build
```

### Test

```bash
cargo test
```

Notes:

- The Rust integration test at `programs/scholarfi-lite-contracts/tests/test_initialize.rs` expects a built program artifact at `target/deploy/scholarfi_lite_contracts.so`.
- Run `anchor build` before running tests that load the `.so`.

## Project Structure

- `programs/scholarfi-lite-contracts/src/lib.rs` - program entrypoints and account contexts.
- `programs/scholarfi-lite-contracts/src/state.rs` - on-chain account/state definitions.
- `programs/scholarfi-lite-contracts/src/instructions/` - per-instruction handlers.
- `programs/scholarfi-lite-contracts/src/error.rs` - custom error codes.
- `programs/scholarfi-lite-contracts/src/constants.rs` - seeds and limits.
