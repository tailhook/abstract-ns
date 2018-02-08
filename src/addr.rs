//! Address type and helper structures to manipulate and introspect it
//!
use std::collections::HashSet;
use std::sync::Arc;
use std::iter::FromIterator;
use std::net::{IpAddr, SocketAddr, AddrParseError};
use std::slice::Iter as VecIter;

use rand::{thread_rng, Rng};
use rand::distributions::{IndependentSample, Range};

/// A type alias for a weight for each name in an address
///
/// (don't rely on actual type, it's likely to change in near future)
pub type Weight = u64;

/// Address that nameservice has returned
///
/// We hide this structure to allow future additions. There is `Builder`
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
/// use abstract_ns::addr::Builder;
///
/// let mut builder = Builder::new();
/// builder.add_addresses(&[(1, "127.0.0.1:80".parse().unwrap())]);
/// let addr = builder.into_address();
/// ```
#[derive(Debug)]
pub struct Builder {
    addresses: Vec<Vec<(Weight, SocketAddr)>>,
}

/// A structure that represents a set of addresses of the same priority
#[derive(Debug)]
pub struct WeightedSet<'a> {
    addresses: &'a [(Weight, SocketAddr)],
}

/// Iterator over `Address` that returns a set of addresses of the same
/// priority on each iteration
#[derive(Debug)]
pub struct PriorityIter<'a>(VecIter<'a, Vec<(Weight, SocketAddr)>>);

/// An owned wrapper around `AddressIter` implementing `IntoIterator`
///
/// Create it with `Address::addresses_at`
#[derive(Debug)]
pub struct OwnedAddressIter(Arc<Internal>, usize, usize);

/// Iterates over individual SocketAddr's (IPs) in the WeightedSet (i.e. a
/// set of addresses having the same priority).
///
/// Note, this iterator effectively discards weights.
#[derive(Debug)]
pub struct AddressIter<'a>(VecIter<'a, (Weight, SocketAddr)>);

impl<'a> Iterator for PriorityIter<'a> {
    type Item = WeightedSet<'a>;
    fn next(&mut self) -> Option<WeightedSet<'a>> {
        self.0.next().map(|vec| WeightedSet {
            addresses: &vec,
        })
    }
}

impl<'a> Iterator for OwnedAddressIter {
    type Item = SocketAddr;
    fn next(&mut self) -> Option<SocketAddr> {
        let n = self.2;
        self.2 += 1;
        self.0.addresses.get(self.1)
            .and_then(|vec| vec.get(n))
            .map(|&(_, addr)| addr)
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
            addresses: vec![iter.into_iter().map(|a| (0, a)).collect()],
        }))
    }
}

impl AsRef<Address> for Address {
    fn as_ref(&self) -> &Address {
        self
    }
}

impl Builder {
    /// Create a new empty address builder
    pub fn new() -> Builder {
        return Builder {
            addresses: vec![Vec::new()],
        }
    }

    /// Add set of addresses of the same priority
    ///
    /// You must add all addresses of the same priority with a single call
    /// to this function. Next call to `add_addresses` will add addresses with
    /// smaller priority
    pub fn add_addresses<'x, I>(&mut self, items: I) -> &mut Builder
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

    /// Returns an owned iterator over addresses at priority
    ///
    /// This is similar to `self.at(pri).addresses()` but returns an owned
    /// object that implements IntoIterator. This might be useful for streams
    /// and futures where borrowed objects don't work
    pub fn addresses_at(&self, priority: usize) -> OwnedAddressIter {
        OwnedAddressIter(self.0.clone(), priority, 0)
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

    /// Parse a list of strings and put it into an address
    ///
    /// This only uses one layer of addresses with same weights. And is mostly
    /// useful for unit tests
    pub fn parse_list<I>(iter: I)
        -> Result<Address, AddrParseError>
        where I: IntoIterator,
              I::Item: AsRef<str>
    {
        Ok(Address(Arc::new(Internal {
            addresses: vec![
                iter.into_iter()
                    .map(|x| x.as_ref().parse().map(|sa| (0, sa)))
                    .collect::<Result<Vec<_>, _>>()?
            ],
        })))
    }
}

impl PartialEq for Address {
    fn eq(&self, other: &Address) -> bool {
        self.0.addresses.len() == other.0.addresses.len() &&
        self.iter().zip(other.iter()).all(|(s, o)| s == o)
    }
}

impl Eq for Address {}


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
            // All addresses are equal
            return Some(thread_rng().choose(self.addresses).unwrap().1)
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

    /// Compares two weighted sets to find out which addresses have been
    /// removed from set or added
    ///
    /// This doesn't compare weights of the addresses
    pub fn compare_addresses(&self, other: &WeightedSet)
        -> (Vec<SocketAddr>, Vec<SocketAddr>)
    {
        // TODO(tailhook) a very naive implementation, optimize
        let mut old = Vec::new();
        let mut new = Vec::new();
        for &(_, a) in self.addresses {
            if !other.addresses.iter().find(|&&(_, a1)| a == a1).is_some() {
                old.push(a);
            }
        }
        for &(_, a) in other.addresses {
            if !self.addresses.iter().find(|&&(_, a1)| a == a1).is_some() {
                new.push(a);
            }
        }
        return (old, new);
    }
}

