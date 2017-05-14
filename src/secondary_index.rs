use errors::RiakErr;
use protobuf::{Message, RepeatedField};
use rpb::riak_kv::{RpbIndexReq, RpbIndexReq_IndexQueryType, RpbIndexResp};
use utils::{IndexRespPrivate, IndexQueryTypePrivate, ProtobufBytes};

/// represents a type of 2i query
#[derive(Clone, Debug)]
pub enum IndexQueryType {
    EQ,
    RANGE,
}

impl IndexQueryTypePrivate for IndexQueryType {
    fn to_rpb(self) -> RpbIndexReq_IndexQueryType {
        match self {
            IndexQueryType::EQ => RpbIndexReq_IndexQueryType::eq,
            IndexQueryType::RANGE => RpbIndexReq_IndexQueryType::range,
        }
    }
}

/// represents a 2i request
#[derive(Clone, Debug)]
pub struct IndexReq(RpbIndexReq);

impl IndexReq {
    /// constructs a new `IndexReq`
    pub fn new<T: Into<Vec<u8>>>(bucket_type: T,
                                 bucket: T,
                                 index: T,
                                 qtype: IndexQueryType)
                                 -> IndexReq {
        let mut req = RpbIndexReq::new();
        req.set_field_type(bucket_type.into());
        req.set_bucket(bucket.into());
        req.set_index(index.into());
        req.set_qtype(qtype.to_rpb());
        IndexReq(req)
    }

    /// get the "pagination_sort" property
    pub fn get_pagination_sort(&self) -> Option<bool> {
        if self.0.has_pagination_sort() {
            Some(self.0.get_pagination_sort())
        } else {
            None
        }
    }

    /// set the "pagination_sort" property
    pub fn set_pagination_sort(&mut self, pagination_sort: bool) {
        self.0.set_pagination_sort(pagination_sort);
    }

    /// get the "term_regex" property
    pub fn get_term_regex(&self) -> Option<&[u8]> {
        if self.0.has_term_regex() {
            Some(self.0.get_term_regex())
        } else {
            None
        }
    }

    /// set the "term_regex" property
    pub fn set_term_regex<T: Into<Vec<u8>>>(&mut self, term_regex: T) {
        self.0.set_term_regex(term_regex.into());
    }

    /// get the "key" property
    pub fn get_key(&self) -> Option<&[u8]> {
        if self.0.has_key() {
            Some(self.0.get_key())
        } else {
            None
        }
    }

    /// set the "key" property
    pub fn set_key<T: Into<Vec<u8>>>(&mut self, key: T) {
        self.0.set_key(key.into());
    }

    /// get the "stream" property
    pub fn get_stream(&self) -> Option<bool> {
        if self.0.has_stream() {
            Some(self.0.get_stream())
        } else {
            None
        }
    }

    /// set the "stream" property
    pub fn set_stream(&mut self, stream: bool) {
        self.0.set_stream(stream);
    }

    // TODO: implement the rest
}

impl ProtobufBytes for IndexReq {
    fn write_to_bytes(self) -> Result<Vec<u8>, RiakErr> {
        match self.0.write_to_bytes() {
            Ok(bytes) => Ok(bytes),
            Err(error) => return Err(RiakErr::ProtobufError(error)),
        }
    }
}

/// represents a 2i response
#[derive(Clone, Debug)]
pub struct IndexResp(RpbIndexResp);

impl IndexResp {
    pub fn new(keys: Vec<Vec<u8>>) -> IndexResp {
        let mut resp = RpbIndexResp::new();
        resp.set_keys(RepeatedField::from_vec(keys));
        resp.set_results(RepeatedField::from_vec(vec![]));
        IndexResp(resp)
    }

    // TODO: implement the rest
}

impl IndexRespPrivate for IndexResp {
    fn new_from_rpb(resp: RpbIndexResp) -> IndexResp {
        IndexResp(resp)
    }
}
