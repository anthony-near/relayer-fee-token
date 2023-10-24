// generated using https://near.org/contractwizard.near/widget/ContractWizardUI

use near_sdk::{
    AccountId, PanicOnDefault, borsh::{BorshDeserialize, BorshSerialize, self},
    env, json_types::U128, near_bindgen,
};
use near_sdk_contract_tools::{
    FungibleToken, hook::Hook, Owner, owner::*, standard::nep141::*, Nep145
};

#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault, Owner, FungibleToken, Nep145)]
#[nep145(force_unregister_hook = "ForceUnregisterHook")]
#[fungible_token(name = "Relayer Fee Token", symbol = "RFT", decimals = 24, no_hooks)]
#[near_bindgen]
pub struct Contract {
    pub storage: LookupMap<AccountId, Vec<u64>>,
}

pub struct ForceUnregisterHook;

impl<'a> Hook<Contract, Nep145ForceUnregister<'a>> for ForceUnregisterHook {
    fn before(_contract: &Contract, _args: &Nep145ForceUnregister<'a>) -> Self {
        Self
    }

    fn after(_contract: &mut Contract, _args: &Nep145ForceUnregister<'a>, _state: Self) {
        log!("After force unregister");
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        let mut contract = Self {
            storage: LookupMap::new(b"s"),
        };

        Nep145Controller::set_storage_balance_bounds(
            &mut contract,
            &StorageBalanceBounds {
                min: U128(0),
                max: None,
            },
        );

        Nep141Controller::mint(&mut contract, "relayer-fee-token.testnet".parse().unwrap(), 1000000000, None);

        contract
    }

    pub fn use_storage(&mut self, num: u64) {
        let storage_usage_start = env::storage_usage();

        let predecessor = env::predecessor_account_id();

        self.storage.insert(predecessor.clone(), (0..num).collect());

        self.storage.flush();

        let storage_usage = env::storage_usage() - storage_usage_start;
        let storage_fee = env::storage_byte_cost() * storage_usage as u128;

        Nep145Controller::lock_storage(self, &predecessor, storage_fee.into())
            .unwrap_or_else(|e| env::panic_str(&format!("Storage lock error: {}", e)));
    }

    pub fn mint(&mut self, account_id: AccountId, amount: U128) {
        Nep141Controller::mint(self, account_id, amount.into(), None);
    }

    pub fn burn(&mut self, amount: U128) {
        Nep141Controller::burn(self, env::predecessor_account_id(), amount.into(), None);
    }
}
