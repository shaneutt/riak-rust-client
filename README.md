# riak-rust-client

[![Build Status](https://travis-ci.org/shaneutt/riak-rust-client.svg?branch=master)](https://travis-ci.org/shaneutt/riak-rust-client)
[![Docs](https://img.shields.io/badge/docs-docs.rs-ff69b4.svg)](https://docs.rs/riak/)
[![crates.io](https://img.shields.io/crates/v/riak.svg)](https://crates.io/crates/riak)
[![License](https://img.shields.io/badge/license-apache-blue.svg)](https://raw.githubusercontent.com/shaneutt/riak-rust-client/master/LICENSE)

A [Riak](https://github.com/basho/riak) client for Rust.

Full documentation [available on docs.rs](https://docs.rs/riak/).

# WARNINGS!

* Development is on hold while Riak has been changing ownership

* Riak TS features are not yet implemented

* This client is still considered under development and is not production ready

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

## Contributing

For issues please be as verbose as you can.

PRs welcome.
