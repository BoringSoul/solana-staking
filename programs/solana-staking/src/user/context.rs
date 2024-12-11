use anchor_lang::prelude::*;
#[account]
#[derive(Copy, Default)]
pub struct UserContext {
   pub deposited_amount: u64,  // 用户存入的代币数量
   pub reward_debt: u64,  // 用户的奖励债务
   pub accumulated_reward: u64,  // 用户的累积奖励
}