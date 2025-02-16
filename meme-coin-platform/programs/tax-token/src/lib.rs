use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod tax_token {
    use super::*;

    pub fn initialize_token(
        ctx: Context<InitializeToken>, 
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
                    authority: ctx.accounts.authority.to_account_info(),
                }
            ), 
            total_supply
        )?;

        // Store token metadata
        let token_meta = TokenMetadata {
            name,
            symbol,
            decimals,
            total_supply,
            tax_percentage,
            creator: *ctx.accounts.authority.key,
        };

        ctx.accounts.token_metadata.set_inner(token_meta);

        Ok(())
    }

    pub fn transfer_with_tax(
        ctx: Context<TransferWithTax>, 
        amount: u64
    ) -> Result<()> {
        // Calculate tax amount
        let token_metadata = &ctx.accounts.token_metadata;
        let tax_amount = amount * token_metadata.tax_percentage as u64 / 100;
        let transfer_amount = amount - tax_amount;

        // Perform transfer with tax deduction
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.from.to_account_info(),
                    to: ctx.accounts.to.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                }
            ),
            transfer_amount
        )?;

        // Transfer tax to designated vault
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.from.to_account_info(),
                    to: ctx.accounts.tax_vault.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                }
            ),
            tax_amount
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeToken<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        mint::decimals = decimals,
        mint::authority = authority.key()
    )]
    pub mint: Account<'info, Mint>,
    
    #[account(
        init,
        payer = authority,
        token::mint = mint,
        token::authority = authority.key()
    )]
    pub token_account: Account<'info, TokenAccount>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<TokenMetadata>()
    )]
    pub token_metadata: Account<'info, TokenMetadata>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct TransferWithTax<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub tax_vault: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub token_metadata: Account<'info, TokenMetadata>,
    pub token_program: Program<'info, Token>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u64,
    pub tax_percentage: u8,
    pub creator: Pubkey,
}
