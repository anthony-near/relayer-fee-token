// generated using https://near.org/contractwizard.near/widget/ContractWizardUI

use near_sdk::{
    AccountId, PanicOnDefault, borsh::{BorshDeserialize, BorshSerialize, self},
    env, json_types::U128, near_bindgen,
};
use near_sdk_contract_tools::{
    FungibleToken, Owner, owner::*, standard::nep141::*
};

#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault, Owner, FungibleToken)]
#[fungible_token(name = "Relayer Fee Token", symbol = "RFT", decimals = 24, no_hooks)]
#[near_bindgen]
pub struct Contract {}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        let mut contract = Self {};

        // nomnomnom.testnet is contract owner and gets initial supply of 1B tokens
        Owner::init(&mut contract, &"nomnomnom.testnet".parse().unwrap());
        Nep141Controller::mint(&mut contract, "nomnomnom.testnet".parse().unwrap(), 1000000000u128, None);

        contract
    }

    pub fn mint(&mut self, account_id: AccountId, amount: U128) {
        <Self as Owner>::require_owner();
        Nep141Controller::mint(self, account_id, amount.into(), None);
    }

    pub fn burn(&mut self, amount: U128) {
        Nep141Controller::burn(self, env::predecessor_account_id(), amount.into(), None);
    }
}
