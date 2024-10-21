# Secure Token Wallet on ICP Blockchain

## Overview

The **Secure Token Wallet** is a Rust-based wallet designed for the **Internet Computer Protocol (ICP)** blockchain. This wallet allows users to send and receive **IRCRC2 tokens** securely, showcasing a robust understanding of Rust and blockchain development principles. 

## Table of Contents

- [Objective](#objective)
- [Features](#features)
- [Requirements](#requirements)
- [Getting Started](#getting-started)
- [Usage](#usage)
- [Testing](#testing)
- [License](#license)
- [Contact](#contact)

## Objective

Create a user-friendly token wallet that supports the essential functionalities of sending and receiving IRCRC2 tokens while implementing security measures to protect users' assets.

## Features

- **Send Tokens:** Effortlessly send IRCRC2 tokens to other wallet addresses.
- **Receive Tokens:** Seamlessly receive tokens and automatically update your balance.
- **Balance Display:** Instantly view the current token balance in your wallet.
- **Basic Wallet Security:** Implement fundamental security features to ensure safe transactions.

## Requirements

### Blockchain Development

- **Smart Contracts:**
  - Utilize **Rust** for developing smart contracts to manage token transactions.
  
- **Deployment:**
  - Deploy the contracts to a local **ICP test network**.

### Smart Contract Features

- Add functionalities for sending and receiving tokens.
- Implement basic security features for the wallet.
- Fetch and display current token balances.

## Getting Started

To get started with the Secure Token Wallet, follow these steps:

1. **Clone the Repository:**
   ```bash
   git clone https://github.com/Hitesh-jadhav/secure-token-wallet.git
   cd secure-token-wallet
   ```

2. **Install Dependencies:**
   Make sure you have Rust and the necessary ICP tools installed. You can install Rust by following the [official Rust installation guide](https://www.rust-lang.org/tools/install).

3. **Set Up ICP Environment:**
   Follow the instructions on the [ICP developer portal](https://sdk.dfinity.org/docs/) to set up your local ICP test environment.

4. **Build the Project:**
   ```bash
   cargo build
   ```

5. **Deploy Smart Contracts:**
   Deploy your smart contracts to the local ICP network using the following command:
   ```bash
   dfx deploy
   ```

## Usage

1. **Send Tokens:**
   Use the wallet interface to input the recipient's address and the amount of IRCRC2 tokens you wish to send.

2. **Receive Tokens:**
   Provide your wallet address to others so they can send you tokens.

3. **Check Balance:**
   View your current token balance in the wallet interface.

## Testing

To ensure the functionality of the smart contracts, unit tests have been developed using the ICP framework. To run the tests, use the command:

```bash
cargo test
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

For any inquiries or contributions, feel free to reach out:

- **Hitesh Rohidas Jadhav**  
  [LinkedIn](https://www.linkedin.com/in/hitesh-jadhav-983b41264/) | [GitHub](https://github.com/Hitesh-jadhav)
