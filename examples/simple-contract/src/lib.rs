use unc_sdk::borsh::{BorshDeserialize, BorshSerialize};
use unc_sdk::{env, log, unc_bindgen};

#[unc_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
#[borsh(crate = "unc_sdk::borsh")]
pub struct Simple {}

#[unc_bindgen]
impl Simple {
    pub fn current_env_data() -> (u64, u64) {
        let now = env::block_timestamp();
        let eh = env::epoch_height();
        log!("Timestamp: {}", now);
        (now, eh)
    }
}
