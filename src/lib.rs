//Based in https://github.com/near-examples/rust-status-message
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
    records: LookupMap<String, String>,
    truck_id: LookupMap<String, String>,
    temp_f: LookupMap<String, String>,
    temp_c: LookupMap<String, f32>,
    truck_plate: LookupMap<String, String>,
    fuel: LookupMap<String, String>,
    location: LookupMap<String, String>,
    date: LookupMap<String, String>,
    // i8 is signed. unsigned integers are also available: u8, u16, u32, u64, u128
    val: u8, 
}

impl Default for ColdChain {
    fn default() -> Self {
        Self {
            records: LookupMap::new(b"r".to_vec()),
            truck_id: LookupMap::new(b"t_f".to_vec()),
            temp_f: LookupMap::new(b"t_f".to_vec()),
            temp_c: LookupMap::new(b"t_c".to_vec()),
            truck_plate: LookupMap::new(b"t_p".to_vec()),
            fuel: LookupMap::new(b"f".to_vec()),
            location: LookupMap::new(b"l".to_vec()),
            date: LookupMap::new(b"d".to_vec()),
            val: 0,
        }
    }
}

#[near_bindgen]
impl ColdChain {

    #[payable] //You pay for a delivery
    pub fn new_delivery(&mut self, temp_c: f32 ){
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
        self.temp_c.insert(&current_account_id, &temp_c);
    }

    pub fn new_arrival(&mut self, temp_c: f32 ){
        assert!(
            self.val < 4, //Current location vs Destiny location
            "Truck come to it's last destiny, "
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
        let amount = env::account_balance();
        let log_account = format!("Account ID {}", &account_id);
        env::log(log_account.as_bytes());

        let log_amount = format!("Amount {}", &amount);
        env::log(log_amount.as_bytes());
        self.reset();
        let amount_payment: Balance = 10000000000000000000000000;
        Promise::new(account_id).transfer(amount_payment);

    }
    //Verify how much balance is in the contract
    pub fn get_balance(&self) -> Option<Balance> {
        return Some(env::account_balance());
    }

    pub fn get_temp_c(&self, account_id: String) -> Option<f32> {
        return self.temp_c.get(&account_id);
    }
    /*
    Not really functional to be inside the smart contract
    It will consume gas for doing a senseless reading
    pub fn get_location_verbose(&self) -> Option<String>  {
        /*0-Tepic, Nayarit (Origin)
        1-Guadalajara, Jalisco
        2-Aguascalientes, Aguascalientes
        3-Leon, Guanajuato
        4-Ciudad de México (Destiny)*/
        match self.val {
            // Match a single value
            0 => env::log(b"Tepic"),
            1 => env::log(b"Reset counter to zero"),
            2 => env::log(b"Reset counter to zero"),
            3 => env::log(b"Reset counter to zero"),
            4 => env::log(b"Reset counter to zero"),
        }
    }
    
    #[payable]
    pub fn set_status(&mut self, message: String) {
        let account_id = env::signer_account_id();

        let log_message = format!("New status {}", &message);
        env::log(log_message.as_bytes());
        self.records.insert(&account_id, &message);
    }

    pub fn get_status(&self, account_id: String) -> Option<String> {
        return self.records.get(&account_id);
    }
    pub fn set_date(&mut self, date: String) {
        let account_id = env::signer_account_id();
        self.date.insert(&account_id, &date);
    }

    pub fn get_date(&self, account_id: String) -> Option<String> {
        return self.date.get(&account_id);
    }
*/

    ///START COUNTING METHODS

    /// Increment the counter.
    ///
    /// Note, the parameter is "&mut self" as this function modifies state.
    /// In the frontend (/src/main.js) this is added to the "changeMethods" array
    /// using near-cli we can call this by:
    ///
    /// ```bash
    /// near call counter.YOU.testnet increment --accountId donation.YOU.testnet
    /// ```


    /// Decrement (subtract from) the counter.
    ///
    /// In (/src/main.js) this is also added to the "changeMethods" array
    /// using near-cli we can call this by:
    ///
    /// ```bash
    /// near call counter.YOU.testnet decrement --accountId donation.YOU.testnet
    /// ```
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
/*
    pub fn set_date(&mut self, date: String) {
        let account_id = env::signer_account_id();
        self.records.insert(&account_id, &date);
    }

    pub fn get_date(&self, account_id: String) -> Option<String> {
        return self.records.get(&account_id);
    }

    pub fn get_all_dates(&self, account_id: String) -> Option<String> {
        return self.date.get(&account_id);
    }
*/
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

    #[test]
    fn set_get_message() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = ColdChain::default();
        contract.set_status("hello".to_string());
        assert_eq!(
            "hello".to_string(),
            contract.get_status("bob_near".to_string()).unwrap()
        );
    }

    #[test]
    fn get_nonexistent_message() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = ColdChain::default();
        assert_eq!(None, contract.get_status("francis.near".to_string()));
    }
}

// unlike the struct's functions above, this function cannot use attributes #[derive(…)] or #[near_bindgen]
// any attempts will throw helpful warnings upon 'cargo build'
// while this function cannot be invoked directly on the blockchain, it can be called from an invoked function
fn after_counter_change() {
    // show helpful warning that i8 (8-bit signed integer) will overflow above 127 or below -128
    env::log("Make sure you don't overflow, my friend.".as_bytes());
}
