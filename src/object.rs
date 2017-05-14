/// Object related structs and functions for Riak objects.
///
/// For more information: https://docs.basho.com/riak/kv/latest/developing/usage/creating-objects/

use errors::RiakErr;
use protobuf::{Message, RepeatedField};
use rpb::riak::RpbPair;
use rpb::riak_kv::{RpbContent, RpbDelReq, RpbGetResp, RpbGetReq, RpbPutReq};
use utils::{FetchObjectRespPrivate, ObjectContentPrivate, ProtobufBytes};

/// `DeleteObjectReq` represents a request to delete an object from Riak
#[derive(Clone, Debug)]
pub struct DeleteObjectReq(RpbDelReq);

impl DeleteObjectReq {
    /// constructs a new `DeleteObjectReq`
    pub fn new<T: Into<Vec<u8>>>(bucket: T, key: T) -> DeleteObjectReq {
        let mut req = DeleteObjectReq(RpbDelReq::new());
        req.0.set_bucket(bucket.into());
        req.0.set_key(key.into());
        req
    }

    /// get the the "bucket_type" property
    pub fn get_bucket_type(&self) -> Option<Vec<u8>> {
        if self.0.has_field_type() {
            Some(self.0.get_field_type().to_vec())
        } else {
            None
        }
    }

    /// set the the "bucket_type" property
    pub fn set_bucket_type<T: Into<Vec<u8>>>(&mut self, bucket_type: T) {
        self.0.set_field_type(bucket_type.into());
    }

    /// get the the "bucket" property
    pub fn get_bucket(&self) -> Vec<u8> {
        self.0.get_bucket().to_vec()
    }

    /// set the the "bucket" property
    pub fn set_bucket<T: Into<Vec<u8>>>(&mut self, bucket: T) {
        self.0.set_bucket(bucket.into());
    }

    /// get the the "key" property
    pub fn get_key(&self) -> Vec<u8> {
        self.0.get_key().to_vec()
    }

    /// set the the "key" property
    pub fn set_key<T: Into<Vec<u8>>>(&mut self, key: T) {
        self.0.set_key(key.into());
    }

    /// get the the "dw" property
    pub fn get_dw(&self) -> Option<u32> {
        if self.0.has_dw() {
            Some(self.0.get_dw())
        } else {
            None
        }
    }

    /// set the the "dw" property
    pub fn set_dw(&mut self, dw: u32) {
        self.0.set_dw(dw);
    }

    /// get the the "pr" property
    pub fn get_pr(&self) -> Option<u32> {
        if self.0.has_pr() {
            Some(self.0.get_pr())
        } else {
            None
        }
    }

    /// set the the "pr" property
    pub fn set_pr(&mut self, pr: u32) {
        self.0.set_pr(pr);
    }

    /// get the the "pw" property
    pub fn get_pw(&self) -> Option<u32> {
        if self.0.has_pw() {
            Some(self.0.get_pw())
        } else {
            None
        }
    }

    /// set the the "pw" property
    pub fn set_pw(&mut self, pw: u32) {
        self.0.set_pw(pw);
    }

    /// get the the "rw" property
    pub fn get_rw(&self) -> Option<u32> {
        if self.0.has_rw() {
            Some(self.0.get_rw())
        } else {
            None
        }
    }

    /// set the the "rw" property
    pub fn set_rw(&mut self, rw: u32) {
        self.0.set_rw(rw);
    }

    /// get the the "r" property
    pub fn get_r(&self) -> Option<u32> {
        if self.0.has_r() {
            Some(self.0.get_r())
        } else {
            None
        }
    }

    /// set the the "r" property
    pub fn set_r(&mut self, r: u32) {
        self.0.set_r(r);
    }

    /// get the the "w" property
    pub fn get_w(&self) -> Option<u32> {
        if self.0.has_w() {
            Some(self.0.get_w())
        } else {
            None
        }
    }

    /// set the the "w" property
    pub fn set_w(&mut self, w: u32) {
        self.0.set_w(w);
    }

    /// get the the "n_val" property
    pub fn get_n_val(&self) -> Option<u32> {
        if self.0.has_n_val() {
            Some(self.0.get_n_val())
        } else {
            None
        }
    }

    /// set the the "n_val" property
    pub fn set_n_val(&mut self, n_val: u32) {
        self.0.set_n_val(n_val);
    }

