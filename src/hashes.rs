use digest::Digest;
use sha1::Sha1;
use strum::EnumString;

pub fn ripemd160(data: &[u8]) -> [u8; 160 / 8] {
    ripemd::Ripemd160::digest(data).into()
}

pub fn sha256(data: &[u8]) -> [u8; 256 / 8] {
    sha2::Sha256::digest(data).into()
}

pub fn sha256d(data: &[u8]) -> [u8; 256 / 8] {
    sha256(&sha256(data))
}

pub fn sha1(data: &[u8]) -> [u8; 160 / 8] {
    Sha1::digest(data).into()
}

/// The HASH160 algorithm in Bitcoin.
///
/// HASH160 = RIPEMD160(SHA256(x))
pub fn hash160(data: &[u8]) -> [u8; 160 / 8] {
    ripemd160(&sha256(data))
}

#[derive(Debug, Copy, Clone, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum DigestType {
    Ripemd160,
    Sha1,
    Sha256,
    Sha256d,
    Hash160,
}
