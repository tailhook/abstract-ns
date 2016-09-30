use std::sync::Arc;
use std::net::SocketAddr;

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
    names: Vec<Vec<(Weight, SocketAddr)>>,
}

/// A builder interface for `Address`
pub struct AddressBuilder {
    names: Vec<Vec<(Weight, SocketAddr)>>,
}

impl AddressBuilder {
    pub fn new() -> AddressBuilder {
        return AddressBuilder {
            names: vec![Vec::new()],
        }
    }
    pub fn into_address(self) -> Address {
        Address(Arc::new(Internal {
            names: self.names,
        }))
    }
}

