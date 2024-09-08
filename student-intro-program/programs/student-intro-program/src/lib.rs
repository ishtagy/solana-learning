use anchor_lang::prelude::*;

declare_id!("6vv7py4b6QB6UtUnChD1uEWKQs9SJ9RdMAm5H9nUArwV");

const MAX_NAME_LENGTH: usize = 20;
const MAX_MESSAGE_LENGTH: usize = 40;
const DISCRIMINATOR: usize = 8;

#[program]
pub mod student_intro_program {
    use super::*;

    pub fn create_account(
        ctx: Context<CreateAccount>,
        name: String,
        message: String,
    ) -> Result<()> {
        require!(
            name.len() <= MAX_NAME_LENGTH,
            StudentAccountError::NameTooLong
        );
        require!(
            message.len() <= MAX_MESSAGE_LENGTH,
            StudentAccountError::MessageTooLong
        );

        let student_account = &mut ctx.accounts.student_account;
        student_account.student = ctx.accounts.initializer.key();
        student_account.name = name;
        student_account.message = message;

        Ok(())
    }

    pub fn update_account(
        ctx: Context<UpdateAccount>,
        name: String,
        message: String,
    ) -> Result<()> {
        let student_account = &mut ctx.accounts.student_account;
        student_account.message = message;

        Ok(())
    }

    pub fn delete_account(_ctx: Context<DeleteAccount>, name: String) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateAccount<'info> {
    #[account(
        init,
        seeds = [name.as_bytes(), initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = DISCRIMINATOR + StudentState::INIT_SPACE
    )]
    pub student_account: Account<'info, StudentState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct UpdateAccount<'info> {
    #[account(
        mut,
        seeds = [name.as_bytes(), initializer.key().as_ref()],
        bump,
        realloc = DISCRIMINATOR + StudentState::INIT_SPACE,
        realloc::payer = initializer,
        realloc::zero = true,
    )]
    pub student_account: Account<'info, StudentState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct DeleteAccount<'info> {
    #[account(
        mut,
        seeds=[name.as_bytes(), initializer.key().as_ref()],
        bump,
        close=initializer
    )]
    pub student_account: Account<'info, StudentState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct StudentState {
    pub student: Pubkey,
    #[max_len(20)]
    pub name: String,
    #[max_len(40)]
    pub message: String,
}

#[error_code]
enum StudentAccountError {
    #[msg("Name is too long")]
    NameTooLong,
    #[msg("Message is too long")]
    MessageTooLong,
}
