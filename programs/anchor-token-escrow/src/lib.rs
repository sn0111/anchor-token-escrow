use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token};

declare_id!("BvyKgBXhcmTyfQ2EuZxR5hBsrrbEZZrbmZtftneCFoD5");

#[program]
pub mod anchor_token_escrow {
    use anchor_spl::token::{set_authority, SetAuthority, transfer, close_account, CloseAccount, Transfer, spl_token::instruction::AuthorityType};

    use super::*;

    pub fn initialize_escrow(ctx: Context<InitializeEscrow>,amount:u64) -> Result<()> {

        let escrow = &mut ctx.accounts.escrow_account;
        escrow.initilizer = ctx.accounts.initilizer.key();
        escrow.initilizer_mint = ctx.accounts.initilizer_mint.key();
        escrow.vault_account = ctx.accounts.vault_account.key();
        escrow.initilizer_receive_account = ctx.accounts.initilizer_receive_account.key();
        escrow.amount = amount;

        let (pda,_bump) = Pubkey::find_program_address(&[b"escrow"], ctx.program_id);

        set_authority(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(), 
                SetAuthority{account_or_mint:ctx.accounts.vault_account.to_account_info(),current_authority:ctx.accounts.initilizer.to_account_info()}), 
                AuthorityType::AccountOwner, 
                Some(pda)
        )?;

        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(), 
                Transfer{
                    from:ctx.accounts.initilizer_deposit_account.to_account_info(),
                    to:ctx.accounts.vault_account.to_account_info(),
                    authority:ctx.accounts.initilizer.to_account_info()
                }
            ), 
            amount
        )?;

        Ok(())
    }

    pub fn exchange_token(ctx: Context<ExchangeToken>,amount:u64) ->Result<()>{

        let (pda, bump) = Pubkey::find_program_address(&[b"escrow"], ctx.program_id);

        let escrow = &mut ctx.accounts.escrow_account;
        
        if escrow.amount != amount{
            return err!(EscrowError::AmountNotEqual);
        }
        if escrow.initilizer_mint != ctx.accounts.initilizer_mint.key(){
            return err!(EscrowError::InitializerMintNotMatched);
        }
        if escrow.initilizer_receive_account != ctx.accounts.initilizer_receive_account.key(){
            return err!(EscrowError::InitializerAccountNotMatched);
        }
        if escrow.vault_account != ctx.accounts.vault_account.key(){
            return err!(EscrowError::InitializerVaultAccountNotMatched);
        }
        if pda != ctx.accounts.vault_pda.key(){
            return err!(EscrowError::InitializerPdaNotMatched);
        }

        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(), 
                Transfer{
                    from:ctx.accounts.receiver_token_account.to_account_info(),
                    to:ctx.accounts.initilizer_receive_account.to_account_info(),
                    authority:ctx.accounts.receiver.to_account_info()
                }
            ), 
            amount
        )?;

        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(), 
                Transfer{
                    from:ctx.accounts.vault_account.to_account_info(),
                    to:ctx.accounts.receiver_receive_account.to_account_info(),
                    authority:ctx.accounts.vault_pda.to_account_info()
                },
                &[&[&b"escrow"[..],&[bump]]]
            ), 
            amount
        )?;

        close_account(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(), 
                CloseAccount{
                    account:ctx.accounts.vault_account.to_account_info(),
                    authority:ctx.accounts.vault_pda.to_account_info(),
                    destination:ctx.accounts.receiver.to_account_info()
                },
                &[&[&b"escrow"[..], &[bump]]]
            )
        )?;

        Ok(())
    }


}

#[account]
pub struct EscrowAccount{
    pub amount:u64,
    pub initilizer:Pubkey,
    pub initilizer_mint:Pubkey,
    pub vault_account:Pubkey,
    pub initilizer_receive_account:Pubkey,
    pub escrow_pda:Pubkey
}

#[derive(Accounts)]
#[instruction(amount:u64)]
pub struct InitializeEscrow<'info> {
    #[account(init,payer=initilizer,space=8+8+32+32+32+32+32)]
    pub escrow_account:Account<'info,EscrowAccount>,
    #[account(mut)]
    pub initilizer: Signer<'info>,
    pub initilizer_mint: Account<'info,Mint>,
    #[account(
        mut,
        constraint = initilizer_deposit_account.amount >= amount
    )]
    pub initilizer_deposit_account: Account<'info,TokenAccount>,
    pub receiver_mint: Account<'info,Mint>,
    #[account(
        init,
        payer=initilizer,
        token::mint=receiver_mint,
        token::authority=initilizer
    )]
    pub initilizer_receive_account:Account<'info,TokenAccount>,
    #[account(
        init,
        payer=initilizer,
        token::mint=initilizer_mint,
        token::authority=initilizer
    )]
    pub vault_account: Account<'info,TokenAccount>,
    pub system_program: Program<'info,System>,
    pub token_program: Program<'info,Token>

}

#[derive(Accounts)]
pub struct ExchangeToken<'info>{
    #[account(mut)]
    pub escrow_account: Account<'info,EscrowAccount>,
    pub receiver_mint: Account<'info,Mint>,
    pub initilizer_mint: Account<'info,Mint>,
    #[account(mut)]
    pub receiver_token_account: Account<'info,TokenAccount>,
    #[account(mut)]
    pub receiver: Signer<'info>,
    #[account(
        init,
        payer=receiver,
        token::mint=initilizer_mint,
        token::authority=receiver
    )]
    pub receiver_receive_account:Account<'info,TokenAccount>,
    #[account(mut)]
    pub initilizer_receive_account:Account<'info,TokenAccount>,
    #[account(mut)]
    pub vault_account:Account<'info,TokenAccount>,
    ///CHECK:
    #[account(mut)]
    pub vault_pda:AccountInfo<'info>,
    pub system_program: Program<'info,System>,
    pub token_program: Program<'info,Token>

}

#[error_code]
pub enum EscrowError {
    #[msg("Initlizer amount doesn't match with receiver amount")]
    AmountNotEqual,
    #[msg("Initlizer mint account not matched")]
    InitializerMintNotMatched,
    #[msg("Initlizer receive account not matched")]
    InitializerAccountNotMatched,
    #[msg("Initlizer mint account not matched")]
    InitializerVaultAccountNotMatched,
    #[msg("Initlizer PDA account not matched")]
    InitializerPdaNotMatched,
}
