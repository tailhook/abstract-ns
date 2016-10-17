=====================
Abstract Name Service
=====================

:Status: Proof of Concept
:Documentation: http://tailhook.github.io/abstract-ns

This rust crate provides just abstract traits which may be used to build
interoperable implementations of name dicovery.

We want abstract_ns to have implementations not only for DNS-based name
discovery but also Zookeeper, Eureka, Etcd, Consul, and whatever other thing
you might imagine. All of them easily configured and interchangeable.

Features:

* Defines what is a name and what is a result of service discovery
* Uses futures-rs for asynchronous stuff
* Has interface to receive updates (name changes)
* Allows some kind of name service routing, i.e. has a way to specify different
  resolvers for different names, for example: serve `*.consul` from local
  consul, other names from conventional DNS servers.

This repository also contains the following crates:

* ``ns-std-threaded`` a name resolution implementation that uses stdlib
  resolver running in a thread pool
* ``ns-dns-tokio`` an pure-rust implementation that uses ``domain`` crate to
  resolve domains asynchronously in ``tokio-core`` main loop

Note: abstract-ns v0.2 is very different product than v0.1


License
=======

Licensed under either of

* Apache License, Version 2.0,
  (./LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (./LICENSE-MIT or http://opensource.org/licenses/MIT)
  at your option.

Contribution
------------

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

