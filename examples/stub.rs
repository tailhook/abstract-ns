extern crate argparse;
extern crate futures;
extern crate abstract_ns;

use std::io::{stderr, Write};
use std::process::exit;

use futures::{Future};
use abstract_ns::{StubResolver, Resolver};
use argparse::{ArgumentParser, Store};

fn main() {
    let mut name = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut name).required()
            .add_argument("hostname", Store, "Name to resolve");
        ap.parse_args_or_exit();
    }
    let mut resolver = StubResolver::new();
    resolver.add_host("localhost", "127.0.0.1".parse().unwrap());
    let res = resolver.resolve(&name).map(|x| {
        println!("{:?}", x)
    }).wait();
    match res {
        Ok(()) => {}
        Err(e) => {
            writeln!(&mut stderr(), "Error: {}", e).ok();
            exit(1);
        }
    }
}
