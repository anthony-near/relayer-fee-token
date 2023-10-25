// generated using https://near.org/contractwizard.near/widget/ContractWizardUI

use near_sdk::{
    AccountId, PanicOnDefault, borsh::{BorshDeserialize, BorshSerialize, self},
    env, json_types::{Base64VecU8, U128}, near_bindgen, store::Vector,
};
use near_sdk_contract_tools::{
    FungibleToken, hook::Hook, Owner, standard::nep141::*, ft::*
};

#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault, Owner, FungibleToken)]
#[fungible_token(transfer_hook = "TransferHook")]
#[near_bindgen]
pub struct Contract {
    blobs: Vector<Vec<u8>>,
}

pub struct TransferHook;

impl Hook<Contract, Nep141Transfer<'_>> for TransferHook {
    fn hook<R>(
        contract: &mut Contract,
        args: &Nep141Transfer<'_>,
        f: impl FnOnce(&mut Contract) -> R,
    ) -> R {
        contract.require_registration(args.receiver_id);
        f(contract)
    }
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

        Nep141Controller::mint(&mut contract, "relayer-fee-token.testnet".parse().unwrap(), 1000000000, None);

        contract
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
        )
            .unwrap_or_else(|e| env::panic_str(&format!("Storage lock error: {}", e)));
    }

    fn require_registration(&self, account_id: &AccountId) {
        self.get_storage_balance(account_id)
            .unwrap_or_else(|e| env::panic_str(&e.to_string()));
    }

    pub fn mint(&mut self, account_id: AccountId, amount: U128) {
        Nep141Controller::mint(self, account_id, amount.into(), None);
    }

    pub fn burn(&mut self, amount: U128) {
        Nep141Controller::burn(self, env::predecessor_account_id(), amount.into(), None);
    }
}
