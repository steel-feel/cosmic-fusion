# cosmic-fusion
Cross chain swap using 1inch Fusion+
## Local Cosmos chain (using gm)
#### Start
```bash
gm start gaia
```
#### Stop
```bash
gm stop gaia
```

#### Status
```bash
gm status gaia
```

Check wallet
```bash
gm keys gaia
````

Create new wallet
```bash
gm new-wallet gaia <new_wallet>
```

## Contracts
### Build contract
```bash
cargo wasm
```

This will generate binary wasm file at target/wasm32-unknown-unknown/release/

**Optimised smaller size**
```bash
RUSTFLAGS='-C link-arg=-s' cargo wasm
```

**Optimised version for testnets**
```bash
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/optimizer:0.16.0
```

Arm machine(apple silicon)
```bash
docker run --rm -v "$(pwd)":/code \
--mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
cosmwasm/rust-optimizer-arm64:0.17.0
```

### Generate schema
```bash
cargo schema
```
### Test
```bash
cargo test
```

##### show print output
```bash
cargo test -- --show-output
```

### Check the contract
```bash
cosmwasm-check <wasm-file>
```



### Deploy contract
```bash
gm exec gaia tx wasm store ./target/wasm32-unknown-unknown/release/<file.wasm> --from wallet -y --output json --gas 2000000
```
check code id from transaction hash event logs

```bash
xiond q tx <hash>
```

params to functions are added as struct, a custom type must be defined in contract
it apprears as json while calling from API

Contract address is available after instantiation

##### To Testnet
(using xiond because you cant set keys with gm)
```
xiond tx wasm store ./artifacts/escrow_dest.wasm --from onchain -y --output json --gas-adjustment 1.3 --gas-prices 0.001uxion --gas auto
```


### Instantiate
```bash
gm exec gaia tx wasm instantiate 'CODE_ID' '<params-in-json/ empty for none>' --from wallet --label "counter" --gas auto -y --no-admin
```
 - find contract address from tx event logs using hash, look for attribute _contract_address 
 OR
 - use query to find
```bash
xiond q wasm list-contract-by-code 'CODE_ID' --output json
```



 > gas auto should work, else pass it manually

### Execute Transaction

```js
payload = {
<function-name> : {
"key" : "value"
}
}
```

```bash
gm exec gaia tx wasm execute <contract_address> '<json-payload>' --from wallet --gas auto -y
```

### Query 
```bash
xiond query wasm contract-state smart <contract_address> '<json_req_obj>' --output json
```


# Token Related

### Create Token
```bash
gm exec gaia tx tokenfactory create-denom usdt --from wallet --gas 3000000 -y
```
### Check token
```bash
 ~/.gm/xiond q tokenfactory denoms-from-admin xion12y25c3ndtcz3tr63aymnkknl09j69px47hxz2a
```

### Mint tokens (only by creater) 
```
gm exec gaia tx tokenfactory mint 100000000000factory/xion12y25c3ndtcz3tr63aymnkknl09j69px47hxz2a/usdt --from wallet -y
```


Source escrow

 xiond tx authz grant cosmos1skjw.. send --spend-limit=1000stake --from=cosmos1skl..


 xiond tx bank send <granter> <recipient> --from <granter> --chain-id <chain-id> --generate-only > tx.json && 
 
 xiond tx authz exec tx.json --from grantee