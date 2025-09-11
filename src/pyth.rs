
use solana_program::{
    clock::Clock, msg, program_error::ProgramError, pubkey::Pubkey, sysvar::Sysvar
};
use pyth_sdk_solana::{
    state::{
        load_price_account, SolanaPriceAccount
    },
    Price,
};


pub fn get_oracle_price_fp32(
    account_data: &[u8],
    base_decimals: u8,
    quote_decimals: u8,
) -> Result<u64, ProgramError> {
    let price_account: &SolanaPriceAccount = load_price_account(account_data)?;
    let price_feed = price_account.to_price_feed(&Pubkey::default());

    let currentg_time = Clock::get().unwrap().unix_timestamp;
    let Price { price, expo, .. } = price_feed
    .get_price_no_older_than(currentg_time, 60)
    .ok_or_else(|| {
        msg!("Cannot parse pyth price, information unavailable.");
        ProgramError::InvalidAccountData
    })?;

    let scaling = 10u128
        .checked_pow(expo.unsigned_abs() as u32)
        .ok_or(ProgramError::InvalidArgument)?;

    let price_fp32 = if expo >= 0 {
        ((price as i128) as u128)
            .checked_shl(32)
            .and_then(|v| v.checked_mul(scaling))
            .ok_or(ProgramError::InvalidArgument)?
    } else {
        ((price as i128) as u128)
            .checked_shl(32)
            .and_then(|v| v.checked_div(scaling))
            .ok_or(ProgramError::InvalidArgument)?
    };

    let corrected_price = price_fp32
        .checked_mul(10u128.pow(quote_decimals as u32))
        .and_then(|v| v.checked_div(10u128.pow(base_decimals as u32)))
        .ok_or(ProgramError::InvalidArgument)?;

    let final_price: u64 = corrected_price
        .try_into()
        .map_err(|_| ProgramError::InvalidArgument)?;

    msg!("Pyth FP32 price value: {:?}", final_price);

    Ok(final_price)
}


// pub fn sol_from_usd(
    
// ) -> Result<u64, ProgramError> {

// }