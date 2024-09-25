use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{ Mint, TokenAccount, TokenInterface };

mod buy_ticket;

declare_id!("DuefNx3LzFS4APsodiwEN6t2vf6W9jUGtbjT37HPFjMx");

// automatically generate module using program idl found in ./idls
declare_program!(lending);
use lending::accounts::{Bank, User};
use lending::program::LendingProtocol;

#[program]
pub mod lossless_lottery {

    use buy_ticket::buy_ticket;

    use super::*;

    pub fn create_lottery(ctx: Context<CreateLottery>, ticket_price: u64, lottery_duration: i64) -> Result<()> { 
        let lottery = &mut ctx.accounts.lottery;
        lottery.authority = *ctx.accounts.signer.key;
        lottery.deadline = Clock::get().unwrap().unix_timestamp + lottery_duration;
        lottery.bump = ctx.bumps.lottery; 
        lottery.winner_chosen = false; 
        lottery.ticket_price = ticket_price;
        lottery.randomness_account = Pubkey::default();
        lottery.number_of_tickets = 0;

        Ok(())
    }


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
    
    // pub fn reveal_winner() -> Result<()> { 

    //     Ok(())
    // }

    pub fn return_funds(ctx: Context<ReturnFunds>) -> Result<()> {
        let lottery = &ctx.accounts.lottery;
        let ticket_price = lottery.ticket_price;

        for ticket_number in 0..lottery.number_of_tickets {
            let seeds = &[ticket_number.to_le_bytes().as_ref(), b"participant".as_ref()];
            let (participant_pda, _bump) = Pubkey::find_program_address(seeds, ctx.program_id);

        }
        let cpi_ctx = CpiContext::new(
            ctx.accounts.lending.to_account_info(),
            lending::cpi::accounts::Withdraw {
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
        lending::cpi::withdraw(cpi_ctx, ticket_price)?;

        Ok(())
    }

    
}

#[derive(Accounts)]
pub struct CreateLottery<'info> { 
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account( 
        init, 
        payer = signer,
        space = 8 + Lottery::INIT_SPACE,
        seeds = [b"lossless-lottery".as_ref()],
        bump,
    )]
    pub lottery: Account<'info, Lottery>,
    pub system_program: AccountInfo<'info>,
}


#[derive(Accounts)]
pub struct ReturnFunds<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub lending: Program<'info, LendingProtocol>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account( 
        mut,
        seeds = [b"lossless-lottery".as_ref()],
        bump,
    )]
    pub lottery: Account<'info, Lottery>,
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

#[derive(Accounts)]
pub struct BuyTicket<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub lending: Program<'info, LendingProtocol>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account( 
        mut,
        seeds = [b"lossless-lottery".as_ref()],
        bump,
    )]
    pub lottery: Account<'info, Lottery>,
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

#[account]
#[derive(InitSpace)]
pub struct Lottery { 
    pub bump: u8, 
    pub winner: u64, 
    pub winner_chosen: bool,
    pub deadline: i64,
    pub number_of_tickets: u64,
    pub ticket_price: u64,
    pub randomness_account: Pubkey, 
    pub authority: Pubkey,
}