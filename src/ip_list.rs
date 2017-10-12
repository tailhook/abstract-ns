//! IpList type which is a list of ip addresses and helper structures to
//! work with ip lists
//!
use std::net::{IpAddr, SocketAddr};
use std::slice::Iter;
use std::sync::Arc;

use rand::{thread_rng, Rng};
use addr::Address;

/// IpList is a wrapper type around `Vec<IpAddr>` which serves the same
/// role as the `Address` but for resolving hostnames (or `A` records) instead
/// of services (i.e. host:port pairs or `SRV` records)
///
/// Similarlty to `Address` this type can be cloned cheaply
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpList(Arc<Vec<IpAddr>>);

/// Iterator over ip addresses in IpList
#[derive(Debug)]
pub struct IpIterator<'a>(Iter<'a, IpAddr>);

impl IpList {
    /// Select one random address to connect to
    ///
    /// This function selects a random address from the list of addresses or
    /// `None` if list is empty.
    pub fn pick_one(&self) -> Option<IpAddr> {
        if self.0.len() == 0 {
            return None
        }
        thread_rng().choose(&self.0[..]).map(|&x| x)
    }

    /// Iterate over IP addresses in the list
    pub fn iter(&self) -> IpIterator {
        IpIterator(self.0.iter())
    }

    /// Create an `Address` object by attaching the specified to all addresses
    pub fn with_port(&self, port: u16) -> Address {
        self.iter().map(|x| SocketAddr::new(*x, port)).collect()
    }
}

impl<'a> Iterator for IpIterator<'a> {
    type Item = &'a IpAddr;
    fn next(&mut self) -> Option<&'a IpAddr> {
        self.0.next()
    }
}

impl<'a> IntoIterator for &'a IpList {
    type Item = &'a IpAddr;
    type IntoIter = IpIterator<'a>;
    fn into_iter(self) -> IpIterator<'a> {
        IpIterator(self.0.iter())
    }
}

impl From<Vec<IpAddr>> for IpList {
    fn from(vec: Vec<IpAddr>) -> IpList {
        IpList(Arc::new(vec))
    }
}
