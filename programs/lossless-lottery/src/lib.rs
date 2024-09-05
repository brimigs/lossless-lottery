use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{ Mint, TokenAccount, TokenInterface };

mod buy_ticket;

declare_id!("DuefNx3LzFS4APsodiwEN6t2vf6W9jUGtbjT37HPFjMx");

// automatically generate module using program idl found in ./idls
declare_program!(lending);
use lending::accounts::{Bank, User};
use lending::program::Lending;

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