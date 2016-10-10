use std::sync::Arc;
use std::iter::FromIterator;
use std::net::{IpAddr, SocketAddr};

use rand::thread_rng;
use rand::distributions::{IndependentSample, Range};

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
///
/// # Example
///
/// ```
/// let builder = AddressBuilder::new();
/// builder.add_addresses([(1, "127.0.0.1:80".parse().unwrap())]);
/// let addr = builder.into_address();
/// ```
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
            addresses: vec![iter.into_iter().map(|a| (1, a)).collect()],
        }))
    }
}

impl AddressBuilder {
    /// Create a new empty address builder
    pub fn new() -> AddressBuilder {
        return AddressBuilder {
            addresses: vec![Vec::new()],
        }
    }

    /// Add set of addresses of the same priority
    ///
    /// You must add all addresses of the same priority with a single call
    /// to this function. Next call to `add_addresses` will add addresses with
    /// smaller priority
    pub fn add_addresses<I>(mut self, items: I) -> AddressBuilder
        where I: IntoIterator<Item=(Weight, SocketAddr)>
    {
        self.addresses.push(Vec::from_iter(items));
        self
    }
    /// Finish building the Address object
    ///
    /// Returns none if there is no single address in the builder
    pub fn into_address(self) -> Address {
        Address(Arc::new(Internal {
            addresses: self.addresses,
        }))
    }
}


impl Address {
    /// Select one random address to connect to
    ///
    /// Picks a single address from the set of high priority addresses, with
    /// the random distribution according to the weights.
    ///
    /// This method is stateless so it can't find out that high priority
    /// addresses are all inaccessible and fallback addresses should be used.
    ///
    /// Returns `None` if address is empty
    pub fn pick_one(&self) -> Option<SocketAddr> {
        if self.0.addresses.len() == 0 || self.0.addresses[0].len() == 0 {
            return None
        }
        let total_weight = self.0.addresses[0].iter().map(|&(w, _)| w).sum();
        if total_weight == 0 {
            return Some(self.0.addresses[0][0].1);
        }
        let range = Range::new(0, total_weight);
        let mut n = range.ind_sample(&mut thread_rng());
        for &(w, addr) in &self.0.addresses[0] {
            if n < w {
                return Some(addr);
            }
            n -= w;
        }
        unreachable!();
    }
}
