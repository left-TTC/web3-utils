use borsh::{BorshDeserialize, BorshSerialize};
use bytemuck::{bytes_of, NoUninit};
use solana_program::{instruction::{AccountMeta, Instruction}, pubkey::Pubkey};

use crate::{borsh_size::BorshSize, wrapped_pod::WrappedPod};

pub trait InstructionsAccount {
    fn get_accounts_vec(&self) -> Vec<AccountMeta>;

    fn get_instruction<P: BorshDeserialize + BorshSerialize + BorshSize>(
        &self,
        program_id: Pubkey,
        instruction_id: u8,
        params: P,
    ) -> Instruction {
        let cap = 1 + params.borsh_len();
        let mut data = Vec::with_capacity(cap);
        #[allow(clippy::uninit_vec)]
        unsafe {
            data.set_len(cap);
        }
        data[0] = instruction_id;
        let mut data_pointer = &mut data[1..];
        params.serialize(&mut data_pointer).unwrap();
        // We check that we have written to the whole buffer to ensure that no undefined bytes remain at the end of data.
        if !data_pointer.is_empty() {
            panic!()
        }

        let accounts_vec = self.get_accounts_vec();
        Instruction {
            program_id,
            accounts: accounts_vec,
            data,
        }
    }
    fn get_instruction_cast<P: NoUninit>(
        &self,
        program_id: Pubkey,
        instruction_id: u8,
        params: P,
    ) -> Instruction {
        let cap = 8 + std::mem::size_of::<P>();
        let mut data = Vec::with_capacity(cap);
        data.push(instruction_id);
        data.extend([0; 7].iter());
        data.extend(bytes_of(&params));

        Instruction {
            program_id,
            accounts: self.get_accounts_vec(),
            data,
        }
    }
    fn get_instruction_wrapped_pod<'a, P: WrappedPod<'a>>(
        &self,
        program_id: Pubkey,
        instruction_id: u8,
        params: P,
    ) -> Instruction {
        let cap = 8 + params.size();
        let mut data = Vec::with_capacity(cap);
        data.push(instruction_id);
        data.extend([0; 7].iter());
        params.export(&mut data);

        Instruction {
            program_id,
            accounts: self.get_accounts_vec(),
            data,
        }
    }
}