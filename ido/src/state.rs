use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
// use serde::{Deserialize, Serialize};

#[derive(
    BorshSerialize, BorshDeserialize, BorshSchema, Debug, PartialEq, Clone,
)]
pub struct AuctionBuffer {
    pub auction_id: u32,
    pub success_flag: u8,                     //是否筹集成功 //0:fail, 1:success
    pub access: u8,                           //公有，私有池 0:public, 1:private
    pub allocation_date: u32,                 // 提取时间
    pub ratio: u32,                           //比例
    pub tokens_offered: u64,                  //筹款上限
    pub funds_raised: u64,                    //已筹款资金
    pub min_allocation: u64,                  //最小参与额度
    pub max_allocation: u64,                  //最大参与额度
    pub pool_opens: u32,                      //开始参与时间
    pub pool_closes: u32,                     //结束参与时间
    pub min_swap_level: u64,                  //最小筹款数量
    pub spl_program_id: [u8; 32],             // 项目方币的地址
    pub spl_program_id_derive: [u8; 32],      // 项目方钱包对他们自己币的派z生地址 给我们发币的地址
    pub underlay_program_id: [u8; 32],        // 募集币的地址
    pub underlay_program_id_derive: [u8; 32], // 项目方募集币的派生地址
    pub poolname: [u8; 32],                   //池子名字
    pub whitelist: Vec<[u8; 32]>,             //白名单
}

#[derive(
    BorshSerialize, BorshDeserialize, BorshSchema, Debug,  PartialEq, Clone,
)]
pub struct AccountData {
    pub contribution: u64, // 累计submit的spl-tokens数量
    pub withdrawn: u64,    // 累计已提取的spl-token或者项目方代币(成功或者失败)
}
