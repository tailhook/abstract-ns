extern crate argparse;
extern crate futures;
extern crate domain;
extern crate tokio_core;
extern crate abstract_ns;
extern crate ns_dns_tokio;

use std::io::{stderr, Write};
use std::process::exit;

use futures::{Future};
use tokio_core::reactor::Core;
use abstract_ns::Resolver;
use argparse::{ArgumentParser, Store};
use domain::resolv;
use ns_dns_tokio::DnsResolver;

fn main() {
    let mut name = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut name).required()
            .add_argument("hostname", Store, "Name to resolve");
        ap.parse_args_or_exit();
    }
    let mut core = Core::new().unwrap();
    let resolver = DnsResolver::new(
        resolv::Resolver::new(&core.handle()).unwrap());
    let res = core.run(resolver.resolve(&name).map(|x| {
        println!("{:?}", x)
    }));
    match res {
        Ok(()) => {}
        Err(e) => {
            writeln!(&mut stderr(), "Error: {}", e).ok();
            exit(1);
        }
    }
}
