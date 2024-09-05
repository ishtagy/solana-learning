use anchor_lang::prelude::*;

declare_id!("BVhNvYPsaGjpP7iUMTCfmaFX41UHxFBbyYUiffy5EQnu");

#[program]
pub mod anchor_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()>{
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        msg!("Counter account created");
        msg!("Current Count: { }", counter.count);
        Ok(())
    }

    pub fn increment(ctx: Context<Update>) -> Result<()>{
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);
        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter count: {}", counter.count);
        Ok(())
    }

    pub fn decrement(ctx: Context<Update>) -> Result<()>{
        let counter = &mut ctx.accounts.counter;

        require!(counter.count != 0, MyError::CounterCannonBeNegative);

        msg!("Previous counter: {}", counter.count);
        counter.count = counter.count.checked_sub(1).unwrap();
        msg!("Counter count: {}", counter.count);
        Ok(())
    }

}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}

#[account]
pub struct Counter{
    pub count: u64,
}

#[error_code]
pub enum MyError {
    #[msg("Counter cannot be negative")]
    CounterCannonBeNegative
}