use pyth_solana_receiver_sdk::price_update::{PriceFeedMessage, PriceUpdateV2, VerificationLevel};
use solana_program::{msg, program_error::ProgramError, pubkey::Pubkey};


pub struct OriginSolanaPriceUpdateV2(pub PriceUpdateV2);

impl OriginSolanaPriceUpdateV2 {
    pub fn new(data: &[u8]) -> Result<OriginSolanaPriceUpdateV2, ProgramError> {
        const TAG: usize = 8;
        let mut offset = TAG;

        // 1. write_authority
        let write_authority = Pubkey::new_from_array(
            data[offset..offset + 32].try_into().map_err(|_| ProgramError::InvalidAccountData)?,
        );
        offset += 32;

        // 2. verification_level (dynamic size)
        let variant = data[offset];
        offset += 1;
        let verification_level = match variant {
            0 => {
                let num = data[offset];
                offset += 1;
                VerificationLevel::Partial { num_signatures: num }
            }
            1 => VerificationLevel::Full,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        // 3. price_message
        let feed_id: [u8; 32] = data[offset..offset + 32].try_into().unwrap();
        offset += 32;

        let price = i64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 8;

        let conf = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 8;

        let exponent = i32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
        offset += 4;

        let publish_time = i64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 8;

        let prev_publish_time = i64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 8;

        let ema_price = i64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 8;

        let ema_conf = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 8;

        let price_message = PriceFeedMessage {
            feed_id,
            price,
            conf,
            exponent,
            publish_time,
            prev_publish_time,
            ema_price,
            ema_conf,
        };

        // 4. posted_slot
        let posted_slot = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 8;

        if offset > data.len() {
            msg!("data length not enough");
            return Err(ProgramError::InvalidAccountData);
        }

        let inner = PriceUpdateV2 {
            write_authority,
            verification_level,
            price_message,
            posted_slot,
        };

        Ok(OriginSolanaPriceUpdateV2(inner))
    }
}
