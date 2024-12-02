use anchor_lang::prelude::*; // 导入Anchor框架的预导入模块
use anchor_spl::token::TokenAccount; // 导入TokenAccount类型
use super::{ // 引入当前模块中定义的其他结构体
   StakingInstance,
   User,
};
use anchor_spl::token::Mint; // 导入Mint类型

#[derive(Accounts)]
#[instruction(
   amount: u64, // 领取的奖励数量
   staking_instance_bump: u8, // 质押实例的种子bump值，用于生成唯一的地址
   _staking_user_bump: u8, // 用户实例的种子bump值，用于生成唯一的地址
)]
pub struct ClaimRewards<'info> {
   #[account(signer)]
   pub authority: AccountInfo<'info>, // 签名者的账户信息，通常是用户的身份，用于确认操作的合法性
   #[account(
       mut, // 表示该账户可能被修改
       constraint = reward_token_mint.mint_authority.unwrap().eq(&staking_instance.key()) // 确保奖励代币的mint权限属于质押实例
   )]
   pub reward_token_mint: Box<Account<'info, Mint>>, // 奖励代币的Mint账户，用于指定奖励代币的类型
   #[account(
       mut, // 表示该账户可能被修改
       associated_token::mint = reward_token_mint, // 关联的Token Mint
       associated_token::authority = authority, // 关联的Token账户所有者
   )]
   pub reward_token_authority_wallet: Box<Account<'info, TokenAccount>>, // 奖励代币接收者的钱包账户
   #[account(
       mut, // 表示该账户可能被修改
       seeds = [crate::STAKING_SEED.as_ref(), staking_instance.authority.as_ref()], // 用于生成质押实例账户地址的种子
       bump = staking_instance_bump, // 生成质押实例账户地址的bump值
   )]
   pub staking_instance: Box<Account<'info, StakingInstance>>, // 质押实例账户，存储质押相关的全局信息
   #[account(
       mut, // 表示该账户可能被修改
       seeds = [
           crate::USER_SEED.as_ref(), // 用户种子
           staking_instance.key().as_ref(), // 质押实例的公钥
           authority.key().as_ref() // 签名者的公钥
       ],
       bump = _staking_user_bump, // 生成用户实例账户地址的bump值
   )]
   pub user_instance: Box<Account<'info, User>>, // 用户实例账户，存储用户的质押信息
   #[account(
       constraint = token_program.key() == crate::TOKEN_PROGRAM_BYTES.parse::<Pubkey>().unwrap(), // 确保指定的程序是Token程序
   )]
   pub token_program: AccountInfo<'info>, // Token程序的账户信息
   pub system_program: Program<'info, System>, // Solana系统程序，用于系统级操作如账户创建
   pub rent: AccountInfo<'info>, // 租金账户信息，用于账户的租金计算
   pub time: Sysvar<'info, Clock>, // 时钟系统变量，用于获取当前时间
}