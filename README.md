
# Cross Contract Call with app.burrow.cash

A contract in Rust to do Cross Contract Call for deposit , increase collateral and burrowing stable coin (USDT)


## Installation



```bash
  git clone https://github.com/SuperBatata/Burrow-Light.git
  cd Burrow-Light
```
## Running Tests

To run tests, run the following command :

Make a deposit :

```bash
    
near call burrow_l.testnet make_deposit_burrow '{"receiver_id":"contract.1638481328.burrow.testnet", "amount":"2000000000000000000000000","msg":""}' --gas=300000000000000 --accountId=YOUR_ACCOUNT_ID --deposit=0.1


```

Increase Collateral :

```bash
    
near call burrow_l.testnet increase_colateral '{"actions":[{"IncreaseCollateral":{"token_id":"wrap.testnet"} }]}' --gas=300000000000000 --accountId=YOUR_ACCOUNT_ID


```

Make Burrow :

```bash
    
near call burrow_l.testnet make_burrow '{"receiver_id":"contract.1638481328.burrow.testnet", "msg": "{\"Execute\":{\"actions\":[{\"Borrow\":{\"token_id\":\"usdt.fakes.testnet\",\"amount\":\"10000000000000000000\"}},{\"Withdraw\":{\"token_id\":\"usdt.fakes.testnet\",\"max_amount\":\"10000000000000000000\"}}]}}"}' --gas=300000000000000 --accountId=YOUR_ACCOUNT_ID


```
