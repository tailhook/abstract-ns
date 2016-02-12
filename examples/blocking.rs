extern crate abstract_ns;

use std::net::SocketAddr;
use abstract_ns::{StdResolver, BlockingResolver};


fn main() {
    let mut std = StdResolver::new();
    let addresses: Vec<SocketAddr> = std.resolve(("google.com", 80)).unwrap();
    for addr in addresses {
        println!("{}", addr);
    }
}
