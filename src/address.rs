use std::sync::Arc;
use std::iter::FromIterator;
use std::net::{IpAddr, SocketAddr};

use {Weight};

/// Address that nameservice has returned
///
/// We hide this structure to allow future additions. There is `AddressBuilder`
/// which provides a forward compatible way to build such address in a resolver
/// and there are methods to extract data from it.
///
/// Internally it's an `Arc` over a structure so it's cheap to clone and you
/// can cache addresses.
#[derive(Clone, Debug)]
pub struct Address(Arc<Internal>);


#[derive(Debug)]
struct Internal {
    addresses: Vec<Vec<(Weight, SocketAddr)>>,
}

/// A builder interface for `Address`
pub struct AddressBuilder {
    addresses: Vec<Vec<(Weight, SocketAddr)>>,
}

impl From<(IpAddr, u16)> for Address {
    fn from((ip, port): (IpAddr, u16)) -> Address {
        Address(Arc::new(Internal {
            addresses: vec![vec![(0, SocketAddr::new(ip, port))]],
        }))
    }
}
impl FromIterator<SocketAddr> for Address {
    fn from_iter<T>(iter: T) -> Self
        where T: IntoIterator<Item=SocketAddr>
    {
        Address(Arc::new(Internal {
            addresses: vec![iter.into_iter().map(|a| (0, a)).collect()],
        }))
    }
}

impl AddressBuilder {
    pub fn new() -> AddressBuilder {
        return AddressBuilder {
            addresses: vec![Vec::new()],
        }
    }
    pub fn into_address(self) -> Address {
        Address(Arc::new(Internal {
            addresses: self.addresses,
        }))
    }
}

