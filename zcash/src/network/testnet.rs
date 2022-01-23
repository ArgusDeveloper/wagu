use super::*;
use crate::format::ZcashFormat;
use wagyu_model::no_std::vec;
use wagyu_model::{AddressError, ChildIndex, Network, NetworkError, PrivateKeyError};

use core::{fmt, str::FromStr};
use serde::Serialize;
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Testnet;

impl Network for Testnet {
    const NAME: &'static str = "testnet";
}

impl ZcashNetwork for Testnet {
    const HD_COIN_TYPE: ChildIndex = ChildIndex::Hardened(1);

    /// Returns the address prefix of the given network.
    fn to_address_prefix(format: &ZcashFormat) -> Vec<u8> {
        match format {
            ZcashFormat::P2PKH => vec![0x1D, 0x25],
            ZcashFormat::P2SH => vec![0x1C, 0xBA],
            ZcashFormat::Sprout => vec![0x16, 0xB6],
            ZcashFormat::Sapling(_) => "ztestsapling".as_bytes().to_vec(),
        }
    }

    /// Returns the network of the given address prefix.
    fn from_address_prefix(prefix: &Vec<u8>) -> Result<Self, AddressError> {
        if prefix.len() < 2 {
            return Err(AddressError::InvalidPrefixLength(prefix.len()));
        }

        match prefix[1] {
            0x25 | 0xBA | 0xB6 | 0x74 => Ok(Self),
            _ => return Err(AddressError::InvalidPrefix(prefix.clone())),
        }
    }

    /// Returns the WIF prefix of the given network.
    fn to_wif_prefix() -> u8 {
        0xEF
    }

    /// Returns the network of the given WIF prefix.
    fn from_wif_prefix(prefix: u8) -> Result<Self, PrivateKeyError> {
        match prefix {
            0xEF => Ok(Self),
            _ => return Err(PrivateKeyError::InvalidPrefix(vec![prefix])),
        }
    }

    /// Returns the prefix for a Sprout spending key.
    fn to_sprout_spending_key_prefix() -> [u8; 2] {
        [0xAC, 0x08]
    }

    /// Returns the prefix for a Sprout viewing key.
    fn to_sprout_viewing_key_prefix() -> [u8; 3] {
        [0xA8, 0xAC, 0x0C]
    }

    /// Returns the Sapling spending key prefix of the given network.
    fn to_sapling_spending_key_prefix() -> String {
        "secret-spending-key-test".into()
    }

    /// Returns the Sapling viewing key prefix of the given network.
    fn to_sapling_viewing_key_prefix() -> String {
        "zviewtestsapling".into()
    }

    /// Returns the extended private key prefix of the given network.
    /// https://github.com/zcash/zips/blob/master/zip-0032.rst#sapling-extended-spending-keys
    fn to_extended_private_key_prefix() -> String {
        "secret-extended-key-test".into()
    }

    /// Returns the network of the given extended private key prefix.
    /// https://github.com/zcash/zips/blob/master/zip-0032.rst#sapling-extended-spending-keys
    fn from_extended_private_key_prefix(prefix: &str) -> Result<Self, NetworkError> {
        match prefix {
            "secret-extended-key-test" => Ok(Self),
            _ => return Err(NetworkError::InvalidExtendedPrivateKeyPrefix(prefix.into())),
        }
    }

    /// Returns the extended public key prefix of the given network.
    /// https://github.com/zcash/zips/blob/master/zip-0032.rst#sapling-extended-full-viewing-keys
    fn to_extended_public_key_prefix() -> String {
        "zxviewtestsapling".into()
    }

    /// Returns the network of the given extended public key prefix.
    /// https://github.com/zcash/zips/blob/master/zip-0032.rst#sapling-extended-full-viewing-keys
    fn from_extended_public_key_prefix(prefix: &str) -> Result<Self, NetworkError> {
        match prefix {
            "zxviewtestsapling" => Ok(Self),
            _ => return Err(NetworkError::InvalidExtendedPublicKeyPrefix(prefix.into())),
        }
    }
}

impl FromStr for Testnet {
    type Err = NetworkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            Self::NAME => Ok(Self),
            _ => Err(NetworkError::InvalidNetwork(s.into())),
        }
    }
}

impl fmt::Display for Testnet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
