# Kujira ICA test

Kujira supports custom wasmbinding and callbacks for interchain accounts.

## Prerequisites

Before starting, make sure you have [rustup](https://rustup.rs/) along with a
recent `rustc` and `cargo` version installed.

And you need to have the `wasm32-unknown-unknown` target installed as well.

You can check that via:

```sh
rustc --version
cargo --version
rustup target list --installed
# if wasm32 is not listed above, run this
rustup target add wasm32-unknown-unknown
```

## Build

```
cargo build --target wasm32-unknown-unknown --release
```

## Preparing the Wasm bytecode for production

Before we upload it to a chain, we need to ensure the smallest output size possible,
as this will be included in the body of a transaction. We also want to have a
reproducible build process, so third parties can verify that the uploaded Wasm
code did indeed come from the claimed rust code.

To solve both these issues, we have produced `rust-optimizer`, a docker image to
produce an extremely small build output in a consistent manner. The suggest way
to run it is this:

```sh
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.12.13
```

Or, If you're on an arm64 machine, you should use a docker image built with arm64.

```sh
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer-arm64:0.12.13
```

This produces an `artifacts` directory with a `PROJECT_NAME.wasm`, as well as
`checksums.txt`, containing the Sha256 hash of the wasm file.

## Interact with Kujira through CLI

### Setup IBC connected chains locally

1. Clone the [Kujira core](https://github.com/antstalepresh/core/tree/ibc_wasm_binding) and install the daemon using `make install`.
2. For counterparty chain, clone the [Terra core](https://github.com/terra-money/core/tree/v2.4.1) and install the daemon using `make install`.
3. For relayer, clone the [Go relayer](https://github.com/cosmos/relayer/tree/v2.4.2) and install the daemon using `make install`.
4. Once both are installed, run the following script for setting up IBC env automatically.

```
bash test/setup_ibc.sh
```

It should end up like this.

```
...
2023-10-12T10:05:22.866532Z	info	Successful transaction	{"provider_type": "cosmos", "chain_id": "terra", "gas_used": 167530, "fees": "3879uluna", "fee_payer": "terra1jy6td9r477fwr4q60adr7lz4anye5y89p5cq7q", "height": 8, "msg_types": ["/ibc.core.client.v1.MsgUpdateClient", "/ibc.core.connection.v1.MsgConnectionOpenTry"], "tx_hash": "21F0A9C6D938247CB35082F9240CCA8A44755479D2BD933B9BFFD0154BDFB550"}
2023-10-12T10:05:32.450954Z	info	Successful transaction	{"provider_type": "cosmos", "chain_id": "kujira", "gas_used": 146348, "fees": "3328ukuji", "fee_payer": "kujira1pqs8apaa94ejf2etsgv7fkdv6c69jv4l0q74gh", "height": 11, "msg_types": ["/ibc.core.client.v1.MsgUpdateClient", "/ibc.core.connection.v1.MsgConnectionOpenAck"], "tx_hash": "F9B7E8C8387FFEE8D9106CFD643A9DCD0884F1B5CE48A748A82655FDA2EC2151"}
2023-10-12T10:05:38.223788Z	info	Successful transaction	{"provider_type": "cosmos", "chain_id": "terra", "gas_used": 130011, "fees": "2903uluna", "fee_payer": "terra1jy6td9r477fwr4q60adr7lz4anye5y89p5cq7q", "height": 11, "msg_types": ["/ibc.core.client.v1.MsgUpdateClient", "/ibc.core.connection.v1.MsgConnectionOpenConfirm"], "tx_hash": "AD9DF7A3C3B85E1CDBB9D286F7C58AAB05E47F795E7C4A21FB553D58529D5F2A"}
2023-10-12T10:05:38.488771Z	info	Connection handshake termination candidate	{"path_name": "kujira-terra", "chain_id": "terra", "client_id": "07-tendermint-0", "termination_client_id": "07-tendermint-0", "observed_client_id": "07-tendermint-0", "termination_counterparty_client_id": "07-tendermint-0", "observed_counterparty_client_id": "07-tendermint-0"}
2023-10-12T10:05:38.488790Z	info	Found termination condition for connection handshake	{"path_name": "kujira-terra", "chain_id": "terra", "client_id": "07-tendermint-0"}
2023-10-12T10:05:38.498030Z	info	Starting event processor for channel handshake	{"src_chain_id": "kujira", "src_port_id": "transfer", "dst_chain_id": "terra", "dst_port_id": "transfer"}
2023-10-12T10:05:38.499847Z	info	Chain is in sync	{"chain_name": "terra", "chain_id": "terra"}
2023-10-12T10:05:38.499908Z	info	Chain is in sync	{"chain_name": "kujira", "chain_id": "kujira"}
2023-10-12T10:05:47.783022Z	info	Successful transaction	{"provider_type": "cosmos", "chain_id": "kujira", "gas_used": 161164, "fees": "3713ukuji", "fee_payer": "kujira1pqs8apaa94ejf2etsgv7fkdv6c69jv4l0q74gh", "height": 14, "msg_types": ["/ibc.core.client.v1.MsgUpdateClient", "/ibc.core.channel.v1.MsgChannelOpenInit"], "tx_hash": "4941B6EAD5431DD7DC66C154ADDD04C1A648B38AEA9D00F8D1448F9F462D9595"}
2023-10-12T10:05:58.583442Z	info	Successful transaction	{"provider_type": "cosmos", "chain_id": "terra", "gas_used": 184197, "fees": "4312uluna", "fee_payer": "terra1jy6td9r477fwr4q60adr7lz4anye5y89p5cq7q", "height": 15, "msg_types": ["/ibc.core.client.v1.MsgUpdateClient", "/ibc.core.channel.v1.MsgChannelOpenTry"], "tx_hash": "1300BED1252972C5ECB246607B244EDF1E53A53E2FA6C328608D5982F5C66D62"}
2023-10-12T10:06:08.171426Z	info	Successful transaction	{"provider_type": "cosmos", "chain_id": "kujira", "gas_used": 123216, "fees": "2727ukuji", "fee_payer": "kujira1pqs8apaa94ejf2etsgv7fkdv6c69jv4l0q74gh", "height": 18, "msg_types": ["/ibc.core.client.v1.MsgUpdateClient", "/ibc.core.channel.v1.MsgChannelOpenAck"], "tx_hash": "43D9F7DCC770A1D4C38EDCE4A226808C1115950E0A6929FFE6A485D6DF4E3403"}
2023-10-12T10:06:08.511883Z	info	Successfully created new channel	{"chain_name": "kujira", "chain_id": "kujira", "channel_id": "channel-0", "connection_id": "connection-0", "port_id": "transfer"}
2023-10-12T10:06:13.948071Z	info	Successful transaction	{"provider_type": "cosmos", "chain_id": "terra", "gas_used": 137128, "fees": "3088uluna", "fee_payer": "terra1jy6td9r477fwr4q60adr7lz4anye5y89p5cq7q", "height": 18, "msg_types": ["/ibc.core.client.v1.MsgUpdateClient", "/ibc.core.channel.v1.MsgChannelOpenConfirm"], "tx_hash": "86832DCCCCF56D0DFD7A6C28ACF93D415B8F0FAEFF2C5F9B262301C6876CE010"}
2023-10-12T10:06:14.512264Z	info	Successfully created new channel	{"chain_name": "terra", "chain_id": "terra", "channel_id": "channel-0", "connection_id": "connection-0", "port_id": "transfer"}
2023-10-12T10:06:14.512309Z	info	Channel handshake termination candidate	{"path_name": "kujira-terra", "chain_id": "terra", "client_id": "07-tendermint-0", "termination_port_id": "transfer", "observed_port_id": "transfer", "termination_counterparty_port_id": "transfer", "observed_counterparty_port_id": "transfer"}
2023-10-12T10:06:14.512316Z	info	Found termination condition for channel handshake	{"path_name": "kujira-terra", "chain_id": "terra", "client_id": "07-tendermint-0"}
==============> Starting relayers...<==============
```

### Upload the optimized cw binary and create a contract instance using following CLI commands.

For uploading,

```
kujirad tx wasm store {cw_root_dir/artifacts/icatest_controller.wasm} --from validator --gas auto --gas-adjustment 1.3 -y --output json --home $HOME/.kujirad --keyring-backend test --chain-id kujira
```

For instantiating,

```
kujirad tx wasm instantiate 1 '{}' --from validator --label "icatest" --gas auto --gas-adjustment 1.3 --no-admin -y --output json  --home $HOME/.kujirad --keyring-backend test --chain-id kujira
```

Now, we are all set for testing ICA account creation, and ICA transations through the binding contract.

### CLI examples for ICA account creation

#### ICA account creation

ICA account creation tx receives connection id, account id, version and tx id.
Tx id is the identifier to check ica callback after the relay operation completion.

```sh
CONTRACT=kujira14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sl4e867
kujirad tx wasm execute $CONTRACT '{"create_account":{"callback":"1","conn_id":"connection-0","acc_id":"1","version":""}}' --from validator --gas auto --gas-adjustment 1.3 -y --output json  --home $HOME/.kujirad --keyring-backend test --chain-id kujira
```

To check the ica callback after waiting up to 30s for relay operation to finish.

```sh
kujirad query wasm contract-state smart $CONTRACT '{"ica_register_callback":{"callback":"1"}}'

# output
data:
  account_id: "1"
  callback: MQ==
  connection_id: connection-0
  result:
    success:
      data: eyJ2ZXJzaW9uIjoiaWNzMjctMSIsImNvbnRyb2xsZXJfY29ubmVjdGlvbl9pZCI6ImNvbm5lY3Rpb24tMCIsImhvc3RfY29ubmVjdGlvbl9pZCI6ImNvbm5lY3Rpb24tMCIsImFkZHJlc3MiOiJ0ZXJyYTFuNGZsZ3d4ZHdybjVmbmZjOHYza253NXhneDk5cnVsejVhc2R2YTY0MGd5azA5M242cmtxemwyd2NwIiwiZW5jb2RpbmciOiJwcm90bzMiLCJ0eF90eXBlIjoic2RrX211bHRpX21zZyJ9
```

To get all the callback keys for ica registrations

```sh
kujirad query wasm contract-state smart $CONTRACT '{"ica_register_callback_keys":{}}'

# Output
data:
- "1"
```

#### ICA account query

After the successful ICA account creation, following query is available to get the ICA account address.

```sh
kujirad query wasm contract-state smart $CONTRACT '{"account":{"conn_id":"connection-0","acc_id":"1"}}'

# output
data:
  address: terra14w042juyrktjjqtxpqe3wruc3qjxyvw8purrdt8utfr28p4j5edq20td26
```

#### ICA account transaction

On ICA test contract, it broadcasts `MsgDelegate` message on host chain.

- Send coins to ICA account

```sh
ICA_ADDRESS=terra14w042juyrktjjqtxpqe3wruc3qjxyvw8purrdt8utfr28p4j5edq20td26

VALIDATOR="validator"
COUNTER_CHAIN_ID="terra"
COUNTER_MONIKER="terra"
COUNTER_HOME="$HOME/.terrad"
COUNTER_BINARY="terrad --home=$COUNTER_HOME"
COUNTER_TX_FLAGS="--keyring-backend test --chain-id $COUNTER_CHAIN_ID --from $VALIDATOR -y --fees=1000uluna"
$COUNTER_BINARY tx bank send $VALIDATOR $ICA_ADDRESS 10000000uluna $COUNTER_TX_FLAGS

# Check balances after the confirmation
$COUNTER_BINARY query bank balances $ICA_ADDRESS
```

- Collect host chain validator to delegate to

```sh
$COUNTER_BINARY query staking validators
COUNTER_VAL_ADDRESS=terravaloper1w738th77nm9vzrvmpxd3u50t4m8p8047wxlnsu
```

- Ensure that `MsgDelegate` is enabled on host chain ica allowed messages

```sh
$COUNTER_BINARY query interchain-accounts host params
```

- Send Delegation Tx through ICA

```sh
kujirad tx wasm execute $CONTRACT '{"send_delegate_tx":{"callback":"2","conn_id":"connection-0","acc_id":"1","validator":"'$COUNTER_VAL_ADDRESS'","amount":{"denom":"uluna", "amount":"1000000"}}}' --from validator --gas auto --gas-adjustment 1.3 -y --output json  --home $HOME/.kujirad --keyring-backend test --chain-id kujira

```

- Check ICA execution callback

```sh
kujirad query wasm contract-state smart $CONTRACT '{"ica_tx_callback":{"callback":"2"}}'

# Output
data:
  account_id: "1"
  callback: Mg==
  connection_id: connection-0
  result:
    success:
      data: Ei0KKy9jb3Ntb3Muc3Rha2luZy52MWJldGExLk1zZ0RlbGVnYXRlUmVzcG9uc2U=
  sequence: 1
```

- Check Delegation from ICA on host chain

```sh
$COUNTER_BINARY query staking delegation $ICA_ADDRESS $COUNTER_VAL_ADDRESS --output=json
# Output
{"delegation":{"delegator_address":"...","validator_address":"...","shares":"1000000.000000000000000000"},"balance":{"denom":"uluna","amount":"1000000"}}
```

##### ICA account failure transaction example

- Send ICA tx with invalid operator address

```sh
kujirad tx wasm execute $CONTRACT '{"send_delegate_tx":{"callback":"4","conn_id":"connection-0","acc_id":"1","validator":"INVALID_OPERATOR","amount":{"denom":"uluna", "amount":"1000000"}}}' --from validator --gas auto --gas-adjustment 1.3 -y --output json  --home $HOME/.kujirad --keyring-backend test --chain-id kujira
```

- Check ica callback

```sh
kujirad query wasm contract-state smart $CONTRACT '{"ica_tx_callback":{"callback":"4"}}'

# Output
data:
  account_id: "1"
  callback: NA==
  connection_id: connection-0
  result:
    error:
      error: 'ABCI code: 7: error handling packet: see events for details'
  sequence: 1
```

To get all the callback keys for ica txs

```sh
kujirad query wasm contract-state smart $CONTRACT '{"ica_tx_callback_keys":{}}'

# Output
data:
- "4"
```
