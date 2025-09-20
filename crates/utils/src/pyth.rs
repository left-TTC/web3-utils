use solana_program::{
    account_info::AccountInfo,
    clock::Clock,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    pubkey,
};
use pyth_solana_receiver_sdk;
use std::convert::TryInto;

use crate::{
    check::{check_account_key},
    price_update::OriginSolanaPriceUpdateV2,
};

pub const PYTH_SOL_USD_FEED: Pubkey = pubkey!("7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE");

pub const PRICE_FEED_DISCRIMATOR: [u8; 8] = [34, 241, 35, 99, 157, 126, 244, 205];

pub fn parse_price(data: &[u8]) -> Result<OriginSolanaPriceUpdateV2, ProgramError> {
    let suffix = &data[..8];
    if suffix != PRICE_FEED_DISCRIMATOR {
        msg!("discrimator err");
        return Err(ProgramError::InvalidArgument);
    }
    Ok(OriginSolanaPriceUpdateV2::new(data)?)
}

/// 获取 Pyth SOL/USD 价格，返回的是 FP32 定点数
pub fn get_oracle_price_fp32(
    account: &AccountInfo,
    clock: &Clock,
    maximum_age: u64,
) -> Result<u128, ProgramError> {
    check_account_key(account, &PYTH_SOL_USD_FEED)?;
    let data = &account.data.borrow();
    let update = parse_price(data)?;

    let actual_feed_id = update.0.price_message.feed_id;

    let pyth_solana_receiver_sdk::price_update::Price { price, exponent, .. } =
        update.0
            .get_price_no_older_than(clock, maximum_age, &actual_feed_id)
            .map_err(|e| {
                msg!("pyth error: {:?}", e);
                ProgramError::InvalidArgument
            })?;

    let raw_price = price as i128;
    let fp32_price = if exponent < 0 {
        (raw_price << 32) / 10i128.pow((-exponent) as u32)
    } else {
        (raw_price << 32) * 10i128.pow(exponent as u32)
    };

    Ok(fp32_price as u128)
}

/// 输入：域名价格 (单位 USD，对标 lamports) 
/// 输出：对应 lamports (u64)
pub fn get_domain_price_sol(
    domain_price_usd: u64,
    sol_pyth_feed_account: &AccountInfo,
    clock: &Clock,
) -> Result<u64, ProgramError> {
    #[cfg(feature = "devnet")]
    let query_deviation = 600_000;
    #[cfg(not(feature = "devnet"))]
    let query_deviation = 60;

    let sol_price_fp32 = get_oracle_price_fp32(sol_pyth_feed_account, clock, query_deviation)?;

    // lamports = (domain_usd << 32) / (sol_usd_price_fp32 / 1e9)
    // (domain_usd * 1e9) / sol_price(USD)
    let usd_amount = domain_price_usd / 1_000_000;

    let lamports = ((usd_amount as u128) << 32)
        .checked_mul(1_000_000_000u128)
        .ok_or(ProgramError::InvalidArgument)?
        / sol_price_fp32;

    msg!("{:?} usd = {:?} lamports", domain_price_usd, lamports);

    Ok(lamports.try_into().map_err(|_| ProgramError::InvalidArgument)?)
}
