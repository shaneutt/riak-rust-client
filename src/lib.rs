//! A Riak client for Rust.
//!
//! This client can be used to communicate with Riak clusters to send and receive objects
//! and other information. Operations are done through the `Client` struct and there are
//! several other structs designed to build data structures for sending and receiving
//! data from a Riak cluster.
//!
//! This client uses Riak's Protocol Buffers API.
//!
//! See the Protocol Buffers API documentation for more info: https://docs.basho.com/riak/kv/latest/developing/api/protocol-buffers/
//!
//! Examples:
//!
//! Storing an object is the most fundamental operation of Riak, it can be done as such:
//!
//! ```
//! use riak::Client;
//! use riak::object::{ObjectContent, StoreObjectReq};
//!
//! // connect to Riak and ping the server
//! let mut riak = Client::new("10.0.0.2:8087").unwrap();
//! riak.ping().unwrap();
//!
//! // prepare the object content
//! let content = ObjectContent::new("This is test data!");
//!
//! // build a request to store the object
//! let mut store_request = StoreObjectReq::new("testbucket", content);
//! store_request.set_key("testkey");
//!
//! // store the object
//! riak.store_object(store_request).unwrap();
//! ```

#[macro_use]
extern crate log;
extern crate protobuf;

// public modules
pub mod bucket;
pub mod data_type;
pub mod errors;
pub mod object;
pub mod preflist;
pub mod secondary_index;
pub mod stream;
pub mod yokozuna;

// private modules
mod connection;
mod rpb;
mod utils;

use bucket::{BucketProps, BucketTypeProps};
use connection::RiakConn;
use data_type::{DataTypeFetchResp, DataTypeFetchReq};
use errors::RiakErr;
use object::{DeleteObjectReq, FetchObjectResp, FetchObjectReq, StoreObjectReq};
use preflist::PreflistItem;
use protobuf::{Message, parse_from_bytes};
use rpb::codes;
use rpb::riak::{RpbGetBucketResp, RpbGetBucketReq, RpbGetServerInfoResp, RpbGetBucketTypeReq,
                RpbResetBucketReq};
use rpb::riak_dt::DtFetchResp;
use rpb::riak_kv::{RpbGetBucketKeyPreflistResp, RpbGetBucketKeyPreflistReq, RpbGetResp,
                   RpbIndexResp, RpbMapRedResp, RpbMapRedReq};
use rpb::riak_search::RpbSearchQueryResp;
use rpb::riak_yokozuna::{RpbYokozunaIndexDeleteReq, RpbYokozunaIndexGetResp, RpbYokozunaIndexGetReq,
                         RpbYokozunaSchema, RpbYokozunaSchemaGetResp, RpbYokozunaSchemaGetReq,
                         RpbYokozunaSchemaPutReq};
use secondary_index::{IndexResp, IndexReq};
use std::net::ToSocketAddrs;
use stream::{BucketStream, KeyStream, SecondaryIndexStream};
use utils::{BucketPropsPrivate, DataTypeFetchRespPrivate, FetchObjectRespPrivate, IndexRespPrivate,
            ProtobufBytes, SearchQueryRespPrivate, YokozunaIndexPrivate};
use yokozuna::{SearchQuery, SearchQueryResp, YokozunaIndex};

// Defaults
static DEFAULT_TIMEOUT: u32 = 3600;

/// `Client` Represents a connection to a Riak server's Protocol Buffers API.
#[derive(Debug)]
pub struct Client {
    connection: RiakConn,
    timeout: u32,
}

impl Client {
    /// Constructs a new `Client` with the timeout for requests with a default timeout.
    ///
    /// # Examples
    ///
    /// ```
    /// use riak::Client;
    ///
    /// let mut riak = Client::new("10.0.0.2:8087").unwrap();
    /// riak.ping().unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// TODO
    pub fn new<A: ToSocketAddrs>(addr: A) -> Result<Client, RiakErr> {
        Client::new_with_timeout(addr, DEFAULT_TIMEOUT)
    }

    /// Constructs a new `Client` with a timeout (in seconds) provided.
    ///
    /// # Examples
    ///
    /// ```
    /// use riak::Client;
    ///
    /// let mut riak = Client::new_with_timeout("10.0.0.2:8087", 3600).unwrap();
    /// riak.ping().unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// TODO
    pub fn new_with_timeout<A: ToSocketAddrs>(addr: A, timeout: u32) -> Result<Client, RiakErr> {
        let connection = match RiakConn::new(addr, timeout) {
            Ok(connection) => connection,
            Err(error) => return Err(error),
        };

        Ok(Client {
            connection: connection,
            timeout: timeout,
        })
    }

