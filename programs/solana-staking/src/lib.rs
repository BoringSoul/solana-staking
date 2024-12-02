use anchor_lang::prelude::*;

declare_id!("GKdPktyTt2dVCRw7Yfw7kJKUrhXFFT5wgxRqCWDnf3dV");

#[program]
pub mod solana_staking {
    use super::*;

    // 初始化质押
    pub fn initialize_staking(
    ctx: Context<InitializeStaking>,  // 初始化质押上下文
    token_per_sec: u64,  // 每秒奖励的代币数量
    ) -> ProgramResult {
        Ok(())
    }

    // 初始化用户
    pub fn initialize_user(
        ctx: Context<InitializeUser>,  // 初始化用户上下文
    ) -> ProgramResult {
        Ok(())
    }

    // 进入质押
    pub fn enter_staking(
        ctx: Context<EnterStaking>,  // 进入质押上下文
    ) -> ProgramResult {
        Ok(())
    }

    // 取消质押
    pub fn cancel_staking(
        ctx: Context<CancelStaking>,  // 取消质押上下文
        staking_instance_bump: u8,  // 质押实例的 bump
    ) -> ProgramResult {
        Ok(())
    }

    // 领取奖励
    pub fn claim_rewards(
        ctx: Context<ClaimRewards>,  // 领取奖励上下文
        amount: u64,  // 领取的奖励数量
        staking_instance_bump: u8,  // 质押实例的 bump
    ) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
