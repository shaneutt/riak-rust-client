/// A list of every bucket on the cluster (Note: These functions can be expensive for the server. Do not
/// use in performance-sensitive code.)
///
/// For more information: https://docs.basho.com/riak/kv/latest/developing/api/protocol-buffers/list-buckets/
///
/// TODO: this mod needs a serious refactor, it's messy

use Client;
use connection::RiakConn;
use errors::RiakErr;
use protobuf::{Message, parse_from_bytes};
use rpb::codes;
use rpb::riak_kv::{RpbListBucketsResp, RpbListBucketsReq, RpbIndexResp, RpbListKeysResp,
                   RpbListKeysReq};
use secondary_index::{IndexReq, IndexResp};
use utils::{IndexRespPrivate, ProtobufBytes};

/// `BucketStream` represents a list of bucket names in Riak
#[derive(Debug)]
pub struct BucketStream {
    connection: RiakConn,
    done: bool,
    first_request_made: bool,
}

impl BucketStream {
    /// constructs a new `BucketStream`
    pub fn new(client: &mut Client) -> Result<BucketStream, RiakErr> {
        let connection = match RiakConn::new(client.connection.peer_addr,
                                             client.connection.timeout) {
            Ok(connection) => connection,
            Err(error) => return Err(error),
        };
        Ok(BucketStream {
            connection: connection,
            done: false,
            first_request_made: false,
        })
    }

    /// return a list of every bucket from the stream
    pub fn all(&mut self) -> Result<Vec<Vec<u8>>, RiakErr> {
        let mut buckets: Vec<Vec<u8>> = Vec::new();

        loop {
            let next_buckets = match self.next() {
                Some(result) => {
                    match result {
                        Ok(next_buckets) => next_buckets,
                        Err(error) => return Err(error),
                    }
                }
                None => break,
            };
            buckets.extend(next_buckets.iter().cloned());
        }

        Ok(buckets)
    }

    /// return the next group of buckets from the stream
    pub fn next(&mut self) -> Option<Result<Vec<Vec<u8>>, RiakErr>> {
        if self.done {
            return None;
        }

        if self.first_request_made {
            // get the next response from Riak
            let response = match self.connection.receive(codes::RpbListBucketsResp) {
                Ok(response) => response,
                Err(error) => return Some(Err(error)),
            };

            // parse the response
            let mut rpb_resp = match parse_from_bytes::<RpbListBucketsResp>(&response) {
                Ok(parsed) => parsed,
                Err(error) => {
                    match self.connection.reconnect() {
                        Ok(()) => (),
                        Err(error) => {
                            debug!("failure during reconnect: {:?}", error);
                            return Some(Err(error));
                        }
                    };
                    return Some(Err(RiakErr::ProtobufError(error)));
                }
            };

            // retrieve the buckets from the rpb response
            let resp_buckets = rpb_resp.take_buckets();

            // get the buckets and convert them into a Vec<Vec<u8>>
            let mut buckets: Vec<Vec<u8>> = Vec::new();
            for bucket in resp_buckets.into_iter() {
                buckets.push(bucket);
            }

            self.done = rpb_resp.get_done();

            Some(Ok(buckets))
        } else {
            // build the request
            let mut request = RpbListBucketsReq::new();
            request.set_stream(true);
            request.set_timeout(self.connection.timeout);
            let bytes = match request.write_to_bytes() {
                Ok(bytes) => bytes,
                Err(error) => return Some(Err(RiakErr::ProtobufError(error))),
            };

            // send the request and get the response
            let response = match self.connection
                .exchange(codes::RpbListBucketsReq, codes::RpbListBucketsResp, &bytes) {
                Ok(response) => response,
                Err(error) => {
                    match self.connection.reconnect() {
                        Ok(()) => (),
                        Err(error) => {
                            debug!("failure during reconnect: {:?}", error);
                            return Some(Err(error));
                        }
                    };
                    return Some(Err(error));
                }
            };

            // parse the response
            let mut rpb_resp = match parse_from_bytes::<RpbListBucketsResp>(&response) {
                Ok(parsed) => parsed,
                Err(error) => {
                    match self.connection.reconnect() {
                        Ok(()) => (),
                        Err(error) => {
                            debug!("failure during reconnect: {:?}", error);
                            return Some(Err(error));
                        }
                    };
                    return Some(Err(RiakErr::ProtobufError(error)));
                }
            };

            // retrieve the buckets from the first response
            let resp_buckets = rpb_resp.take_buckets();

            // store all found buckets
            let mut buckets: Vec<Vec<u8>> = Vec::new();
            for bucket in resp_buckets.into_iter() {
                buckets.push(bucket);
            }

            self.first_request_made = true;
            self.done = rpb_resp.get_done();

            Some(Ok(buckets))
        }
    }
}

