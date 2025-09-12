use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

pub trait BorshSize: BorshDeserialize + BorshSerialize {
    fn borsh_len(&self) -> usize;
}

impl BorshSize for () {
    fn borsh_len(&self) -> usize {
        0
    }
}

impl BorshSize for u8 {
    fn borsh_len(&self) -> usize {
        1
    }
}

impl BorshSize for u16 {
    fn borsh_len(&self) -> usize {
        2
    }
}

impl BorshSize for u32 {
    fn borsh_len(&self) -> usize {
        4
    }
}

impl BorshSize for u64 {
    fn borsh_len(&self) -> usize {
        8
    }
}

impl BorshSize for u128 {
    fn borsh_len(&self) -> usize {
        16
    }
}

impl BorshSize for i8 {
    fn borsh_len(&self) -> usize {
        1
    }
}

impl BorshSize for i16 {
    fn borsh_len(&self) -> usize {
        2
    }
}

impl BorshSize for i32 {
    fn borsh_len(&self) -> usize {
        4
    }
}

impl BorshSize for i64 {
    fn borsh_len(&self) -> usize {
        8
    }
}

impl BorshSize for i128 {
    fn borsh_len(&self) -> usize {
        16
    }
}

impl BorshSize for bool {
    fn borsh_len(&self) -> usize {
        1
    }
}

impl BorshSize for Pubkey {
    fn borsh_len(&self) -> usize {
        32
    }
}

impl BorshSize for String {
    fn borsh_len(&self) -> usize {
        4 + self.len()
    }
}

impl<T: BorshSize> BorshSize for Option<T> {
    fn borsh_len(&self) -> usize {
        match self.as_ref() {
            Some(a) => 1 + a.borsh_len(),
            None => 1,
        }
    }
}

impl<T: BorshSize> BorshSize for Vec<T> {
    fn borsh_len(&self) -> usize {
        if self.is_empty() {
            4
        } else {
            4 + self.len() * self[0].borsh_len()
        }
    }
}