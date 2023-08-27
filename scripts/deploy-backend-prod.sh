#!/bin/bash
VETKD_CANISTER_ID="$(dfx canister --network ic id vetkd_system_api)"
eval dfx deploy main --network ic --identity deployer --with-cycles 1000000000000 --argument \
"'(record {
    vetkd_canister_id = \"$VETKD_CANISTER_ID\"; 
    key_name = \"test_key_1\";
})'"