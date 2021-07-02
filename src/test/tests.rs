use super::*;
use near_sdk::MockedBlockchain;
use near_sdk::{testing_env, VMContext};
#[test]
fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
    VMContext {
        current_account_id: "alice_near".to_string(),
        signer_account_id: "bob_near".to_string(),
        signer_account_pk: vec![0, 1, 2],
        predecessor_account_id: "carol_near".to_string(),
        input,
        block_index: 0,
        block_timestamp: 0,
        account_balance: 0,
        account_locked_balance: 0,
        storage_usage: 0,
        attached_deposit: 0,
        prepaid_gas: 10u64.pow(18),
        random_seed: vec![0, 1, 2],
        is_view,
        output_data_receivers: vec![],
        epoch_height: 0,
    }
}


#[test]
fn test_new_delivery() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let contract = ColdChain::default();
    assert!(contract.new_delivery('{"temp_c": -4.0, "payment_account_id": "alan1.testnet"}'));
}

#[test]
 /// Reset to zero.
 pub fn reset(&mut self) {
    self.val = 0;
    // Another way to log is to cast a string into bytes, hence "b" below:
    env::log(b"Reset counter to zero");

#[test]

pub fn decrement(&mut self) {
    // note: subtracting one like this is an easy way to accidentally overflow
    // real smart contracts will want to have safety checks
    // e.g. self.val = i8::wrapping_sub(self.val, 1);
    // https://doc.rust-lang.org/std/primitive.i8.html#method.wrapping_sub
    self.val -= 1;
    let log_message = format!("Decreased number to {}", self.val);
    env::log(log_message.as_bytes());
    after_counter_change();

#[test]
 //Verify how much balance is in the contract
 pub fn get_balance(&self) -> Option<Balance> {
    return Some(env::account_balance());
}
#[test]
pub fn get_temp_c(&self, account_id: String) -> Option<f32> {
    return self.temp_c.get(&account_id);
}
#[test]
pub fn get_payment_account_id(&self) -> String {
    return self.payment_account_id.to_string();
}
#[test]
pub fn get_initialized(&self) -> bool {
    return self.initialized;
