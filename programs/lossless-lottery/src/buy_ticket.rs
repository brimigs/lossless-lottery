use anchor_lang::prelude::*;

declare_program!(lending);
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{TokenAccount, TokenInterface};
use lending::accounts::{Bank, User};
use lending::program::Lending;

pub fn buy_ticket(ctx: Context<BuyTicket>) -> Result<()> {
    let ticket_price = ctx.accounts.lottery.ticket_price;

    let cpi_ctx = CpiContext::new(
        ctx.accounts.lending.to_account_info(),
        lending::cpi::accounts::Deposit { 
            signer: ctx.accounts.signer.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            bank: ctx.accounts.bank.to_account_info(),
            bank_token_account: ctx.accounts.bank_token_account.to_account_info(),
            user_account: ctx.accounts.user_account.to_account_info(),
            user_token_account: ctx.accounts.user_token_account.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        },
    ); 
    lending::cpi::deposit(cpi_ctx, ticket_price)?;

    Ok(())
}

#[derive(Accounts)]
pub struct BuyTicket<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account( 
        mut,
        seeds = [b"lossless-lottery".as_ref()],
        bump,
    )]
    pub lottery: Account<'info, Lottery>,
    pub lending: Program<'info, Lending>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut, 
        seeds = [mint.key().as_ref()],
        bump,
    )]  
    pub bank: Account<'info, Bank>,
    #[account(
        mut, 
        seeds = [b"treasury", mint.key().as_ref()],
        bump, 
    )]  
    pub bank_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut, 
        seeds = [signer.key().as_ref()],
        bump,
    )]  
    pub user_account: Account<'info, User>,
    #[account( 
        init_if_needed, 
        payer = signer,
        associated_token::mint = mint, 
        associated_token::authority = signer,
        associated_token::token_program = token_program,
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>, 
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
