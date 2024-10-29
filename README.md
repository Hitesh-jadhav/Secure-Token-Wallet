
# Secure Token Wallet

A secure token wallet built on the Internet Computer Protocol (ICP) blockchain, utilizing Rust for secure token transfers, minting, and querying balances.

---

## Table of Contents

1. [Project Overview](#project-overview)
2. [Features](#features)
3. [Getting Started](#getting-started)
4. [Installation](#installation)
5. [Usage](#usage)
6. [Testing](#testing)
7. [File Structure](#file-structure)
8. [Contributing](#contributing)
9. [License](#license)

---

## Project Overview

The **Secure Token Wallet** is a blockchain-based wallet application that enables secure, efficient token transactions on the ICP blockchain. Built with Rust, this application allows users to mint tokens, check balances, and perform transfers between accounts.

---

## Features

- **Mint Tokens**: Create tokens for a given account.
- **Transfer Tokens**: Securely transfer tokens between accounts.
- **Query Balance**: Check the balance of any account.

---

## Getting Started

### Prerequisites

To run this project, you will need:

- [DFX SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install) (Internet Computer SDK)
- [Rust](https://www.rust-lang.org/tools/install) programming language and package manager
- [Node.js and npm](https://nodejs.org/) (for front-end testing, if applicable)

---

## Installation

Follow these steps to set up and run the project:

1. **Clone the Repository**:
    ```bash
    git clone https://github.com/Hitesh-jadhav/Secure-Token-Wallet.git
    cd Secure-Token-Wallet
    ```

2. **Install Dependencies**:
   - Install Rust if not already installed:
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```
   - Install DFX SDK:
     ```bash
     sh -ci "$(curl -fsSL https://internetcomputer.org/docs/current/developer-docs/setup/install)"
     ```

3. **Start Local Replica**:
   ```bash
   dfx start --background
   ```

4. **Deploy the Canister**:
   ```bash
   dfx deploy
   ```

---

## Usage

Once deployed, you can interact with the canisters using the following methods.

### Check Balance

```bash
dfx canister call secure_token_wallet_backend get_balance '(principal "<principal_id>")'
```

### Transfer Tokens

```bash
dfx canister call secure_token_wallet_backend transfer '(principal "<from_principal>", principal "<to_principal>", <amount>)'
```

### Mint Tokens

```bash
dfx canister call secure_token_wallet_backend mint '(principal "<principal_id>", <amount>)'
```

---

## Testing

To run the test suite:

1. **Run Tests**:
    ```bash
    cargo test
    ```

2. **Verify Output**: Ensure all test cases pass without errors.

---

## File Structure

```
Secure-Token-Wallet/
├── src/
│   ├── secure_token_wallet_backend/
│   │   ├── lib.rs                  # Core backend functionality
│   └── secure_token_wallet_frontend/
│       └── (Optional front-end code for interaction)
├── README.md                       # Documentation
└── Cargo.toml                       # Rust package configuration
```

---

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements.

---

## License

This project is licensed under the MIT License.
