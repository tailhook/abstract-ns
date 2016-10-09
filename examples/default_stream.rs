extern crate argparse;
extern crate futures;
extern crate abstract_ns;

use futures::{Future};
use futures::stream::{Stream};
use abstract_ns::{MemResolver, Resolver};
use argparse::{ArgumentParser, Store};

fn main() {
    let mut name = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut name).required()
            .add_argument("hostname", Store, "Name to resolve");
        ap.parse_args_or_exit();
    }
    let mut resolver = MemResolver::new();
    resolver.add_host("localhost", "127.0.0.1".parse().unwrap());
    resolver.subscribe(&name).for_each(|x| {
        println!("Addresses: {:?}", x);
        println!("Pick one: {}", x.pick_one().unwrap());
        println!("Note: this example hangs, it's fine");
        Ok(())
    }).wait().unwrap();
}
