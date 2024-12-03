use anchor_lang::prelude::*;

pub mod base;
pub mod entity;

declare_id!("GKdPktyTt2dVCRw7Yfw7kJKUrhXFFT5wgxRqCWDnf3dV");

#[program]
pub mod solana_staking {
    use super::*;

    // 初始化质押
    pub fn initialize_staking(
    ctx: Context<entity::InitializeStaking>,  // 初始化质押上下文
    token_per_sec: u64,  // 每秒奖励的代币数量
    ) -> ProgramResult {
        let staking_instance = &mut ctx.accounts.staking_instance;  // 获取质押实例
        staking_instance.authority= ctx.accounts.authority.key().clone();  // 设置权限
        staking_instance.reward_token_per_sec = token_per_sec;  // 设置每秒奖励的代币数量
        staking_instance.last_reward_timestamp = ctx.accounts.time.unix_timestamp as u64;  // 设置最后奖励时间戳
        staking_instance.accumulated_reward_per_share = 0;  // 初始化每股累计奖励
        staking_instance.reward_token_mint = ctx
            .accounts
            .reward_token_mint
            .to_account_info()
            .key()
            .clone();  // 设置奖励代币的 mint
        staking_instance.allowed_collection_address = ctx
            .accounts
            .allowed_collection_address
            .key()
            .clone();  // 设置允许的NFT集合地址
        Ok(())
    }

    // 初始化用户
    pub fn initialize_user(
        ctx: Context<entity::InitializeUser>,  // 初始化用户上下文
    ) -> ProgramResult {
        let user_instance = &mut ctx.accounts.user_instance;  // 获取用户实例
        user_instance.deposited_amount = 0;  // 初始化存入数量
        user_instance.reward_debt = 0;  // 初始化奖励债务
        user_instance.accumulated_reward = 0;  // 初始化累计奖励
        Ok(())
    }

    // 进入质押
    pub fn enter_staking(
        ctx: Context<entity::EnterStaking>,  // 进入质押上下文
    ) -> ProgramResult {
        let data = &mut ctx.accounts.nft_token_metadata.try_borrow_data()?;  // 获取NFT元数据
        let val = mpl_token_metadata::state::Metadata::deserialize(&mut &data[..])?;  // 反序列化元数据
        let collection_not_proper = val
            .data
            .creators
            .as_ref()
            .unwrap()
            .iter()
            .filter(|item|{
                ctx.accounts.allowed_collection_address.key() ==
                    item.address && item.verified
            })
            .count() == 0;  // 验证NFT集合
        if collection_not_proper || val.mint != ctx.accounts.nft_token_mint.key() {
            msg!("error");
            return Ok(());
        }
        let staking_instance = &mut ctx.accounts.staking_instance;  // 获取质押实例
        let user_instance = &mut ctx.accounts.user_instance;  // 获取用户实例
        let current_timestamp = ctx.accounts.time.unix_timestamp as u64;  // 获取当前时间戳
        
        // 更新奖励池
        update_reward_pool(
            current_timestamp,
            staking_instance,
            user_instance,
        );

        // 执行NFT转移
        let cpi_accounts = Transfer {
            to: ctx.accounts.nft_token_program_wallet.to_account_info(),
            from: ctx.accounts.nft_token_authority_wallet.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let context = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(context, 1)?;

        user_instance.deposited_amount = user_instance
            .deposited_amount
            .checked_add(1)
            .unwrap();  // 更新用户存入数量
        staking_instance.total_shares = staking_instance
            .total_shares
            .checked_add(1)
            .unwrap();  // 更新总份额
        update_reward_debt(
            staking_instance,
            user_instance,
        );
        Ok(())
    }

    // 取消质押
    pub fn cancel_staking(
        ctx: Context<entity::CancelStaking>,  // 取消质押上下文
        staking_instance_bump: u8,  // 质押实例的 bump
    ) -> ProgramResult {
        let data = &mut ctx.accounts.nft_token_metadata.try_borrow_data()?;  // 获取NFT元数据
        msg!("borrow");
        let val = mpl_token_metadata::state::Metadata::deserialize(&mut &data[..])?;  // 反序列化元数据
        msg!("deser");
        let collection_not_proper = val
            .data
            .creators
            .as_ref()
            .unwrap()
            .iter()
            .filter(|item|{
                ctx.accounts.allowed_collection_address.key() == item.address && item.verified
            })
            .count() == 0;  // 验证NFT集合
        msg!("count");
        if collection_not_proper || val.mint != ctx.accounts.nft_token_mint.key() {
            msg!("error");
            return Ok(());
        }
 
        let staking_instance = &mut ctx.accounts.staking_instance;  // 获取质押实例
        let user_instance = &mut ctx.accounts.user_instance;  // 获取用户实例
        let current_timestamp = ctx.accounts.time.unix_timestamp as u64;  // 获取当前时间戳
        msg!("get accounts");
        update_reward_pool(
            current_timestamp,
            staking_instance,
            user_instance,
        );
        msg!("upd pool");
        store_pending_reward(
            staking_instance,
            user_instance,
        );
 
        // 执行NFT转移
        let cpi_accounts = Transfer {
            to: ctx.accounts.nft_token_authority_wallet.to_account_info(),
            from: ctx.accounts.nft_token_program_wallet.to_account_info(),
            authority: staking_instance.clone().to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let context = CpiContext::new(cpi_program, cpi_accounts);
        let authority_seeds = &[
            &STAKING_SEED[..],
            staking_instance.authority.as_ref(),
            &[staking_instance_bump]
        ];
        token::transfer(context.with_signer(&[&authority_seeds[..]]), 1)?;
 
        user_instance.deposited_amount = user_instance
            .deposited_amount
            .checked_sub(1)
            .unwrap();  // 更新用户存入数量
        staking_instance.total_shares = staking_instance
            .total_shares
            .checked_sub(1)
            .unwrap();  // 更新总份额
        update_reward_debt(
            staking_instance,
            user_instance,
        );
        Ok(())
    }

    // 领取奖励
    pub fn claim_rewards(
        ctx: Context<entity::ClaimRewards>,  // 领取奖励上下文
        amount: u64,  // 领取的奖励数量
        staking_instance_bump: u8,  // 质押实例的 bump
    ) -> ProgramResult {
        let staking_instance = &mut ctx.accounts.staking_instance;  // 获取质押实例
        let user_instance = &mut ctx.accounts.user_instance;  // 获取用户实例
        let current_timestamp = ctx.accounts.time.unix_timestamp as u64;  // 获取当前时间戳
        base::update_reward_pool(
            current_timestamp,
            staking_instance,
            user_instance,
        );
        base::store_pending_reward(
            staking_instance,
            user_instance,
        );
 
        // 执行代币铸造
        let cpi_accounts = MintTo {
            mint: ctx.accounts.reward_token_mint.to_account_info(),
            to: ctx.accounts.reward_token_authority_wallet.to_account_info(),
            authority: staking_instance.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let context = CpiContext::new(cpi_program, cpi_accounts);
        let authority_seeds = &[&STAKING_SEED[..],
        staking_instance.authority.as_ref(),
        &[staking_instance_bump]
        ];
 
        let amount = if amount == 0 {
            user_instance.accumulated_reward
        } else {
            amount
        };
        user_instance.accumulated_reward = user_instance
            .accumulated_reward
            .checked_sub(amount)
            .unwrap();
 
        token::mint_to(context.with_signer(&[&authority_seeds[..]]), amount)?;
        base::update_reward_debt(
            staking_instance,
            user_instance,
        );
        Ok(())
    }
}