/// `KeyStream` represents a list of keys in a Riak bucket
#[derive(Debug)]
pub struct KeyStream {
    bucket: Vec<u8>,
    connection: RiakConn,
    done: bool,
    first_request_made: bool,
}

impl KeyStream {
    /// constructs a new `KeyStream`
    pub fn new(client: &mut Client, bucket: Vec<u8>) -> Result<KeyStream, RiakErr> {
        let connection = match RiakConn::new(client.connection.peer_addr,
                                             client.connection.timeout) {
            Ok(connection) => connection,
            Err(error) => return Err(error),
        };
        Ok(KeyStream {
            bucket: bucket,
            connection: connection,
            done: false,
            first_request_made: false,
        })
    }

    /// return a list of all the keys from the stream
    pub fn all(&mut self) -> Result<Vec<Vec<u8>>, RiakErr> {
        let mut keys: Vec<Vec<u8>> = Vec::new();

        loop {
            let new_keys = match self.next() {
                Some(result) => {
                    match result {
                        Ok(new_keys) => new_keys,
                        Err(error) => return Err(error),
                    }
                }
                None => break,
            };
            keys.extend(new_keys);
        }

        Ok(keys)
    }

    /// return the next group of keys from the stream
    pub fn next(&mut self) -> Option<Result<Vec<Vec<u8>>, RiakErr>> {
        if self.done {
            return None;
        }

        if self.first_request_made {
            // get the next response from Riak
            let response = match self.connection.receive(codes::RpbListKeysResp) {
                Ok(response) => response,
                Err(error) => return Some(Err(error)),
            };

            // parse the response
            let mut rpb_resp = match parse_from_bytes::<RpbListKeysResp>(&response) {
                Ok(parsed) => parsed,
                Err(error) => {
                    match self.connection.reconnect() {
                        Ok(()) => (),
                        Err(error) => {
                            debug!("failure during reconnect: {:?}", error);
                            return Some(Err(error));
                        }
                    };
                    return Some(Err(RiakErr::ProtobufError(error)));
                }
            };

            // retrieve the keys from the response
            let resp_keys = rpb_resp.take_keys();

            // get the keys and convert them into a Vec<Vec<u8>>
            let mut keys: Vec<Vec<u8>> = Vec::new();
            for key in resp_keys.into_iter() {
                keys.push(key);
            }

            self.done = rpb_resp.get_done();

            Some(Ok(keys))
        } else {
            // build the request
            let mut request = RpbListKeysReq::new();

            request.set_bucket(self.bucket.clone());
            request.set_timeout(self.connection.timeout);

            let bytes = match request.write_to_bytes() {
                Ok(bytes) => bytes,
                Err(error) => return Some(Err(RiakErr::ProtobufError(error))),
            };

            // send the request and get the response
            let response = match self.connection
                .exchange(codes::RpbListKeysReq, codes::RpbListKeysResp, &bytes) {
                Ok(response) => response,
                Err(error) => {
                    match self.connection.reconnect() {
                        Ok(()) => (),
                        Err(error) => {
                            debug!("failure during reconnect: {:?}", error);
                            return Some(Err(error));
                        }
                    };
                    return Some(Err(error));
                }
            };

            // parse the response
            let mut rpb_resp = match parse_from_bytes::<RpbListKeysResp>(&response) {
                Ok(parsed) => parsed,
                Err(error) => {
                    match self.connection.reconnect() {
                        Ok(()) => (),
                        Err(error) => {
                            debug!("failure during reconnect: {:?}", error);
                            return Some(Err(error));
                        }
                    };
                    return Some(Err(RiakErr::ProtobufError(error)));
                }
            };

            // retrieve the keys from the first response
            let resp_keys = rpb_resp.take_keys();

            // store all found keys
            let mut keys: Vec<Vec<u8>> = Vec::new();
            for key in resp_keys.into_iter() {
                keys.push(key);
            }

            self.first_request_made = true;
            self.done = rpb_resp.get_done();

            Some(Ok(keys))
        }
    }
}

