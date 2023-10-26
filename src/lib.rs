// generated using https://near.org/contractwizard.near/widget/ContractWizardUI

use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
    json_types::{Base64VecU8, U128},
    near_bindgen,
    store::Vector,
    PanicOnDefault,
};
use near_sdk_contract_tools::ft::*;

#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault, FungibleToken)]
#[near_bindgen]
pub struct Contract {
    blobs: Vector<Vec<u8>>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        let mut contract = Self {
            blobs: Vector::new(b"b"),
        };

        contract.set_metadata(&FungibleTokenMetadata::new(
            "Relayer Fee Token".to_string(),
            "RFT".to_string(),
            24,
        ));

        contract
    }

    pub fn mint(&mut self, amount: U128) {
        Nep141Controller::mint(
            self,
            &Nep141Mint {
                amount: amount.into(),
                receiver_id: &env::predecessor_account_id(),
                memo: None,
            },
        ).unwrap();
    }

    pub fn use_storage(&mut self, blob: Base64VecU8) {
        let storage_start = env::storage_usage();
        let blob = blob.into();
        self.blobs.push(blob);
        self.blobs.flush();
        let storage_end = env::storage_usage();
        self.lock_storage(
            &env::predecessor_account_id(),
            ((storage_end - storage_start) as u128 * env::storage_byte_cost()).into(),
        ).unwrap_or_else(|e| env::panic_str(&format!("Storage lock error: {}", e)));
    }
}