impl<'a> PartialEq for WeightedSet<'a> {
    fn eq(&self, other: &WeightedSet) -> bool {
        // Very naive implementation, we might optimize it
        // But we must make sure that order doesn't matter
        // TODO(tailhook) optimize it, validate in case some adresses
        // are duplicated
        if self.addresses.len() != other.addresses.len() {
            return false;
        }
        for &pair in self.addresses {
            if !other.addresses.iter().find(|&&pair1| pair == pair1).is_some()
            {
                return false;
            }
        }
        for &pair in other.addresses {
            if !self.addresses.iter().find(|&&pair1| pair == pair1).is_some()
            {
                return false;
            }
        }
        return true;
    }
}

/// Union `Address` values into another address
///
/// Currently we return an Address having only priority 0 with all addresses
/// contained in every input address's priority zero. Duplicates are removed.
/// All addresses will have same weight
pub fn union<I>(iter: I) -> Address
    where I: IntoIterator,
          I::Item: AsRef<Address>,
{
    let mut set = HashSet::new();
    for child in iter {
        set.extend(child.as_ref().at(0).addresses());
    }
    return set.into_iter().collect();
}

#[cfg(test)]
mod test {

    use super::{Address, union};
    use std::collections::HashSet;
    use std::net::{SocketAddr, IpAddr};
    use std::str::FromStr;

    use futures::Future;
    use futures::stream::{Stream, iter_ok};

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

    #[test]
    fn test_eq() {
        let a1 = [ "127.0.0.1:1234", "10.0.0.1:3456" ]
            .iter()
            .map(|x| SocketAddr::from_str(x).unwrap())
            .collect::<Address>();

        let a2 = [ "127.0.0.1:1234", "10.0.0.1:3456" ]
            .iter()
            .map(|x| SocketAddr::from_str(x).unwrap())
            .collect::<Address>();

        assert_eq!(a1, a2);
    }

    #[test]
    fn test_eq_reverse() {
        let a1 = [ "127.0.0.1:1234", "10.0.0.1:3456" ]
            .iter()
            .map(|x| SocketAddr::from_str(x).unwrap())
            .collect::<Address>();

        let a2 = [ "10.0.0.1:3456", "127.0.0.1:1234" ]
            .iter()
            .map(|x| SocketAddr::from_str(x).unwrap())
            .collect::<Address>();

        assert_eq!(a1, a2);
    }

    #[test]
    fn test_ne() {
        let a1 = [ "127.0.0.1:1234", "10.0.0.1:5555" ]
            .iter()
            .map(|x| SocketAddr::from_str(x).unwrap())
            .collect::<Address>();

        let a2 = [ "10.0.0.1:3456", "127.0.0.1:1234" ]
            .iter()
            .map(|x| SocketAddr::from_str(x).unwrap())
            .collect::<Address>();

        assert_ne!(a1, a2);
    }

    #[test]
    fn test_diff() {
        let a1 = [ "127.0.0.1:1234", "10.0.0.1:3456" ]
            .iter()
            .map(|x| SocketAddr::from_str(x).unwrap())
            .collect::<Address>();

        let a2 = [ "127.0.0.2:1234", "10.0.0.1:3456" ]
            .iter()
            .map(|x| SocketAddr::from_str(x).unwrap())
            .collect::<Address>();

        let l1 = a1.iter().next().unwrap();
        let l2 = a2.iter().next().unwrap();

        assert_eq!(l1.compare_addresses(&l2),
            (vec![SocketAddr::from_str("127.0.0.1:1234").unwrap()],
             vec![SocketAddr::from_str("127.0.0.2:1234").unwrap()]));
    }


    #[test]
    fn test_union() {
        let a1 = Address::parse_list(
            &[ "127.0.0.1:1234", "10.0.0.1:3456" ]
            ).unwrap();
        let a2 = Address::parse_list(
            &[ "127.0.0.2:1234", "10.0.0.1:3456" ]
            ).unwrap();

        let a = union([a1, a2].iter());
        assert_eq!(a.at(0).addresses().collect::<HashSet<_>>(), vec![
            SocketAddr::from_str("127.0.0.1:1234").unwrap(),
            SocketAddr::from_str("127.0.0.2:1234").unwrap(),
            SocketAddr::from_str("10.0.0.1:3456").unwrap(),
            ].into_iter().collect::<HashSet<_>>());
        // check for no duplicates
        assert_eq!(a.at(0).addresses().collect::<Vec<_>>().len(), 3);
    }

    fn check_type<S: Stream>(stream: S) -> S
        where S::Item: IntoIterator<Item=SocketAddr>
    {
        stream
    }

    #[test]
    fn test_addresses_at_lifetime() {
        assert_eq!(2usize,
            check_type(
                iter_ok::<_, ()>(vec![Address::parse_list(
                    &["127.0.0.1:8080", "172.0.0.1:8010"]
                    ).unwrap()])
                .map(|a| a.addresses_at(0))
            ).map(|a| a.into_iter().count())
            .collect().wait().unwrap().into_iter().sum());
    }
}
