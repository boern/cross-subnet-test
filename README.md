# Cross IC subnet test


## Cross IC subnet test sub and pub
```
# deploy subscriber on application subnet
dfx deploy subscriber --ic

# deploy publisher on fiduciary subnet
# first create canister use ledger and get canister id,like: vu5xo-waaaa-aaaar-qac6a-cai 
dfx ledger create-canister rv3oc-smtnf-i2ert-ryxod-7uj7v-j7z3q-qfa5c-bhz35-szt3n-k3zks-fqe --subnet-type fiduciary --ic --amount 0.55
# install publisher wasm
dfx canister install vu5xo-waaaa-aaaar-qac6a-cai --wasm .dfx/ic/canisters/publisher/publisher.wasm --ic --mode=reinstall

# cross canister call setup_subscribe
dfx canister call subscriber setup_subscribe '(principal"vu5xo-waaaa-aaaar-qac6a-cai","apple")' --ic
# get call delay
dfx canister call subscriber get_inner_call_time '()' --ic

# cross canister call publisher publish
dfx canister call vu5xo-waaaa-aaaar-qac6a-cai publish '(record { "topic" = "apple"; "value" = 8 })' --ic
# get call delay
dfx canister call vu5xo-waaaa-aaaar-qac6a-cai get_inner_call_time '()' --ic

```

## Cross IC subnet test ecdsa_example 

```
dfx deploy ecdsa_example --ic
dfx canister call ecdsa_example public_key '()' --ic
dfx canister call ecdsa_example get_inner_call_time '()' --ic
dfx canister call ecdsa_example sign '("Hi,Boern")' --ic
dfx canister call ecdsa_example get_inner_call_time '()' --ic
```