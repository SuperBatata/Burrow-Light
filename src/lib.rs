// use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
// use near_sdk::json_types::U128;
// use near_sdk::{
//     env, ext_contract, log, near_bindgen, AccountId, Gas, PanicOnDefault, Promise, PromiseResult,
// };

// pub const TGAS: u64 = 1_000_000_000_000;
// pub const NO_DEPOSIT: u128 = 0;
// pub const XCC_SUCCESS: u64 = 1;

// #[near_bindgen]
// #[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
// pub struct Contract {
//     pub hello_account: AccountId,
// }

// #[ext_contract(ext_ft)]
// pub trait PotatoToken {
//     fn ft_balance_of(&self, account_id: AccountId, amount: U128) -> Promise;
//     fn query_meta_callback(&mut self) -> String;
// }

// #[near_bindgen]
// impl Contract {
//     #[init]
//     #[private] // Public - but only callable by env::current_account_id()
//     pub fn new(hello_account: AccountId) -> Self {
//         assert!(!env::state_exists(), "Already initialized");
//         Self {
//             hello_account: hello_account,
//         }
//     }

//     // Public - query external greeting
//     // pub fn query_metadata(&self) -> Promise {
//     //     // Make sure there is enough GAS to execute the callback
//     //     assert!(
//     //         env::prepaid_gas() >= Gas::from(20 * TGAS),
//     //         "Please attach at least 20 TGAS"
//     //     );

//     //     // Create a promise to call HelloNEAR.get_greeting()
//     //     let promise =
//     //         potato_token::ft_metadata(self.hello_account.clone(), NO_DEPOSIT, Gas::from(5 * TGAS));

//     //     let callback_promise: Promise = potato_token::query_meta_callback(
//     //         env::current_account_id(),
//     //         NO_DEPOSIT,
//     //         Gas::from(5 * TGAS),
//     //     );
//     //     return promise.then(callback_promise);
//     // }

//     // #[private] // Public - but only callable by env::current_account_id()
//     // pub fn query_meta_callback(&self) -> String {
//     //     let metadaResult = match env::promise_result(0) {
//     //         PromiseResult::Successful(value) => {
//     //             near_sdk::serde_json::from_slice::<String>(&value).unwrap()
//     //         }
//     //         _ => {
//     //             log!("There was an error contacting Hello NEAR");
//     //             return "".to_string();
//     //         }
//     //     };

//     //     return metadaResult;
//     // }

//     pub fn my_callback(&self) -> String {
//         assert_eq!(env::promise_results_count(), 1, "This is a callback method");

//         // handle the result from the cross contract call this method is a callback for
//         match env::promise_result(0) {
//             PromiseResult::NotReady => unreachable!(),
//             PromiseResult::Failed => "oops!".to_string(),
//             PromiseResult::Successful(result) => {
//                 let balance = near_sdk::serde_json::from_slice::<U128>(&result).unwrap();
//                 if balance.0 > 100000 {
//                     "Wow!".to_string()
//                 } else {
//                     "Hmmmm".to_string()
//                 }
//             }
//         }
//     }

use std::result;

//     pub fn my_method(&self) -> Promise {
//         ext_ft::ft_balance_of(
//             "some_account_id.near".to_string(), // ft_balance_of takes an account_id as a parameter
//             &"potato_token.testnet",            // contract account id
//             0,                                  // yocto NEAR to attach
//             5_000_000_000_000,                  // gas to attach
//         )
//     }
// }
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde_json::json;
use near_sdk::{
    env, ext_contract, log, near_bindgen, AccountId, Balance, Gas, MethodMetadata, PanicOnDefault,
    Promise, PromiseResult,
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
}

#[near_bindgen]
impl Contract {
    #[init]
    #[private] // Public - but only callable by env::current_account_id()
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
}
