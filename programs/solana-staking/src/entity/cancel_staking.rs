use anchor_lang::prelude::*; // 导入 Anchor 框架的预导入模块
use anchor_spl::token::{
    Mint,
    TokenAccount
}; // 导入 TokenAccount 类型，用于表示 SPL 代币账户
use crate::base::{ // 引入当前模块中定义的其他结构体
    StakingInstance,
    User,
 };

#[derive(Accounts)]
#[instruction(
   staking_instance_bump: u8, // 质押实例的种子bump值
   _staking_user_bump: u8, // 用户实例的种子bump值
)]
pub struct CancelStaking<'info> {
   #[account(mut)]
   pub authority: Signer<'info>, // 签名者的账户信息，通常是用户，用于验证操作的合法性
   #[account(
       mut, // 表示该账户可能被修改
       constraint = reward_token_mint.mint_authority.unwrap().eq(&staking_instance.key())
       // 确保奖励代币的铸造权限属于质押实例
   )]
   pub reward_token_mint: Box<Account<'info, Mint>>, // 奖励代币的铸造账户，用于指定奖励代币的类型
   #[account(mut)]
   pub nft_token_mint: Box<Account<'info, Mint>>, // NFT代币的铸造账户，用于指定NFT代币的类型
   #[account(
       constraint = nft_token_metadata.owner == &nft_program_id.key()
       // 确保NFT元数据账户的所有者是NFT程序
   )]
   pub nft_token_metadata: AccountInfo<'info>, // NFT元数据账户，存储与NFT相关的元数据
   #[account(
       mut, // 表示该账户可能被修改
       constraint = nft_token_authority_wallet
        .clone().into_inner().deref().owner == authority.key(),
       // 确保NFT代币的持有者是操作的签名者
       constraint = nft_token_authority_wallet
       .clone().into_inner().deref().mint == nft_token_mint.key()
       // 确保钱包中的代币是指定的NFT代币
   )]
   pub nft_token_authority_wallet: Box<Account<'info, TokenAccount>>, // NFT代币持有者的钱包账户
   #[account(
       mut, // 表示该账户可能被修改
       constraint = nft_token_program_wallet
       .clone().into_inner().deref().owner == staking_instance.key(),
       // 确保NFT代币的程序钱包所有者是质押实例
       constraint = nft_token_program_wallet
       .clone().into_inner().deref().mint == nft_token_mint.key()
       // 确保程序钱包中的代币是指定的NFT代币
   )]
   pub nft_token_program_wallet: Box<Account<'info, TokenAccount>>, // 存储NFT代币的程序钱包
   #[account(
       mut,
       seeds = [crate::STAKING_SEED.as_ref(), staking_instance.authority.as_ref()],
       bump = staking_instance_bump, // 生成质押实例账户地址的bump值
   )]
   pub staking_instance: Account<'info, StakingInstance>, // 质押实例账户，包含质押相关的全局信息
   #[account(
       mut, 
       seeds = [
           crate::USER_SEED.as_ref(), // 用户种子
           staking_instance.key().as_ref(), // 质押实例的公钥
           authority.key().as_ref() // 签名者的公钥
       ],
       bump = _staking_user_bump, // 生成用户实例账户地址的bump值
   )]
   pub user_instance: Account<'info, User>, // 用户实例账户，存储用户的质押信息
   #[account(
       constraint = allowed_collection_address.key()
           == staking_instance.allowed_collection_address,
       // 确保允许的NFT集合地址与质押实例中存储的地址一致
   )]
   pub allowed_collection_address: AccountInfo<'info>, // 允许的NFT集合的账户地址
   #[account(
       constraint = 
           token_program.key() == crate::TOKEN_PROGRAM_BYTES.parse::<Pubkey>().unwrap(),
       // 确保指定的程序是Token程序
   )]
   pub token_program: AccountInfo<'info>, // Token程序的账户信息
   #[account(
       constraint = 
           nft_program_id.key() == 
           crate::NFT_TOKEN_PROGRAM_BYTES.parse::<Pubkey>().unwrap(),
       // 确保指定的程序是NFT程序
   )]
   pub nft_program_id: AccountInfo<'info>, // NFT程序的账户信息
   pub system_program: Program<'info, System>, // Solana系统程序，用于系统级操作如账户创建
   pub rent: AccountInfo<'info>, // 租金账户信息，用于账户的租金计算
   pub time: Sysvar<'info, Clock>, // 时钟系统变量，用于获取当前时间
}