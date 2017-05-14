/// Riak Search integrates Apache Solr for indexing and querying Riak KV
///
/// For more information: https://docs.basho.com/riak/kv/latest/developing/usage/search/

use errors::RiakErr;
use protobuf::{Message, RepeatedField};
use rpb::riak_yokozuna::{RpbYokozunaIndex, RpbYokozunaIndexPutReq};
use rpb::riak_search::{RpbSearchQueryReq, RpbSearchQueryResp};
use utils::{ProtobufBytes, SearchQueryRespPrivate, YokozunaIndexPrivate};

/// `YokozunaIndex` represents an index for Riak search
#[derive(Clone, Debug)]
pub struct YokozunaIndex(RpbYokozunaIndex);

impl YokozunaIndex {
    /// constructs a new `YokozunaIndex`
    pub fn new<T: Into<Vec<u8>>>(name: T) -> YokozunaIndex {
        let mut rpb_yokozuna_index = RpbYokozunaIndex::new();
        rpb_yokozuna_index.set_name(name.into());
        YokozunaIndex(rpb_yokozuna_index)
    }

    /// get the "name" property
    pub fn get_name(&self) -> Vec<u8> {
        self.0.get_name().to_vec()
    }

    /// set the "name" property
    pub fn set_name<T: Into<Vec<u8>>>(&mut self, name: T) {
        self.0.set_name(name.into());
    }

    /// get the "schema" property
    pub fn get_schema(&self) -> Option<Vec<u8>> {
        if self.0.has_schema() {
            Some(self.0.get_schema().to_vec())
        } else {
            None
        }
    }

    /// set the "schema" property
    pub fn set_schema<T: Into<Vec<u8>>>(&mut self, schema: T) {
        self.0.set_schema(schema.into());
    }

    /// get the "n_val" property
    pub fn get_n_val(&self) -> Option<u32> {
        if self.0.has_n_val() {
            Some(self.0.get_n_val())
        } else {
            None
        }
    }

    /// set the "n_val" property
    pub fn set_n_val(&mut self, n_val: u32) {
        self.0.set_n_val(n_val);
    }
}

impl ProtobufBytes for YokozunaIndex {
    fn write_to_bytes(self) -> Result<Vec<u8>, RiakErr> {
        let mut req = RpbYokozunaIndexPutReq::new();
        req.set_index(self.0.clone());
        match req.write_to_bytes() {
            Ok(bytes) => Ok(bytes),
            Err(error) => return Err(RiakErr::ProtobufError(error)),
        }
    }
}

impl YokozunaIndexPrivate for YokozunaIndex {
    fn new_from_rpb_yokozuna_index(rpb_yokozuna_index: RpbYokozunaIndex) -> YokozunaIndex {
        YokozunaIndex(rpb_yokozuna_index)
    }
}

/// `SearchQuery` represents a query that can be performed on Riak
#[derive(Clone, Debug)]
pub struct SearchQuery(RpbSearchQueryReq);

impl SearchQuery {
    /// constructs a new `SearchQuery`
    pub fn new<T: Into<Vec<u8>>>(q: T, index: T) -> SearchQuery {
        let mut req = RpbSearchQueryReq::new();
        req.set_q(q.into());
        req.set_index(index.into());
        SearchQuery(req)
    }

    /// get the "q" property
    pub fn get_q(&self) -> Vec<u8> {
        self.0.get_q().to_vec()
    }

    /// set the "q" property
    pub fn set_q<T: Into<Vec<u8>>>(&mut self, q: T) {
        self.0.set_q(q.into());
    }

    /// get the "index" property
    pub fn get_index(&self) -> Vec<u8> {
        self.0.get_index().to_vec()
    }

    /// set the "index" property
    pub fn set_index<T: Into<Vec<u8>>>(&mut self, index: T) {
        self.0.set_index(index.into());
    }

    /// get the "rows" property
    pub fn get_rows(&self) -> Option<u32> {
        if self.0.has_rows() {
            Some(self.0.get_rows())
        } else {
            None
        }
    }

