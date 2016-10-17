=====================
Abstract Name Service
=====================

:Status: Proof of Concept
:Documentation: http://tailhook.github.io/abstract-ns

This rust crate provides just abstract traits which may be used to build
interoperable implementations of name dicovery.

Features:

* Defines what is a name and what is a result of service discovery
* Uses futures-rs for asynchronous stuff
* Has interface to receive updates (name changes)
* Allows some kind of DNS routing, is a way to specify different sources
  for different names, for example: serve `*.consul` from local consul,
  other names from conventional DNS servers.

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

