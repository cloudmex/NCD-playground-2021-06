mod test;

//New info is being saved in the contract
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::collections::Vector;
use near_sdk::json_types::{Base58PublicKey, U128};
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, Balance, EpochHeight, Promise, PromiseResult,
    PublicKey, 
};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ColdChain {
    //Information to be saved Truck ID, Temp (ºF and ºC), Truck plate, Fuel liters, Current Location, timestamp
    temp_c: LookupMap<String, f32>,
    payment_account_id: String,
    initialized: bool,
    // i8 is signed. unsigned integers are also available: u8, u16, u32, u64, u128
    val: u8, 
}

impl Default for ColdChain {
    fn default() -> Self {
        Self {
            temp_c: LookupMap::new(b"t_c".to_vec()),
            payment_account_id: env::current_account_id(),
            initialized: false,
            val: 0,
        }
    }
}


#[near_bindgen]
impl ColdChain {

    #[payable] //You pay 10 NEARS for a delivery
    pub fn new_delivery(&mut self, temp_c: f32, payment_account_id: String){

        assert!(
            !self.initialized, //check that contract is not initialized
            "Contract is alredy initialized, finish the travel to "
        );
        let amount = env::attached_deposit();
        //the id of who is calling the contract
        let signer_account_id = env::signer_account_id();
        let log_signer_account_id = format!("Signer Account ID {}", &signer_account_id );
        env::log(log_signer_account_id.as_bytes());

        // The id of the account that was the previous contract in the chain of cross-contract calls.
        let predecessor_account_id = env::predecessor_account_id();
        let log_predecessor_account_id = format!("Predecessor Account ID {}", &predecessor_account_id );
        env::log(log_predecessor_account_id.as_bytes());
        
        // The id of the account that owns the current contract.
        let current_account_id = env::current_account_id(); 
        let log_current_account_id = format!("Current Account ID {}", &current_account_id );
        env::log(log_current_account_id.as_bytes());

        assert_ne!(
            signer_account_id, 
            current_account_id, 
            "Contract cannot initialize a delivery by it's self"
        );
        assert_eq!(
            amount, 
             //10 NEARs required for delivery
            10000000000000000000000000, //How can this number by smaller? something as nears_2_yocto() and yocto_2_nears()
            "Payment for rent truck"
        );

        assert_eq!(
            self.val, //Current location
            0,
            "Delivery needs to be in origin"
        );
        self.payment_account_id = payment_account_id;
        self.initialized = true;
        self.temp_c.insert(&current_account_id, &temp_c);
    }
    /*
    //This method is experimental
    fn set_payment_account_id(&mut self, payment_account_id: AccountId) {
        let account_id = env::signer_account_id();

        let log_message = format!("New status {}", &payment_account_id);
        env::log(log_message.as_bytes());
        self.payment_account_id.insert(&account_id, &message);
    }
    */

    //In each arrival the temp_c is saved
    //Looking if we can define a range of temp_c
    pub fn new_arrival(&mut self, temp_c: f32 ){

        assert!(
            self.initialized, //check if the contract is initialized
            "Contract is not initialized"
        );
        assert!(
            self.val < 4, //Current location vs Destiny location
            "Delivery its in the last destiny, "
        );
        self.increment();
        let account_id = env::signer_account_id();
        self.temp_c.insert(&account_id, &temp_c);
    }


    fn increment(&mut self) {
        // note: adding one like this is an easy way to accidentally overflow
        // real smart contracts will want to have safety checks
        // e.g. self.val = i8::wrapping_add(self.val, 1);
        // https://doc.rust-lang.org/std/primitive.i8.html#method.wrapping_add
        self.val += 1;
        let log_message = format!("Current location {}", self.val);
        env::log(log_message.as_bytes());
        after_counter_change();
    }


    pub fn get_location(&self) -> u8 {
        return self.val;
    }


