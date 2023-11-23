# Kujira ICA test

```sh
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.12.13

bash test/setup_ibc.sh

kujirad tx wasm store ./artifacts/icatest_controller.wasm --from validator --gas auto --gas-adjustment 1.3 -y --output json --home $HOME/.kujirad --keyring-backend test --chain-id kujira

kujirad tx wasm instantiate 1 '{}' --from validator --label "ibc" --gas auto --gas-adjustment 1.3 --no-admin -y --output json  --home $HOME/.kujirad --keyring-backend test --chain-id kujira

CONTRACT=kujira14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sl4e867

kujirad tx wasm execute $CONTRACT '{"create_account":{"tx_id":1,"conn_id":"connection-0","acc_id":"1","version":""}}' --from validator --gas auto --gas-adjustment 1.3 -y --output json  --home $HOME/.kujirad --keyring-backend test --chain-id kujira

# kujirad tx wasm execute $CONTRACT '{"create_account":{"conn_id":"connection-0","acc_id":"1","version":"ics27-1"}}' --from validator --gas auto --gas-adjustment 1.3 -y --output json  --home $HOME/.kujirad --keyring-backend test --chain-id kujira

kujirad query wasm contract-state smart $CONTRACT '{"account":{"conn_id":"connection-0","acc_id":"1"}}'

kujirad query wasm contract-state smart $CONTRACT '{"ica_callback":{"tx_id":1}}'

ICA_ADDRESS=terra1e8t35pqen4f5exzznwet829r7vd82k3emsa8aq9es5kd2dyn726qurq7dq

VALIDATOR="validator"
COUNTER_CHAIN_ID="terra"
COUNTER_MONIKER="terra"
COUNTER_HOME="$HOME/.terrad"
COUNTER_BINARY="terrad --home=$COUNTER_HOME"
COUNTER_TX_FLAGS="--keyring-backend test --chain-id $COUNTER_CHAIN_ID --from $VALIDATOR -y --fees=1000uluna"
$COUNTER_BINARY tx bank send $VALIDATOR $ICA_ADDRESS 10000000uluna $COUNTER_TX_FLAGS

# $COUNTER_BINARY query bank balances $ICA_ADDRESS

# $COUNTER_BINARY query interchain-accounts host params
# $COUNTER_BINARY query staking validators
COUNTER_VAL_ADDRESS=terravaloper1nea2gprul95vglryw9zxsm02hfm28s4p05ptg4

# send ica delegate tx
kujirad tx wasm execute $CONTRACT '{"send_delegate_tx":{"tx_id":2,"conn_id":"connection-0","acc_id":"1","validator":"'$COUNTER_VAL_ADDRESS'","amount":{"denom":"uluna", "amount":"1000000"}}}' --from validator --gas auto --gas-adjustment 1.3 -y --output json  --home $HOME/.kujirad --keyring-backend test --chain-id kujira

kujirad query wasm contract-state smart $CONTRACT '{"ica_callback":{"tx_id":2}}'
$COUNTER_BINARY query staking delegation $ICA_ADDRESS $COUNTER_VAL_ADDRESS --output=json
# {"delegation":{"delegator_address":"...","validator_address":"...","shares":"1000000.000000000000000000"},"balance":{"denom":"uluna","amount":"1000000"}}


kujirad tx wasm execute $CONTRACT '{"create_account":{"tx_id":3,"conn_id":"connection-0","acc_id":"2","version":""}}' --from validator --gas auto --gas-adjustment 1.3 -y --output json  --home $HOME/.kujirad --keyring-backend test --chain-id kujira

kujirad query wasm contract-state smart $CONTRACT '{"account":{"conn_id":"connection-0","acc_id":"2"}}'
ICA_ADDRESS=terra1s5dxemdxh0xcz33txwfd9qv6weaxr4qhuh7gyql0d5spuxqzzcfsa48k2z
$COUNTER_BINARY tx bank send $VALIDATOR $ICA_ADDRESS 10000000uluna $COUNTER_TX_FLAGS
# $COUNTER_BINARY query bank balances $ICA_ADDRESS

# send ica delegate tx
kujirad tx wasm execute $CONTRACT '{"send_delegate_tx":{"tx_id":4,"conn_id":"connection-0","acc_id":"2","validator":"'$COUNTER_VAL_ADDRESS'","amount":{"denom":"uluna", "amount":"1000000"}}}' --from validator --gas auto --gas-adjustment 1.3 -y --output json  --home $HOME/.kujirad --keyring-backend test --chain-id kujira

kujirad query wasm contract-state smart $CONTRACT '{"ica_callback":{"tx_id":4}}'

$COUNTER_BINARY query staking delegation $ICA_ADDRESS $COUNTER_VAL_ADDRESS --output=json
# {"delegation":{"delegator_address":"...","validator_address":"...","shares":"1000000.000000000000000000"},"balance":{"denom":"uluna","amount":"1000000"}}
```
