// TODO redo the "Private" traits
use data_type::{DataTypeFetchResp, DataTypeMapEntry, DataTypeMapField, DataTypeValue};
use errors::RiakErr;
use object::{FetchObjectResp, ObjectContent};
use rpb::riak::RpbBucketProps;
use rpb::riak_dt::{DtFetchResp, DtValue, MapEntry, MapField};
use rpb::riak_kv::{RpbContent, RpbGetResp, RpbIndexReq_IndexQueryType, RpbIndexResp};
use rpb::riak_search::RpbSearchQueryResp;
use rpb::riak_yokozuna::RpbYokozunaIndex;
use secondary_index::IndexResp;
use yokozuna::{SearchQueryResp, YokozunaIndex};

pub trait ProtobufBytes {
    fn write_to_bytes(self) -> Result<Vec<u8>, RiakErr>;
}

pub trait BucketPropsPrivate {
    fn set_props(&mut self, RpbBucketProps);
}

pub trait YokozunaIndexPrivate {
    fn new_from_rpb_yokozuna_index(RpbYokozunaIndex) -> YokozunaIndex;
}

pub trait ObjectContentPrivate {
    fn into_rpb(self) -> RpbContent;
    fn new_from_rpb(RpbContent) -> ObjectContent;
}

pub trait FetchObjectRespPrivate {
    fn new_from_rpb(RpbGetResp) -> FetchObjectResp;
}

pub trait SearchQueryRespPrivate {
    fn new_from_rpb(RpbSearchQueryResp) -> SearchQueryResp;
}

pub trait DataTypeValuePrivate {
    fn new_from_rpb(DtValue) -> DataTypeValue;
}

pub trait DataTypeMapEntryPrivate {
    fn new_from_rpb(MapEntry) -> DataTypeMapEntry;
}

pub trait DataTypeMapFieldPrivate {
    fn new_from_rpb(MapField) -> DataTypeMapField;
}

pub trait DataTypeFetchRespPrivate {
    fn new_from_rpb(DtFetchResp) -> DataTypeFetchResp;
}

pub trait IndexQueryTypePrivate {
    fn to_rpb(self) -> RpbIndexReq_IndexQueryType;
}

pub trait IndexRespPrivate {
    fn new_from_rpb(RpbIndexResp) -> IndexResp;
}
