# NCD-playground-2021-06
This is a playground done for the NCD bootcamp on june 2021. Maybe in comming commits will have a proposal.

# Considerations
This is a project done in a UNIX-based system, as linux or MacOS.

If you are running Windows ~~please change to UNIX~~, use something as WSL for command terminal.

# How to run the smart contract
Is 
```bash
./build.sh
```

**Note**: If permissions required write the following  
```bash 
rustup target add wasm32-unknown-unknown

```bash
chmod +x ./build.sh
```

# Deploy the smart contract inside the testnet
```bash
near dev-deploy --wasmFile res/ncd_playground.wasm
```

```bash
source neardev/dev-account.env 
```

```bash
echo $CONTRACT_NAME
```

```bash
near call $CONTRACT_NAME set_status '{"message": "aloha!"}' --accountId $CONTRACT_NAME
```
# Testing the contract

# Credits
This projects recovers ideas and pieces of code from https://github.com/near-examples/rust-status-message
