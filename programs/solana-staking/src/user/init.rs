use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};  // 导入Anchor框架的预导入模块

use crate::stake::StakingContext;

use super::UserContext;

#[derive(Accounts)]
#[instruction(
   _staking_instance_bump: u8,  // 质押实例的种子bump值，用于生成唯一的地址
   _staking_user_bump: u8,  // 用户实例的种子bump值，用于生成唯一的地址
)]
pub struct InitializeUser<'info> {
   #[account(mut)]
   pub authority: Signer<'info>,  // 签名者，通常是用户的身份
   #[account(
       init,  // 表示该账户是初始化的账户
       seeds = [  // 用于生成用户实例账户地址的种子
           crate::USER_SEED.as_ref(),  // 用户种子
           staking_instance.key().as_ref(),  // 质押实例的公钥
           authority.key().as_ref()  // 签名者的公钥
       ],
       bump,  // 生成用户实例账户地址的bump值
       payer = authority,  // 为创建该账户支付费用的账户
       space = 8 * 100
   )]
   pub user_instance: Box<Account<'info, UserContext>>,  // 用户实例账户，存储用户的质押信息
   #[account(
       mut,  // 表示该账户可能被修改
       seeds = [crate::STAKING_SEED.as_ref(), staking_instance.authority.as_ref()],  // 用于生成质押实例账户地址的种子
       bump = _staking_instance_bump,  // 生成质押实例账户地址的bump值
   )]
   pub staking_instance: Account<'info, StakingContext>,  // 质押实例账户，存储质押相关的全局信息
   pub system_program: Program<'info, System>,  // Solana系统程序，用于创建账户等系统级操作
   pub rent: AccountInfo<'info>,  // 租金账户信息，用于账户的租金计算
   pub time: Sysvar<'info, Clock>,  // 时钟系统变量，用于获取当前时间
}

pub fn init_user(
    ctx: Context<InitializeUser>,  // 初始化用户上下文
) -> ProgramResult {
    let user_instance = &mut ctx.accounts.user_instance;  // 获取用户实例
    user_instance.deposited_amount = 0;  // 初始化存入数量
    user_instance.reward_debt = 0;  // 初始化奖励债务
    user_instance.accumulated_reward = 0;  // 初始化累计奖励
    Ok(())
}