    /// Set the timeout (in seconds) allowed for future requests.
    ///
    /// # Examples
    ///
    /// ```
    /// use riak::Client;
    ///
    /// let mut riak = Client::new("10.0.0.2:8087").unwrap();
    /// riak.set_timeout(3600);
    /// ```
    pub fn set_timeout(&mut self, timeout: u32) {
        self.timeout = timeout;
    }

    /// Reconnect to the Riak server originally connected to when this client was initiated.
    ///
    /// # Examples
    ///
    /// ```
    /// use riak::Client;
    ///
    /// let mut riak = Client::new("10.0.0.2:8087").unwrap();
    /// riak.reconnect().unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// TODO
    pub fn reconnect(&mut self) -> Result<(), RiakErr> {
        self.connection.reconnect()
    }

    /// Sends a ping message to Riak and returns a Result.
    ///
    /// # Examples
    ///
    /// ```
    /// use riak::Client;
    ///
    /// let mut riak = Client::new("10.0.0.2:8087").unwrap();
    /// riak.ping().unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// TODO
    pub fn ping(&mut self) -> Result<(), RiakErr> {
        let ping_data: Vec<u8> = vec![];
        match self.connection.exchange(codes::RpbPingReq, codes::RpbPingResp, &ping_data) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Get the node name and server version of the Riak server reached.
    ///
    /// # Examples
    ///
    /// ```
    /// use riak::Client;
    ///
    /// let mut riak = Client::new("10.0.0.2:8087").unwrap();
    /// let (node, version) = riak.server_info().unwrap();
    ///
    /// println!("Connected to node {}, running Riak version {}", node, version);
    /// ```
    pub fn server_info(&mut self) -> Result<(String, String), RiakErr> {
        // send an `RpbGetServerInfoReq` and get the response bytes
        let response = match self.connection.exchange(codes::RpbGetServerInfoReq,
                                                      codes::RpbGetServerInfoResp,
                                                      &vec![]) {
            Ok(response) => response,
            Err(error) => return Err(error),
        };

        // parse the response bytes into an `RpbGetBucketResp` struct
        let rpb_get_server_info_resp = match parse_from_bytes::<RpbGetServerInfoResp>(&response) {
            Ok(response) => response,
            Err(error) => return Err(RiakErr::ProtobufError(error)),
        };

        // return the node name and server version as strings
        Ok((String::from_utf8_lossy(rpb_get_server_info_resp.get_node()).into_owned(),
            String::from_utf8_lossy(rpb_get_server_info_resp.get_server_version()).into_owned()))
    }

    /// Produces a stream of bucket names.
    ///
    /// Caution: This call can be expensive for the server. Do not use in performance-sensitive code.
    ///
    /// # Examples
    ///
    /// ```
    /// use riak::Client;
    ///
    /// let mut riak = Client::new("10.0.0.2:8087").unwrap();
    /// let mut bucketstream = riak.stream_buckets().unwrap();
    ///
    /// loop {
    ///     match bucketstream.next() {
    ///         Some(buckets) => println!("found buckets {:?}", buckets.unwrap()),
    ///         None => break,
    ///     };
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// TODO
    pub fn stream_buckets(&mut self) -> Result<BucketStream, RiakErr> {
        BucketStream::new(self)
    }

    /// Produces a list of bucket names.
    ///
    /// Caution: This call can be expensive for the server. Do not use in performance-sensitive code.
    ///
    /// # Examples
    ///
    /// ```
    /// use riak::Client;
    ///
    /// let mut riak = Client::new("10.0.0.2:8087").unwrap();
    /// let buckets = riak.list_buckets().unwrap();
    ///
    /// for bucket in buckets.iter() {
    ///     println!("found bucket named {:?}", bucket);
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// TODO
    pub fn list_buckets(&mut self) -> Result<Vec<Vec<u8>>, RiakErr> {
        let mut bucket_stream = match self.stream_buckets() {
            Ok(bucket_stream) => bucket_stream,
            Err(error) => return Err(error),
        };

        bucket_stream.all()
    }

    /// Sets the properties for a bucket given a bucket name.
    ///
    /// # Examples
    ///
    /// ```
    /// use riak::Client;
    /// use riak::bucket::BucketProps;
    ///
    /// let mut riak = Client::new("10.0.0.2:8087").unwrap();
    ///
    /// let mut bucket_props = BucketProps::new("testbucket");
    /// bucket_props.set_backend("leveldb");
    ///
    /// riak.set_bucket_properties(bucket_props).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// TODO
    pub fn set_bucket_properties(&mut self, bucket_props: BucketProps) -> Result<(), RiakErr> {
        // convert the `BucketProps` to bytes
        let bytes = match bucket_props.write_to_bytes() {
            Ok(b) => b,
            Err(err) => return Err(err),
        };

        // make the exchange and check for any errors
        match self.connection.exchange(codes::RpbSetBucketReq, codes::RpbSetBucketResp, &bytes) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Retrieves bucket properties for a bucket given a bucket name.
    ///
    /// # Examples
    ///
    /// ```
    /// use riak::Client;
    ///
    /// let mut riak = Client::new("10.0.0.2:8087").unwrap();
    ///
    /// riak.get_bucket_properties("testbucket").unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// TODO
    pub fn get_bucket_properties<T: Into<Vec<u8>>>(&mut self,
                                                   bucket_name: T)
                                                   -> Result<BucketProps, RiakErr> {
        // get the bucket name as Vec<u8>
        let bucket_name = bucket_name.into();

        // build a protobuf request
        let mut req = RpbGetBucketReq::new();
        req.set_bucket(bucket_name.clone());

        // parse the protobuf request into bytes
        let bytes = match req.write_to_bytes() {
            Ok(bytes) => bytes,
            Err(error) => return Err(RiakErr::ProtobufError(error)),
        };

        // make the exchange with the server and get the response bytes
        let response = match self.connection
            .exchange(codes::RpbGetBucketReq, codes::RpbGetBucketResp, &bytes) {
            Ok(response) => response,
            Err(error) => return Err(error),
        };

        // parse the response bytes into an `RpbGetBucketResp` struct
        let mut rpb_get_bucket_resp = match parse_from_bytes::<RpbGetBucketResp>(&response) {
            Ok(rpb_get_bucket_resp) => rpb_get_bucket_resp,
            Err(error) => return Err(RiakErr::ProtobufError(error)),
        };

        // pull the `RpbBucketProps` out of the response to build the `BucketProps` struct
        let rpb_bucket_props = rpb_get_bucket_resp.take_props();
        let mut bucket_props = BucketProps::new(bucket_name);
        bucket_props.set_props(rpb_bucket_props);

        // return the resulting `BucketProps`
        Ok(bucket_props)
    }

    /// Assigns a set of bucket properties to a bucket type.
    ///
    /// # Examples
    ///
    /// ```
    /// use riak::Client;
    /// use riak::bucket::BucketTypeProps;
    ///
    /// let mut riak = Client::new("10.0.0.2:8087").unwrap();
    ///
    /// let mut bucket_type_props = BucketTypeProps::new("testbuckettype");
    /// bucket_type_props.set_backend("leveldb");
    ///
    /// riak.set_bucket_type_properties(bucket_type_props).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// TODO
    pub fn set_bucket_type_properties(&mut self,
                                      bucket_type_props: BucketTypeProps)
                                      -> Result<(), RiakErr> {
        // convert the `BucketProps` to protobuf bytes
        let bytes = match bucket_type_props.write_to_bytes() {
            Ok(b) => b,
            Err(err) => return Err(err),
        };

        // make the exchange and check for any errors
        match self.connection
            .exchange(codes::RpbSetBucketTypeReq, codes::RpbSetBucketResp, &bytes) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Gets the bucket properties associated with a bucket type.
    ///
    /// # Examples
    ///
    /// ```
    /// use riak::Client;
    ///
    /// let mut riak = Client::new("10.0.0.2:8087").unwrap();
    ///
    /// riak.get_bucket_type_properties("testbuckettype").unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// TODO
    pub fn get_bucket_type_properties<T: Into<Vec<u8>>>(&mut self,
                                                        bucket_type_name: T)
                                                        -> Result<BucketTypeProps, RiakErr> {
        let bucket_type_name = bucket_type_name.into();

        // build the request
        let mut req = RpbGetBucketTypeReq::new();
        req.set_field_type(bucket_type_name.clone());

        // parse the request into bytes
        let bytes = match req.write_to_bytes() {
            Ok(bytes) => bytes,
            Err(error) => return Err(RiakErr::ProtobufError(error)),
        };

        // make the exchange and get the response bytes
        let response = match self.connection
            .exchange(codes::RpbGetBucketTypeReq, codes::RpbGetBucketResp, &bytes) {
            Ok(response) => response,
            Err(error) => return Err(error),
        };

        // if a proper response is received, parse it into a `RpbGetBucketResp`
        let mut rpb_get_bucket_resp = match parse_from_bytes::<RpbGetBucketResp>(&response) {
            Ok(rpb_get_bucket_resp) => rpb_get_bucket_resp,
            Err(error) => return Err(RiakErr::ProtobufError(error)),
        };

        // if no parsing errors occur, build a `BucketTypeProps` to return
        let rpb_bucket_props = rpb_get_bucket_resp.take_props();
        let mut bucket_type_props = BucketTypeProps::new(bucket_type_name);
        bucket_type_props.set_props(rpb_bucket_props);

        Ok(bucket_type_props)
    }

    /// Resets the properties for a bucket
    ///
    /// # Examples
    ///
    /// ```
    /// use riak::Client;
    ///
    /// let mut riak = Client::new("10.0.0.2:8087").unwrap();
    ///
    /// riak.reset_bucket("testbuckettype", "testbucket").unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// TODO
    pub fn reset_bucket<T: Into<Vec<u8>>>(&mut self,
                                          bucket_type_name: T,
                                          bucket_name: T)
                                          -> Result<(), RiakErr> {
        // build a `RpbResetBucketReq` requests
        let mut request = RpbResetBucketReq::new();
        request.set_field_type(bucket_type_name.into());
        request.set_bucket(bucket_name.into());

        // parse the request into bytes
        let bytes = match request.write_to_bytes() {
            Ok(bytes) => bytes,
            Err(error) => return Err(RiakErr::ProtobufError(error)),
        };

        // make the exchange with the server, no return needed on success
        match self.connection
            .exchange(codes::RpbResetBucketReq, codes::RpbResetBucketResp, &bytes) {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }

    /// Produces a stream of keys from a bucket given a bucket name.
    ///
    /// Note: This operation requires traversing all keys stored in the cluster and should not be used in production.
    ///
    /// # Examples
    ///
    /// ```
    /// use riak::Client;
    ///
    /// let mut riak = Client::new("10.0.0.2:8087").unwrap();
    ///
    /// let mut keystream = riak.stream_keys("testbucket").unwrap();
    ///
    /// loop {
    ///     match keystream.next() {
    ///         Some(keys_result) => println!("found keys {:?}", keys_result.unwrap()),
    ///         None => break,
    ///     };
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// TODO
    pub fn stream_keys<T: Into<Vec<u8>>>(&mut self, bucket: T) -> Result<KeyStream, RiakErr> {
        KeyStream::new(self, bucket.into())
    }

    /// Produces a list of keys provided a bucket name
    ///
    /// Note: This operation requires traversing all keys stored in the cluster and should not be used in production.
    ///
    /// # Examples
    ///
    /// ```
    /// use riak::Client;
    ///
    /// let mut riak = Client::new("10.0.0.2:8087").unwrap();
    ///
    /// let keys = riak.list_keys("testbucket").unwrap();
    ///
    /// for key in keys.iter() {
    ///     println!("found key in bucket testbucket named {:?}", key);
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// TODO
    pub fn list_keys<T: Into<Vec<u8>>>(&mut self, bucket: T) -> Result<Vec<Vec<u8>>, RiakErr> {
        match KeyStream::new(self, bucket.into()) {
            Ok(mut keys) => keys.all(),
            Err(error) => Err(error),
        }
    }

    /// Stores an object on the Riak server.
    ///
    /// # Examples
    ///
    /// ```
    /// use riak::Client;
    /// use riak::object::{ObjectContent, StoreObjectReq};
    ///
    /// let mut riak = Client::new("10.0.0.2:8087").unwrap();
    ///
    /// let content = ObjectContent::new("This is a test!");
    /// let mut req = StoreObjectReq::new("testbucket", content);
    /// req.set_key("testkey");
    ///
    /// riak.store_object(req).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// TODO
    pub fn store_object(&mut self, req: StoreObjectReq) -> Result<(), RiakErr> {
        // convert the request to protobuf bytes
        let bytes = match req.write_to_bytes() {
            Ok(bytes) => bytes,
            Err(err) => return Err(err),
        };

        // make the exchange and check for errors
        match self.connection.exchange(codes::RpbPutReq, codes::RpbPutResp, &bytes) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Fetches an object from the Riak server.
    ///
    /// # Examples
    ///
    /// ```
    /// use riak::Client;
    /// use riak::object::FetchObjectReq;
    ///
    /// let mut riak = Client::new("10.0.0.2:8087").unwrap();
    ///
    /// let req = FetchObjectReq::new("testbucket", "testkey");
    /// let object = riak.fetch_object(req).unwrap();
    /// println!("testkey object contained: {:?}", object);
    /// ```
    ///
    /// # Errors
    ///
    /// TODO
    pub fn fetch_object(&mut self, req: FetchObjectReq) -> Result<FetchObjectResp, RiakErr> {
        // convert the request to protobuf bytes
        let bytes = match req.write_to_bytes() {
            Ok(b) => b,
            Err(err) => return Err(err),
        };

        // make the exchange and get the response bytes
        let response = match self.connection
            .exchange(codes::RpbGetReq, codes::RpbGetResp, &bytes) {
            Ok(response) => response,
            Err(error) => return Err(error),
        };

        // parse the response into an `RpbGetResp` struct
        let rpb_get_resp = match parse_from_bytes::<RpbGetResp>(&response) {
            Ok(rpb_get_resp) => rpb_get_resp,
            Err(err) => return Err(RiakErr::ProtobufError(err)),
        };

        // if no errors occur build a `FetchObjectResp` to return
        Ok(FetchObjectResp::new_from_rpb(rpb_get_resp))
    }

    /// Deletes an object from Riak
    ///
    /// # Examples
    ///
    /// ```
    /// use riak::Client;
    /// use riak::object::DeleteObjectReq;
    ///
    /// let mut riak = Client::new("10.0.0.2:8087").unwrap();
    ///
    /// let mut request = DeleteObjectReq::new("testbucket", "testkey");
    /// request.set_dw(3);
    ///
    /// riak.delete_object(request).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// TODO
    pub fn delete_object(&mut self, request: DeleteObjectReq) -> Result<(), RiakErr> {
        // parse the request into bytes to send out
        let bytes = match request.write_to_bytes() {
            Ok(bytes) => bytes,
            Err(error) => return Err(error),
        };

        // make the exchange, nothing to return on success
        match self.connection.exchange(codes::RpbDelReq, codes::RpbDelResp, &bytes) {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }

    /// Fetch the preflist for a bucket/key combination.
    ///
    /// # Examples
    ///
    /// ```
    /// use riak::Client;
    ///
    /// let mut riak = Client::new("10.0.0.2:8087").unwrap();
    /// let preflist = riak.fetch_preflist("testbucket", "testkey").unwrap();
    ///
    /// for preflist_item in preflist {
    ///     if preflist_item.is_primary {
    ///         println!("found primary partition {:?} for key {:?} on node {:?}", preflist_item.partition, "testkey", preflist_item.node);
    ///     }
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// TODO
    pub fn fetch_preflist<T: Into<Vec<u8>>>(&mut self,
                                            bucket: T,
                                            key: T)
                                            -> Result<Vec<PreflistItem>, RiakErr> {
        // build the request
        let mut req = RpbGetBucketKeyPreflistReq::new();
        req.set_bucket(bucket.into());
        req.set_key(key.into());

        // parse the request into bytes
        let bytes = match req.write_to_bytes() {
            Ok(bytes) => bytes,
            Err(error) => return Err(RiakErr::ProtobufError(error)),
        };

        // send the request and check the response
        let response = match self.connection
            .exchange(codes::RpbGetBucketKeyPreflistReq,
                      codes::RpbGetBucketKeyPreflistResp,
                      &bytes) {
            Ok(response) => response,
            Err(error) => return Err(error),
        };

        // parse the response
        let rpb_preflist_resp = match parse_from_bytes::<RpbGetBucketKeyPreflistResp>(&response) {
            Ok(parsed) => parsed,
            Err(err) => return Err(RiakErr::ProtobufError(err)),
        };

        // get each item and build a `PreflistItem` for it
        let mut preflist: Vec<PreflistItem> = Vec::new();
        for preflist_item in rpb_preflist_resp.get_preflist() {
            let node = String::from_utf8_lossy(preflist_item.get_node()).into_owned();
            let converted = PreflistItem::new(preflist_item.get_partition(),
                                              &node,
                                              preflist_item.get_primary());
            preflist.push(converted);
        }

        Ok(preflist)
    }

    /// Create a search schema
    ///
    /// # Examples
    ///
    /// ```
    /// use riak::Client;
    /// use std::fs::File;
    /// use std::io::Read;
    ///
    /// let mut riak = Client::new("10.0.0.2:8087").unwrap();
    ///
    /// let mut xml: Vec<u8> = Vec::new();
    /// let mut file = File::open("/tmp/riak-rust-client-default-schema.xml").unwrap();
    /// let _ = file.read_to_end(&mut xml).unwrap();
    ///
    /// let schema_name = "schedule".to_string().into_bytes();
    /// riak.set_yokozuna_schema(schema_name, xml).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// TODO
    pub fn set_yokozuna_schema<T: Into<Vec<u8>>>(&mut self,
                                                 name: T,
                                                 content: T)
                                                 -> Result<(), RiakErr> {
        // build a `RpbYokozunaSchema`
        let mut schema = RpbYokozunaSchema::new();
        schema.set_name(name.into());
        schema.set_content(content.into());

        // build a `RpbYokozunaSchemaPutReq` request
        let mut req = RpbYokozunaSchemaPutReq::new();
        req.set_schema(schema);

        // parse the request into bytes to send
        let bytes = match req.write_to_bytes() {
            Ok(bytes) => bytes,
            Err(error) => return Err(RiakErr::ProtobufError(error)),
        };

        // make the exchange, no return needed on success
        match self.connection
            .exchange(codes::RpbYokozunaSchemaPutReq, codes::RpbPutResp, &bytes) {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }

    /// Retrieve a search schema
    ///
    /// # Examples
    ///
    /// ```
    /// use riak::Client;
    ///
    /// let mut riak = Client::new("10.0.0.2:8087").unwrap();
    ///
    /// riak.get_yokozuna_schema("schedule").unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// TODO
    pub fn get_yokozuna_schema<T: Into<Vec<u8>>>(&mut self, name: T) -> Result<Vec<u8>, RiakErr> {
        // build a request
        let mut req = RpbYokozunaSchemaGetReq::new();
        req.set_name(name.into());

        // parse the request into bytes
        let bytes = match req.write_to_bytes() {
            Ok(bytes) => bytes,
            Err(error) => return Err(RiakErr::ProtobufError(error)),
        };

        // make the exchange and get the response bytes
        let response = match self.connection.exchange(codes::RpbYokozunaSchemaGetReq,
                                                      codes::RpbYokozunaSchemaGetResp,
                                                      &bytes) {
            Ok(response) => response,
            Err(error) => return Err(error),
        };

        // parse the response into a `RpbYokozunaSchemaGetResp`
        let mut rpb_yokozuna_schema_get_resp =
            match parse_from_bytes::<RpbYokozunaSchemaGetResp>(&response) {
                Ok(parsed) => parsed,
                Err(error) => return Err(RiakErr::ProtobufError(error)),
            };

        // grab the schema out of the response
        let mut rpb_yokozuna_schema = rpb_yokozuna_schema_get_resp.take_schema();

        // return the content of the schema
        Ok(rpb_yokozuna_schema.take_content())
    }

    /// set a search index
    ///
    /// # Examples
    ///
    /// ```
    /// use riak::Client;
    /// use riak::yokozuna::YokozunaIndex;
    ///
    /// let mut riak = Client::new("10.0.0.2:8087").unwrap();
    ///
    /// let index = YokozunaIndex::new("myindex");
    /// riak.set_yokozuna_index(index).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// TODO
    pub fn set_yokozuna_index(&mut self, index: YokozunaIndex) -> Result<(), RiakErr> {
        let bytes = match index.write_to_bytes() {
            Ok(bytes) => bytes,
            Err(error) => return Err(error),
        };

        match self.connection.exchange(codes::RpbYokozunaIndexPutReq, codes::RpbPutResp, &bytes) {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }

    /// get a search index
    ///
    /// # Examples
    ///
    /// ```
    /// use riak::Client;
    ///
    /// let mut riak = Client::new("10.0.0.2:8087").unwrap();
    ///
    /// riak.get_yokozuna_index("myindex").unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// TODO
    pub fn get_yokozuna_index<T: Into<Vec<u8>>>(&mut self,
                                                name: T)
                                                -> Result<Vec<YokozunaIndex>, RiakErr> {
        // build the request
        let mut req = RpbYokozunaIndexGetReq::new();
        req.set_name(name.into());

        // parse the request into bytes
        let bytes = match req.write_to_bytes() {
            Ok(bytes) => bytes,
            Err(error) => return Err(RiakErr::ProtobufError(error)),
        };

        // exchange the request and get the response bytes
        let response = match self.connection.exchange(codes::RpbYokozunaIndexGetReq,
                                                      codes::RpbYokozunaIndexGetResp,
                                                      &bytes) {
            Ok(response) => response,
            Err(error) => return Err(error),
        };

        // parse the response
        let mut rpb_yokozuna_index_get_resp =
            match parse_from_bytes::<RpbYokozunaIndexGetResp>(&response) {
                Ok(rpb_yokozuna_index_get_resp) => rpb_yokozuna_index_get_resp,
                Err(error) => return Err(RiakErr::ProtobufError(error)),
            };

        // build a Vec of all the indexes in the response and return it
        let mut indexes: Vec<YokozunaIndex> = Vec::new();
        for rpb_yokozuna_index in rpb_yokozuna_index_get_resp.take_index().into_iter() {
            let index = YokozunaIndex::new_from_rpb_yokozuna_index(rpb_yokozuna_index);
            indexes.push(index);
        }
        Ok(indexes)
    }

    /// Deletes an index
    ///
    /// # Examples
    ///
    /// ```
    /// use riak::Client;
    /// use riak::yokozuna::YokozunaIndex;
    ///
    /// let mut riak = Client::new("10.0.0.2:8087").unwrap();
    ///
    /// let index = YokozunaIndex::new("deleteme");
    /// riak.set_yokozuna_index(index).unwrap();
    ///
    /// riak.delete_yokozuna_index("deleteme").unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// TODO
    pub fn delete_yokozuna_index<T: Into<Vec<u8>>>(&mut self, name: T) -> Result<(), RiakErr> {
        // build the request
        let mut req = RpbYokozunaIndexDeleteReq::new();
        req.set_name(name.into());

        // parse the request into bytes
        let bytes = match req.write_to_bytes() {
            Ok(bytes) => bytes,
            Err(error) => return Err(RiakErr::ProtobufError(error)),
        };

        // make the exchange with the server and get the response bytes
        match self.connection
            .exchange(codes::RpbYokozunaIndexDeleteReq, codes::RpbDelResp, &bytes) {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }

    /// Perform a Riak Search
    ///
    /// # Examples
    ///
    /// ```
    /// use riak::Client;
    /// use riak::yokozuna::SearchQuery;
    ///
    /// let mut riak = Client::new("10.0.0.2:8087").unwrap();
    ///
    /// let mut query = SearchQuery::new("test*", "myindex");
    /// query.set_df("_yz_id");
    ///
    /// riak.search(query).unwrap();
    /// ```
    pub fn search(&mut self, query: SearchQuery) -> Result<SearchQueryResp, RiakErr> {
        // parse the query into `RpbSearchQueryReq` bytes
        let bytes = match query.write_to_bytes() {
            Ok(bytes) => bytes,
            Err(error) => return Err(error),
        };

        // make the exchange and get the response
        let response = match self.connection
            .exchange(codes::RpbSearchQueryReq, codes::RpbSearchQueryResp, &bytes) {
            Ok(response) => response,
            Err(error) => return Err(error),
        };

        // convert the response to an `RpbSearchQueryResp`
        let rpb_search_query_resp = match parse_from_bytes::<RpbSearchQueryResp>(&response) {
            Ok(rpb_search_query_resp) => rpb_search_query_resp,
            Err(error) => return Err(RiakErr::ProtobufError(error)),
        };

        Ok(SearchQueryResp::new_from_rpb(rpb_search_query_resp))
    }

    /// Perform a MapReduce Job
    ///
    /// # Examples
    ///
    /// ```
    /// use riak::Client;
    ///
    /// let mut riak = Client::new("10.0.0.2:8087").unwrap();
    ///
    /// let job = r#"
    /// {"inputs": "bucket_501653", "query": [
    ///     {"map": {
    ///         "arg": null,
    ///         "name": "Riak.mapValuesJson",
    ///         "language": "javascript",
    ///         "keep": false
    ///     }},
    ///     {"reduce": {
    ///         "arg": null,
    ///         "name": "Riak.reduceSum",
    ///         "language": "javascript",
    ///         "keep": true
    ///     }}
    /// ]}
    /// "#;
    ///
    /// riak.mapreduce(job, "application/json").unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// TODO
    pub fn mapreduce<T: Into<Vec<u8>>>(&mut self,
                                       request: T,
                                       content_type: T)
                                       -> Result<Vec<Vec<u8>>, RiakErr> {
        // create the `RpbMapRedReq`
        let mut req = RpbMapRedReq::new();
        req.set_request(request.into());
        req.set_content_type(content_type.into());

        // write the request to bytes
        let bytes = match req.write_to_bytes() {
            Ok(bytes) => bytes,
            Err(error) => return Err(RiakErr::ProtobufError(error)),
        };

        let mut data: Vec<Vec<u8>> = Vec::new();
        let mut done = false;
        while !done {
            // get the response from the server
            let response = try!(self.connection
                .exchange(codes::RpbMapRedReq, codes::RpbMapRedResp, &bytes));

            // parse the response into an `RpbMapRedResp`
            let mut rpb_map_red_req = match parse_from_bytes::<RpbMapRedResp>(&response) {
                Ok(rpb_map_red_req) => rpb_map_red_req,
                Err(error) => return Err(RiakErr::ProtobufError(error)),
            };

            // push the response data onto `data` and check for `done`
            data.push(rpb_map_red_req.take_response());
            done = rpb_map_red_req.get_done();
        }

        Ok(data)
    }

    /// Data Type Fetch
    ///
    /// # Examples
    ///
    /// ```
    /// use riak::Client;
    /// use riak::data_type::DataTypeFetchReq;
    ///
    /// let mut riak = Client::new("10.0.0.2:8087").unwrap();
    ///
    /// let req = DataTypeFetchReq::new("sets", "bucket", "key");
    ///
    /// riak.data_type_fetch(req).unwrap();
    /// ```
    pub fn data_type_fetch(&mut self,
                           request: DataTypeFetchReq)
                           -> Result<DataTypeFetchResp, RiakErr> {
        // convert the `DataTypeFetchReq` request to bytes
        let bytes = match request.write_to_bytes() {
            Ok(bytes) => bytes,
            Err(e) => return Err(e),
        };

        // get the response from the server
        let response = try!(self.connection
            .exchange(codes::DtFetchReq, codes::DtFetchResp, &bytes));

        // convert the response to `DtFetchResp`
        let data_type_fetch_resp = match parse_from_bytes::<DtFetchResp>(&response) {
            Ok(data_type_fetch_resp) => data_type_fetch_resp,
            Err(e) => return Err(RiakErr::ProtobufError(e)),
        };

        // convert the `DtFetchResp` to `DataTypeFetchResp`
        Ok(DataTypeFetchResp::new_from_rpb(data_type_fetch_resp))
    }

    /// Secondary Index Request - Streaming
    ///
    /// # Examples
    ///
    /// ```
    /// use riak::Client;
    /// use riak::bucket::BucketTypeProps;
    /// use riak::object::{ObjectContent, StoreObjectReq};
    /// use riak::secondary_index::{IndexReq, IndexQueryType};
    ///
    /// let mut riak = Client::new("10.0.0.2:8087").unwrap();
    ///
    /// let mut bucket_type_props = BucketTypeProps::new("2itest");
    /// bucket_type_props.set_backend("leveldb");
    ///
    /// riak.set_bucket_type_properties(bucket_type_props).unwrap();
    ///
    /// let mut content = ObjectContent::new("This is a test!");
    /// content.set_indexes(vec![("test_bin".as_bytes().to_vec(), "test2i".as_bytes().to_vec())]);
    ///
    /// let mut req = StoreObjectReq::new("2itestbucket", content);
    /// req.set_bucket_type("2itest");
    /// req.set_key("testkey2i");
    ///
    /// riak.store_object(req).unwrap();
    ///
    /// let mut req = IndexReq::new("2itest", "2itestbucket", "test_bin", IndexQueryType::EQ);
    /// req.set_key("test2i");
    /// req.set_pagination_sort(true);
    ///
    /// let mut stream = riak.secondary_index_request_streaming(req).unwrap();
    /// stream.all().unwrap();
    /// ```
    pub fn secondary_index_request_streaming(&mut self, mut req: IndexReq) -> Result<SecondaryIndexStream, RiakErr> {
        req.set_stream(true);
        SecondaryIndexStream::new(self, req)
    }

    /// Secondary Index Request - Non-Streaming
    ///
    /// # Examples
    ///
    /// ```
    /// use riak::Client;
    /// use riak::bucket::BucketTypeProps;
    /// use riak::object::{ObjectContent, StoreObjectReq};
    /// use riak::secondary_index::{IndexReq, IndexQueryType};
    ///
    /// let mut riak = Client::new("10.0.0.2:8087").unwrap();
    ///
    /// let mut bucket_type_props = BucketTypeProps::new("2itest");
    /// bucket_type_props.set_backend("leveldb");
    ///
    /// riak.set_bucket_type_properties(bucket_type_props).unwrap();
    ///
    /// let mut content = ObjectContent::new("This is a test!");
    /// content.set_indexes(vec![("test_bin".as_bytes().to_vec(), "test2i".as_bytes().to_vec())]);
    ///
    /// let mut req = StoreObjectReq::new("2itestbucket", content);
    /// req.set_bucket_type("2itest");
    /// req.set_key("testkey2i");
    ///
    /// riak.store_object(req).unwrap();
    ///
    /// let mut req = IndexReq::new("2itest", "2itestbucket", "test_bin", IndexQueryType::EQ);
    /// req.set_key("test2i");
    /// req.set_pagination_sort(true);
    ///
    /// riak.secondary_index_request_non_streaming(req).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// TODO
    pub fn secondary_index_request_non_streaming(&mut self, mut req: IndexReq) -> Result<IndexResp, RiakErr> {
        // ensure streaming is off
        req.set_stream(false);

        // convert the `IndexReq` to bytes
        let bytes = match req.write_to_bytes() {
            Ok(bytes) => bytes,
            Err(error) => return Err(error),
        };

        // get the response from the server
        let response = try!(self.connection
            .exchange(codes::RpbIndexReq, codes::RpbIndexResp, &bytes));

        // convert the response to `RpbIndexResp`
        let rpb_index_resp = match parse_from_bytes::<RpbIndexResp>(&response) {
            Ok(rpb_index_resp) => rpb_index_resp,
            Err(error) => return Err(RiakErr::ProtobufError(error)),
        };

        // wrap the `RpbIndexResp`
        Ok(IndexResp::new_from_rpb(rpb_index_resp))
    }
}
