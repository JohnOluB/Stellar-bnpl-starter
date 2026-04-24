#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};

/// Detailed information about a borrower's credit line.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreditLine {
    pub limit: i128,
    pub used: i128,
    pub is_active: bool,
}

/// Storage keys for the credit line contract.
#[contracttype]
pub enum DataKey {
    CreditLine(Address),
    Admin,
}

#[contract]
pub struct CreditLineContract;

#[contractimpl]
impl CreditLineContract {
    /// Initialize the contract with an admin address.
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    /// Open or update a credit line for a borrower. Only callable by Admin.
    /// 
    /// ### Arguments
    /// * `borrower` - The address of the borrower.
    /// * `limit` - The new credit limit for the borrower.
    pub fn set_limit(env: Env, borrower: Address, limit: i128) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).expect("Not initialized");
        admin.require_auth();

        if limit < 0 {
            panic!("Limit cannot be negative");
        }

        let mut line = env.storage().instance().get(&DataKey::CreditLine(borrower.clone()))
            .unwrap_or(CreditLine { limit: 0, used: 0, is_active: true });
        
        line.limit = limit;
        line.is_active = true;

        env.storage().instance().set(&DataKey::CreditLine(borrower.clone()), &line);

        env.events().publish(
            (Symbol::new(&env, "limit_set"), borrower),
            limit
        );
    }

    /// Draw down from the credit line.
    /// 
    /// ### Arguments
    /// * `borrower` - The address of the borrower drawing credit.
    /// * `amount` - The amount to draw.
    pub fn draw(env: Env, borrower: Address, amount: i128) {
        borrower.require_auth();

        if amount <= 0 {
            panic!("Draw amount must be positive");
        }

        let mut line: CreditLine = env.storage().instance().get(&DataKey::CreditLine(borrower.clone()))
            .expect("No credit line found");

        if !line.is_active {
            panic!("Credit line is inactive");
        }

        if line.used + amount > line.limit {
            panic!("Insufficient credit limit");
        }

        line.used += amount;
        env.storage().instance().set(&DataKey::CreditLine(borrower.clone()), &line);

        env.events().publish(
            (Symbol::new(&env, "credit_draw"), borrower),
            amount
        );
    }

    /// Repay an amount to free up credit. 
    /// This should be called by the repayment contract or the admin upon payment receipt.
    /// 
    /// ### Arguments
    /// * `borrower` - The address of the borrower whose credit is being freed.
    /// * `amount` - The amount repaid.
    pub fn repay(env: Env, borrower: Address, amount: i128) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).expect("Not initialized");
        admin.require_auth(); // Tightened: only admin (or authorized repayment contract) can credit back

        if amount <= 0 {
            panic!("Repay amount must be positive");
        }

        let mut line: CreditLine = env.storage().instance().get(&DataKey::CreditLine(borrower.clone()))
            .expect("No credit line found");

        line.used = if line.used > amount { line.used - amount } else { 0 };
        env.storage().instance().set(&DataKey::CreditLine(borrower.clone()), &line);

        env.events().publish(
            (Symbol::new(&env, "credit_repay"), borrower),
            amount
        );
    }

    /// Get credit line details for a borrower.
    pub fn get_line(env: Env, borrower: Address) -> CreditLine {
        env.storage().instance().get(&DataKey::CreditLine(borrower))
            .expect("No credit line found")
    }
}
