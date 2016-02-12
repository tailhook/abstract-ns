extern crate abstract_ns;

use std::net::SocketAddr;
use abstract_ns::{resolver_thread, StdResolver, AsyncResolver};


fn main() {
    let (_join_handle, mut std) = resolver_thread(|| {
        StdResolver::new()
    });
    let moz = std.resolve_async(("mozilla.org", 80));
    let rust = std.resolve_async(("rust-lang.org", 80));

    println!("Asked to mozilla.org and rust-lang.org");

    println!("mozilla.org:");
    let addresses: Vec<SocketAddr> = moz.recv()
        .expect("resolver thread crashed")
        .expect("name resolution failure");
    for addr in addresses {
        println!("    {}", addr);
    }
    println!("rust-lang.org:");
    let addresses: Vec<SocketAddr> = rust.recv()
        .expect("resolver thread crashed")
        .expect("name resolution failure");
    for addr in addresses {
        println!("    {}", addr);
    }
}
