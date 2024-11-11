import {safeToBigInt, SigningResult, useWasm} from "./lib.ts";

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
     * An array of witness item hex strings.
     *
     * <data1>, <data2>, ...
     */
    witness: string[]
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
        witness: [],
    };
}

export function defaultTx(): Transaction {
    return {
        version: 1,
        lockTime: 0,
        in: [],
        out: [],
    }
}

export function defaultTxOut(): TxOut {
    return {
        amount: 0,
        scriptPubKey: '',
    };
}

export type SigningType = 'legacy' | 'p2wpkh' | 'p2wsh';

export interface SignatureParams {
    signingType: SigningType,
    txoScriptPubKey: string,
    sighashTypeName: SigHashKey,
    secretKey: string,
    /**
     * only used in p2wpkh & p2wsh (segwit)
     */
    amount?: string,
    /**
     * only used in p2wsh
     */
    witnessScript?: string,
}

export const SIGHASH_OPTIONS: { label: string, value: SigHashKey, code: number }[] = [
    {label: 'All - 0x01', value: 'All', code: 0x01},
    {label: 'None - 0x02', value: 'None', code: 0x02},
    {label: 'Single - 0x03', value: 'Single', code: 0x03},
    {label: 'All+AnyoneCanPay - 0x81', value: 'AllPlusAnyoneCanPay', code: 0x81},
    {label: 'None+AnyoneCanPay - 0x82', value: 'NonePlusAnyoneCanPay', code: 0x82},
    {label: 'Single+AnyoneCanPay - 0x83', value: 'SinglePlusAnyoneCanPay', code: 0x83},
];

export function sigHashCode(value: SigHashKey) {
    return SIGHASH_OPTIONS.find(x => x.value === value)!!.code;
}

export type SigHashKey = 'All' | 'None' | 'Single'
    | 'AllPlusAnyoneCanPay' | 'NonePlusAnyoneCanPay' | 'SinglePlusAnyoneCanPay';

export function signForSignature(params: SignatureParams, tx: Transaction, index: number) {
    let wasm = useWasm();
    let amount = safeToBigInt(params.amount || '');
    let signature = wasm.TxBuilder.sign_tx(
        JSON.stringify(tx),
        index,
        params.txoScriptPubKey,
        sigHashCode(params.sighashTypeName),
        params.secretKey,
        params.witnessScript || '',
        amount,
        params.signingType,
    );
    let publicKey = wasm.TxBuilder.secret_to_public_key_compressed(params.secretKey);
    let result: SigningResult = {
        signature,
        publicKey,
    };
    return result;
}

export function validateTxSerialization(tx: Transaction) {
    try {
        useWasm().TxBuilder.json_to_tx_hex(JSON.stringify(tx))
    } catch (e: any) {
        throw 'Transaction serialization failed. Please fill the missing fields.';
    }
}

export type NetworkType = 'bitcoin' | 'testnet' | 'testnet4' | 'sigtest' | 'regtest';
export let GLOBAL_NETWORK: NetworkType = 'bitcoin';

export const updateNetwork = (x: NetworkType) => {
    GLOBAL_NETWORK = x;
    console.log(x);
}
