# riak-rust-client

[![Build Status](https://github.com/shaneutt/riak-rust-client/actions/workflows/rust.yml/badge.svg)](https://github.com/shaneutt/riak-rust-client/actions/workflows/rust.yml)
[![Docs](https://img.shields.io/badge/docs-docs.rs-ff69b4.svg)](https://docs.rs/riak/)
[![crates.io](https://img.shields.io/crates/v/riak.svg)](https://crates.io/crates/riak)
[![License](https://img.shields.io/badge/license-mit-blue.svg)](https://raw.githubusercontent.com/shaneutt/riak-rust-client/master/LICENSE)

A [Riak](https://github.com/basho/riak) client for Rust.

Full documentation [available on docs.rs](https://docs.rs/riak/).

# Important Notes

When this client was originally written I was working at Basho technologies (we were the the original developers of Riak). Shortly after I started this client the company ended up [having significant problems][end] and I ended up leaving the company. I shelved and archived this repository uncertain of Riak's future and otherwise not having a reason to continue developing it. 6 years later it came to my attention that there is some active usage and interest in the continuation of this client, so it was un-archived and made available for forking. Where it goes from here depends on interest: if you're interested in contributing and helping move it forward I'm open to [new issues][new], [discussions][disc] and [PRs][prs]. If contributions and interest keeps coming in I will consider quantifying and organizing a `v1` release of the client. Otherwise if there's not much engagement in the coming year I'll likely archive it once again.

[end]:https://www.theregister.com/2017/07/31/end_of_the_road_for_basho_as_court_puts_biz_into_receivership/
[new]:https://github.com/shaneutt/riak-rust-client/issues/new
[disc]:https://github.com/shaneutt/riak-rust-client/discussions/new
[prs]:https://github.com/shaneutt/riak-rust-client/compare

# Description

This client allows you to connect to the [Protocol Buffers API](https://docs.basho.com/riak/kv/latest/developing/api/protocol-buffers/) of a [Riak Node](http://basho.com/products/) and use the functionality provided to send data, retrieve data, and otherwise communicate with Riak.

This client communicates directly with the Protocol Buffer API on a specified Riak node, and does not provide any abstractions around a cluster of nodes.

# Requirements

* This client is tested against Rust's stable, beta, and nightly branches. It should work with any modern Rust.

* Intended to work with Riak KV 2.1.4+

# Installation

Add `riak` as a `Cargo.toml` dependency to your project:

```
[dependencies]
riak = "*"
```

Select a specific version.

# Usage

## Examples

Storing an object:

```rust
use riak::Client;
use riak::object::{ObjectContent, StoreObjectReq};

// connect to Riak and ping the server
let mut riak = Client::new("10.0.0.2:8087").unwrap();
riak.ping().unwrap();

// prepare the object contents
let contents = ObjectContent::new("This is test data!".as_bytes());

// build a request to store the object
let mut store_request = StoreObjectReq::new("testbucket", contents);
store_request.set_key("testkey");

// store the object
riak.store_object(store_request).unwrap();
```
