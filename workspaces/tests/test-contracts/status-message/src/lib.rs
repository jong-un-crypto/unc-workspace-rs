use unc_sdk::borsh::{BorshDeserialize, BorshSerialize};
use unc_sdk::{env, log, unc_bindgen, AccountId};
use std::collections::HashMap;

#[unc_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
#[borsh(crate = "unc_sdk::borsh")]
pub struct StatusMessage {
    records: HashMap<AccountId, String>,
}

#[unc_bindgen]
impl StatusMessage {
    #[payable]
    pub fn set_status(&mut self, message: String) {
        let account_id = env::signer_account_id();
        log!("{} set_status with message {}", account_id, message);
        self.records.insert(account_id, message);
    }

    pub fn get_status(&self, account_id: AccountId) -> Option<String> {
        log!("get_status for account_id {}", account_id);
        self.records.get(&account_id).cloned()
    }
}
