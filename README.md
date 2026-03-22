# stellar-bnpl-starter

> A full-stack Buy Now Pay Later starter kit built on Stellar and Soroban —
> on-chain credit lines, installment repayments, and wallet-connected checkout,
> ready to fork and deploy.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](http://makeapullrequest.com)
[![Drips Wave Program](https://img.shields.io/badge/Drips-Wave%20Program-8B5CF6)](https://drips.network)

---

## Overview

`stellar-bnpl-starter` gives developers and fintech builders a complete, forkable
foundation for launching a Buy Now Pay Later product on Stellar. It ships with three
modular Soroban contracts (credit line, repayment schedule, escrow), a Next.js frontend
covering the full borrower and lender journey, and a TypeScript SDK layer tying
everything together.

Fork it, customize the credit scoring logic, plug in your own merchant flow, and ship.

---

## Technical Architecture

- **Frontend:** Next.js 14 App Router with TypeScript and Tailwind CSS — three
  distinct route groups: `/checkout` for the borrower purchase flow, `/dashboard/borrower`
  for repayment tracking, and `/dashboard/lender` for liquidity and yield management
- **Credit Line Contract:** Rust/Soroban contract managing per-wallet credit
  allocations — open credit line, draw down, close, and query available credit;
  configurable credit limit and interest rate parameters
- **Repayment Contract:** Tracks installment schedules on-chain — stores due dates,
  amounts, and payment status per loan ID; emits events on payment receipt and default
- **Escrow Contract:** Holds disbursed USDC between lender funding and merchant
  release — integrates with the repayment contract to unlock funds upon confirmed
  installment completion
- **TypeScript Client:** Typed SDK classes for all three contracts, built on
  `@stellar/stellar-sdk`, with `@creit.tech/stellar-wallets-kit` for wallet
  connection across Freighter, xBull, and Lobstr

---

## 💧 Drips Wave Program

This repository is an active participant in the
**[Drips Wave Program](https://drips.network)** — a funding mechanism that rewards
open-source contributors for resolving scoped GitHub issues with on-chain streaming
payments.

### How to Contribute & Earn

**Step 1 — Register on Drips**
Visit [drips.network](https://drips.network) and connect your Ethereum-compatible wallet.
Your wallet address is where reward streams will be sent.

**Step 2 — Browse Open Issues**
Head to the [Issues tab](../../issues). Issues are labeled by complexity tier:

| Label           | Complexity | Typical Scope                                                                  |
|-----------------|------------|--------------------------------------------------------------------------------|
| `drips:trivial` | Trivial    | UI copy fix, loading state improvement, add a contract view function           |
| `drips:medium`  | Medium     | New repayment schedule type, borrower dashboard chart, contract event handler  |
| `drips:high`    | High       | Credit scoring module, lender liquidity pool, multi-currency support           |

**Step 3 — Claim an Issue**
Comment `/claim` on the issue you want. The maintainer will assign it.
One active claim per contributor at a time.

**Step 4 — Submit Your Work**
Open a Pull Request with `Closes #XX`. Contract changes must include Rust unit tests.
Frontend changes must be responsive and include component-level tests.

**Step 5 — Earn Rewards**
Your wallet begins receiving a continuous Drips stream upon PR merge.
No invoices, no delays.

---

## Project Structure
```
stellar-bnpl-starter/
├── src/
│   ├── app/
│   │   ├── checkout/             # Purchase flow: amount, installment plan, confirm
│   │   ├── dashboard/
│   │   │   ├── borrower/         # Active loans, upcoming payments, payment history
│   │   │   └── lender/           # Deployed capital, yield earned, active positions
│   │   └── repayment/            # Make payment UI, payment confirmation
│   ├── components/
│   │   ├── checkout/             # CheckoutStepper, InstallmentPicker, PaySummary
│   │   ├── dashboard/            # LoanCard, RepaymentSchedule, YieldChart
│   │   └── shared/               # WalletConnect, TxStatus, NetworkBadge
│   ├── hooks/                    # useCreditLine, useRepayment, useEscrow, useWallet
│   ├── lib/                      # Contract clients, Stellar SDK setup
│   ├── utils/                    # Installment calculators, date formatters, USDC helpers
│   └── styles/                   # Tailwind config, design tokens
├── contracts/
│   ├── src/
│   │   ├── credit_line/          # Credit allocation Soroban contract (Rust)
│   │   ├── repayment/            # Installment schedule Soroban contract (Rust)
│   │   └── escrow/               # Fund escrow Soroban contract (Rust)
│   └── tests/                    # Rust integration tests for all contracts
├── scripts/                      # Contract deploy scripts, testnet seed
├── tests/
│   ├── unit/                     # Hook logic, calculator utils, client mocks
│   └── integration/              # Full BNPL flow against local Stellar node
├── public/                       # Brand assets, token logos
├── config/                       # Zod env schemas
├── .env.example
├── package.json
├── tsconfig.json
└── README.md
```

---

## Quick Start
```bash
cp .env.example .env
# Fill in: SOROBAN_RPC_URL, NETWORK_PASSPHRASE, USDC_CONTRACT_ID

# Deploy contracts to testnet
npm run contracts:deploy

# Start the frontend
npm install && npm run dev
```

Open [http://localhost:3000/checkout](http://localhost:3000/checkout) to walk through
the borrower purchase flow.

---

## License

MIT © stellar-bnpl-starter contributors
