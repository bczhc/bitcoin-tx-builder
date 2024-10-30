import * as wasm from 'wasm-lib';

export function useWasm() {
    return wasm
}

export function to_hex_padded(byte: number, padded_length: number) {
    let s = byte.toString(16);
    if (s.length < padded_length) {
        return '0'.repeat(padded_length - s.length) + s;
    }
    return s;
}

export function stringifyFallible(f: () => any): string {
    try {
        return f().toString();
    } catch (e: any) {
        return e.toString();
    }
}

export function safeParseInt(x: string) {
    let result = parseInt(x);
    if (isNaN(result)) {
        return 0;
    }
    return result;
}
