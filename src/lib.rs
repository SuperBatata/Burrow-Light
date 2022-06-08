use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use near_sdk::{
    env, ext_contract, log, near_bindgen, AccountId, Gas, PanicOnDefault, Promise, PromiseResult,
};
use std::convert::{TryFrom, TryInto};
use std::str::{self, FromStr};
pub const TGAS: u64 = 1_000_000_000_000;
pub const NO_DEPOSIT: u128 = 0;
pub const XCC_SUCCESS: u64 = 1;
pub const YOCTO_NEAR: u128 = 1_000_000_000_000_000_000_000_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub burrow_contract: AccountId,
    pub wrapper_contract: AccountId,
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

    pub fn new(burrow_contract: AccountId, wrapper_contract: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            burrow_contract: burrow_contract,
            wrapper_contract: wrapper_contract,
        }
    }

    #[payable]
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

        let account = "wrap.testnet";

        let promise = ext_ft::ft_transfer_call(
            amount,
            msg,
            receiver_id.to_string(),
            AccountId::from_str(account).unwrap(),
            1,
            Gas(50000000000000),
        );

        return promise;
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
}

// amount,
// msg,
// receiver_id.to_string(),
// self.wrapper_contract.clone(),
// YOCTO_NEAR,
// Gas::from(30 * TGAS),
