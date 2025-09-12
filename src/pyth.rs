use solana_program::{
    account_info::AccountInfo,
    clock::Clock,
    msg,
    program_error::ProgramError,
    pubkey::{Pubkey},
    pubkey,
    sysvar::Sysvar,
};
use pyth_solana_receiver_sdk::{
    self,
    price_update::Price,
};
use std::convert::TryInto;

use crate::{check::{check_account_key}, price_update::OriginSolanaPriceUpdateV2};

pub const SOL_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

pub const PYTH_SOL_USD_FEED: Pubkey = pubkey!("7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE");

pub const PRICE_FEED_DISCRIMATOR: [u8; 8] = [34, 241, 35, 99, 157, 126, 244, 205];

pub const PYTH_PRICE_FEED: [u8; 32] = [
    239, 13, 139, 111, 218, 44, 235, 164, 29, 161, 93, 64, 149, 209, 218, 57, 42, 13,
    47, 142, 208, 198, 199, 188, 15, 76, 250, 200, 194, 128, 181, 109,
];

pub fn parse_price(data: &[u8]) -> Result<OriginSolanaPriceUpdateV2, ProgramError> {
    // now the pyth accounts are anchor account
    let suffix = &data[..8];
    if suffix != PRICE_FEED_DISCRIMATOR {
        return Err(ProgramError::InvalidArgument);
    }
    let update = OriginSolanaPriceUpdateV2::new(data)?;

    Ok(update)
}
 
pub fn get_oracle_price_fp32_v2(
    account: &AccountInfo,
    clock: &Clock,
    maximum_age: u64,
) -> Result<u64, ProgramError> {
    check_account_key(account, &PYTH_SOL_USD_FEED)?;

    let data = &account.data.borrow();
    let update = parse_price(data)?;

    let Price { price, exponent, .. } = update.0
        .get_price_no_older_than(clock, maximum_age, &PYTH_PRICE_FEED)
        .map_err(|_| ProgramError::InvalidArgument)?;

    let price = if exponent > 0 {
        ((price as u128) << 32) * 10u128.pow(exponent as u32)
    } else {
        ((price as u128) << 32) / 10u128.pow((-exponent) as u32)
    };

    let corrected_price = (price * 10u128.pow(6)) / 10u128.pow(9);

    let final_price: u64 = corrected_price
        .try_into()
        .map_err(|_| ProgramError::InvalidArgument)?;

    msg!("Pyth SOL/USD FP32 price: {:?}", final_price);

    Ok(final_price)
}


pub fn get_domain_price_sol(
    domain_price_usd: u64,
    sol_pyth_feed_account: AccountInfo,
) -> Result<u64, ProgramError> {

    let clock = Clock::get()
        .map_err(|_| ProgramError::InvalidArgument)?;

    #[cfg(feature="devnet")]
    let query_deviation = 6000;
    #[cfg(not(feature="devnet"))]
    let query_deviation = 60;

    let sol_price = get_oracle_price_fp32_v2(
        &sol_pyth_feed_account, &clock, query_deviation)
        .map_err(|_| ProgramError::InvalidArgument)?;

    Ok(domain_price_usd * sol_price)
}