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

    #[test]
    fn test_transfer() {
        let sender = Principal::anonymous();
        let receiver = Principal::anonymous();
        let mut token = create_account(sender, 1000);
        assert_eq!(token.balance, 1000);

        transfer(sender, receiver, 100);
        assert_eq!(get_balance(sender), 900);
        assert_eq!(get_balance(receiver), 100);
    }
}
