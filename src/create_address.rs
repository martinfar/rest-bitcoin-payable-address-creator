use std::fmt;
use rust_base58::ToBase58;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use crypto::ripemd160::Ripemd160;
use bech32::{self, ToBase32, WriteBase32};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum Payload {
    PubkeyHash(Vec<u8>),
    ScriptHash(Vec<u8>),
    WitnessProgram {
        version: u8,
        program: Vec<u8>,
    },
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Network {
    Mainnet,
    Testnet
}

#[derive(Serialize, Deserialize)]
pub struct BitcoinAddress {
    network: Network,
    payload: Payload
}

impl fmt::Display for BitcoinAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.payload {
            Payload::PubkeyHash(payload) | Payload::ScriptHash(payload) => {
                self.format_base58_address(f, payload)
            },
            Payload::WitnessProgram { version, program } => {
                self.format_bech32_address(f, *version, program)
            }
        }
    }
}

impl BitcoinAddress {
    pub fn p2pkh(public_key: &[u8], network: Network) -> Self {
        Self::new(network, Payload::PubkeyHash(hash160(public_key)))
    }

    pub fn p2sh(script: &[u8], network: Network) -> Self {
        Self::new(network, Payload::ScriptHash(hash160(script)))
    }

    pub fn p2wpkh(public_key: &[u8], network: Network) -> Self {
        Self::new(network, Payload::WitnessProgram {
            version: 0,
            program: hash160(public_key),
        })
    }

    pub fn p2wsh(script: &[u8], network: Network) -> Self {
        Self::new(network, Payload::WitnessProgram {
            version: 0,
            program: sha256(script),
        })
    }

    fn new(network: Network, payload: Payload) -> Self {
        Self { network, payload }
    }

    fn format_base58_address(&self, f: &mut fmt::Formatter, payload: &[u8]) -> fmt::Result {
        let prefix = match (&self.network, &self.payload) {
            (Network::Mainnet, Payload::PubkeyHash(_)) => vec![0x00],
            (Network::Testnet, Payload::PubkeyHash(_)) => vec![0x6F],
            (Network::Mainnet, Payload::ScriptHash(_)) => vec![0x05],
            (Network::Testnet, Payload::ScriptHash(_)) => vec![0xC4],
            _ => unreachable!(),
        };

        let mut address = prefix;
        address.extend_from_slice(payload);
        let checksum = double_sha256(&address);
        address.extend_from_slice(&checksum[..4]);

        write!(f, "{}", address.to_base58())
    }

    fn format_bech32_address(&self, f: &mut fmt::Formatter, version: u8, program: &[u8]) -> fmt::Result {
        let prefix = match self.network {
            Network::Mainnet => "bc",
            Network::Testnet => "tb",
        };

        let mut bech32_writer = bech32::Bech32Writer::new(prefix, f)?;
        bech32_writer.write_u5(bech32::u5::try_from_u8(version).unwrap())?;
        for b in program.to_base32() {
            bech32_writer.write_u5(b)?;
        }
        Ok(())
    }
}

fn double_sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    let mut hash = vec![0; hasher.output_bytes()];
    hasher.input(data);
    hasher.result(&mut hash);
    hasher.reset();
    hasher.input(&hash);
    hasher.result(&mut hash);
    hash
}

fn hash160(data: &[u8]) -> Vec<u8> {
    let sha256 = sha256(data);
    ripemd160(&sha256)
}

fn sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    let mut hash = vec![0; hasher.output_bytes()];
    hasher.input(data);
    hasher.result(&mut hash);
    hash
}

fn ripemd160(data: &[u8]) -> Vec<u8> {
    let mut ripemder = Ripemd160::new();
    let mut hash = vec![0; ripemder.output_bytes()];
    ripemder.input(data);
    ripemder.result(&mut hash);
    hash
}