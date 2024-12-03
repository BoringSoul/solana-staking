use anchor_lang::prelude::*;
use crate::base::StakingInstance;  // 引入定义在同一模块中的StakingInstance结构体
use anchor_spl::token::Mint;  

// 定义InitializeStaking结构体，用于初始化质押
#[derive(Accounts)]
#[instruction(
   token_per_sec: u64,  // 每秒奖励的代币数量
   _staking_instance_bump: u8,  // 质押实例的 bump
)]

pub struct InitializeStaking<'info> {
   #[account(mut)]
   pub authority: Signer<'info>,  
   #[account(
       mut,  
       constraint = reward_token_mint  // 添加约束条件
           .mint_authority  // 约束mint_authority字段
           .unwrap() == staking_instance.key(),  // 确保 mint_authority与staking_instance的公钥相等
   )]
   pub reward_token_mint: Box<Account<'info, Mint>>,  // 声明reward_token_mint账户类型为Box<Account<'info, Mint>>
   #[account(  // 声明staking_instance账户的初始化及约束条件
       init,  // 声明staking_instance账户需要初始化
       seeds = [crate::STAKING_SEED.as_ref(), authority.key().as_ref()],  // 指定seeds参数，用于创建PDA（Program Derived Address）
       bump,  // 指定bump参数，用于创建PDA
       //space = 8 + core::mem::size_of::<StakingInstance>(),  // 为staking_instance账户分配空间（此行被注释掉）
       payer = authority,  // 声明authority账户为支付者
   )]
   pub staking_instance: Account<'info, StakingInstance>,  // 声明staking_instance账户类型为Account<'info, StakingInstance>
   pub allowed_collection_address: AccountInfo<'info>,  // 声明allowed_collection_address账户类型为AccountInfo<'info>
   pub system_program: Program<'info, System>,  // 声明system_program账户类型为Program<'info, System>
   pub rent: AccountInfo<'info>,  // 声明rent账户类型为AccountInfo<'info>
   pub time: Sysvar<'info, Clock>,  // 声明time账户类型为Sysvar<'info, Clock>，用于获取当前时间
}
