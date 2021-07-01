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
near dev-deploy --wasmFile res/ncd_playground.wasm
```bash 
chmod +x ./build.sh
```
# Midnight notes for ideation
## Cold chain tracking
Imagine you are a truck driver with a cold chain refrigator... 
-A truck visit distinct cities across Mexico delivering COVID-19 vaccines with ultra frozens refrigerators
-In each city the truck have to leave information about Truck ID, Temp (ºF and ºC), Truck plate, Fuel liters, Current Location, timestamp
-When the truck arrives the last destionation (city), the truck driver can withdraw the payment.
-If the Temp goes out of range the payment is canceled and returned to smart contract.
-To start a new contract you need to pay the truck owner when you return to the origin

## Locations 
-Tepic, Nayarit (Origin)
-Guadalajara, Jalisco
-Aguascalientes, Aguascalientes
-Leon, Guanajuato
-Ciudad de México (Destiny)



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
