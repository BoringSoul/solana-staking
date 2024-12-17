use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use stake::*;
use user::*;
use reward::*;

pub mod constants;
pub mod stake;
pub mod user;
pub mod reward;

declare_id!("GKdPktyTt2dVCRw7Yfw7kJKUrhXFFT5wgxRqCWDnf3dV");

#[program]
pub mod solana_staking {

    use super::*;

    // 初始化质押
    pub fn init_staking(
    ctx: Context<InitializeStaking>,  // 初始化质押上下文
    token_per_sec: u64,  // 每秒奖励的代币数量
    ) -> ProgramResult {
        stake::init::init_staking(
            ctx,
            token_per_sec,
        )
    }

    // 初始化用户
    pub fn init_user(
        ctx: Context<InitializeUser>,  // 初始化用户上下文
    ) -> ProgramResult {
        user::init::init_user(ctx)
    }

    // 进入质押
    pub fn enter_staking(
        ctx: Context<EnterStaking>,  // 进入质押上下文
    ) -> ProgramResult {
        stake::enter::enter_staking(ctx)
    }

    // 取消质押
    pub fn cancel_staking(
        ctx: Context<CancelStaking>,  // 取消质押上下文
        staking_instance_bump: u8,  // 质押实例的 bump
    ) -> ProgramResult {
        stake::canel::cancel_staking(ctx, staking_instance_bump)
    }

    // 领取奖励
    pub fn claim_rewards(
        ctx: Context<ClaimRewards>,  // 领取奖励上下文
        amount: u64,  // 领取的奖励数量
        staking_instance_bump: u8,  // 质押实例的 bump
    ) -> ProgramResult {
        reward::claim::claim_rewards(
            ctx,
            amount,
            staking_instance_bump,
        )
    }
}