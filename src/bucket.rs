/// Bucket related structs for dealing with Riak buckets.
///
/// For more information: https://docs.basho.com/riak/kv/latest/learn/concepts/buckets/

use errors::RiakErr;
use protobuf::{Message, RepeatedField};
use rpb::riak::{RpbBucketProps, RpbCommitHook, RpbModFun, RpbSetBucketReq, RpbSetBucketTypeReq};
use std::collections::HashMap;
use utils::{BucketPropsPrivate, ProtobufBytes};

/// `BucketProps` represents the properties that can bet set on a bucket
#[derive(Clone, Debug)]
pub struct BucketProps(RpbBucketProps, Vec<u8>);

impl BucketProps {
    /// constructs a new `BucketProps`
    pub fn new<T: Into<Vec<u8>>>(bucket_name: T) -> BucketProps {
        BucketProps(RpbBucketProps::new(), bucket_name.into())
    }

    /// get the "bucket" property
    pub fn get_bucket(&self) -> &[u8] {
        self.1.as_slice()
    }

    /// set the "bucket" property
    pub fn set_bucket<T: Into<Vec<u8>>>(&mut self, bucket: T) {
        self.1 = bucket.into();
    }

    /// get the "allow_mult" property
    pub fn get_allow_mult(&self) -> Option<bool> {
        if self.0.has_allow_mult() {
            Some(self.0.get_allow_mult())
        } else {
            None
        }
    }

    /// set the "allow_mult" property
    pub fn set_allow_mult(&mut self, allow_mult: bool) {
        self.0.set_allow_mult(allow_mult)
    }

    /// get the "backend" property
    pub fn get_backend(&self) -> Option<&[u8]> {
        if self.0.has_backend() {
            Some(self.0.get_backend())
        } else {
            None
        }
    }

