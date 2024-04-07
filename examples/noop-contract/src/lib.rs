use unc_sdk::borsh::{BorshDeserialize, BorshSerialize};
use unc_sdk::unc_bindgen;

#[unc_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
#[borsh(crate = "unc_sdk::borsh")]
pub struct Noop;

#[unc_bindgen]
impl Noop {
    pub fn noop() {}
}
