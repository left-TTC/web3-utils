use pyth_solana_receiver_sdk::price_update::{PriceFeedMessage, PriceUpdateV2, VerificationLevel};
use solana_program::{msg, program_error::ProgramError, pubkey::Pubkey};



pub struct OriginSolanaPriceUpdateV2(pub PriceUpdateV2);

impl OriginSolanaPriceUpdateV2 {
    
    pub fn new(data: &[u8]) -> Result<OriginSolanaPriceUpdateV2, ProgramError> {
        
        const TAG: usize = 8;
        const WRITE_AUTH_LEN: usize = 32;
        const VERIFICATION_LEVEL_LEN: usize = 2;
        const PRICE_MESSAGE_LEN: usize = 84;  //32 + 8 + 8 + 4 + 8 + 8 + 8 + 8
        const POSTED_SLOT_LEN: usize = 8;

        let need = TAG + WRITE_AUTH_LEN + VERIFICATION_LEVEL_LEN + PRICE_MESSAGE_LEN + POSTED_SLOT_LEN;
        if data.len() < need {
            msg!("construct length err");
            return Err(ProgramError::InvalidAccountData);
        }
        msg!("length ok");

        let mut offset = 8;

        let write_authority = Pubkey::new_from_array(
            data[offset..offset + WRITE_AUTH_LEN].try_into().unwrap(),
        );
        offset += WRITE_AUTH_LEN;

        msg!("if there is not error, means verification_level error");

        let ver_bytes: [u8; VERIFICATION_LEVEL_LEN] = data[offset..offset + VERIFICATION_LEVEL_LEN]
            .try_into()
            .unwrap();
        let verification_level = match ver_bytes[0] {
            0 => VerificationLevel::Partial { num_signatures: ver_bytes[1] },
            1 => VerificationLevel::Full,
            _ => return Err(ProgramError::InvalidAccountData),
        };
        offset += VERIFICATION_LEVEL_LEN;

        // construct the price message

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

        let posted_slot = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());

        let inner = PriceUpdateV2 {
            write_authority,
            verification_level,
            price_message,
            posted_slot,
        };

        Ok(OriginSolanaPriceUpdateV2(inner))
    }
}