    /// get the the "sloppy_quorum" property
    pub fn get_sloppy_quorum(&self) -> Option<bool> {
        if self.0.has_sloppy_quorum() {
            Some(self.0.get_sloppy_quorum())
        } else {
            None
        }
    }

    /// set the the "sloppy_quorum" property
    pub fn set_sloppy_quorum(&mut self, sloppy_quorum: bool) {
        self.0.set_sloppy_quorum(sloppy_quorum);
    }

    /// get the the "timeout" property
    pub fn get_timeout(&self) -> Option<u32> {
        if self.0.has_timeout() {
            Some(self.0.get_timeout())
        } else {
            None
        }
    }

    /// set the the "timeout" property
    pub fn set_timeout(&mut self, timeout: u32) {
        self.0.set_timeout(timeout);
    }

    /// get the the "vclock" property
    pub fn get_vclock(&self) -> Option<Vec<u8>> {
        if self.0.has_vclock() {
            let vclock = self.0.get_vclock().to_vec();
            Some(vclock)
        } else {
            None
        }
    }

    /// set the the "vclock" property
    pub fn set_vclock<T: Into<Vec<u8>>>(&mut self, vclock: T) {
        self.0.set_vclock(vclock.into());
    }
}

impl ProtobufBytes for DeleteObjectReq {
    fn write_to_bytes(self) -> Result<Vec<u8>, RiakErr> {
        match self.0.write_to_bytes() {
            Ok(bytes) => Ok(bytes),
            Err(error) => Err(RiakErr::ProtobufError(error)),
        }
    }
}

/// `StoreObjectReq` represents the data used to store an object in Riak
#[derive(Clone, Debug)]
pub struct StoreObjectReq(RpbPutReq, ObjectContent);

impl StoreObjectReq {
    /// constructs a new `StoreObjectReq`
    pub fn new<T: Into<Vec<u8>>>(bucket: T, content: ObjectContent) -> StoreObjectReq {
        let mut rpb_put_req = RpbPutReq::new();
        rpb_put_req.set_bucket(bucket.into());
        StoreObjectReq(rpb_put_req, content)
    }

    /// get the "bucket" property
    pub fn get_bucket(&self) -> &[u8] {
        self.0.get_bucket()
    }

    /// set the "bucket" property
    pub fn set_bucket<T: Into<Vec<u8>>>(&mut self, bucket: T) {
        self.0.set_bucket(bucket.into());
    }

    /// get the "content" property
    pub fn get_content(&self) -> &ObjectContent {
        &self.1
    }

    /// set the "content" property
    pub fn set_content(&mut self, content: ObjectContent) {
        self.1 = content;
    }

    /// get the "asis" property
    pub fn get_asis(&self) -> Option<bool> {
        if self.0.has_asis() {
            Some(self.0.get_asis())
        } else {
            None
        }
    }

    /// set the "asis" property
    pub fn set_asis(&mut self, asis: bool) {
        self.0.set_asis(asis);
    }

    /// get the "bucket_type" property
    pub fn get_bucket_type(&self) -> Option<&[u8]> {
        if self.0.has_field_type() {
            Some(self.0.get_field_type())
        } else {
            None
        }
    }

    /// set the "bucket_type" property
    pub fn set_bucket_type<T: Into<Vec<u8>>>(&mut self, bucket_type: T) {
        self.0.set_field_type(bucket_type.into());
    }

    /// get the "if_none_match" property
    pub fn get_if_none_match(&self) -> Option<bool> {
        if self.0.has_if_none_match() {
            Some(self.0.get_if_none_match())
        } else {
            None
        }
    }

    /// set the "if_none_match" property
    pub fn set_if_none_match(&mut self, if_none_match: bool) {
        self.0.set_if_none_match(if_none_match);
    }

    /// get the "if_not_modified" property
    pub fn get_if_not_modified(&self) -> Option<bool> {
        if self.0.has_if_not_modified() {
            Some(self.0.get_if_not_modified())
        } else {
            None
        }
    }

