// Constants for Riak Protocol Buffers API Message Codes
//
// Documentation: https://docs.basho.com/riak/kv/latest/developing/api/protocol-buffers/#message-codes
//
//

#![allow(dead_code)] // FIXME;
#![allow(non_upper_case_globals)]
pub const RpbErrorResp: u8 = 0;
pub const RpbPingReq: u8 = 1;
pub const RpbPingResp: u8 = 2;
pub const RpbGetClientIdReq: u8 = 3;
pub const RpbGetClientIdResp: u8 = 4;
pub const RpbSetClientIdReq: u8 = 5;
pub const RpbSetClientIdResp: u8 = 6;
pub const RpbGetServerInfoReq: u8 = 7;
pub const RpbGetServerInfoResp: u8 = 8;
pub const RpbGetReq: u8 = 9;
pub const RpbGetResp: u8 = 10;
pub const RpbPutReq: u8 = 11;
pub const RpbPutResp: u8 = 12;
pub const RpbDelReq: u8 = 13;
pub const RpbDelResp: u8 = 14;
pub const RpbListBucketsReq: u8 = 15;
pub const RpbListBucketsResp: u8 = 16;
pub const RpbListKeysReq: u8 = 17;
pub const RpbListKeysResp: u8 = 18;
pub const RpbGetBucketReq: u8 = 19;
pub const RpbGetBucketResp: u8 = 20;
pub const RpbSetBucketReq: u8 = 21;
pub const RpbSetBucketResp: u8 = 22;
pub const RpbMapRedReq: u8 = 23;
pub const RpbMapRedResp: u8 = 24;
pub const RpbIndexReq: u8 = 25;
pub const RpbIndexResp: u8 = 26;
pub const RpbSearchQueryReq: u8 = 27;
pub const RpbSearchQueryResp: u8 = 28;
pub const RpbResetBucketReq: u8 = 29;
pub const RpbResetBucketResp: u8 = 30;
pub const RpbGetBucketTypeReq: u8 = 31;
pub const RpbSetBucketTypeReq: u8 = 32;
pub const RpbGetBucketKeyPreflistReq: u8 = 33;
pub const RpbGetBucketKeyPreflistResp: u8 = 34;
pub const RpbCSBucketReq: u8 = 40;
pub const RpbCSBucketResp: u8 = 41;
pub const RpbCounterUpdateReq: u8 = 50;
pub const RpbCounterUpdateResp: u8 = 51;
pub const RpbCounterGetReq: u8 = 52;
pub const RpbCounterGetResp: u8 = 53;
pub const RpbYokozunaIndexGetReq: u8 = 54;
pub const RpbYokozunaIndexGetResp: u8 = 55;
pub const RpbYokozunaIndexPutReq: u8 = 56;
pub const RpbYokozunaIndexDeleteReq: u8 = 57;
pub const RpbYokozunaSchemaGetReq: u8 = 58;
pub const RpbYokozunaSchemaGetResp: u8 = 59;
pub const RpbYokozunaSchemaPutReq: u8 = 60;
pub const DtFetchReq: u8 = 80;
pub const DtFetchResp: u8 = 81;
pub const DtUpdateReq: u8 = 82;
pub const DtUpdateResp: u8 = 83;
pub const TsQueryReq: u8 = 90;
pub const TsQueryResp: u8 = 91;
pub const TsPutReq: u8 = 92;
pub const TsPutResp: u8 = 93;
pub const TsDelReq: u8 = 94;
pub const TsDelResp: u8 = 95;
pub const TsGetReq: u8 = 96;
pub const TsGetResp: u8 = 97;
pub const TsListKeysReq: u8 = 98;
pub const TsListKeysResp: u8 = 99;
pub const RpbAuthReq: u8 = 253;
pub const RpbAuthResp: u8 = 254;
pub const RpbStartTls: u8 = 255;
