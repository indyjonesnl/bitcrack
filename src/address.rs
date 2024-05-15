extern crate bitcoin;

use bitcoin::address::Address;
use bitcoin::key::{PrivateKey, PublicKey};
use bitcoin::network::Network;
use bitcoin::secp256k1::{Secp256k1, SecretKey, Signing};
use std::str::FromStr;

pub struct BitcoinAddress {
    pub private_key: PrivateKey,
    pub public_key: PublicKey,
    pub address: Option<Address>,
    pub hex: String,
}

impl BitcoinAddress {
    pub fn new(secp: &Secp256k1<impl Signing>, hex: &str, create_address: bool) -> Self {
        let secret_key = SecretKey::from_str(hex).unwrap();

        let private_key = PrivateKey {
            compressed: true,
            network: Network::Bitcoin,
            inner: secret_key,
        };

        let public_key = PublicKey::from_private_key(secp, &private_key);

        if create_address {
            return Self {
                private_key,
                public_key,
                address: Some(Address::p2pkh(&public_key, Network::Bitcoin)),
                hex: hex.to_string(),
            };
        }

        Self {
            private_key,
            public_key,
            address: None,
            hex: hex.to_string(),
        }
    }
}