/// SecondaryIndexStream represents a streaming 2i search
#[derive(Debug)]
pub struct SecondaryIndexStream {
    connection: RiakConn,
    done: bool,
    first_request_made: bool,
    request: IndexReq,
}

impl SecondaryIndexStream {
    /// constructs a new `SecondaryIndexStream`
    pub fn new(client: &mut Client, request: IndexReq) -> Result<SecondaryIndexStream, RiakErr> {
        let connection = match RiakConn::new(client.connection.peer_addr,
                                             client.connection.timeout) {
            Ok(connection) => connection,
            Err(error) => return Err(error),
        };
        Ok(SecondaryIndexStream {
            connection: connection,
            done: false,
            first_request_made: false,
            request: request,
        })
    }

    /// retrieves all the IndexResp for the IndexReq
    pub fn all(&mut self) -> Result<Vec<IndexResp>, RiakErr> {
        let mut responses: Vec<IndexResp> = Vec::new();
        while !self.done {
            match self.next() {
                Some(Ok(resp)) => responses.push(resp),
                Some(Err(error)) => return Err(error),
                None => {
                    self.done = true;
                }
            };
        }
        Ok(responses)
    }

    /// retrieves the next IndexResp
    pub fn next(&mut self) -> Option<Result<IndexResp, RiakErr>> {
        // short circuit if there's nothing more
        if self.done {
            return None;
        }

        if self.first_request_made {
            // get the next response from the server
            let response = match self.connection.receive(codes::RpbIndexResp) {
                Ok(rpb_index_resp) => rpb_index_resp,
                Err(error) => return Some(Err(error)),
            };

            // convert the response to `RpbIndexResp`
            let rpb_index_resp = match parse_from_bytes::<RpbIndexResp>(&response) {
                Ok(rpb_index_resp) => rpb_index_resp,
                Err(error) => return Some(Err(RiakErr::ProtobufError(error))),
            };

            self.done = rpb_index_resp.get_done();

            // wrap the `RpbIndexResp`
            Some(Ok(IndexResp::new_from_rpb(rpb_index_resp)))
        } else {
            // convert the `IndexReq` to bytes
            let bytes = match self.request.clone().write_to_bytes() {
                Ok(bytes) => bytes,
                Err(error) => return Some(Err(error)),
            };

            // get the response from the server
            let response = match self.connection
                .exchange(codes::RpbIndexReq, codes::RpbIndexResp, &bytes) {
                Ok(rpb_index_resp) => rpb_index_resp,
                Err(error) => return Some(Err(error)),
            };

            // convert the response to `RpbIndexResp`
            let rpb_index_resp = match parse_from_bytes::<RpbIndexResp>(&response) {
                Ok(rpb_index_resp) => rpb_index_resp,
                Err(error) => return Some(Err(RiakErr::ProtobufError(error))),
            };

            self.done = rpb_index_resp.get_done();

            // wrap the `RpbIndexResp`
            Some(Ok(IndexResp::new_from_rpb(rpb_index_resp)))
        }
    }
}