    /// set the "if_not_modified" property
    pub fn set_if_not_modified(&mut self, if_not_modified: bool) {
        self.0.set_if_not_modified(if_not_modified);
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

    /// get the "return_body" property
    pub fn get_return_body(&self) -> Option<bool> {
        if self.0.has_return_body() {
            Some(self.0.get_return_body())
        } else {
            None
        }
    }

    /// set the "return_body" property
    pub fn set_return_body(&mut self, return_body: bool) {
        self.0.set_return_body(return_body);
    }

    /// get the "return_head" property
    pub fn get_return_head(&self) -> Option<bool> {
        if self.0.has_return_head() {
            Some(self.0.get_return_head())
        } else {
            None
        }
    }

    /// set the "return_head" property
    pub fn set_return_head(&mut self, return_head: bool) {
        self.0.set_return_head(return_head);
    }

    /// get the "sloppy_quorum" property
    pub fn get_sloppy_quorum(&self) -> Option<bool> {
        if self.0.has_sloppy_quorum() {
            Some(self.0.get_sloppy_quorum())
        } else {
            None
        }
    }

    /// set the "sloppy_quorum" property
    pub fn set_sloppy_quorum(&mut self, sloppy_quorum: bool) {
        self.0.set_sloppy_quorum(sloppy_quorum);
    }

    /// get the "timeout" property
    pub fn get_timeout(&self) -> Option<u32> {
        if self.0.has_timeout() {
            Some(self.0.get_timeout())
        } else {
            None
        }
    }

    /// set the "timeout" property
    pub fn set_timeout(&mut self, timeout: u32) {
        self.0.set_timeout(timeout);
    }

    /// get the "vclock" property
    pub fn get_vclock(&self) -> Option<&[u8]> {
        if self.0.has_vclock() {
            Some(self.0.get_vclock())
        } else {
            None
        }
    }

    /// set the "vclock" property
    pub fn set_vclock<T: Into<Vec<u8>>>(&mut self, vclock: T) {
        self.0.set_vclock(vclock.into());
    }

    /// get the "dw" property
    pub fn get_dw(&self) -> Option<u32> {
        if self.0.has_dw() {
            Some(self.0.get_dw())
        } else {
            None
        }
    }

    /// set the "dw" property
    pub fn set_dw(&mut self, dw: u32) {
        self.0.set_dw(dw);
    }

    /// get the "pw" property
    pub fn get_pw(&self) -> Option<u32> {
        if self.0.has_pw() {
            Some(self.0.get_pw())
        } else {
            None
        }
    }

    /// set the "pw" property
    pub fn set_pw(&mut self, pw: u32) {
        self.0.set_pw(pw);
    }

    /// get the "w" property
    pub fn get_w(&self) -> Option<u32> {
        if self.0.has_w() {
            Some(self.0.get_w())
        } else {
            None
        }
    }

    /// set the "w" property
    pub fn set_w(&mut self, w: u32) {
        self.0.set_w(w);
    }
}

impl ProtobufBytes for StoreObjectReq {
    fn write_to_bytes(mut self) -> Result<Vec<u8>, RiakErr> {
        if !self.0.has_content() {
            self.0.set_content(self.1.into_rpb())
        }
        match self.0.write_to_bytes() {
            Ok(bytes) => Ok(bytes),
            Err(error) => Err(RiakErr::ProtobufError(error)),
        }
    }
}

/// `FetchObjectReq` represents the data used to perform a fetch object request
#[derive(Clone, Debug)]
pub struct FetchObjectReq(RpbGetReq);

impl FetchObjectReq {
    /// constructs a new `FetchObjectReq`
    pub fn new<T: Into<Vec<u8>>>(bucket: T, key: T) -> FetchObjectReq {
        let mut req = RpbGetReq::new();
        req.set_bucket(bucket.into());
        req.set_key(key.into());
        FetchObjectReq(req)
    }

    /// get the "bucket" property
    pub fn get_bucket(&self) -> &[u8] {
        self.0.get_bucket()
    }

    /// set the "bucket" property
    pub fn set_bucket<T: Into<Vec<u8>>>(&mut self, bucket: T) {
        self.0.set_bucket(bucket.into());
    }

    /// get the "key" property
    pub fn get_key(&self) -> &[u8] {
        self.0.get_key()
    }

    /// set the "key" property
    pub fn set_key<T: Into<Vec<u8>>>(&mut self, key: T) {
        self.0.set_key(key.into());
    }

    /// get the "r" property
    pub fn get_r(&self) -> Option<u32> {
        if self.0.has_r() {
            Some(self.0.get_r())
        } else {
            None
        }
    }

    /// set the "r" property
    pub fn set_r(&mut self, r: u32) {
        self.0.set_r(r);
    }

    /// get the "pr" property
    pub fn get_pr(&self) -> Option<u32> {
        if self.0.has_pr() {
            Some(self.0.get_pr())
        } else {
            None
        }
    }

    /// set the "pr" property
    pub fn set_pr(&mut self, pr: u32) {
        self.0.set_pr(pr);
    }

