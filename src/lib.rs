use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use near_sdk::{
    env, ext_contract, log, near_bindgen, AccountId, Gas, PanicOnDefault, Promise, PromiseResult,
};
use std::str;

pub const TGAS: u64 = 1_000_000_000_000;
pub const NO_DEPOSIT: u128 = 0;
pub const XCC_SUCCESS: u64 = 1;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub potato_token: AccountId,
}

// cross call contract to get metadata from potato token
#[ext_contract(ext_ft)]
pub trait PotatoToken {
    #[payable]

    fn ft_metadata(&mut self) -> String;

    fn query_greeting_callback(&mut self) -> String;
    fn ft_transfer_call(&mut self, amount: String, msg: String, receiver_id: String);
    fn make_deposit_callback(&mut self) -> bool;
}

#[near_bindgen]
impl Contract {
    #[init]

    pub fn new(potato_token: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            potato_token: potato_token,
        }
    }

    pub fn query_metadata(&self) -> Promise {
        assert!(
            env::prepaid_gas() >= Gas::from(20 * TGAS),
            "Please attach at least 20 TGAS"
        );
        let promise =
            ext_ft::ft_metadata(self.potato_token.clone(), NO_DEPOSIT, Gas::from(5 * TGAS));

        return promise.then(ext_ft::query_greeting_callback(
            env::current_account_id(),
            NO_DEPOSIT,
            Gas::from(5 * TGAS),
        ));
    }

    #[private] // Public - but only callable by env::current_account_id()
    pub fn query_greeting_callback(&self) -> String {
        // Get response, return "" if failed
        let ft_potato_metadata: String = match env::promise_result(0) {
            PromiseResult::Successful(value) => str::from_utf8(&value).unwrap().to_string(),
            _ => {
                log!("There was an error contacting Hello NEAR");
                return "".to_string();
            }
        };

        return ft_potato_metadata.to_string();
    }

    pub fn make_deposit_burrow(
        &mut self,
        amount: String,
        receiver_id: AccountId,
        msg: String,
    ) -> Promise {
        assert!(
            env::prepaid_gas() >= Gas::from(20 * TGAS),
            "Please attach at least 20 TGAS"
        );

        let promise = ext_ft::ft_transfer_call(
            (&mut self.potato_token).to_string(),
            amount,
            msg,
            receiver_id,
            NO_DEPOSIT,
            Gas::from(5 * TGAS),
        );

        // Create a callback change_greeting_callback
        return promise.then(ext_ft::make_deposit_callback(
            env::current_account_id(),
            NO_DEPOSIT,
            Gas::from(10 * TGAS),
        ));
    }

    #[private] // Public - but only callable by env::current_account_id()
    pub fn make_deposit_callback(&self) -> bool {
        let deposit_burrow: String = match env::promise_result(0) {
            PromiseResult::Successful(value) => str::from_utf8(&value).unwrap().to_string(),
            _ => {
                log!("There was an error contacting Hello NEAR");
                return false;
            }
        };

        return deposit_burrow.to_string() == "true".to_string();
    }


    pub fn hello (&mut self) -> String {
        return "Hello world".to_string();
    }
}
