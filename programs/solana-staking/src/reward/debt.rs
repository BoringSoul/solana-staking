use crate::{StakingContext, UserContext};


// 更新用户奖励债务
pub fn update_reward_debt(
    staking_instance: &mut StakingContext,  // 质押实例
    user_instance: &mut UserContext,  // 用户实例
) {
    // 计算并更新用户奖励债务
    user_instance.reward_debt = user_instance.deposited_amount
        .checked_mul(staking_instance.accumulated_reward_per_share)
        .unwrap()
        .checked_div(COMPUTATION_DECIMALS)
        .unwrap();
}