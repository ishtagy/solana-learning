use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{mint_to, Mint, MintTo, Token, TokenAccount};

declare_id!("chFika848UNparAoDNFXVmuGtCkfiQZfJanvvotUXSw");

const MIN_RATING: u8 = 1;
const MAX_RATING: u8 = 5;
const MAX_TITLE_LENGTH: usize = 20;
const MAX_DESCRIPTION_LENGTH: usize = 50;
const DISCRIMINATOR: usize = 8;

#[program]
pub mod anchor_movie_review_program {
    use super::*;

    pub fn add_movie_review(
        ctx: Context<AddMovieReview>,
        title: String,
        description: String,
        rating: u8,
    ) -> Result<()> {
        require!(
            rating >= MIN_RATING && rating <= MAX_RATING,
            MovieReviewError::InvalidRating
        );

        require!(
            title.len() <= MAX_TITLE_LENGTH,
            MovieReviewError::TitleTooLong
        );

        require!(
            description.len() <= MAX_DESCRIPTION_LENGTH,
            MovieReviewError::DescriptionTooLong
        );

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.reviewer = ctx.accounts.initializer.key();
        movie_review.title = title;
        movie_review.description = description;
        movie_review.rating = rating;

        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.mint.to_account_info(),
                },
                &[&["mint".as_bytes(), &[ctx.bumps.mint]]],
            ),
            10u64.pow(7),
        )?;

        Ok(())
    }

    pub fn update_movie_review(
        ctx: Context<UpdateMovieReview>,
        title: String,
        description: String,
        rating: u8,
    ) -> Result<()> {
        require!(
            rating >= MIN_RATING && rating <= MAX_RATING,
            MovieReviewError::InvalidRating,
        );

        require!(
            description.len() <= MAX_DESCRIPTION_LENGTH,
            MovieReviewError::DescriptionTooLong,
        );

        msg!("Movie review account space reallocated");

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.rating = rating;
        movie_review.description = description;

        Ok(())
    }

    pub fn delete_movie_review(_ctx: Context<DeleteMovieReview>, title: String) -> Result<()> {
        msg!("Movie review for {} deleted", title);
        Ok(())
    }

    pub fn initialize_token_mint(_ctx: Context<InitializeMint>) -> Result<()> {
        msg!("Token mint initialization");
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title:String, description:String)]
pub struct AddMovieReview<'info> {
    #[account(
        init, seeds = [title.as_bytes(), initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = DISCRIMINATOR + MovieAccountState::INIT_SPACE
    )]
    pub movie_review: Account<'info, MovieAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    #[account(
        seeds = ["mint".as_bytes()],
        bump,
        mut
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = initializer,
        associated_token::mint = mint,
        associated_token::authority = initializer
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
#[instruction(title:String)]
pub struct UpdateMovieReview<'info> {
    #[account(mut,
        seeds = [title.as_bytes(), initializer.key().as_ref()],
        bump,
        realloc = DISCRIMINATOR + MovieAccountState::INIT_SPACE,
        realloc::payer=initializer,
        realloc::zero = true, )]
    pub movie_review: Account<'info, MovieAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteMovieReview<'info> {
    #[account(
        mut,
        seeds=[title.as_bytes(), initializer.key().as_ref()],
        bump,
        close=initializer
    )]
    pub movie_review: Account<'info, MovieAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct MovieAccountState {
    pub reviewer: Pubkey,
    pub rating: u8,
    #[max_len(20)]
    pub title: String,
    #[max_len(50)]
    pub description: String,
}

#[error_code]
enum MovieReviewError {
    #[msg("Rating must be between 1 and 5")]
    InvalidRating,

    #[msg("Movie Title too long")]
    TitleTooLong,
    #[msg("Movie Description too long")]
    DescriptionTooLong,
}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(
        init,
        seeds = ["mint".as_bytes()],
        bump,
        payer = user,
        mint::decimals = 6,
        mint::authority = mint,
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}
