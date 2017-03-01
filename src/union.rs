use std::collections::HashSet;
use std::borrow::Borrow;

use futures::{Poll, Async, Stream};
use futures::stream::BoxStream;

use {Address, Error};

/// Get the stream that emits address that is a union (superset) of all
/// adderesses of underlying streams
///
/// Note we barely append all underlying addresses (IPs) discarding weights
pub fn union_stream(streams: Vec<BoxStream<Address, Error>>)
    -> Union
{
    Union {
        buf: vec![None; streams.len()],
        streams: streams,
    }
}

/// Represents a stream that is the result of `union_stream()`
pub struct Union {
    streams: Vec<BoxStream<Address, Error>>,
    buf: Vec<Option<Address>>,
}

impl Stream for Union {
    type Item = Address;
    type Error = Error;
    fn poll(&mut self) -> Poll<Option<Address>, Error> {
        let mut changed = false;
        for (i, s) in self.streams.iter_mut().enumerate() {
            match s.poll()? {
                Async::Ready(Some(a)) => {
                    self.buf[i] = Some(a);
                    changed = true;
                }
                Async::NotReady => {}
                // TODO(tailhook) should we delete stream here?
                Async::Ready(None) => {}
            }
        }
        if !changed {
            return Ok(Async::NotReady);
        }
        Ok(Async::Ready(Some(union_addresses(
            self.buf.iter().filter_map(|x| x.as_ref())))))
    }
}

/// Union `Address` values into another address
///
/// Currently we return an Address having only priority 0 with all addresses
/// contained in every input address's priority zero. Duplicates are removed.
/// All addresses will have same weight
pub fn union_addresses<'x, I, B>(iter: I) -> Address
    where I: Iterator<Item=B>,
          B: Borrow<Address>,
{
    let mut set = HashSet::new();
    for child in iter {
        set.extend(child.borrow().at(0).addresses());
    }
    return set.into_iter().collect();
}

#[cfg(test)]
mod test {

    use Address;
    use super::union_addresses;
    use std::net::SocketAddr;
    use std::str::FromStr;
    use std::collections::HashSet;

    #[test]
    fn test_union() {
        let a1 = [ "127.0.0.1:1234", "10.0.0.1:3456" ]
            .iter()
            .map(|x| SocketAddr::from_str(x).unwrap())
            .collect::<Address>();

        let a2 = [ "127.0.0.2:1234", "10.0.0.1:3456" ]
            .iter()
            .map(|x| SocketAddr::from_str(x).unwrap())
            .collect::<Address>();
        let a = union_addresses([a1, a2].iter());
        assert_eq!(a.at(0).addresses().collect::<HashSet<_>>(), vec![
            SocketAddr::from_str("127.0.0.1:1234").unwrap(),
            SocketAddr::from_str("127.0.0.2:1234").unwrap(),
            SocketAddr::from_str("10.0.0.1:3456").unwrap(),
            ].into_iter().collect::<HashSet<_>>());
        // check for no duplicates
        assert_eq!(a.at(0).addresses().collect::<Vec<_>>().len(), 3);
    }
}
