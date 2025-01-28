#!/bin/bash

ANVIL_STATE=$1

# Remove the fields transactions, blocks block.prevrandao and block.timestamp
jq 'del(.transactions)' $ANVIL_STATE > temp.json && cat temp.json > $ANVIL_STATE
jq 'del(.blocks)' $ANVIL_STATE > temp.json && cat temp.json > $ANVIL_STATE
jq 'del(.block.prevrandao)' $ANVIL_STATE > temp.json && cat temp.json > $ANVIL_STATE
jq 'del(.block.timestamp)' $ANVIL_STATE > temp.json && cat temp.json > $ANVIL_STATE