    /// set the "backend" property
    pub fn set_backend<T: Into<Vec<u8>>>(&mut self, backend: T) {
        self.0.set_backend(backend.into());
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

    /// get the "big_vclock" property
    pub fn get_big_vclock(&self) -> Option<u32> {
        if self.0.has_big_vclock() {
            Some(self.0.get_big_vclock())
        } else {
            None
        }
    }

    /// set the "big vclock" property
    pub fn set_big_vclock(&mut self, big_vclock: u32) {
        self.0.set_big_vclock(big_vclock);
    }

    /// get the "chash_keyfun" property
    pub fn get_chash_keyfun(&self) -> Option<(&[u8], &[u8])> {
        if self.0.has_chash_keyfun() {
            let modfun = self.0.get_chash_keyfun();
            Some((modfun.get_module(), modfun.get_function()))
        } else {
            None
        }
    }

    /// set the "chash_keyfun" property
    pub fn set_chash_keyfun<T: Into<Vec<u8>>>(&mut self, module: T, function: T) {
        let mut modfun = RpbModFun::new();
        modfun.set_module(module.into());
        modfun.set_function(function.into());
        self.0.set_chash_keyfun(modfun);
    }

    /// get the "consistent" property
    pub fn get_consistent(&self) -> Option<bool> {
        if self.0.has_consistent() {
            Some(self.0.get_consistent())
        } else {
            None
        }
    }

    /// set the "consistent" property
    pub fn set_consistent(&mut self, consistent: bool) {
        self.0.set_consistent(consistent);
    }

    /// get the "datatype" property
    pub fn get_datatype(&self) -> Option<&[u8]> {
        if self.0.has_datatype() {
            Some(self.0.get_datatype())
        } else {
            None
        }
    }

    /// set the "datatype" property
    pub fn set_datatype<T: Into<Vec<u8>>>(&mut self, datatype: T) {
        self.0.set_datatype(datatype.into());
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

    /// get the "has_postcommit" property
    pub fn get_has_postcommit(&self) -> Option<bool> {
        if self.0.has_has_postcommit() {
            Some(self.0.get_has_postcommit())
        } else {
            None
        }
    }

    /// set the "has_postcommit" property
    pub fn set_has_postcommit(&mut self, has_postcommit: bool) {
        self.0.set_has_postcommit(has_postcommit);
    }

    /// get the "has_precommit" property
    pub fn get_has_precommit(&self) -> Option<bool> {
        if self.0.has_has_precommit() {
            Some(self.0.get_has_precommit())
        } else {
            None
        }
    }

    /// set the "has_precommit" property
    pub fn set_has_precommit(&mut self, has_precommit: bool) {
        self.0.set_has_precommit(has_precommit);
    }

    /// get the "hll_precision" property
    pub fn get_hll_precision(&self) -> Option<u32> {
        if self.0.has_hll_precision() {
            Some(self.0.get_hll_precision())
        } else {
            None
        }
    }

    /// set the "hll_precision" property
    pub fn set_hll_precision(&mut self, hll_precision: u32) {
        self.0.set_hll_precision(hll_precision);
    }

    /// get the "last_write_wins" property
    pub fn get_last_write_wins(&self) -> Option<bool> {
        if self.0.has_last_write_wins() {
            Some(self.0.get_last_write_wins())
        } else {
            None
        }
    }

    /// set the "last_write_wins" property
    pub fn set_last_write_wins(&mut self, last_write_wins: bool) {
        self.0.set_last_write_wins(last_write_wins);
    }

    /// get the "linkfun" property
    pub fn get_linkfun(&self) -> Option<(&[u8], &[u8])> {
        if self.0.has_linkfun() {
            let modfun = self.0.get_linkfun();
            Some((modfun.get_module(), modfun.get_function()))
        } else {
            None
        }
    }

    /// set the "linkfun" property
    pub fn set_linkfun<T: Into<Vec<u8>>>(&mut self, module: T, function: T) {
        let mut linkfun = RpbModFun::new();
        linkfun.set_module(module.into());
        linkfun.set_function(function.into());
        self.0.set_linkfun(linkfun);
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

    /// get the "old_vclock" property
    pub fn get_old_vclock(&self) -> Option<u32> {
        if self.0.has_old_vclock() {
            Some(self.0.get_old_vclock())
        } else {
            None
        }
    }

    /// set the "old_vclock" property
    pub fn set_old_vclock(&mut self, old_vclock: u32) {
        self.0.set_old_vclock(old_vclock);
    }

    /// get the "postcommit" property
    pub fn get_postcommit(&self) -> Option<HashMap<Vec<u8>, (Vec<u8>, Vec<u8>)>> {
        let mut postcommit = self.0.get_postcommit().to_vec();
        if postcommit.len() > 0 {
            let mut commithooks: HashMap<Vec<u8>, (Vec<u8>, Vec<u8>)> = HashMap::new();
            for mut commithook in postcommit.iter_mut() {
                let mut modfun = commithook.take_modfun();
                let name = commithook.take_name();
                let module = modfun.take_module();
                let function = modfun.take_function();
                commithooks.insert(name, (module, function));
            }
            Some(commithooks)
        } else {
            None
        }
    }

    /// set the "postcommit" property
    pub fn set_postcommit(&mut self, postcommit: HashMap<Vec<u8>, (Vec<u8>, Vec<u8>)>) {
        let mut commithooks: Vec<RpbCommitHook> = Vec::new();
        for (name, module_and_function) in postcommit.into_iter() {
            let mut modfun = RpbModFun::new();
            modfun.set_module(module_and_function.0);
            modfun.set_function(module_and_function.1);

            let mut commithook = RpbCommitHook::new();
            commithook.set_name(name);
            commithook.set_modfun(modfun);

            commithooks.push(commithook);
        }
        self.0.set_postcommit(RepeatedField::from_vec(commithooks));
    }

    /// get the "precommit" property
    pub fn get_precommit(&self) -> Option<HashMap<Vec<u8>, (Vec<u8>, Vec<u8>)>> {
        let mut precommit = self.0.get_precommit().to_vec();
        if precommit.len() > 0 {
            let mut commithooks: HashMap<Vec<u8>, (Vec<u8>, Vec<u8>)> = HashMap::new();
            for commithook in precommit.iter_mut() {
                let name = commithook.take_name();
                let mut modfun = commithook.take_modfun();
                let module = modfun.take_module();
                let function = modfun.take_function();
                commithooks.insert(name, (module, function));
            }
            Some(commithooks)
        } else {
            None
        }
    }

    /// set the "precommit" property
    pub fn set_precommit(&mut self, precommit: HashMap<Vec<u8>, (Vec<u8>, Vec<u8>)>) {
        let mut commithooks: Vec<RpbCommitHook> = Vec::new();
        for (name, module_and_function) in precommit.into_iter() {
            let mut modfun = RpbModFun::new();
            modfun.set_module(module_and_function.0);
            modfun.set_function(module_and_function.1);

            let mut commithook = RpbCommitHook::new();
            commithook.set_name(name);
            commithook.set_modfun(modfun);

            commithooks.push(commithook);
        }
        self.0.set_precommit(RepeatedField::from_vec(commithooks));
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

    /// get the "rw" property
    pub fn get_rw(&self) -> Option<u32> {
        if self.0.has_rw() {
            Some(self.0.get_rw())
        } else {
            None
        }
    }

    /// set the "rw" property
    pub fn set_rw(&mut self, rw: u32) {
        self.0.set_rw(rw);
    }

    /// get the "search_index" property
    pub fn get_search_index(&self) -> Option<&[u8]> {
        if self.0.has_search_index() {
            Some(self.0.get_search_index())
        } else {
            None
        }
    }

    /// set the "search_index" property
    pub fn set_search_index<T: Into<Vec<u8>>>(&mut self, search_index: T) {
        self.0.set_search_index(search_index.into());
    }

    /// get the "search" property
    pub fn get_search(&self) -> Option<bool> {
        if self.0.has_search() {
            Some(self.0.get_search())
        } else {
            None
        }
    }

    /// set the "search" property
    pub fn set_search(&mut self, search: bool) {
        self.0.set_search(search);
    }

    /// get the "small_vclock" property
    pub fn get_small_vclock(&self) -> Option<u32> {
        if self.0.has_small_vclock() {
            Some(self.0.get_small_vclock())
        } else {
            None
        }
    }

    /// set the "small_vclock" property
    pub fn set_small_vclock(&mut self, small_vclock: u32) {
        self.0.set_small_vclock(small_vclock);
    }

    /// get the "write_once" property
    pub fn get_write_once(&self) -> Option<bool> {
        if self.0.has_write_once() {
            Some(self.0.get_write_once())
        } else {
            None
        }
    }

    /// set the "write_once" property
    pub fn set_write_once(&mut self, write_once: bool) {
        self.0.set_write_once(write_once);
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

    /// get the "young_vclock" property
    pub fn get_young_vclock(&self) -> Option<u32> {
        if self.0.has_young_vclock() {
            Some(self.0.get_young_vclock())
        } else {
            None
        }
    }

    /// set the "young_vclock" property
    pub fn set_young_vclock(&mut self, young_vclock: u32) {
        self.0.set_young_vclock(young_vclock);
    }
}

impl ProtobufBytes for BucketProps {
    fn write_to_bytes(self) -> Result<Vec<u8>, RiakErr> {
        let mut req = RpbSetBucketReq::new();
        req.set_bucket(self.1);
        req.set_props(self.0);
        match req.write_to_bytes() {
            Ok(bytes) => Ok(bytes),
            Err(error) => Err(RiakErr::ProtobufError(error)),
        }
    }
}

impl BucketPropsPrivate for BucketProps {
    fn set_props(&mut self, props: RpbBucketProps) {
        self.0 = props;
    }
}

/// `BucketTypeProps` represents the properties that can bet set on a bucket type
#[derive(Clone, Debug)]
pub struct BucketTypeProps(RpbBucketProps, Vec<u8>);

impl BucketTypeProps {
    /// constructs a new `BucketTypeProps`
    pub fn new<T: Into<Vec<u8>>>(bucket_type_name: T) -> BucketTypeProps {
        BucketTypeProps(RpbBucketProps::new(), bucket_type_name.into())
    }

    /// get the "bucket" property
    pub fn get_bucket(&self) -> &[u8] {
        self.1.as_slice()
    }

    /// set the "bucket" property
    pub fn set_bucket<T: Into<Vec<u8>>>(&mut self, bucket: T) {
        self.1 = bucket.into();
    }

    /// get the "allow_mult" property
    pub fn get_allow_mult(&self) -> Option<bool> {
        if self.0.has_allow_mult() {
            Some(self.0.get_allow_mult())
        } else {
            None
        }
    }

    /// set the "allow_mult" property
    pub fn set_allow_mult(&mut self, allow_mult: bool) {
        self.0.set_allow_mult(allow_mult)
    }

    /// get the "backend" property
    pub fn get_backend(&self) -> Option<&[u8]> {
        if self.0.has_backend() {
            Some(self.0.get_backend())
        } else {
            None
        }
    }

    /// set the "backend" property
    pub fn set_backend<T: Into<Vec<u8>>>(&mut self, backend: T) {
        self.0.set_backend(backend.into());
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

    /// get the "big_vclock" property
    pub fn get_big_vclock(&self) -> Option<u32> {
        if self.0.has_big_vclock() {
            Some(self.0.get_big_vclock())
        } else {
            None
        }
    }

    /// set the "big vclock" property
    pub fn set_big_vclock(&mut self, big_vclock: u32) {
        self.0.set_big_vclock(big_vclock);
    }

    /// get the "chash_keyfun" property
    pub fn get_chash_keyfun(&self) -> Option<(&[u8], &[u8])> {
        if self.0.has_chash_keyfun() {
            let modfun = self.0.get_chash_keyfun();
            Some((modfun.get_module(), modfun.get_function()))
        } else {
            None
        }
    }

    /// set the "chash_keyfun" property
    pub fn set_chash_keyfun<T: Into<Vec<u8>>>(&mut self, module: T, function: T) {
        let mut modfun = RpbModFun::new();
        modfun.set_module(module.into());
        modfun.set_function(function.into());
        self.0.set_chash_keyfun(modfun);
    }

    /// get the "consistent" property
    pub fn get_consistent(&self) -> Option<bool> {
        if self.0.has_consistent() {
            Some(self.0.get_consistent())
        } else {
            None
        }
    }

    /// set the "consistent" property
    pub fn set_consistent(&mut self, consistent: bool) {
        self.0.set_consistent(consistent);
    }

    /// get the "datatype" property
    pub fn get_datatype(&self) -> Option<&[u8]> {
        if self.0.has_datatype() {
            Some(self.0.get_datatype())
        } else {
            None
        }
    }

    /// set the "datatype" property
    pub fn set_datatype<T: Into<Vec<u8>>>(&mut self, datatype: T) {
        self.0.set_datatype(datatype.into());
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

    /// get the "has_postcommit" property
    pub fn get_has_postcommit(&self) -> Option<bool> {
        if self.0.has_has_postcommit() {
            Some(self.0.get_has_postcommit())
        } else {
            None
        }
    }

    /// set the "has_postcommit" property
    pub fn set_has_postcommit(&mut self, has_postcommit: bool) {
        self.0.set_has_postcommit(has_postcommit);
    }

    /// get the "has_precommit" property
    pub fn get_has_precommit(&self) -> Option<bool> {
        if self.0.has_has_precommit() {
            Some(self.0.get_has_precommit())
        } else {
            None
        }
    }

    /// set the "has_precommit" property
    pub fn set_has_precommit(&mut self, has_precommit: bool) {
        self.0.set_has_precommit(has_precommit);
    }

    /// get the "hll_precision" property
    pub fn get_hll_precision(&self) -> Option<u32> {
        if self.0.has_hll_precision() {
            Some(self.0.get_hll_precision())
        } else {
            None
        }
    }

    /// set the "hll_precision" property
    pub fn set_hll_precision(&mut self, hll_precision: u32) {
        self.0.set_hll_precision(hll_precision);
    }

    /// get the "last_write_wins" property
    pub fn get_last_write_wins(&self) -> Option<bool> {
        if self.0.has_last_write_wins() {
            Some(self.0.get_last_write_wins())
        } else {
            None
        }
    }

    /// set the "last_write_wins" property
    pub fn set_last_write_wins(&mut self, last_write_wins: bool) {
        self.0.set_last_write_wins(last_write_wins);
    }

    /// get the "linkfun" property
    pub fn get_linkfun(&self) -> Option<(&[u8], &[u8])> {
        if self.0.has_linkfun() {
            let modfun = self.0.get_linkfun();
            Some((modfun.get_module(), modfun.get_function()))
        } else {
            None
        }
    }

    /// set the "linkfun" property
    pub fn set_linkfun<T: Into<Vec<u8>>>(&mut self, module: T, function: T) {
        let mut linkfun = RpbModFun::new();
        linkfun.set_module(module.into());
        linkfun.set_function(function.into());
        self.0.set_linkfun(linkfun);
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

    /// get the "old_vclock" property
    pub fn get_old_vclock(&self) -> Option<u32> {
        if self.0.has_old_vclock() {
            Some(self.0.get_old_vclock())
        } else {
            None
        }
    }

    /// set the "old_vclock" property
    pub fn set_old_vclock(&mut self, old_vclock: u32) {
        self.0.set_old_vclock(old_vclock);
    }

    /// get the "postcommit" property
    pub fn get_postcommit(&self) -> Option<HashMap<Vec<u8>, (Vec<u8>, Vec<u8>)>> {
        let mut postcommit = self.0.get_postcommit().to_vec();
        if postcommit.len() > 0 {
            let mut commithooks: HashMap<Vec<u8>, (Vec<u8>, Vec<u8>)> = HashMap::new();
            for mut commithook in postcommit.iter_mut() {
                let mut modfun = commithook.take_modfun();
                let name = commithook.take_name();
                let module = modfun.take_module();
                let function = modfun.take_function();
                commithooks.insert(name, (module, function));
            }
            Some(commithooks)
        } else {
            None
        }
    }

    /// set the "postcommit" property
    pub fn set_postcommit(&mut self, postcommit: HashMap<Vec<u8>, (Vec<u8>, Vec<u8>)>) {
        let mut commithooks: Vec<RpbCommitHook> = Vec::new();
        for (name, module_and_function) in postcommit.into_iter() {
            let mut modfun = RpbModFun::new();
            modfun.set_module(module_and_function.0);
            modfun.set_function(module_and_function.1);

            let mut commithook = RpbCommitHook::new();
            commithook.set_name(name);
            commithook.set_modfun(modfun);

            commithooks.push(commithook);
        }
        self.0.set_postcommit(RepeatedField::from_vec(commithooks));
    }

    /// get the "precommit" property
    pub fn get_precommit(&self) -> Option<HashMap<Vec<u8>, (Vec<u8>, Vec<u8>)>> {
        let mut precommit = self.0.get_precommit().to_vec();
        if precommit.len() > 0 {
            let mut commithooks: HashMap<Vec<u8>, (Vec<u8>, Vec<u8>)> = HashMap::new();
            for commithook in precommit.iter_mut() {
                let name = commithook.take_name();
                let mut modfun = commithook.take_modfun();
                let module = modfun.take_module();
                let function = modfun.take_function();
                commithooks.insert(name, (module, function));
            }
            Some(commithooks)
        } else {
            None
        }
    }

    /// set the "precommit" property
    pub fn set_precommit(&mut self, precommit: HashMap<Vec<u8>, (Vec<u8>, Vec<u8>)>) {
        let mut commithooks: Vec<RpbCommitHook> = Vec::new();
        for (name, module_and_function) in precommit.into_iter() {
            let mut modfun = RpbModFun::new();
            modfun.set_module(module_and_function.0);
            modfun.set_function(module_and_function.1);

            let mut commithook = RpbCommitHook::new();
            commithook.set_name(name);
            commithook.set_modfun(modfun);

            commithooks.push(commithook);
        }
        self.0.set_precommit(RepeatedField::from_vec(commithooks));
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

    /// get the "rw" property
    pub fn get_rw(&self) -> Option<u32> {
        if self.0.has_rw() {
            Some(self.0.get_rw())
        } else {
            None
        }
    }

    /// set the "rw" property
    pub fn set_rw(&mut self, rw: u32) {
        self.0.set_rw(rw);
    }

    /// get the "search_index" property
    pub fn get_search_index(&self) -> Option<&[u8]> {
        if self.0.has_search_index() {
            Some(self.0.get_search_index())
        } else {
            None
        }
    }

    /// set the "search_index" property
    pub fn set_search_index<T: Into<Vec<u8>>>(&mut self, search_index: T) {
        self.0.set_search_index(search_index.into());
    }

    /// get the "search" property
    pub fn get_search(&self) -> Option<bool> {
        if self.0.has_search() {
            Some(self.0.get_search())
        } else {
            None
        }
    }

    /// set the "search" property
    pub fn set_search(&mut self, search: bool) {
        self.0.set_search(search);
    }

    /// get the "small_vclock" property
    pub fn get_small_vclock(&self) -> Option<u32> {
        if self.0.has_small_vclock() {
            Some(self.0.get_small_vclock())
        } else {
            None
        }
    }

    /// set the "small_vclock" property
    pub fn set_small_vclock(&mut self, small_vclock: u32) {
        self.0.set_small_vclock(small_vclock);
    }

    /// get the "write_once" property
    pub fn get_write_once(&self) -> Option<bool> {
        if self.0.has_write_once() {
            Some(self.0.get_write_once())
        } else {
            None
        }
    }

    /// set the "write_once" property
    pub fn set_write_once(&mut self, write_once: bool) {
        self.0.set_write_once(write_once);
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

    /// get the "young_vclock" property
    pub fn get_young_vclock(&self) -> Option<u32> {
        if self.0.has_young_vclock() {
            Some(self.0.get_young_vclock())
        } else {
            None
        }
    }

    /// set the "young_vclock" property
    pub fn set_young_vclock(&mut self, young_vclock: u32) {
        self.0.set_young_vclock(young_vclock);
    }
}

impl ProtobufBytes for BucketTypeProps {
    fn write_to_bytes(self) -> Result<Vec<u8>, RiakErr> {
        let mut req = RpbSetBucketTypeReq::new();
        req.set_field_type(self.1);
        req.set_props(self.0);
        match req.write_to_bytes() {
            Ok(bytes) => Ok(bytes),
            Err(error) => Err(RiakErr::ProtobufError(error)),
        }
    }
}

impl BucketPropsPrivate for BucketTypeProps {
    fn set_props(&mut self, props: RpbBucketProps) {
        self.0 = props;
    }
}

#[cfg(test)]
mod tests {
    use super::{BucketProps, BucketTypeProps};

    #[test]
    fn bucket_props() {
        // get_bucket() & set_bucket()
        let mut props = BucketProps::new("test_bucket");
        assert_eq!(b"test_bucket", props.get_bucket());
        props.set_bucket("test_bucket_two");
        assert_eq!(b"test_bucket_two", props.get_bucket());
    }

    #[test]
    fn bucket_type_props() {
        let mut props = BucketTypeProps::new("test_bucket_type");
        assert_eq!(b"test_bucket_type", props.get_bucket());
        props.set_bucket("test_bucket_type_two");
        assert_eq!(b"test_bucket_type_two", props.get_bucket());
    }
}
