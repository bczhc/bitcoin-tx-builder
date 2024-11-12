#![feature(yeet_expr)]

use crate::errors::{AnyhowExt, ResultExt};
use crate::hashes::{hash160, ripemd160, sha1, sha256, sha256d, DigestType};
use anyhow::anyhow;
use bitcoin::address::script_pubkey;
use bitcoin::address::script_pubkey::{BuilderExt, ScriptExt as ScriptExt2};
use bitcoin::hashes::Hash;
use bitcoin::key::Secp256k1;
use bitcoin::opcodes::all::{OP_PUSHBYTES_1, OP_PUSHBYTES_75, OP_PUSHDATA1};
use bitcoin::script::{PushBytes, ScriptBufExt, ScriptExt};
use bitcoin::secp256k1::{Message, SecretKey};
use bitcoin::sighash::SighashCache;
use bitcoin::transaction::Version;
use bitcoin::{
    absolute, consensus, Address, Amount, EcdsaSighashType, KnownHrp, Network, OutPoint,
    PrivateKey, PublicKey, Script, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Witness,
};
use hex::FromHexError;
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use strum::{EnumString, IntoStaticStr};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Bitcoin;

#[wasm_bindgen]
impl Bitcoin {
    pub fn parse_script_hex(hex_string: &str) -> crate::Result<String> {
        let result: anyhow::Result<_> = try {
            let vec = parse_hex_lossy(hex_string)?;
            let script_buf = ScriptBuf::from(vec);
            script_buf.to_asm_string()
        };
        result.map_err_string()
    }

    pub fn base58_check(hex: &str) -> crate::Result<String> {
        let result: anyhow::Result<_> =
            try { bitcoin::base58::encode_check(&parse_hex_lossy(hex)?) };
        result.map_err_string()
    }

    pub fn parse_hex_str(hex: &str) -> crate::Result<Vec<u8>> {
        let result = parse_hex_lossy(hex);
        result.map_err_string()
    }

