#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreditLine { pub lender: Address, pub borrower: Address, pub limit: i128, pub balance: i128, pub active: bool }

#[contracttype]
pub enum DataKey { CreditLine(Address) }

#[contract]
pub struct CreditLineContract;

#[contractimpl]
impl CreditLineContract {
      pub fn setup_credit_line(env: Env, lender: Address, borrower: Address, limit: i128) {
                lender.require_auth();
                let credit_line = CreditLine { lender, borrower: borrower.clone(), limit, balance: 0, active: true };
                env.storage().instance().set(&DataKey::CreditLine(borrower), &credit_line);
      }
      pub fn update_balance(env: Env, borrower: Address, amount: i128) {
                let mut credit_line: CreditLine = env.storage().instance().get(&DataKey::CreditLine(borrower.clone())).expect("Credit line not found");
                credit_line.balance += amount;
                if credit_line.balance > credit_line.limit { panic!("Credit limit exceeded"); }
                env.storage().instance().set(&DataKey::CreditLine(borrower), &credit_line);
                env.events().publish((Symbol::new(&env, "balance_updated"), credit_line.borrower), credit_line.balance);
      }
      pub fn get_credit_line(env: Env, borrower: Address) -> CreditLine { env.storage().instance().get(&DataKey::CreditLine(borrower)).expect("Credit line not found") }
}
