// #[ic_cdk::query]
// fn greet(name: String) -> String {
//     format!("Hello, {}!", name)
// }


use candid::{CandidType, Deserialize, Principal};
use ic_cdk_macros::{init, query, update};
use std::collections::HashMap;

// Struct to store balances and token supply
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Token {
    pub owner: Principal,
    pub balance: u64,
}

type TokenLedger = HashMap<Principal, Token>;

static mut LEDGER: Option<TokenLedger> = None;

#[init]
fn init() {
    // Initialize the ledger as an empty HashMap
    unsafe {
        LEDGER = Some(HashMap::new());
    }
}

// Function to create a new account with initial tokens
fn create_account(owner: Principal, initial_balance: u64) -> Token {
    Token { owner, balance: initial_balance }
}

// Fetch the current token balance of an address
#[query]
fn get_balance(account: Principal) -> u64 {
    unsafe {
        match LEDGER.as_ref().unwrap().get(&account) {
            Some(token) => token.balance,
            None => 0, // Return 0 if the account does not exist
        }
    }
}

// Function to transfer tokens between accounts
#[update]
fn transfer(from: Principal, to: Principal, amount: u64) -> String {
    unsafe {
        let ledger = LEDGER.as_mut().unwrap();

        // Ensure the sender has enough tokens
        if let Some(sender_token) = ledger.get_mut(&from) {
            if sender_token.balance < amount {
                return "Insufficient balance".to_string();
            }
            sender_token.balance -= amount;

            // Update the receiver's balance
            let receiver_token = ledger.entry(to).or_insert(create_account(to, 0));
            receiver_token.balance += amount;

            "Transfer successful".to_string()
        } else {
            "Sender does not have an account".to_string()
        }
    }
}

// Function to mint tokens for testing purposes (not in production)
#[update]
fn mint(account: Principal, amount: u64) -> String {
    unsafe {
        let ledger = LEDGER.as_mut().unwrap();
        let token = ledger.entry(account).or_insert(create_account(account, 0));
        token.balance += amount;

        "Tokens minted".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Principal;

    // Helper function to create a Principal ID for testing
    fn test_principal(id: u64) -> Principal {
        Principal::from_slice(&id.to_be_bytes())
    }

    #[test]
    fn test_create_account() {
        // Create a new account with an initial balance
        let account_owner = test_principal(1);
        let token = create_account(account_owner, 1000);

        assert_eq!(token.owner, account_owner);
        assert_eq!(token.balance, 1000);
    }

    #[test]
    fn test_get_balance() {
        let account_owner = test_principal(2);

        // Initialize the ledger with an account
        unsafe {
            LEDGER = Some(HashMap::new());
            LEDGER.as_mut().unwrap().insert(account_owner, create_account(account_owner, 500));
        }

        // Test if the balance is correct
        let balance = get_balance(account_owner);
        assert_eq!(balance, 500);
    }

    #[test]
    fn test_transfer_success() {
        let sender = test_principal(3);
        let receiver = test_principal(4);

        unsafe {
            LEDGER = Some(HashMap::new());
            // Create accounts for sender and receiver
            LEDGER.as_mut().unwrap().insert(sender, create_account(sender, 1000));
            LEDGER.as_mut().unwrap().insert(receiver, create_account(receiver, 200));
        }

        // Transfer 300 tokens from sender to receiver
        let result = transfer(sender, receiver, 300);
        assert_eq!(result, "Transfer successful".to_string());

        // Check the balances after transfer
        assert_eq!(get_balance(sender), 700);
        assert_eq!(get_balance(receiver), 500);
    }

    #[test]
    fn test_transfer_insufficient_balance() {
        let sender = test_principal(5);
        let receiver = test_principal(6);

        unsafe {
            LEDGER = Some(HashMap::new());
            // Create accounts with low balance for sender
            LEDGER.as_mut().unwrap().insert(sender, create_account(sender, 100));
            LEDGER.as_mut().unwrap().insert(receiver, create_account(receiver, 50));
        }

        // Try to transfer more tokens than available
        let result = transfer(sender, receiver, 200);
        assert_eq!(result, "Insufficient balance".to_string());

        // Ensure balances remain unchanged
        assert_eq!(get_balance(sender), 100);
        assert_eq!(get_balance(receiver), 50);
    }

    #[test]
    fn test_transfer_no_sender_account() {
        let sender = test_principal(7);
        let receiver = test_principal(8);

        unsafe {
            LEDGER = Some(HashMap::new());
            // No account created for the sender
            LEDGER.as_mut().unwrap().insert(receiver, create_account(receiver, 50));
        }

        // Try to transfer from a non-existent account
        let result = transfer(sender, receiver, 50);
        assert_eq!(result, "Sender does not have an account".to_string());

        // Ensure receiver's balance remains unchanged
        assert_eq!(get_balance(receiver), 50);
    }

    #[test]
    fn test_mint_tokens() {
        let account_owner = test_principal(9);

        unsafe {
            LEDGER = Some(HashMap::new());
            // Create an account and mint tokens for it
            let result = mint(account_owner, 500);
            assert_eq!(result, "Tokens minted".to_string());
        }

        // Check the balance after minting
        assert_eq!(get_balance(account_owner), 500);
    }

    #[test]
    fn test_mint_existing_account() {
        let account_owner = test_principal(10);

        unsafe {
            LEDGER = Some(HashMap::new());
            // Create an account with an initial balance
            LEDGER.as_mut().unwrap().insert(account_owner, create_account(account_owner, 100));

            // Mint additional tokens to the same account
            let result = mint(account_owner, 500);
            assert_eq!(result, "Tokens minted".to_string());
        }

        // Check the balance after minting
        assert_eq!(get_balance(account_owner), 600);
    }
}