    pub fn withdraw(&mut self) {
    /*
        assert!(
            self.initialized, //Current location vs Destiny location
            "There is no payment for cold chain deliverry "
        );
     */    
        assert_eq!(
            self.val, //Current location
            4,
            "Truck needs to be in final destiny"
        );
        let signer_account_id = env::signer_account_id();
        let current_account_id = env::current_account_id(); 


        assert_ne!(
            signer_account_id, 
            current_account_id, 
            "Contract cannot initialize a delivery by it's self"
        );
        /*
       assert!(
            amount > self.get_balance(), //Current location vs Destiny location
            "Not enough payment for the contract"
        );
        */
        let account_id = env::signer_account_id();
        //let account_id = env::current_account_id();
        //let mut account_id = &self.payment_account_id;
        let amount = env::account_balance();
        let log_account = format!("Account ID {}", &account_id);
        env::log(log_account.as_bytes());

        let log_amount = format!("Amount {}", &amount);
        env::log(log_amount.as_bytes());

        self.reset();
        self.initialized = false;
        let amount_payment: Balance = 10000000000000000000000000;
        Promise::new(self.payment_account_id.to_string()).transfer(amount_payment);

    }
    //Verify how much balance is in the contract
    pub fn get_balance(&self) -> Option<Balance> {
        return Some(env::account_balance());
    }

    pub fn get_temp_c(&self, account_id: String) -> Option<f32> {
        return self.temp_c.get(&account_id);
    }

    pub fn get_payment_account_id(&self) -> String {
        return self.payment_account_id.to_string();
    }

    pub fn get_initialized(&self) -> bool {
        return self.initialized;
    }

    ///This methods are just for TESTING PURPOSES, 
    //IF YOU DONT NEED THEM YOU SHOULD COMMENT OUT 
    //THE METHODS decrement() and reset()

    pub fn decrement(&mut self) {
        // note: subtracting one like this is an easy way to accidentally overflow
        // real smart contracts will want to have safety checks
        // e.g. self.val = i8::wrapping_sub(self.val, 1);
        // https://doc.rust-lang.org/std/primitive.i8.html#method.wrapping_sub
        self.val -= 1;
        let log_message = format!("Decreased number to {}", self.val);
        env::log(log_message.as_bytes());
        after_counter_change();
    }

    /// Reset to zero.
    pub fn reset(&mut self) {
        self.val = 0;
        // Another way to log is to cast a string into bytes, hence "b" below:
        env::log(b"Reset counter to zero");
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

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
    
    
   /* #[test]
    fn test_new_delivery() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let contract = ColdChain::default();
        assert!(contract.new_delivery('{"temp_c": -4.0, "payment_account_id": "alan1.testnet"}'));
    }

    #[test]
    fn test_get_initialized(){
        let context = get_context(vec![], false);
        testing_env!(context);
        let contract = ColdChain::default();
        assert_eq!(false,contract.get_initialized());
    }

    #[test]
    fn test_new_delivery(){
        let context = get_context(vec![], false);
        testing_env!(context);
        let contract = ColdChain::default();
        assert_eq!(false,contract.new_delivery());
    }

    #[test]
    fn test_new_arrival(){
        let context = get_context(vec![], false);
        testing_env!(context);
        let contract = ColdChain::default();
        assert_eq!(false,contract.new_arrival());
    }

    #[test]
    fn test_withdraw(){
        let context = get_context(vec![], false);
        testing_env!(context);
        let contract = ColdChain::default();
        assert_eq!(false,contract.withdraw());
    }
    */

}

// unlike the struct's functions above, this function cannot use attributes #[derive(…)] or #[near_bindgen]
// any attempts will throw helpful warnings upon 'cargo build'
// while this function cannot be invoked directly on the blockchain, it can be called from an invoked function
fn after_counter_change() {
    // show helpful warning that i8 (8-bit signed integer) will overflow above 127 or below -128
    env::log("Make sure you don't overflow, my friend.".as_bytes());
}