    /// set the "rows" property
    pub fn set_rows(&mut self, rows: u32) {
        self.0.set_rows(rows);
    }

    /// get the "start" property
    pub fn get_start(&self) -> Option<u32> {
        if self.0.has_start() {
            Some(self.0.get_start())
        } else {
            None
        }
    }

    /// set the "start" property
    pub fn set_start(&mut self, start: u32) {
        self.0.set_start(start);
    }

    /// get the "sort" property
    pub fn get_sort(&self) -> Option<Vec<u8>> {
        if self.0.has_sort() {
            Some(self.0.get_sort().to_vec())
        } else {
            None
        }
    }

    /// set the "sort" property
    pub fn set_sort<T: Into<Vec<u8>>>(&mut self, sort: T) {
        self.0.set_sort(sort.into());
    }

    /// get the "filter" property
    pub fn get_filter(&self) -> Option<Vec<u8>> {
        if self.0.has_filter() {
            Some(self.0.get_filter().to_vec())
        } else {
            None
        }
    }

    /// set the "filter" property
    pub fn set_filter<T: Into<Vec<u8>>>(&mut self, filter: T) {
        self.0.set_filter(filter.into());
    }

    /// get the "df" property
    pub fn get_df(&self) -> Option<Vec<u8>> {
        if self.0.has_df() {
            Some(self.0.get_df().to_vec())
        } else {
            None
        }
    }

    /// set the "df" property
    pub fn set_df<T: Into<Vec<u8>>>(&mut self, df: T) {
        self.0.set_df(df.into());
    }

    /// get the "op" property
    pub fn get_op(&self) -> Option<Vec<u8>> {
        if self.0.has_op() {
            Some(self.0.get_op().to_vec())
        } else {
            None
        }
    }

    /// set the "op" property
    pub fn set_op<T: Into<Vec<u8>>>(&mut self, op: T) {
        self.0.set_op(op.into());
    }

    /// get the "fl" property
    pub fn get_fl(&self) -> Option<Vec<Vec<u8>>> {
        if self.0.get_fl().len() > 0 {
            Some(self.0.get_fl().to_vec())
        } else {
            None
        }
    }

    /// set the "fl" property
    pub fn set_fl(&mut self, fl: Vec<Vec<u8>>) {
        self.0.set_fl(RepeatedField::from_vec(fl));
    }

    /// get the "presort" property
    pub fn get_presort(&self) -> Option<Vec<u8>> {
        if self.0.has_presort() {
            Some(self.0.get_presort().to_vec())
        } else {
            None
        }
    }

    /// set the "presort" property
    pub fn set_presort<T: Into<Vec<u8>>>(&mut self, presort: T) {
        self.0.set_presort(presort.into());
    }
}

impl ProtobufBytes for SearchQuery {
    fn write_to_bytes(self) -> Result<Vec<u8>, RiakErr> {
        match self.0.write_to_bytes() {
            Ok(bytes) => Ok(bytes),
            Err(error) => Err(RiakErr::ProtobufError(error)),
        }
    }
}

/// `SearchQueryResp` represents the response for a successful query
#[derive(Clone, Debug)]
pub struct SearchQueryResp {
    pub pairs: Vec<(Vec<u8>, Option<Vec<u8>>)>,
    pub max_score: Option<f32>,
    pub num_found: Option<u32>,
}

impl SearchQueryRespPrivate for SearchQueryResp {
    fn new_from_rpb(mut rpb: RpbSearchQueryResp) -> SearchQueryResp {
        let max_score = rpb.get_max_score();
        let num_found = rpb.get_num_found();

        let mut pairs: Vec<(Vec<u8>, Option<Vec<u8>>)> = Vec::new();
        for mut rpb_search_doc in rpb.take_docs().into_iter() {
            for mut pair in rpb_search_doc.take_fields().into_iter() {
                if pair.has_value() {
                    pairs.push((pair.take_key(), Some(pair.take_value())));
                } else {
                    pairs.push((pair.take_key(), None))
                }
            }
        }

        SearchQueryResp {
            pairs: pairs,
            max_score: Some(max_score),
            num_found: Some(num_found),
        }
    }
}
