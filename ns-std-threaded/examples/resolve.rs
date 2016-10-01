extern crate argparse;
extern crate futures;
extern crate futures_cpupool;
extern crate abstract_ns;
extern crate ns_std_threaded;

use std::io::{stderr, Write};
use std::process::exit;

use futures::{Future};
use abstract_ns::Resolver;
use argparse::{ArgumentParser, Store};
use futures_cpupool::CpuPool;
use ns_std_threaded::ThreadedResolver;

fn main() {
    let mut name = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut name).required()
            .add_argument("hostname", Store, "Name to resolve");
        ap.parse_args_or_exit();
    }
    let resolver = ThreadedResolver::new(CpuPool::new(20));
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
