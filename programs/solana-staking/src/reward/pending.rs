use crate::{StakingContext, UserContext};


// 存储待领取的奖励
pub fn store_pending_reward(
    staking_instance: &mut StakingContext,  // 质押实例
    user_instance: &mut UserContext,  // 用户实例
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