    pub fn digest(data: &[u8], name: &str) -> crate::Result<String> {
        let result: anyhow::Result<_> = try {
            let r#type = DigestType::from_str(name)?;
            match r#type {
                DigestType::Ripemd160 => hex::encode(ripemd160(data)),
                DigestType::Sha256 => hex::encode(sha256(data)),
                DigestType::Sha256d => hex::encode(sha256d(data)),
                DigestType::Hash160 => hex::encode(hash160(data)),
                DigestType::Sha1 => hex::encode(sha1(data)),
            }
        };
        result.map_err_string()
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsTx {
    version: u8,
    lock_time: u32,
    r#in: Vec<JsTxIn>,
    out: Vec<JsTxOut>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsTxIn {
    outpoint_tx_id: String,
    outpoint_index: u32,
    sequence: u32,
    script_sig: String,
    witness: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsTxOut {
    amount: u64,
    script_pub_key: String,
}

#[wasm_bindgen]
pub struct TxBuilder;

impl TryFrom<JsTx> for Transaction {
    type Error = anyhow::Error;

    fn try_from(tx: JsTx) -> Result<Self, Self::Error> {
        let tx = Transaction {
            version: Version(tx.version as i32),
            lock_time: absolute::LockTime::from_consensus(tx.lock_time),
            input: tx
                .r#in
                .iter()
                .map(|x| {
                    Ok::<_, anyhow::Error>(TxIn {
                        previous_output: OutPoint {
                            txid: x.outpoint_tx_id.parse()?,
                            vout: x.outpoint_index,
                        },
                        script_sig: ScriptBuf::from_hex(&x.script_sig)?,
                        sequence: Sequence(x.sequence),
                        witness: {
                            let mut w = Witness::new();
                            for x in &x.witness {
                                w.push(parse_hex_lossy(x)?);
                            }
                            w
                        },
                    })
                })
                .collect::<Result<_, _>>()?,
            output: tx
                .out
                .iter()
                .map(|x| {
                    Ok::<_, anyhow::Error>(TxOut {
                        value: Amount::from_sat(x.amount),
                        script_pubkey: ScriptBuf::from_hex(&x.script_pub_key)?,
                    })
                })
                .collect::<Result<_, _>>()?,
        };
        Ok(tx)
    }
}

impl TryFrom<TxIn> for JsTxIn {
    type Error = anyhow::Error;

    fn try_from(tx: TxIn) -> Result<Self, Self::Error> {
        Ok(JsTxIn {
            outpoint_tx_id: tx.previous_output.txid.to_string(),
            outpoint_index: tx.previous_output.vout,
            sequence: tx.sequence.0,
            script_sig: tx.script_sig.hex(),
            witness: tx.witness.iter().map(|x| x.hex()).collect(),
        })
    }
}

impl TryFrom<TxOut> for JsTxOut {
    type Error = anyhow::Error;

    fn try_from(tx: TxOut) -> Result<Self, Self::Error> {
        Ok(JsTxOut {
            amount: tx.value.to_sat(),
            script_pub_key: tx.script_pubkey.hex(),
        })
    }
}

impl TryFrom<Transaction> for JsTx {
    type Error = anyhow::Error;

    fn try_from(tx: Transaction) -> Result<Self, Self::Error> {
        let tx = JsTx {
            version: tx.version.0 as u8,
            lock_time: tx.lock_time.to_consensus_u32(),
            r#in: tx
                .input
                .into_iter()
                .map(JsTxIn::try_from)
                .collect::<Result<_, _>>()?,
            out: tx
                .output
                .into_iter()
                .map(JsTxOut::try_from)
                .collect::<Result<_, _>>()?,
        };
        Ok(tx)
    }
}

impl FromStr for JsTx {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tx: JsTx = serde_json::from_str(s)?;
        Ok(tx)
    }
}

trait EncodeHex {
    fn hex(&self) -> String;
}

impl<T> EncodeHex for T
where
    T: AsRef<[u8]>,
{
    fn hex(&self) -> String {
        hex::encode(self.as_ref())
    }
}

/// Returns the bitcoin signature: `(<ecdsa-signature> <sighash-flag>)`
pub fn sign_sighash(
    sighash: impl AsRef<[u8]>,
    secret: &SecretKey,
    sighash_flag: EcdsaSighashType,
) -> Vec<u8> {
    let message = Message::from_digest(sighash.as_ref().try_into().expect("Length must be 32"));
    let mut signature = Secp256k1::default()
        .sign_ecdsa(&message, secret)
        .serialize_der()
        .to_vec();
    signature.push(sighash_flag as u8);
    signature
}

/// Parses hex string ignoring all whitespaces.
fn parse_hex_lossy(hex: impl AsRef<str>) -> Result<Vec<u8>, FromHexError> {
    let filtered = hex
        .as_ref()
        .chars()
        .filter(|x| !x.is_whitespace())
        .collect::<String>();
    hex::decode(&filtered)
}

#[wasm_bindgen]
impl TxBuilder {
    pub fn json_to_tx_hex(json: &str) -> crate::Result<String> {
        let r: anyhow::Result<_> = try {
            let tx: JsTx = serde_json::from_str(json)?;
            let tx = Transaction::try_from(tx)?;
            tx.consensus_encode_hex()
        };
        r.map_err_string()
    }

    pub fn address_to_script_pub_key(addr: &str, network: &str) -> crate::Result<String> {
        let r: anyhow::Result<_> = try {
            let network = Network::from_str(network)?;
            let script = addr
                .parse::<Address<_>>()?
                .require_network(network)?
                .script_pubkey();
            script.hex()
        };
        r.map_err_string()
    }

    pub fn script_to_asm(hex: &str) -> crate::Result<String> {
        let r: anyhow::Result<_> =
            try { Script::from_bytes(&parse_hex_lossy(hex)?).to_asm_string() };
        r.map_err_string()
    }

    pub fn sign_tx(
        tx_json: &str,
        index: u32,
        txo_script_pubkey: &str,
        sighash_type: u32,
        secret_key: &str,
        // for p2wsh signing use
        witness_script: Option<String>,
        // for p2wsh and p2wpkh signing use
        amount: Option<u64>,
        signing_type: &str,
    ) -> crate::Result<String> {
        let r: anyhow::Result<_> = try {
            let tx: JsTx = tx_json.parse()?;
            let tx: Transaction = tx.try_into()?;
            let txo_script_pubkey = ScriptBuf::from_bytes(parse_hex_lossy(txo_script_pubkey)?);
            let secret = SecretKey::from_slice(&parse_hex_lossy(secret_key)?)?;
            let sighash_type = EcdsaSighashType::from_consensus(sighash_type);

            let signing_type = SigningType::from_str(signing_type)?;
            let sighash = match signing_type {
                SigningType::Legacy => {
                    let cache = SighashCache::new(tx);
                    let sighash = cache.legacy_signature_hash(
                        index as usize,
                        &txo_script_pubkey,
                        sighash_type as u32,
                    )?;
                    sighash.to_byte_array()
                }
                SigningType::P2wpkh => {
                    let mut cache = SighashCache::new(tx);
                    let sighash = cache.p2wpkh_signature_hash(
                        index as usize,
                        &txo_script_pubkey,
                        Amount::from_sat(amount.ok_or(anyhow!("Missing commitment amount"))?),
                        sighash_type,
                    )?;
                    sighash.to_byte_array()
                }
                SigningType::P2wsh => {
                    let mut cache = SighashCache::new(tx);
                    let sighash = cache.p2wsh_signature_hash(
                        index as usize,
                        &Self::parse_script_hex(
                            &witness_script.ok_or(anyhow!("Missing witness_script"))?,
                        )?,
                        Amount::from_sat(amount.ok_or(anyhow!("Missing commitment amount"))?),
                        sighash_type,
                    )?;
                    sighash.to_byte_array()
                }
            };
            let signature = sign_sighash(sighash, &secret, sighash_type);
            signature.hex()
        };
        r.map_err_string()
    }

    pub fn secret_to_public_key_compressed(secret: &str) -> crate::Result<String> {
        let r: anyhow::Result<_> = try {
            let ec = SecretKey::from_slice(&parse_hex_lossy(secret)?)?;
            let public = ec.public_key(&Default::default());
            public.serialize().hex()
        };
        r.map_err_string()
    }

    pub fn wif_to_ec_hex(wif: &str) -> crate::Result<String> {
        let r: anyhow::Result<_> = try {
            let private_key = PrivateKey::from_wif(wif)?;
            private_key.inner.as_ref().hex()
        };
        r.map_err_string()
    }

    pub fn wif_to_public_key(wif: &str) -> crate::Result<String> {
        let r: anyhow::Result<_> = try {
            let k = PrivateKey::from_wif(wif)?;
            let public_key = k.public_key(&Default::default());
            if public_key.compressed {
                public_key.inner.serialize().hex()
            } else {
                public_key.inner.serialize_uncompressed().hex()
            }
        };
        r.map_err_string()
    }

    pub fn generate_p2sh_pub_key(redeem_hex: &str) -> crate::Result<String> {
        let r: anyhow::Result<_> = try {
            let bytes = parse_hex_lossy(redeem_hex)?;
            let script = Script::from_bytes(&bytes);
            let p2sh = ScriptExt2::to_p2sh(script)?;
            p2sh.hex()
        };
        r.map_err_string()
    }

    pub fn script_sig_for_p2sh(script_sig_hex: &str, redeem_hex: &str) -> crate::Result<String> {
        let r: anyhow::Result<_> = try {
            let mut buf = ScriptBuf::from_bytes(parse_hex_lossy(script_sig_hex)?);
            buf.push_slice(<&PushBytes>::try_from(
                parse_hex_lossy(redeem_hex)?.as_slice(),
            )?);
            buf.hex()
        };
        r.map_err_string()
    }

    pub fn tx_hex_to_json(hex: &str) -> crate::Result<String> {
        let r: anyhow::Result<_> = try {
            let result: Result<Transaction, ConsensusErrorDesc> =
                consensus::deserialize(&parse_hex_lossy(hex)?).map_err(Into::into);
            let tx = result?;
            let tx = JsTx::try_from(tx)?;
            serde_json::to_string(&tx)?
        };
        r.map_err_string()
    }

    fn parse_script_hex(hex: &str) -> anyhow::Result<ScriptBuf> {
        Ok(Script::from_bytes(&parse_hex_lossy(hex)?).into())
    }

    pub fn create_p2wsh_address(witness_script: &str, network: &str) -> crate::Result<String> {
        let r: anyhow::Result<_> = try {
            let network: Network = network.parse()?;
            let address = Address::p2wsh(
                Script::from_bytes(&parse_hex_lossy(witness_script)?),
                KnownHrp::from(network),
            )?;
            address.to_string()
        };
        r.map_err_string()
    }

    pub fn create_p2wsh_script_pubkey(witness_script: &str) -> crate::Result<String> {
        let r: anyhow::Result<_> = try {
            let script = <ScriptBuf as script_pubkey::ScriptBufExt>::new_p2wsh(
                Self::parse_script_hex(witness_script)?.wscript_hash()?,
            );
            script.hex()
        };
        r.map_err_string()
    }

    pub fn create_p2wpkh_script_sig(signature: &str, pubkey: &str) -> crate::Result<String> {
        let r: anyhow::Result<_> = try {
            // <signature> <public-key>
            let script = ScriptBuf::builder()
                .push_slice_try_from(&parse_hex_lossy(signature)?)?
                .push_key(PublicKey::from_slice(&parse_hex_lossy(pubkey)?)?)
                .into_script();
            script.hex()
        };
        r.map_err_string()
    }

    pub fn script_info(script: &str) -> crate::Result<String> {
        let r: anyhow::Result<_> = try {
            let script = Self::parse_script_hex(script)?;
            let script_type = match () {
                _ if script.is_empty() => Some("Empty"),
                _ if script.is_p2sh() => Some("P2SH"),
                _ if script.is_p2tr() => Some("P2TR"),
                _ if script.is_p2wpkh() => Some("P2WPKH"),
                _ if script.is_p2wsh() => Some("P2WSH"),
                _ if script.is_p2pkh() => Some("P2PKH"),
                _ if script.is_p2pk() => Some("P2PK"),
                _ if script.is_op_return() => Some("OP_RETURN"),
                _ if script.is_witness_program() => Some("Witness Program"),
                _ => None,
            };
            let op_return_data =
                extract_op_return(&script).map(|x| String::from_utf8_lossy(x).to_string());
            let info = ScriptInfo {
                asm: &script.to_asm_string(),
                r#type: script_type.map(Into::into),
                op_return_data,
            };
            serde_json::to_string(&info).unwrap()
        };
        r.map_err_string()
    }

    pub fn op_return_script_pubkey(text: &str) -> crate::Result<String> {
        let r: anyhow::Result<_> =
            try { ScriptBuf::new_op_return(<&PushBytes>::try_from(text.as_bytes())?).hex() };
        r.map_err_string()
    }

    pub fn split_comma_hex(text: &str) -> crate::Result<Vec<String>> {
        let r: anyhow::Result<_> = try {
            text.split(',').map(|x| {
                parse_hex_lossy(x).map(|x| x.hex())
            }).collect::<Result<Vec<_>, _>>()?
        };
        r.map_err_string()
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ScriptInfo<'a> {
    r#type: Option<String>,
    asm: &'a str,
    op_return_data: Option<String>,
}

#[derive(EnumString, IntoStaticStr, Debug)]
#[strum(ascii_case_insensitive)]
enum SigningType {
    Legacy,
    P2wpkh,
    P2wsh,
}

pub trait ScriptsBuilderExt
where
    Self: Sized,
{
    fn push_slice_try_from(self, slice: &[u8]) -> anyhow::Result<Self>;
}

impl ScriptsBuilderExt for bitcoin::script::Builder {
    fn push_slice_try_from(self, slice: &[u8]) -> anyhow::Result<Self> {
        Ok(self.push_slice(<&PushBytes>::try_from(slice)?))
    }
}

trait ConsensusEncodeExt {
    fn consensus_encode_hex(&self) -> String;
}

impl<T> ConsensusEncodeExt for T
where
    T: consensus::Encodable,
{
    fn consensus_encode_hex(&self) -> String {
        consensus::encode::serialize(self).hex()
    }
}

#[derive(Debug)]
struct ConsensusErrorDesc(String);

impl Display for ConsensusErrorDesc {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ConsensusErrorDesc {}

impl From<consensus::encode::Error> for ConsensusErrorDesc {
    fn from(value: consensus::encode::Error) -> Self {
        Self(format!("{value}"))
    }
}

/// Extract the data from an `OP_RETURN` script.
pub fn extract_op_return(script: &Script) -> Option<&[u8]> {
    if !script.is_op_return() {
        return None;
    }

    let bytes = script.as_bytes();

    // merely OP_RETURN
    if bytes.len() == 1 {
        return None;
    }

    // OP_RETURN <OP_PUSHBYTES_1..=OP_PUSHBYTES_75> <data>
    if bytes.get(1).is_none() {
        return None;
    }
    if (OP_PUSHBYTES_1.to_u8()..=OP_PUSHBYTES_75.to_u8()).contains(&bytes[1]) {
        let pushed_len = (bytes[1] - OP_PUSHBYTES_1.to_u8() + 1) as usize;
        if bytes.len() - 2 < pushed_len {
            return None;
        }
        let data = &bytes[2..(2 + pushed_len)];
        return Some(data);
    }

    // OP_RETURN <OP_PUSHDATA1> <length> <data>
    if bytes.get(1).is_none() || bytes.get(2).is_none() {
        return None;
    }
    if bytes[1] == OP_PUSHDATA1.to_u8() {
        let len = bytes[2] as usize;
        if bytes.len() - 3 < len {
            return None;
        }
        return Some(&bytes[3..(3 + len)]);
    }

    None
}

#[cfg(test)]
mod test {}
