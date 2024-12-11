use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult}; // 导入Anchor框架的预导入模块
use anchor_spl::token::{
    self, Mint, MintTo, TokenAccount
}; // 导入TokenAccount类型
use crate::stake::StakingContext;
use crate::user::UserContext;

use super::{pending, pool, debt};

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
   pub staking_instance: Box<Account<'info, StakingContext>>, // 质押实例账户，存储质押相关的全局信息
   #[account(
       mut, // 表示该账户可能被修改
       seeds = [
           crate::USER_SEED.as_ref(), // 用户种子
           staking_instance.key().as_ref(), // 质押实例的公钥
           authority.key().as_ref() // 签名者的公钥
       ],
       bump = _staking_user_bump, // 生成用户实例账户地址的bump值
   )]
   pub user_instance: Box<Account<'info, UserContext>>, // 用户实例账户，存储用户的质押信息
   #[account(
       constraint = token_program.key() == crate::TOKEN_PROGRAM_BYTES.parse::<Pubkey>().unwrap(), // 确保指定的程序是Token程序
   )]
   pub token_program: AccountInfo<'info>, // Token程序的账户信息
   pub system_program: Program<'info, System>, // Solana系统程序，用于系统级操作如账户创建
   pub rent: AccountInfo<'info>, // 租金账户信息，用于账户的租金计算
   pub time: Sysvar<'info, Clock>, // 时钟系统变量，用于获取当前时间
}

// 领取奖励
pub fn claim_rewards(
    ctx: Context<ClaimRewards>,  // 领取奖励上下文
    amount: u64,  // 领取的奖励数量
    staking_instance_bump: u8,  // 质押实例的 bump
) -> ProgramResult {
    let staking_instance = &mut ctx.accounts.staking_instance;  // 获取质押实例
    let user_instance = &mut ctx.accounts.user_instance;  // 获取用户实例
    let current_timestamp = ctx.accounts.time.unix_timestamp as u64;  // 获取当前时间戳
    pool::update_reward_pool(
        current_timestamp,
        staking_instance,
        user_instance,
    );
    pending::store_pending_reward(
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
    debt::update_reward_debt(
        staking_instance,
        user_instance,
    );
    Ok(())
}