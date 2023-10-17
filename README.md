# relayer-fee-token
Testnet Fungible Token used to cover fees paid by the Relayer in concert with the relayer demo. 

FT contract created using https://near.org/contractwizard.near/widget/ContractWizardUI

## Pre-requisites
To develop Rust contracts you would need to:
* Install [Rustup](https://rustup.rs/):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
* Add wasm target to your toolchain:
```bash
rustup target add wasm32-unknown-unknown
```
* Install [near-cli](https://github.com/near/near-cli#installation) 
```bash
npm install -g near-cli
```

## Build
```bash
cargo build --all --target wasm32-unknown-unknown --release
```

## Deploy to Testnet 
```bash
near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/relayer_fee_token.wasm
```
Behind the scenes, this is creating an account and deploying a contract to it. On the console, notice a message like:

>Done deploying to dev-1234567890123

In this instance, the account is `dev-1234567890123`. A file has been created containing a key pair to
the account, located at `neardev/dev-account`. To make the next few steps easier, we're going to set an
environment variable containing this development account id and use that when copy/pasting commands.
Run this command to the environment variable:
```bash
source neardev/dev-account.env
```

You can tell if the environment variable is set correctly if your command line prints the account name after this command:
```bash
echo $CONTRACT_NAME
```

## Initialize Contract
The next command will initialize the contract using the `new` method:
```bash
near call $CONTRACT_NAME new --accountId $CONTRACT_NAME
```

To get the fungible token metadata:
```bash
near view $CONTRACT_NAME ft_metadata
```

To check the initial balance of the account that tokens were minted to:
```bash
near view $CONTRACT_NAME ft_balance_of '{"account_id": "relayer-fee-token.testnet"}'
```

## Send RFT to another Account
Add storage deposit for Bob's account (an account you'd like to send RFT to):
```bash
near call $CONTRACT_NAME storage_deposit '{"account_id": "bob.testnet"}' --accountId your_local_credentials_account.testnet --amount 0.00125
```

Check balance of Bob's account, it should be `0` for now:
```bash
near view $CONTRACT_NAME ft_balance_of '{"account_id": "bob.testnet"}'
```

Transfer tokens to Bob from the contract that minted these fungible tokens, exactly 1 yoctoNEAR of deposit should be attached:
```bash
near call $CONTRACT_NAME ft_transfer '{"receiver_id": "bob.testnet", "amount": "19"}' --accountId $CONTRACT_NAME --amount 0.000000000000000000000001
```

Check balance of Bob's account again, it should be `19` for now:
```bash
near view $CONTRACT_NAME ft_balance_of '{"account_id": "bob.testnet"}'
```
