use anchor_lang::prelude::*;  // 导入Anchor框架的预导入模块
use super::{  // 引入当前模块中定义的其他结构体
   StakingInstance,
   User,
};

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
       bump = _staking_user_bump,  // 生成用户实例账户地址的bump值
       payer = authority,  // 为创建该账户支付费用的账户
   )]
   pub user_instance: Box<Account<'info, User>>,  // 用户实例账户，存储用户的质押信息
   #[account(
       mut,  // 表示该账户可能被修改
       seeds = [crate::STAKING_SEED.as_ref(), staking_instance.authority.as_ref()],  // 用于生成质押实例账户地址的种子
       bump = _staking_instance_bump,  // 生成质押实例账户地址的bump值
   )]
   pub staking_instance: Account<'info, StakingInstance>,  // 质押实例账户，存储质押相关的全局信息
   pub system_program: Program<'info, System>,  // Solana系统程序，用于创建账户等系统级操作
   pub rent: AccountInfo<'info>,  // 租金账户信息，用于账户的租金计算
   pub time: Sysvar<'info, Clock>,  // 时钟系统变量，用于获取当前时间
}