use std::sync::Arc;
use std::iter::FromIterator;
use std::net::{IpAddr, SocketAddr};
use std::slice::Iter as VecIter;

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
/// use abstract_ns::AddressBuilder;
///
/// let mut builder = AddressBuilder::new();
/// builder.add_addresses(&[(1, "127.0.0.1:80".parse().unwrap())]);
/// let addr = builder.into_address();
/// ```
pub struct AddressBuilder {
    addresses: Vec<Vec<(Weight, SocketAddr)>>,
}

/// A structure that represents a set of addresses of the same priority
pub struct WeightedSet<'a> {
    addresses: &'a [(Weight, SocketAddr)],
}

/// Iterator over `Address` that returns a set of addresses of the same
/// priority on each iteration
pub struct PriorityIter<'a>(VecIter<'a, Vec<(Weight, SocketAddr)>>);

/// Iterates over individual SocketAddr's (IPs) in the WeightedSet (i.e. a
/// set of addresses having the same priority).
///
/// Note, this iterator effectively discards weights.
pub struct AddressIter<'a>(VecIter<'a, (Weight, SocketAddr)>);

impl<'a> Iterator for PriorityIter<'a> {
    type Item = WeightedSet<'a>;
    fn next(&mut self) -> Option<WeightedSet<'a>> {
        self.0.next().map(|vec| WeightedSet {
            addresses: &vec,
        })
    }
}

impl<'a> Iterator for AddressIter<'a> {
    type Item = SocketAddr;
    fn next(&mut self) -> Option<SocketAddr> {
        self.0.next().map(|&(_weight, addr)| addr)
    }
}

impl From<(IpAddr, u16)> for Address {
    fn from((ip, port): (IpAddr, u16)) -> Address {
        Address(Arc::new(Internal {
            addresses: vec![vec![(0, SocketAddr::new(ip, port))]],
        }))
    }
}

impl From<SocketAddr> for Address {
    fn from(addr: SocketAddr) -> Address {
        Address(Arc::new(Internal {
            addresses: vec![vec![(0, addr)]],
        }))
    }
}

impl<'a> From<&'a [SocketAddr]> for Address {
    fn from(addr: &[SocketAddr]) -> Address {
        Address(Arc::new(Internal {
            addresses: vec![
                addr.iter().map(|&a| (0, a)).collect()
            ],
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
    pub fn add_addresses<'x, I>(&mut self, items: I) -> &mut AddressBuilder
        where I: IntoIterator<Item=&'x (Weight, SocketAddr)>
    {
        self.addresses.push(items.into_iter().cloned().collect());
        self
    }
    /// Finish building the Address object
    ///
    /// Returns none if there is no single address in the builder
    pub fn into_address(self) -> Address {
        Address(Arc::new(Internal {
            addresses: self.addresses.into_iter()
                .filter(|vec| vec.len() > 0)
                .collect(),
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
        self.at(0).pick_one()
    }

    /// Returns the set of the hosts for the same priority
    ///
    /// Note: original priorities are lost. This method has contiguous array
    /// of sets of hosts. The highest priority hosts returned by `.at(0)`.
    ///
    /// If no hosts the priority exists returns an empty set
    ///
    /// Use `iter()` to iterate over `WeightedSet`'s by priority
    pub fn at(&self, priority: usize) -> WeightedSet {
        self.0.addresses.get(priority)
            .map(|vec| WeightedSet { addresses: vec })
            .unwrap_or(WeightedSet{ addresses: &[] })
    }

    /// Returns iterator over `WeightedSet`'s starting from high priority set
    pub fn iter(&self) -> PriorityIter {
        PriorityIter(self.0.addresses.iter())
    }
}


impl<'a> WeightedSet<'a> {
    /// Select one random address to connect to
    ///
    /// This function selects a host according to the random distribution
    /// according to the weights.
    ///
    /// Returns `None` if the set is empty
    pub fn pick_one(&self) -> Option<SocketAddr> {
        if self.addresses.len() == 0 {
            return None
        }
        let total_weight = self.addresses.iter().map(|&(w, _)| w).sum();
        if total_weight == 0 {
            return Some(self.addresses[0].1);
        }
        let range = Range::new(0, total_weight);
        let mut n = range.ind_sample(&mut thread_rng());
        for &(w, addr) in self.addresses {
            if n < w {
                return Some(addr);
            }
            n -= w;
        }
        unreachable!();
    }
    /// Returns iterator over underlying addresses
    ///
    /// This effectively discards weights, but may be useful for cases where
    /// you treat addresses as a set. For example to find out whether two
    /// `Address` values intersect over `SocketAddr`.
    pub fn addresses(&self) -> AddressIter {
        AddressIter(self.addresses.iter())
    }
}


#[cfg(test)]
mod test {

    use super::Address;
    use std::net::{SocketAddr, IpAddr};
    use std::str::FromStr;

    #[test]
    fn test_iter() {
        let ab = [ "127.0.0.1:1234", "10.0.0.1:3456" ]
            .iter()
            .map(|x| SocketAddr::from_str(x).unwrap())
            .collect::<Address>();
        let r = ab.iter()
            .map(|x| x.addresses().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(r, vec![
            [ "127.0.0.1:1234", "10.0.0.1:3456" ]
            .iter()
            .map(|x| SocketAddr::from_str(x).unwrap())
            .collect::<Vec<_>>()
        ]);
    }

    #[test]
    fn from_socket_addr() {
        Address::from(SocketAddr::from_str("127.0.0.1:1234").unwrap());
    }

    #[test]
    fn from_ip() {
        Address::from((IpAddr::from_str("127.0.0.1").unwrap(), 1234));
    }

    #[test]
    fn from_slice() {
        Address::from(&[SocketAddr::from_str("127.0.0.1:1234").unwrap()][..]);
    }
}
