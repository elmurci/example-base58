use anyhow::{Context, Result};

#[derive(Debug)]
pub enum Version {
    NodePublic,
    NodePrivate,
}

impl From<Version> for u8 {
    fn from(version: Version) -> Self {
        match version {
            Version::NodePublic => 28,
            Version::NodePrivate => 32,
        }
    }
}

pub fn base58_to_hex(base58_string: &str, version: Version) -> Result<String> {
    let decb58 = base58_decode(version, base58_string).context("Invalid base58 string")?;
    Ok(hex::encode(decb58))
}

pub fn base58_encode<I: AsRef<[u8]>>(input: I) -> String {
    bs58::encode(input)
        .with_alphabet(bs58::Alphabet::RIPPLE)
        .into_string()
}

pub fn base58_decode<I: AsRef<[u8]>>(version: Version, input: I) -> bs58::decode::Result<Vec<u8>> {
    bs58::decode(input)
        .with_alphabet(bs58::Alphabet::RIPPLE)
        .with_check(Some(version.into()))
        .into_vec()
        .map(|mut vec| {
            let _ = vec.remove(0);
            vec
        })
}
