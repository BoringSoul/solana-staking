use anchor_lang::prelude::*;

#[account]
#[derive(Copy, Default)]
pub struct StakingInstance {
   pub authority: Pubkey,  // 质押的授权公钥
   pub reward_token_per_sec: u64,  // 每秒奖励的代币数量
   pub reward_token_mint: Pubkey,  // 奖励代币的mint地址
   pub allowed_collection_address: Pubkey,  // 允许的NFT集合地址
   pub accumulated_reward_per_share: u64,  // 每份奖励的累积量
   pub last_reward_timestamp: u64,  // 上次奖励时间戳
   pub total_shares: u64,  // 总份额
}


#[account]
#[derive(Copy, Default)]
pub struct User {
   pub deposited_amount: u64,  // 用户存入的代币数量
   pub reward_debt: u64,  // 用户的奖励债务
   pub accumulated_reward: u64,  // 用户的累积奖励
}


// 更新奖励池
pub fn update_reward_pool(
    current_timestamp: u64,  // 当前时间戳
    staking_instance: &mut StakingInstance,  // 质押实例
    #[allow(unused_variables)]  // 允许未使用变量
    user_instance: &mut User,  // 用户实例
) {
    // 计算从上次更新奖励到现在的时间内产生的奖励收入
    let income = staking_instance.reward_token_per_sec
        .checked_mul(current_timestamp
        .checked_sub(staking_instance.last_reward_timestamp)
        .unwrap())
        .unwrap();
    // 更新每股的累计奖励
    staking_instance.accumulated_reward_per_share =
        staking_instance.accumulated_reward_per_share
        .checked_add(income.checked_mul(COMPUTATION_DECIMALS).unwrap()
        .checked_div(staking_instance.total_shares)
        .unwrap_or(0))
        .unwrap();
    staking_instance.last_reward_timestamp = current_timestamp;  // 更新最后奖励时间戳
}

// 存储待领取的奖励
pub fn store_pending_reward(
    staking_instance: &mut StakingInstance,  // 质押实例
    user_instance: &mut User,  // 用户实例
) {
    // 计算并更新用户累计奖励
    user_instance.accumulated_reward = user_instance.accumulated_reward
        .checked_add(user_instance.deposited_amount
        .checked_mul(staking_instance.accumulated_reward_per_share)
        .unwrap()
        .checked_div(COMPUTATION_DECIMALS)
        .unwrap()
        .checked_sub(user_instance.reward_debt)
        .unwrap())
        .unwrap();
}

// 更新用户奖励债务
pub fn update_reward_debt(
    staking_instance: &mut StakingInstance,  // 质押实例
    user_instance: &mut User,  // 用户实例
) {
    // 计算并更新用户奖励债务
    user_instance.reward_debt = user_instance.deposited_amount
        .checked_mul(staking_instance.accumulated_reward_per_share)
        .unwrap()
        .checked_div(COMPUTATION_DECIMALS)
        .unwrap();
}