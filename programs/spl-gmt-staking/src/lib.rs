// pub mod utils;
use {
    // crate::utils::*,
    anchor_lang::{prelude::*, solana_program::program::{invoke,invoke_signed}},
    spl_token::state,
    };
    declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
    #[program]
    pub mod basic_staking {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
    Ok(())
    }
    pub fn stake(ctx:Context<Stake>,amount: u64) -> ProgramResult{
        let account = &mut ctx.accounts.holder;
        //invoke to transfer token into staking contract
        let token_program = ctx.accounts.token_program.clone();
        let source = ctx.accounts.stake_token.clone();
        let destination = ctx.accounts.stake_pot.clone();
       let authority = ctx.accounts.authority.clone();
      invoke(&spl_token::instruction::transfer(
             token_program.key,
             source.key,
             destination.key,
             authority.key,
             &[],
             amount,
            )?,
            &[source, destination, authority, token_program],
        );
       account.stake_amount += amount;
      let now_ts = Clock::get().unwrap().unix_timestamp;
      account.stake_time = now_ts;
      Ok(())
    }
    pub fn unstake(ctx:Context<Stake>,amount:u64) -> ProgramResult{
       let account = &mut ctx.accounts.holder;
      //invoke to transfer token into holder address
      account.stake_amount -= amount;
      let now_ts = Clock::get().unwrap().unix_timestamp;
      account.stake_time = now_ts;
      Ok(())
    }
    pub fn claim(ctx:Context<Stake>) -> ProgramResult{
       let account = &mut ctx.accounts.holder;
      //invoke to transfer reward token into holder
      Ok(())
    }
    }
    #[derive(Accounts)]
    pub struct Initialize {}
    #[derive(Accounts)]
    pub struct SetAuthority<'info>{
    #[account(mut, signer)]
    authority: AccountInfo<'info>,
    #[account(mut)]
    new_authority: AccountInfo<'info>,
    #[account(mut, owner = spl_token::id())]
    stake_pot:AccountInfo<'info>,
    #[account(address=spl_token::id())]
    token_program:AccountInfo<'info>,
    }
    #[derive(Accounts)]
    pub struct Stake<'info>{
    #[account(mut, signer)]
    authority: AccountInfo<'info>,
    #[account(mut,signer)]
    pub holder:Account<'info,StakeAccount>,
    #[account(mut, owner=spl_token::id())]
    stake_token: AccountInfo<'info>,
    #[account(mut, owner=spl_token::id())]
    stake_pot: AccountInfo<'info>,
    #[account(address=spl_token::id())]
    token_program:AccountInfo<'info>,
    }
    #[account]
    pub struct StakeAccount{
    pub stake_amount:u64,
    pub stake_time:i64,
    }