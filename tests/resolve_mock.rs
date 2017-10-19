extern crate abstract_ns;
extern crate futures;
extern crate tokio_core;

use futures::{Future, Stream};
use futures::future::{FutureResult, ok};
use abstract_ns::{HostResolve, Resolve, Name, Address, IpList, Error};
use abstract_ns::{Subscribe, HostSubscribe};


#[derive(Debug)]
struct HostMock;

#[derive(Debug)]
struct SvcMock;

#[derive(Debug)]
struct Mock;


impl HostResolve for Mock {
    type HostFuture = FutureResult<IpList, Error>;
    fn resolve_host(&self, _name: &Name) -> Self::HostFuture {
        ok(vec!["127.0.0.1".parse().unwrap()].into())
    }
}

impl HostResolve for HostMock {
    type HostFuture = FutureResult<IpList, Error>;
    fn resolve_host(&self, _name: &Name) -> Self::HostFuture {
        ok(vec!["127.0.0.2".parse().unwrap()].into())
    }
}

impl Resolve for SvcMock {
    type Future = FutureResult<Address, Error>;
    fn resolve(&self, _name: &Name) -> Self::Future {
        ok(["127.0.0.2:443".parse().unwrap()][..].into())
    }
}

impl Resolve for Mock {
    type Future = FutureResult<Address, Error>;
    fn resolve(&self, _name: &Name) -> Self::Future {
        ok(["127.0.0.1:443".parse().unwrap()][..].into())
    }
}


#[test]
fn test_mock_host() {
    assert_eq!(
        Mock.resolve_host(&"localhost".parse().unwrap()).wait().unwrap(),
        IpList::parse_list(&["127.0.0.1"]).unwrap()
    );
    assert_eq!(
        HostMock.resolve_host(&"localhost".parse().unwrap()).wait().unwrap(),
        IpList::parse_list(&["127.0.0.2"]).unwrap()
    );
}

#[test]
fn test_mock_srv() {
    assert_eq!(
        Mock.resolve(&"localhost".parse().unwrap()).wait().unwrap(),
        Address::parse_list(&["127.0.0.1:443"]).unwrap()
    );
    assert_eq!(
        SvcMock.resolve(&"localhost".parse().unwrap()).wait().unwrap(),
        Address::parse_list(&["127.0.0.2:443"]).unwrap()
    );
}

#[test]
fn test_null_pasthrough() {
    assert_eq!(
        SvcMock.null_host_resolver()
        .resolve(&"localhost".parse().unwrap()).wait().unwrap(),
        Address::parse_list(&["127.0.0.2:443"]).unwrap()
    );
    assert_eq!(
        HostMock.null_service_resolver()
        .resolve_host(&"localhost".parse().unwrap()).wait().unwrap(),
        IpList::parse_list(&["127.0.0.2"]).unwrap()
    );
}

#[test]
#[should_panic(expected="NameNotFound")]
fn test_null_service() {
    HostMock.null_service_resolver()
    .resolve(&"localhost".parse().unwrap()).wait().unwrap();
}

#[test]
#[should_panic(expected="NameNotFound")]
fn test_null_host() {
    SvcMock.null_host_resolver()
    .resolve_host(&"localhost".parse().unwrap()).wait().unwrap();
}

fn all_traits<T: Resolve + HostResolve + Subscribe + HostSubscribe>(_: T) { }

#[test]
fn test_mock_sub() {
    assert_eq!(
        Mock.frozen_subscriber()
        .subscribe(&"localhost".parse().unwrap())
            .wait().next().unwrap().unwrap(),
        Address::parse_list(&["127.0.0.1:443"]).unwrap()
    );
    assert_eq!(
        Mock.frozen_subscriber()
        .subscribe_host(&"localhost".parse().unwrap())
            .wait().next().unwrap().unwrap(),
        IpList::parse_list(&["127.0.0.1"]).unwrap()
    );
}

#[test]
fn test_traits() {
    all_traits(Mock.frozen_subscriber());
    all_traits(HostMock.null_service_resolver().frozen_subscriber());
    all_traits(SvcMock.null_host_resolver().frozen_subscriber());
}
