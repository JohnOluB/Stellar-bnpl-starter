#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec, token, Symbol, Map};

/// Represents a single payment installment in a loan schedule.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Installment {
    pub amount: i128,
    pub due_date: u64,
    pub paid_at: u64, // 0 if not paid
}

/// The lifecycle status of a loan.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LoanStatus {
    Active,
    Completed,
    Defaulted,
}

/// The core loan structure holding all terms and installment data.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Loan {
    pub borrower: Address,
    pub lender: Address,
    pub merchant: Address,
    pub total_amount: i128,
    pub installments: Vec<Installment>,
    pub status: LoanStatus,
}

/// Storage keys for the repayment contract.
#[contracttype]
pub enum DataKey {
    Loan(u64), // loan_id
    LastLoanId,
}

#[contract]
pub struct RepaymentContract;

#[contractimpl]
impl RepaymentContract {
    /// Create a new loan with a set of installments.
    /// 
    /// ### Arguments
    /// * `borrower` - The address of the borrower receiving the credit.
    /// * `lender` - The address of the lender funding the loan.
    /// * `merchant` - The address of the merchant receiving payment.
    /// * `installments` - A list of installments defining the repayment schedule.
    pub fn create_loan(
        env: Env,
        borrower: Address,
        lender: Address,
        merchant: Address,
        installments: Vec<Installment>,
    ) -> u64 {
        lender.require_auth();

        if installments.is_empty() {
            panic!("Installments cannot be empty");
        }

        let mut last_id: u64 = env.storage().instance().get(&DataKey::LastLoanId).unwrap_or(0);
        last_id += 1;

        let mut total_amount: i128 = 0;
        for i in 0..installments.len() {
            let inst = installments.get(i).unwrap();
            if inst.amount <= 0 {
                panic!("Installment amount must be positive");
            }
            total_amount += inst.amount;
        }

        let loan = Loan {
            borrower,
            lender,
            merchant,
            total_amount,
            installments,
            status: LoanStatus::Active,
        };

        env.storage().instance().set(&DataKey::Loan(last_id), &loan);
        env.storage().instance().set(&DataKey::LastLoanId, &last_id);

        env.events().publish(
            (Symbol::new(&env, "loan_created"), last_id),
            total_amount
        );

        last_id
    }

    /// Make a payment for a specific installment of a loan.
    /// 
    /// ### Arguments
    /// * `borrower` - The address of the borrower making the payment.
    /// * `loan_id` - The unique identifier of the loan.
    /// * `installment_idx` - The index of the installment being paid.
    /// * `token_address` - The address of the token used for payment.
    pub fn pay(env: Env, borrower: Address, loan_id: u64, installment_idx: u32, token_address: Address) {
        borrower.require_auth();

        let mut loan: Loan = env.storage().instance().get(&DataKey::Loan(loan_id)).expect("Loan not found");
        
        if loan.borrower != borrower {
            panic!("Unauthorized borrower");
        }

        if let LoanStatus::Active = loan.status {
            // Proceed
        } else {
            panic!("Loan is not active");
        }

        let mut installments = loan.installments.clone();
        let mut installment = installments.get(installment_idx).expect("Invalid installment index");

        if installment.paid_at > 0 {
            panic!("Installment already paid");
        }

        // Perform token transfer from borrower to lender
        let token_client = token::Client::new(&env, &token_address);
        token_client.transfer(&borrower, &loan.lender, &installment.amount);

        // Update installment
        installment.paid_at = env.ledger().timestamp();
        installments.set(installment_idx, installment);
        loan.installments = installments;

        // Check if all installments are paid
        let mut all_paid = true;
        for i in 0..loan.installments.len() {
            if loan.installments.get(i).unwrap().paid_at == 0 {
                all_paid = false;
                break;
            }
        }

        if all_paid {
            loan.status = LoanStatus::Completed;
            env.events().publish(
                (Symbol::new(&env, "loan_completed"), loan_id),
                loan.total_amount
            );
        }

        env.storage().instance().set(&DataKey::Loan(loan_id), &loan);

        env.events().publish(
            (Symbol::new(&env, "payment_received"), loan_id, installment_idx),
            installment.amount
        );
    }

    /// Query loan details by ID.
    pub fn get_loan(env: Env, loan_id: u64) -> Loan {
        env.storage().instance().get(&DataKey::Loan(loan_id)).expect("Loan not found")
    }
}
