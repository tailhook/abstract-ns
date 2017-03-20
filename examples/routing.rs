extern crate argparse;
extern crate futures;
extern crate futures_cpupool;
extern crate domain;
extern crate abstract_ns;
extern crate tokio_core;
extern crate ns_dns_tokio;
extern crate ns_std_threaded;

use std::io::{stderr, Write};
use std::process::exit;

use futures::{Future};
use futures_cpupool::CpuPool;
use tokio_core::reactor::Core;
use domain::resolv::{self, conf};
use abstract_ns::{Resolver, RouterBuilder};
use argparse::{ArgumentParser, Store};


fn main() {
    let mut name = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut name).required()
            .add_argument("hostname", Store, "Name to resolve");
        ap.parse_args_or_exit();
    }

    let mut lp = Core::new().unwrap();

    let mut router = RouterBuilder::new();
    // Explicitly add localhost, to have less surprises
    router.add_ip("localhost", "127.0.0.1".parse().unwrap());
    // *.consul names always resolve at localhost with port 8600
    let mut conf = conf::ResolvConf::new();
    conf.servers.push(conf::ServerConf::new(
        "127.0.0.1:8600".parse().unwrap()));
    conf.finalize();
    router.add_suffix("consul",
        ns_dns_tokio::DnsResolver::new_from_resolver(
            resolv::Resolver::from_conf(&lp.handle(), conf)));
    router.add_default(
        ns_std_threaded::ThreadedResolver::new(CpuPool::new(1)));

    let resolver = router.into_resolver();
    let res = lp.run(resolver.resolve(&name).map(|x| {
        println!("Addresses: {:?}", x);
        println!("Pick one: {}", x.pick_one().unwrap());
    }));
    match res {
        Ok(()) => {}
        Err(e) => {
            writeln!(&mut stderr(), "Error: {}", e).ok();
            exit(1);
        }
    }
}