    /// get the "basic_quorum" property
    pub fn get_basic_quorum(&self) -> Option<bool> {
        if self.0.has_basic_quorum() {
            Some(self.0.get_basic_quorum())
        } else {
            None
        }
    }

    /// set the "basic_quorum" property
    pub fn set_basic_quorum(&mut self, basic_quorum: bool) {
        self.0.set_basic_quorum(basic_quorum);
    }

    /// get the "notfound_ok" property
    pub fn get_notfound_ok(&self) -> Option<bool> {
        if self.0.has_notfound_ok() {
            Some(self.0.get_notfound_ok())
        } else {
            None
        }
    }

    /// set the "notfound_ok" property
    pub fn set_notfound_ok(&mut self, notfound_ok: bool) {
        self.0.set_notfound_ok(notfound_ok);
    }

    /// get the "if_modified" property
    pub fn get_if_modified(&self) -> Option<&[u8]> {
        if self.0.has_if_modified() {
            Some(self.0.get_if_modified())
        } else {
            None
        }
    }

    /// set the "if_modified" property
    pub fn set_if_modified<T: Into<Vec<u8>>>(&mut self, if_modified: T) {
        self.0.set_if_modified(if_modified.into());
    }

    /// get the "head" property
    pub fn get_head(&self) -> Option<bool> {
        if self.0.has_head() {
            Some(self.0.get_head())
        } else {
            None
        }
    }

    /// set the "head" property
    pub fn set_head(&mut self, head: bool) {
        self.0.set_head(head);
    }

    /// get the "deleted_vclock" property
    pub fn get_deleted_vclock(&self) -> Option<bool> {
        if self.0.has_deletedvclock() {
            Some(self.0.get_deletedvclock())
        } else {
            None
        }
    }

    /// set the "deleted_vclock" property
    pub fn set_deleted_vclock(&mut self, deleted_vclock: bool) {
        self.0.set_deletedvclock(deleted_vclock);
    }

    /// get the "timeout" property
    pub fn get_timeout(&self) -> Option<u32> {
        if self.0.has_timeout() {
            Some(self.0.get_timeout())
        } else {
            None
        }
    }

    /// set the "timeout" property
    pub fn set_timeout(&mut self, timeout: u32) {
        self.0.set_timeout(timeout);
    }

    /// get the "sloppy_quorum" property
    pub fn get_sloppy_quorum(&self) -> Option<bool> {
        if self.0.has_sloppy_quorum() {
            Some(self.0.get_sloppy_quorum())
        } else {
            None
        }
    }

    /// set the "sloppy_quorum" property
    pub fn set_sloppy_quorum(&mut self, sloppy_quorum: bool) {
        self.0.set_sloppy_quorum(sloppy_quorum);
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

    /// get the "bucket_type" property
    pub fn get_bucket_type(&self) -> Option<&[u8]> {
        if self.0.has_field_type() {
            Some(self.0.get_field_type())
        } else {
            None
        }
    }

    /// set the "bucket_type" property
    pub fn set_bucket_type<T: Into<Vec<u8>>>(&mut self, bucket_type: T) {
        self.0.set_field_type(bucket_type.into());
    }
}

impl ProtobufBytes for FetchObjectReq {
    fn write_to_bytes(self) -> Result<Vec<u8>, RiakErr> {
        match self.0.write_to_bytes() {
            Ok(bytes) => Ok(bytes),
            Err(err) => Err(RiakErr::ProtobufError(err)),
        }
    }
}

/// `FetchObjectResp` represents the response received from Riak when storing an object
#[derive(Clone, Debug)]
pub struct FetchObjectResp {
    pub content: Vec<ObjectContent>,
    pub vclock: Vec<u8>,
    pub unchanged: Option<bool>,
}

impl FetchObjectRespPrivate for FetchObjectResp {
    fn new_from_rpb(mut rpb: RpbGetResp) -> FetchObjectResp {
        let mut contents: Vec<ObjectContent> = Vec::new();

        let rpb_content = rpb.take_content();
        for content in rpb_content.into_iter() {
            contents.push(ObjectContent::new_from_rpb(content));
        }

        let mut response = FetchObjectResp {
            content: contents,
            vclock: rpb.take_vclock(),
            unchanged: None,
        };

        if rpb.has_unchanged() {
            response.unchanged = Some(rpb.get_unchanged());
        }

        response
    }
}


/// `ObjectContent` represents the contents of a Riak object
#[derive(Clone, Debug)]
pub struct ObjectContent(RpbContent);

impl ObjectContent {
    pub fn new<T: Into<Vec<u8>>>(value: T) -> ObjectContent {
        let mut rpb_content = RpbContent::new();
        rpb_content.set_value(value.into());
        ObjectContent(rpb_content)
    }

