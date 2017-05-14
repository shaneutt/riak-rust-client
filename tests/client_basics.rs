extern crate riak;

use riak::Client;
use riak::bucket::{BucketProps, BucketTypeProps};
use riak::object::{DeleteObjectReq, FetchObjectReq, ObjectContent, StoreObjectReq};
use riak::yokozuna::{SearchQuery, YokozunaIndex};
use std::fs::File;
use std::io::Read;

#[test]
fn test_basics() {
    // connect and ping
    let mut riak = Client::new("10.0.0.2:8087").unwrap();
    riak.ping().unwrap();

    // get the server info
    let (node, version) = riak.server_info().unwrap();
    println!("connected to node {} running Riak version {}",
             node,
             version);

    // set bucket properties
    let mut bucket_props = BucketProps::new("testbucket");
    bucket_props.set_backend("leveldb");
    riak.set_bucket_properties(bucket_props).unwrap();

    // get the properties back from the server
    let bucket_props = riak.get_bucket_properties("testbucket").unwrap();
    let found_backend = bucket_props.get_backend().unwrap();
    assert_eq!(found_backend, "leveldb".as_bytes());

    // store an object
    let contents = ObjectContent::new("this is a test".as_bytes());
    let mut req = StoreObjectReq::new("testbucket", contents);
    req.set_key("testkey");
    riak.store_object(req).unwrap();

    // fetch an object
    let req = FetchObjectReq::new("testbucket", "testkey");
    let object = riak.fetch_object(req).unwrap();
    let contents = object.content;
    let content = contents.first().unwrap();
    assert_eq!(content.get_value(), "this is a test".as_bytes());

    // delete an object
    let req = DeleteObjectReq::new("testbucket", "testkey");
    riak.delete_object(req).unwrap();

    // make sure deleted object is gone
    let req = FetchObjectReq::new("testbucket", "testkey");
    let object = riak.fetch_object(req).unwrap();
    assert_eq!(object.content.len(), 0);

    // list the available buckets
    let buckets = riak.list_buckets().unwrap();
    let mut bucket_exists = false;
    for bucket in buckets.iter() {
        if *bucket == "testbucket".as_bytes() {
            bucket_exists = true;
        }
    }
    assert!(bucket_exists);

    // list the available keys
    let keys = riak.list_keys("testbucket").unwrap();
    let mut key_exists = false;
    for key in keys.iter() {
        if *key == "testkey".as_bytes() {
            key_exists = true;
        }
    }
    assert!(key_exists);

    // fetch the preflist for testbucket/testkey
    let preflist = riak.fetch_preflist("testbucket", "testkey").unwrap();
    let mut lives_on_nodes: u8 = 0;
    let mut has_primary_node = false;
    for preflist_item in preflist.iter() {
        lives_on_nodes = lives_on_nodes + 1;
        if preflist_item.is_primary {
            has_primary_node = true;
        }
    }
    assert_eq!(lives_on_nodes, 3);
    assert!(has_primary_node);

    // set properties for a bucket type
    let mut bucket_props = BucketTypeProps::new("testbuckettype");
    bucket_props.set_backend("leveldb");
    riak.set_bucket_type_properties(bucket_props).unwrap();

    // get the properties back for a bucket type and verify them
    let bucket_props = riak.get_bucket_type_properties("testbuckettype").unwrap();
    assert_eq!(bucket_props.get_backend().expect("could not get backend"),
               "leveldb".as_bytes());

    // set a search schema
    let mut xml: Vec<u8> = Vec::new();
    let mut file = File::open("/tmp/riak-rust-client-default-schema.xml").unwrap();
    let _ = file.read_to_end(&mut xml).unwrap();

    let schema_name = "schedule".to_string().into_bytes();
    riak.set_yokozuna_schema(schema_name.clone(), xml.clone()).unwrap();

    // retrieve the search schema
    let schema = riak.get_yokozuna_schema(schema_name.clone()).unwrap();
    assert_eq!(schema, xml);

    // set a search index
    let index_name = "myindex".to_string().into_bytes();
    let mut index = YokozunaIndex::new(index_name.clone());
    index.set_schema(schema_name);
    index.set_n_val(3);
    riak.set_yokozuna_index(index).unwrap();

    // get the search index
    let index = riak.get_yokozuna_index(index_name.clone()).unwrap();
    assert_eq!(index[0].get_name(), index_name);

    // run a search
    let mut query = SearchQuery::new("test*", "myindex");
    query.set_df("_yz_id");
    riak.search(query).unwrap();

    // run a MapReduce job
    let job = r#"
    {"inputs": "bucket_501653", "query": [
        {"map": {
            "arg": null,
            "name": "Riak.mapValuesJson",
            "language": "javascript",
            "keep": false
        }},
        {"reduce": {
            "arg": null,
            "name": "Riak.reduceSum",
            "language": "javascript",
            "keep": true
        }}
    ]}
    "#;
    riak.mapreduce(job, "application/json").unwrap();
}
