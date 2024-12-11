use anchor_lang::prelude::*;
#[account]
#[derive(Copy, Default)]
pub struct StakingContext {
   pub authority: Pubkey,  // 质押的授权公钥
   pub reward_token_per_sec: u64,  // 每秒奖励的代币数量
   pub reward_token_mint: Pubkey,  // 奖励代币的mint地址
   pub allowed_collection_address: Pubkey,  // 允许的NFT集合地址
   pub accumulated_reward_per_share: u64,  // 每份奖励的累积量
   pub last_reward_timestamp: u64,  // 上次奖励时间戳
   pub total_shares: u64,  // 总份额
}
