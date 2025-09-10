use solana_program::{
    program_error::ProgramError, msg
};







// pub fn get_oracle_price_fp32(
//     account_data: &[u8],
//     base_decimals: u8,
//     quote_decimals: u8,
// ) -> Result<u64, ProgramError> {
//     #[cfg(feature = "mock-oracle")]
//     {
//         // Mock testing oracle
//         if account_data.len() == 8 {
//             return Ok(u64::from_le_bytes(account_data[0..8].try_into().unwrap()));
//         }
//     };

//     // Pyth Oracle
//     let price_account = load_price_account(account_data)?;
//     let Price { price, expo, .. } = price_account
//         .to_price_feed(&Pubkey::default())
//         .get_current_price()
//         .ok_or_else(|| {
//             msg!("Cannot parse pyth price, information unavailable.");
//             ProgramError::InvalidAccountData
//         })?;
//     let price = if expo > 0 {
//         ((price as u128) << 32) * 10u128.pow(expo as u32)
//     } else {
//         ((price as u128) << 32) / 10u128.pow((-expo) as u32)
//     };

//     let corrected_price =
//         (price * 10u128.pow(quote_decimals as u32)) / 10u128.pow(base_decimals as u32);

//     let final_price = corrected_price.try_into().unwrap();

//     msg!("Pyth FP32 price value: {:?}", final_price);

//     Ok(final_price)
// }


// pub fn sol_from_usd(

// ) -> Result<u64, ProgramError> {

// }