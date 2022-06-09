use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use near_sdk::{env, ext_contract, near_bindgen, AccountId, Gas, PanicOnDefault, Promise};
use serde_json::Value;

use std::str::{self, FromStr};
use std::vec;
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

#[ext_contract(ext_ft)]
pub trait PotatoToken {
    #[payable]
    fn ft_metadata(&mut self) -> String;
    fn query_greeting_callback(&mut self) -> String;
    fn ft_transfer_call(&mut self, amount: String, msg: String, receiver_id: String);
    fn make_deposit_callback(&mut self) -> bool;
    fn oracle_call(&mut self, receiver_id: String, msg: String);
}

#[ext_contract(ext_collateral)]

pub trait Collateral {
    fn execute(&mut self, actions: Vec<Value>);
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

    #[payable]

    pub fn increase_colateral(&mut self, actions: Vec<Value>) -> Promise {
        assert!(
            env::prepaid_gas() >= Gas::from(20 * TGAS),
            "Please attach at least 20 TGAS"
        );

        
        


        let account = "contract.1638481328.burrow.testnet";
        let promise = ext_collateral::execute(
            actions,
            AccountId::from_str(account).unwrap(),
            1,
            Gas(50000000000000),
        );

        return promise;
    }

    pub fn make_burrow(&mut self, msg: String, receiver_id: AccountId) -> Promise {
        let account = "priceoracle.testnet";
        let promise = ext_ft::oracle_call(
            receiver_id.to_string(),
            msg,
            AccountId::from_str(account).unwrap(),
            1,
            Gas(150000000000000),
        );
        return promise;
    }
}