    /// get the "value" property
    pub fn get_value(&self) -> &[u8] {
        self.0.get_value()
    }

    /// set the "value" property
    pub fn set_value<T: Into<Vec<u8>>>(&mut self, value: T) {
        self.0.set_value(value.into());
    }

    /// get the "content_type" property
    pub fn get_content_type(&self) -> Option<&[u8]> {
        if self.0.has_content_type() {
            Some(self.0.get_content_type())
        } else {
            None
        }
    }

    /// set the "content_type" property
    pub fn set_content_type<T: Into<Vec<u8>>>(&mut self, content_type: T) {
        self.0.set_content_type(content_type.into());
    }

    /// get the "charset" property
    pub fn get_charset(&self) -> Option<&[u8]> {
        if self.0.has_charset() {
            Some(self.0.get_charset())
        } else {
            None
        }
    }

    /// set the "charset" property
    pub fn set_charset<T: Into<Vec<u8>>>(&mut self, charset: T) {
        self.0.set_charset(charset.into());
    }

    /// get the "content_encoding" property
    pub fn get_content_encoding(&self) -> Option<&[u8]> {
        if self.0.has_content_encoding() {
            Some(self.0.get_content_encoding())
        } else {
            None
        }
    }

    /// set the "content_encoding" property
    pub fn set_content_encoding<T: Into<Vec<u8>>>(&mut self, content_encoding: T) {
        self.0.set_content_encoding(content_encoding.into());
    }

    /// get the "vtag" property
    pub fn get_vtag(&self) -> Option<&[u8]> {
        if self.0.has_vtag() {
            Some(self.0.get_vtag())
        } else {
            None
        }
    }

    /// set the "vtag" property
    pub fn set_vtag<T: Into<Vec<u8>>>(&mut self, vtag: T) {
        self.0.set_vtag(vtag.into());
    }

    /// get the "last_mod" property
    pub fn get_last_mod(&self) -> Option<u32> {
        if self.0.has_last_mod() {
            Some(self.0.get_last_mod())
        } else {
            None
        }
    }

    /// set the "last_mod" property
    pub fn set_last_mod(&mut self, last_mod: u32) {
        self.0.set_last_mod(last_mod);
    }

    /// get the "last_mod_usecs" property
    pub fn get_last_mod_usecs(&self) -> Option<u32> {
        if self.0.has_last_mod_usecs() {
            Some(self.0.get_last_mod_usecs())
        } else {
            None
        }
    }

    /// set the "last_mod_usecs" property
    pub fn set_last_mod_usecs(&mut self, last_mod_usecs: u32) {
        self.0.set_last_mod_usecs(last_mod_usecs);
    }

    /// get the "deleted" property
    pub fn get_deleted(&self) -> Option<bool> {
        if self.0.has_deleted() {
            Some(self.0.get_deleted())
        } else {
            None
        }
    }

    /// set the "deleted" property
    pub fn set_deleted(&mut self, deleted: bool) {
        self.0.set_deleted(deleted);
    }

    /// get the "indexes" property
    pub fn get_indexes(&self) -> Option<Vec<(Vec<u8>, Vec<u8>)>> {
        if self.0.get_indexes().len() > 0 {
            let mut indexes: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
            for pair in self.0.get_indexes() {
                indexes.push((pair.get_key().to_vec(), pair.get_value().to_vec()));
            }
            Some(indexes)
        } else {
            None
        }
    }

    /// set the "indexes" property
    pub fn set_indexes(&mut self, indexes: Vec<(Vec<u8>, Vec<u8>)>) {
        let mut rpb_indexes: Vec<RpbPair> = Vec::new();
        for (key, value) in indexes.into_iter() {
            let mut newpair = RpbPair::new();
            newpair.set_key(key);
            newpair.set_value(value);
            rpb_indexes.push(newpair);
        }

        self.0.set_indexes(RepeatedField::from_vec(rpb_indexes));
    }
}

impl ObjectContentPrivate for ObjectContent {
    fn into_rpb(self) -> RpbContent {
        self.0
    }

    fn new_from_rpb(rpb_content: RpbContent) -> ObjectContent {
        ObjectContent(rpb_content)
    }
}
