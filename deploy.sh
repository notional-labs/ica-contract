#!/bin/bash

#NODE="tcp://localhost:2281"
NODE=""
ACCOUNT="mykey"
CONTRACT_DIR="artifacts/ibc_reflect_send.wasm"
SLEEP_TIME="15s"
CHAIN_ID="test-1"


RES=$(digd tx wasm store "$CONTRACT_DIR" --from "$ACCOUNT" -y --output json --chain-id "$CHAIN_ID" --gas 35000000 --fees 875000stake -y --output json)
echo $RES

if [ "$(echo $RES | jq -r .raw_log)" != "[]" ]; then
	# exit
	echo "ERROR = $(echo $RES | jq .raw_log)"
	exit 1
else
	echo "STORE SUCCESS"
fi

TXHASH=$(echo $RES | jq -r .txhash)

echo $TXHASH

sleep for chain to update
sleep "$SLEEP_TIME"

RAW_LOG=$(digd query tx "$TXHASH" --chain-id "$CHAINID" --node "$NODE" -o json | jq -r .raw_log)

echo $RAW_LOG

CODE_ID=$(echo $RAW_LOG | jq -r .[0].events[1].attributes[0].value)

echo $CODE_ID

INIT_JSON=$(digd tx wasm instantiate 13 {} --from $ACCOUNT --label "instantiate contract" -y --chain-id $CHAIN_ID --gas 10000000 --fees 100000000stake -o json --no-admin)

echo "INIT_JSON = $INIT_JSON"

if [ "$(echo $INIT_JSON | jq -r .raw_log)" != "[]" ]; then
	# exit
	echo "ERROR = $(echo $INIT_JSON | jq .raw_log)"
	exit 1
else
	echo "INSTANTIATE SUCCESS"
fi

# sleep for chain to update
sleep "$SLEEP_TIME"

RAW_LOG=$(digd query tx "$(echo $INIT_JSON | jq -r .txhash)" --chain-id "$CHAINID" --node "$NODE" --output json | jq -r .raw_log)

echo "RAW_LOG = $RAW_LOG"

CONTRACT_ADDRESS=$(echo $RAW_LOG | jq -r .[0].events[0].attributes[0].value)

echo "CONTRACT ADDRESS = $CONTRACT_ADDRESS"

