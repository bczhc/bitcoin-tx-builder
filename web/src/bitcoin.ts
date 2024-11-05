export interface Transaction {
    version: number,
    lockTime: number,
    in: TxIn[],
    out: TxOut[],
}

export interface TxIn {
    outpointTxId: string,
    outpointIndex: number,
    sequence: number,
    scriptSig: string,
    /**
     * Witness arrays separated by commas
     *
     * <data1>, <data2>, ...
     */
    witness: string,
}

export interface TxOut {
    amount: number,
    scriptPubKey: string,
}

export const CHECK_DIGITS = x => /^\d*$/.test(x);

export function defaultTxIn(): TxIn {
    return {
        outpointTxId: '',
        outpointIndex: 0,
        scriptSig: '',
        sequence: 0xffffffff,
        witness: '',
    };
}

export function defaultTxOut(): TxOut {
    return {
        amount: 0,
        scriptPubKey: '',
    };
}

export type NetworkType = 'bitcoin' | 'testnet' | 'testnet4' | 'sigtest' | 'regtest';
export let GLOBAL_NETWORK: NetworkType = 'bitcoin';

export const updateNetwork = (x: NetworkType) => {
    GLOBAL_NETWORK = x;
    console.log(x);
}
