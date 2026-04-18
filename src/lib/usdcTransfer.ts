import { Asset, Operation, TransactionBuilder, Horizon, Transaction } from '@stellar/stellar-sdk';
import { env } from '../env';

/**
 * Builds a USDC payment operation for Stellar Classic.
 */
export function buildUSDCPayment(destination: string, amount: string, source?: string) {
      return Operation.payment({
              destination,
              asset: new Asset('USDC', env.USDC_CONTRACT_ID),
              amount,
              source,
      });
}

/**
 * High-level utility to build, sign and submit a USDC transfer.
 * Note: This assumes a Horizon server is accessible at the base of the RPC URL.
 */
export async function transferUSDC(
      from: string,
      to: string,
      amount: string,
      signTransaction: (tx: Transaction) => Promise<Transaction>
    ): Promise<string> {
      const horizonUrl = env.SOROBAN_RPC_URL.includes('soroban') 
    ? env.SOROBAN_RPC_URL.replace(/soroban\/rpc\/?$/, '') 
              : env.SOROBAN_RPC_URL;

  const server = new Horizon.Server(horizonUrl);

  try {
          const account = await server.loadAccount(from);
          const asset = new Asset('USDC', env.USDC_CONTRACT_ID);

        const transaction = new TransactionBuilder(account, {
                  fee: Horizon.FeeStats.MAX_FEE_AT_PEAK || '100000',
                  networkPassphrase: env.NETWORK_PASSPHRASE,
        })
            .addOperation(Operation.payment({
                        destination: to,
                        asset,
                        amount,
            }))
            .setTimeout(60)
            .build();

        const signedTx = await signTransaction(transaction);
          const result = await server.submitTransaction(signedTx);
          return result.hash;
  } catch (error: any) {
          const opResult = error.response?.data?.extras?.result_codes?.operations?.[0];
          if (opResult === 'op_underfunded') {
                    throw new Error('Insufficient USDC balance');
          }
          if (opResult === 'op_no_trust') {
                    throw new Error('Missing USDC trustline');
          }
          throw error;
  }
}
