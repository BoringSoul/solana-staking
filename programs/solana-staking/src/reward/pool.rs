use crate::{StakingContext, UserContext};


// 更新奖励池
pub fn update_reward_pool(
    current_timestamp: u64,  // 当前时间戳
    staking_instance: &mut StakingContext,  // 质押实例
    #[allow(unused_variables)]  // 允许未使用变量
    user_instance: &mut UserContext,  // 用户实例
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
