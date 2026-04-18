/**
 * Utilities for handling USDC amounts on Stellar.
 * USDC has 7 decimal places (stroops) on Stellar.
 */

const STROOP_DECIMALS = 7;
const STROOPS_PER_USDC = BigInt(10 ** STROOP_DECIMALS);

/**
 * Converts a USDC amount (human readable) to stroops (bigint).
 * @param amount The amount in USDC (e.g., "10.5" or 10.5)
 * @returns The amount in stroops as a BigInt
 */
export function toStroops(amount: string | number): bigint {
    const value = typeof amount === 'number' ? amount.toString() : amount;
    const [integers, decimals = ''] = value.split('.');

  let stroopsStr = integers + decimals.padEnd(STROOP_DECIMALS, '0').slice(0, STROOP_DECIMALS);
    return BigInt(stroopsStr);
}

/**
 * Converts stroops to a human-readable USDC amount.
 * @param stroops The amount in stroops (bigint or string)
 * @returns The amount in USDC as a number
 */
export function fromStroops(stroops: bigint | string): number {
    const value = typeof stroops === 'string' ? BigInt(stroops) : stroops;
    return Number(value) / Number(STROOPS_PER_USDC);
}

/**
 * Formats a USDC amount for display.
 * @param amount The amount in USDC
 * @returns A formatted string (e.g., "$1,234.56")
 */
export function formatUSDC(amount: number): string {
    return new Intl.NumberFormat('en-US', {
          style: 'currency',
          currency: 'USD',
          minimumFractionDigits: 2,
          maximumFractionDigits: 2,
    }).format(amount);
}
