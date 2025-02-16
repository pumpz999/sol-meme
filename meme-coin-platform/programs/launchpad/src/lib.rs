use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};
use anchor_spl::associated_token::AssociatedToken;

declare_id!("HWEHzrf1uts7Noq3Qz8qRxMsm9QxTmAUMdKkwQUMzQhh");

#[program]
pub mod launchpad {
    use super::*;

    pub fn create_token(
        ctx: Context<CreateToken>, 
        name: String, 
        symbol: String, 
        decimals: u8,
        total_supply: u64,
        tax_percentage: u8
    ) -> Result<()> {
        // Create token mint
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.creator.to_account_info(),
                }
            ), 
            total_supply
        )?;

        // Store token launch details
        let launch_details = TokenLaunchDetails {
            name,
            symbol,
            decimals,
            total_supply,
            tax_percentage,
            creator: *ctx.accounts.creator.key,
            launch_timestamp: Clock::get()?.unix_timestamp,
        };

        ctx.accounts.launch_details.set_inner(launch_details);

        Ok(())
    }

    pub fn create_liquidity_pool(
        ctx: Context<CreateLiquidityPool>, 
        sol_amount: u64,
        token_amount: u64
    ) -> Result<()> {
        // Validate liquidity amounts
        require!(sol_amount > 0 && token_amount > 0, LaunchpadError::InvalidLiquidityAmount);

        // Transfer SOL to liquidity pool
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.creator.to_account_info(),
                to: ctx.accounts.liquidity_pool.to_account_info(),
            }
        );
        anchor_lang::system_program::transfer(cpi_context, sol_amount)?;

        // Transfer tokens to liquidity pool
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.token_account.to_account_info(),
                    to: ctx.accounts.pool_token_account.to_account_info(),
                    authority: ctx.accounts.creator.to_account_info(),
                }
            ),
            token_amount
        )?;

        // Update launch details with pool information
        ctx.accounts.launch_details.liquidity_pool = ctx.accounts.liquidity_pool.key();
        ctx.accounts.launch_details.sol_liquidity = sol_amount;
        ctx.accounts.launch_details.token_liquidity = token_amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    
    #[account(
        init,
        payer = creator,
        mint::decimals = decimals,
        mint::authority = creator.key()
    )]
    pub mint: Account<'info, Mint>,
    
    #[account(
        init,
        payer = creator,
        associated_token::mint = mint,
        associated_token::authority = creator
    )]
    pub token_account: Account<'info, TokenAccount>,
    
    #[account(
        init,
        payer = creator,
        space = 8 + std::mem::size_of::<TokenLaunchDetails>()
    )]
    pub launch_details: Account<'info, TokenLaunchDetails>,
    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CreateLiquidityPool<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub pool_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub liquidity_pool: SystemAccount<'info>,
    
    #[account(mut)]
    pub launch_details: Account<'info, TokenLaunchDetails>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenLaunchDetails {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u64,
    pub tax_percentage: u8,
    pub creator: Pubkey,
    pub launch_timestamp: i64,
    pub liquidity_pool: Pubkey,
    pub sol_liquidity: u64,
    pub token_liquidity: u64,
}

#[error_code]
pub enum LaunchpadError {
    #[msg("Invalid liquidity amount")]
    InvalidLiquidityAmount,
}
