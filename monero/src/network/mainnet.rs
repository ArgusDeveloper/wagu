use super::*;
use crate::format::MoneroFormat;
use wagyu_model::no_std::vec;
use wagyu_model::{AddressError, Network, NetworkError};

use core::{fmt, str::FromStr};
use serde::Serialize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Mainnet;

impl Network for Mainnet {
    const NAME: &'static str = "mainnet";
}

impl MoneroNetwork for Mainnet {
    /// Returns the address prefix of the given network.
    /// https://github.com/monero-project/monero/blob/3ad4ecd4ff52f011ee94e0e80754b965b82f072b/src/cryptonote_config.h#L153&L155
    fn to_address_prefix(format: &MoneroFormat) -> u8 {
        match format {
            MoneroFormat::Standard => 18,
            MoneroFormat::Integrated(_) => 19,
            MoneroFormat::Subaddress(_, _) => 42,
        }
    }

    /// Returns the network of the given address prefix.
    /// https://github.com/monero-project/monero/blob/3ad4ecd4ff52f011ee94e0e80754b965b82f072b/src/cryptonote_config.h#L153&L155
    fn from_address_prefix(prefix: u8) -> Result<Self, AddressError> {
        match prefix {
            18 | 19 | 42 => Ok(Self),
            _ => Err(AddressError::InvalidPrefix(vec![prefix])),
        }
    }
}

impl FromStr for Mainnet {
    type Err = NetworkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            Self::NAME => Ok(Self),
            _ => Err(NetworkError::InvalidNetwork(s.into())),
        }
    }
}

impl fmt::Display for Mainnet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
