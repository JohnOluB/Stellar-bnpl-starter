#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, token, Symbol};

/// Storage keys for the escrow contract.
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
          Lender,
          Borrower,
          Merchant,
          RepaymentContract,
          Amount,
          Token,
          IsReleased,
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
          /// Initialize the escrow with the participants and the amount.
    /// 
    /// ### Arguments
    /// * `lender` - The address of the lender who funded the escrow.
    /// * `borrower` - The address of the borrower.
    /// * `merchant` - The address of the merchant who will receive the funds.
    /// * `repayment_contract` - The address of the repayment contract authorized to release funds.
    /// * `token` - The address of the token (e.g., USDC) being held.
    /// * `amount` - The total amount to be held in escrow.
    pub fn initialize(
                  env: Env,
                  lender: Address,
                  borrower: Address,
                  merchant: Address,
                  repayment_contract: Address,
                  token: Address,
                  amount: i128,
              ) {
                  if env.storage().instance().has(&DataKey::Lender) {
                                    panic!("Already initialized");
                  }
                  if amount <= 0 {
                                    panic!("Amount must be positive");
                  }

              env.storage().instance().set(&DataKey::Lender, &lender);
                  env.storage().instance().set(&DataKey::Borrower, &borrower);
                  env.storage().instance().set(&DataKey::Merchant, &merchant);
                  env.storage().instance().set(&DataKey::RepaymentContract, &repayment_contract);
                  env.storage().instance().set(&DataKey::Token, &token);
                  env.storage().instance().set(&DataKey::Amount, &amount);
                  env.storage().instance().set(&DataKey::IsReleased, &false);
    }

    /// Release funds to the merchant. 
    /// This should only be called by the repayment contract once all installments are verified.
    pub fn release(env: Env) {
                  let repayment_contract: Address = env.storage().instance().get(&DataKey::RepaymentContract).unwrap();
                  repayment_contract.require_auth();

              let is_released: bool = env.storage().instance().get(&DataKey::IsReleased).unwrap();
                  if is_released {
                                    panic!("Funds already released");
                  }

              let merchant: Address = env.storage().instance().get(&DataKey::Merchant).unwrap();
                  let token: Address = env.storage().instance().get(&DataKey::Token).unwrap();
                  let amount: i128 = env.storage().instance().get(&DataKey::Amount).unwrap();

              let token_client = token::Client::new(&env, &token);
                  token_client.transfer(&env.current_contract_address(), &merchant, &amount);

              env.storage().instance().set(&DataKey::IsReleased, &true);

              env.events().publish(
                                (Symbol::new(&env, "release"), merchant),
                                amount
                            );
    }

    /// Refund funds to the lender in case of a major default or cancellation.
    /// 
    /// This can be called by the lender to reclaim funds if the terms are not met.
    pub fn refund(env: Env) {
                  let lender: Address = env.storage().instance().get(&DataKey::Lender).unwrap();
                  lender.require_auth();

              let is_released: bool = env.storage().instance().get(&DataKey::IsReleased).unwrap();
                  if is_released {
                                    panic!("Funds already released");
                  }

              let token: Address = env.storag
