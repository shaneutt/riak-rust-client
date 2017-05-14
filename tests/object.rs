#![allow(non_upper_case_globals)]
extern crate riak;

use std::env;

use riak::Client;
use riak::bucket::*;
use riak::object::*;


const bucket_type: &'static str = "tests/object.rs";
const bucket: &'static str = "test_bucket";
const key: &'static str = "test_key";

#[test]
fn test_create_object_basic() {
    // get the test node
    let riak_node = match env::var("RIAK_RUST_TEST_NODE") {
        Ok(val) => val,
        Err(_) => "10.0.0.2:8087".to_string(),
    };

    // connect to Riak
    let mut riak = Client::new(&riak_node[..]).unwrap();
    riak.ping().unwrap();

    // set up the bucket type
    setup_bucket_type(bucket_type);

    // prepare the object content
    let mut content = ObjectContent::new("Hello Riak!");
    content.set_content_type("text/plain");
    content.set_charset("UTF-8");

    // prepare a store request
    let mut store_request = StoreObjectReq::new(bucket, content);
    store_request.set_bucket_type(bucket_type);
    store_request.set_key(key);
    store_request.set_dw(3);

    // make the store request
    riak.store_object(store_request).unwrap();

    // prepare a fetch request
    let mut fetch_request = FetchObjectReq::new(bucket, key);
    fetch_request.set_bucket_type(bucket_type);

    // make the fetch request
    let fetch_response = riak.fetch_object(fetch_request).unwrap();

    // check the object
    let content = fetch_response.content;
    println!("Number of siblings: {:?}", content.len());
}

fn setup_bucket_type<T: Into<Vec<u8>>>(bucket_type_name: T) {
    // convert the bucket_type_name to Vec<u8>
    let bucket_type_name = bucket_type_name.into();

    // connect to Riak
    let mut riak = Client::new("10.0.0.2:8087").unwrap();
    riak.ping().unwrap();

    // build properties for the bucket type
    let mut props = BucketTypeProps::new(bucket_type_name);
    props.set_backend("leveldb");

    // set the properties for the bucket type
    riak.set_bucket_type_properties(props).unwrap();
}
