# **AA Wrap Demo** &middot; [![license](https://img.shields.io/badge/license-MIT-blue)](https://shields.io)

<!-- This demo showcases how the AA Wrap can be used to integrate account abstraction functionality into different deployment platforms – web browsers, mobile, gaming and beyond. -->

<img src="images/multiplatform.png" width="500x">

## What's the AA Wrap?

The AA Wrap is a Polywrap-powered version of the [Safe Account Abstraction (AA) SDK](https://docs.safe.global/learn/safe-core/safe-core-account-abstraction-sdk).

The Safe AA SDK lets developers build ["smart accounts"](https://docs.safe.global/learn/what-is-a-smart-contract-account), accounts with fully customizable functionality. Smart accounts aim to bring much-needed UX improvements to web3.

Today, the AA Wrap includes functionality to:

- create customizable, secure, and modular smart accounts.
- enable a gas-less user experience via Gelato, offer sponsored transactions, and allow for fee payments in ERC-20 tokens.

## Demo Overview

In this demo, you'll be executing scripts that showcase the AA Wrap's main value props: **multi-platform** and **composability**.

<img src="images/fetch.png" width="300x">

1. **Multi–platform** – The same AA Wrap is fetched from IPFS and then integrated into two different programming environments: JavaScript and Rust.

2. **Composability** – The AA Wrap's functionality can be easily extended by composing it with other Wraps. The current AA Wrap is composed of 3 separate Wraps: Ethers, Gelato Relay and Safe{Core}.

## Demo Setup

First, you'll set up the demo.

1. Change directory to `js`:

```
cd js
```

2. Update the salt nonce in `deploy-safe.ts` to `0x` followed by a few randomly chosen digits. For example,

This salt nonce is used later to create the new Safe address.

```ts
const SALT_NONCE = "0x185593";
```

3. Run the `deploy-safe.ts` script to deploy the Safe smart account.

```
yarn deploy
```

> For the purposes of this demo, we're deploying to the Goerli test network. You can view your Safe on [Goerli Etherscan block explorer](https://goerli.etherscan.io/).

4. Create a `.env` file at the root of the repo and add in the Safe address:

You should have received the Safe address after deploying it.

Copy and paste this address into your `.env` file:

```bash
SAFE_ADDRESS="0x..."
```

5. Finally, add the a Safe transaction signer by running:

```
yarn add-owner
```

Congratulations! You're done configuring the demo and deploying your own Safe smart account. 🥳

### Demo Scripts

The scripts below show complex transactions that your smart account can execute. Run `yarn` and then the script name to execute it.

| Script              | Description                                                   |
| ------------------- | ------------------------------------------------------------- |
| `execute`           | Updates a stored value on a smart contract                    |
| `execute-sponsored` | Uses the Gelato relay Wrap to execute a sponsored transaction |
| `execute-multisend` | Sends transactions to multiple recipients                     |

## Multi-platform Support: Rust

One of the powerful advantages of the AA Wrap is that it can be used in any app, as long as the app has a Polywrap client library installed.

The above scripts were run in a JavaScript environment. We've also prepared scripts that run in **Rust** below.

First, you need to be in the `rs` directory.

Then, update the salt nonce in the `deploy.rs` file:

```rs
    let deployment_input = DeploymentArgs {
        input: DeploymentInput {
            safe_account_config: AccountConfig {
                owners: vec![signer_address.unwrap()],
                threshold: 1,
            },
            safe_deployment_config: DeploymentConfig {
                salt_nonce: "0x...".to_string(),
            },
            connection: None,
        },
    };
```

Then run:

```
cargo run --bin --release <script>
```

Where `<script>` is one of the following:

| Script       | Description                     |
| ------------ | ------------------------------- |
| `deploy`     | Deploys your Safe smart account |
| `add_owners` | Add owners to the Safe          |
| `get_owners` | Get the list of owners          |

# Community & Contributing

The AA Wrap was developed and maintained by the [Polywrap DAO](https://polywrap.io/). We believe that the AA Wrap brings a new depth of code re-useability and modularity to the already powerful account abstraction suite that the Safe team built.

To learn more about Polywrap and contribute to the AA Wrap, please join our [Discord community](https://discord.gg/qK9S46gTbF).
