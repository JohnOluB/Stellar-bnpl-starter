import { z } from 'zod';

const envSchema = z.object({
    SOROBAN_RPC_URL: z.string().url(),
    NETWORK_PASSPHRASE: z.string(),
    CREDIT_LINE_CONTRACT_ID: z.string().regex(/^C[A-Z2-7]{55}$/),
    REPAYMENT_CONTRACT_ID: z.string().regex(/^C[A-Z2-7]{55}$/),
    ESCROW_CONTRACT_ID: z.string().regex(/^C[A-Z2-7]{55}$/),
    USDC_CONTRACT_ID: z.string().regex(/^C[A-Z2-7]{55}$/),
    NEXT_PUBLIC_APP_URL: z.string().url().default('http://localhost:3000'),
    NEXT_PUBLIC_NETWORK: z.enum(['testnet', 'mainnet']).default('testnet'),
    PORT: z.coerce.number().default(3000),
    NODE_ENV: z.enum(['development', 'production', 'test']).default('development'),
});

export const env = envSchema.parse(process.env);
