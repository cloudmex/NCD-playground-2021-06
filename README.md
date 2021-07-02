# NCD-playground-2021-06
This is a playground done for the NCD bootcamp on june 2021. ~~Maybe in comming commits will have a proposal.~~ The project is now a cold chain deliveries tracker.

# Considerations
This is a project done in a UNIX-based system, as linux or MacOS.

If you are running Windows ~~please change to UNIX~~, use something as WLS for command terminal.

# How to run the smart contract
First is required to compile the smart contract into binaries 
```bash
./build.sh
```
In my case i needed extra permissions, so i give them to the file.

```bash 
chmod +x ./build.sh
```
After use the near dev-deploy command and indicate where is your .wasm file. You can also use near deploy if you prefered to have it mainnet.
**Note**: If permissions required write the following
```bash
near dev-deploy --wasmFile res/ncd_playground.wasm
```

# Cold chain tracking
Imagine you are a want to send a cold chain package.

- Pay 10 NEAR for a new delivery for starting it, select the account to pay when the delivery is done.
- Record temperature in each arrival to a new location.
- Once the delivery is in the last location, withdraw 10 NEAR as payment for completing succesfully the delivery.
- Restart the cycle.

Also you can:
- Check the status of the contract if it is initialized.
- Get extra information of delivery (Truck plate, trucker name, temperature in ºF, fuel level, etc.)
- If the Temp goes out of range the payment is canceled and returned to smart contract.
- To start a new contract you need to pay the truck owner when you return to the origin

## Locations (Simulated)
- 0 Tepic, Nayarit (Origin)
- 1 Guadalajara, Jalisco
- 2 Aguascalientes, Aguascalientes
- 3 Leon, Guanajuato
- 4 Ciudad de México (Destiny)

**Note**: We had de idea to do a get_location_verbose about this in the smart contract but we think was senseless and we could do that in frontend.

# Deploy the smart contract inside the testnet
```bash
near dev-deploy --wasmFile res/ncd_playground.wasm
```

**Note** It can be required to delete this files to run correctly the SC
```bash
source neardev/dev-account.env 
```
```bash
echo $CONTRACT_NAME
```
Define your account inside your environment
```bash
export MY_ACCOUNT='mytest.testnet'
echo $MY_ACCOUNT
```
# Commands in the smar contract


You can verify the status of the contract about a delivery by using get_initilized()
```bash
near call $CONTRACT_NAME get_initialized --accountId $MY_ACCOUNT
```
A new delivery will start using new_delivery(), you need to send 10 NEAR (--amount 10) and a JSON with the initial Temperature (ºC) and the account that will withdray the 10 NEARS if the delivery finish succesfully
```bash
near call $CONTRACT_NAME new_delivery '{"temp_c": -4.0, "payment_account_id": "alan1.testnet"}' --accountId $MY_ACCOUNT --amount 10
```
Creat a new arrival to a new location, temperature is saved.
```bash
near call $CONTRACT_NAME new_arrival '{"temp_c": -8.9}' --accountId 'alantest.testnet'  
```
If delivery is succesful, withdraw can be done and would be send to the account defined in new_delivery()
```bash
near call $CONTRACT_NAME withdraw --accountId $MY_ACCOUNT
```
JUST FOR DEBUG, decrement the location index, and reset the location index to 0.
```bash
near call $CONTRACT_NAME decrement --accountId $MY_ACCOUNT
near call $CONTRACT_NAME reset --accountId $MY_ACCOUNT 
```
## Other commands inside the contract
Get location, range 0-4, 0 is origin and 4 is destiny.
```bash
near call $CONTRACT_NAME get_location --accountId $MY_ACCOUNT   
```
Get balance of smart contract, returns the balance in yocto.
```bash
near call $CONTRACT_NAME get_balance --accountId $MY_ACCOUNT    
```
Get temperatue, returns temp in a floating number.
```bash
near call $CONTRACT_NAME get_temp --accountId $MY_ACCOUNT    
```
Get payment account id (payment.test/payment.near)
```bash
near call $CONTRACT_NAME get_payment_account_id --accountId $MY_ACCOUNT  
```
Get initialized, returns a boolean.
```bash
near call $CONTRACT_NAME get_initialized--accountId $MY_ACCOUNT  
```
# Testing the contract
Use cargo for running tests inside the contract
```bash
cargo test 
```
# Wireframing
Figma link: https://www.figma.com/file/w4YSyB4ML1wtcSy6IueL9j/Cold-chain-Wireframe?node-id=1392%3A1130

# Loom video
Video demo: https://www.loom.com/share/7ef30fb4deaf4088aad42600992dff97

# What to do next? Some ideas for improving the code
In this ideation section, we give some items that could improve the code, but time in NCD bootcamp was not enough to do that.
* Add extra information about the delivery to be tracked (Truck plate, Driver ID, fuel level, etc.)
* Have multiple deliveries running at the same time in the smart contract.
* Define custom payment amount for delivery.
* Define a safe range for temperature, if it is out of range return the money.

# Credits
This projects recovers ideas from many repositories, a list of those i remember.
-https://github.com/near/core-contracts
-https://github.com/near-examples/rust-status-message
-https://github.com/Learn-NEAR/NCD-08--Communite/blob/master/assembly/index.ts
