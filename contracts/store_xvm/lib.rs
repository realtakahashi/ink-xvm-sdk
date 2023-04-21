#![cfg_attr(not(feature = "std"), no_std)]

/// EVM ID (from astar runtime)
const EVM_ID: u8 = 0x0F;

#[ink::contract(env = xvm_environment::XvmDefaultEnvironment)]
mod store_xvm {
    use ethabi::{
        ethereum_types::{
            H160,
            U256,
        },
        Token,
    };
    use hex_literal::hex;
    use ink::prelude::vec::Vec;

    const STORE_SELECTOR: [u8; 4] = hex!["6057361d"];

    #[ink(storage)]
    pub struct StoreXvm {
        evm_address: [u8; 20],
    }

    impl StoreXvm {
        #[ink(constructor)]
        pub fn new(evm_address: [u8; 20]) -> Self {
            Self { evm_address }
        }

        #[ink(message)]
        pub fn store(&mut self, value: u128) -> bool {
            let encoded_input = Self::store_encode( value.into());
            self.env()
                .extension()
                .xvm_call(
                    super::EVM_ID,
                    Vec::from(self.evm_address.as_ref()),
                    encoded_input,
                )
                .is_ok()
        }

        fn store_encode(value: U256) -> Vec<u8> {
            let mut encoded = STORE_SELECTOR.to_vec();
            let input = [Token::Uint(value)];
            encoded.extend(&ethabi::encode(&input));
            encoded
        }

    }

}
