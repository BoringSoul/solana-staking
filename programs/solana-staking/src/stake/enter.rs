use anchor_lang::prelude::*; use anchor_lang::solana_program::entrypoint::ProgramResult;
// 导入 Anchor 框架的预导入模块
use anchor_spl::token::{
    self, Mint, TokenAccount, Transfer
}; // 导入 TokenAccount 类型，用于表示 SPL 代币账户
use crate::stake::context::StakingContext;
use crate::user::context::UserContext;

#[derive(Accounts)]
#[instruction(
   _staking_instance_bump: u8, // 质押实例的种子bump值
   _staking_user_bump: u8, // 用户实例的种子bump值
)]
pub struct EnterStaking<'info> {
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
        .clone().into_inner().owner == authority.key(),
       // 确保NFT代币的持有者是操作的签名者
       constraint = nft_token_authority_wallet
       .clone().into_inner().mint == nft_token_mint.key()
       // 确保钱包中的代币是指定的NFT代币
   )]
   pub nft_token_authority_wallet: Box<Account<'info, TokenAccount>>, // NFT代币持有者的钱包账户
   #[account(
       mut, // 表示该账户可能被修改
       constraint = nft_token_program_wallet
       .clone().into_inner().owner == staking_instance.key(),
       // 确保NFT代币的程序钱包所有者是质押实例
       constraint = nft_token_program_wallet
       .clone().into_inner().mint == nft_token_mint.key()
       // 确保程序钱包中的代币是指定的NFT代币
   )]
   pub nft_token_program_wallet: Box<Account<'info, TokenAccount>>, // 存储NFT代币的程序钱包
   #[account(
       mut,
       seeds = [crate::constants::STAKING_SEED.as_ref(),staking_instance.authority.as_ref()],
       bump = _staking_instance_bump, // 生成质押实例账户地址的bump值
   )]
   pub staking_instance: Account<'info, StakingContext>, // 质押实例账户，包含质押相关的全局信息
   #[account(
       mut,
       seeds = [
           crate::constants::USER_SEED.as_ref(), // 用户种子
           staking_instance.key().as_ref(), // 质押实例的公钥
           authority.key().as_ref() // 签名者的公钥
       ],
       bump = _staking_user_bump, // 生成用户实例账户地址的bump值
   )]
   pub user_instance: Account<'info, UserContext>, // 用户实例账户，存储用户的质押信息
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
   pub time: Sysvar<'info,Clock>, // 时钟系统变量，用于获取当前时间
}

pub fn enter_staking(
    ctx: Context<EnterStaking>,  // 进入质押上下文
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
    crate::reward::pool::update_reward_pool(
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
    crate::reward::debt::update_reward_debt(
        staking_instance,
        user_instance,
    );
    Ok(())
}