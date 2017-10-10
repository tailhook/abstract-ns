//! IpList type which is a list of ip addresses and helper structures to
//! work with ip lists
//!
use std::net::IpAddr;
use std::slice::Iter;
use std::sync::Arc;


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
