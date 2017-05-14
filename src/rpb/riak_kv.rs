// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct RpbGetClientIdResp {
    // message fields
    client_id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbGetClientIdResp {}

impl RpbGetClientIdResp {
    pub fn new() -> RpbGetClientIdResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbGetClientIdResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbGetClientIdResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbGetClientIdResp,
        };
        unsafe {
            instance.get(RpbGetClientIdResp::new)
        }
    }

    // required bytes client_id = 1;

    pub fn clear_client_id(&mut self) {
        self.client_id.clear();
    }

    pub fn has_client_id(&self) -> bool {
        self.client_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.client_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_client_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.client_id.is_none() {
            self.client_id.set_default();
        };
        self.client_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_client_id(&mut self) -> ::std::vec::Vec<u8> {
        self.client_id.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_client_id(&self) -> &[u8] {
        match self.client_id.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_client_id_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.client_id
    }

    fn mut_client_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.client_id
    }
}

impl ::protobuf::Message for RpbGetClientIdResp {
    fn is_initialized(&self) -> bool {
        if self.client_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.client_id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.client_id.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.client_id.as_ref() {
            os.write_bytes(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbGetClientIdResp {
    fn new() -> RpbGetClientIdResp {
        RpbGetClientIdResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbGetClientIdResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "client_id",
                    RpbGetClientIdResp::get_client_id_for_reflect,
                    RpbGetClientIdResp::mut_client_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbGetClientIdResp>(
                    "RpbGetClientIdResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbGetClientIdResp {
    fn clear(&mut self) {
        self.clear_client_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbGetClientIdResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbGetClientIdResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbSetClientIdReq {
    // message fields
    client_id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbSetClientIdReq {}

impl RpbSetClientIdReq {
    pub fn new() -> RpbSetClientIdReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbSetClientIdReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbSetClientIdReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbSetClientIdReq,
        };
        unsafe {
            instance.get(RpbSetClientIdReq::new)
        }
    }

    // required bytes client_id = 1;

    pub fn clear_client_id(&mut self) {
        self.client_id.clear();
    }

    pub fn has_client_id(&self) -> bool {
        self.client_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.client_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_client_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.client_id.is_none() {
            self.client_id.set_default();
        };
        self.client_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_client_id(&mut self) -> ::std::vec::Vec<u8> {
        self.client_id.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_client_id(&self) -> &[u8] {
        match self.client_id.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_client_id_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.client_id
    }

    fn mut_client_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.client_id
    }
}

impl ::protobuf::Message for RpbSetClientIdReq {
    fn is_initialized(&self) -> bool {
        if self.client_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.client_id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.client_id.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.client_id.as_ref() {
            os.write_bytes(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbSetClientIdReq {
    fn new() -> RpbSetClientIdReq {
        RpbSetClientIdReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbSetClientIdReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "client_id",
                    RpbSetClientIdReq::get_client_id_for_reflect,
                    RpbSetClientIdReq::mut_client_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbSetClientIdReq>(
                    "RpbSetClientIdReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbSetClientIdReq {
    fn clear(&mut self) {
        self.clear_client_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbSetClientIdReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbSetClientIdReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbGetReq {
    // message fields
    bucket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    r: ::std::option::Option<u32>,
    pr: ::std::option::Option<u32>,
    basic_quorum: ::std::option::Option<bool>,
    notfound_ok: ::std::option::Option<bool>,
    if_modified: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    head: ::std::option::Option<bool>,
    deletedvclock: ::std::option::Option<bool>,
    timeout: ::std::option::Option<u32>,
    sloppy_quorum: ::std::option::Option<bool>,
    n_val: ::std::option::Option<u32>,
    field_type: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbGetReq {}

impl RpbGetReq {
    pub fn new() -> RpbGetReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbGetReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbGetReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbGetReq,
        };
        unsafe {
            instance.get(RpbGetReq::new)
        }
    }

    // required bytes bucket = 1;

    pub fn clear_bucket(&mut self) {
        self.bucket.clear();
    }

    pub fn has_bucket(&self) -> bool {
        self.bucket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bucket(&mut self, v: ::std::vec::Vec<u8>) {
        self.bucket = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bucket(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.bucket.is_none() {
            self.bucket.set_default();
        };
        self.bucket.as_mut().unwrap()
    }

    // Take field
    pub fn take_bucket(&mut self) -> ::std::vec::Vec<u8> {
        self.bucket.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_bucket(&self) -> &[u8] {
        match self.bucket.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_bucket_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.bucket
    }

    fn mut_bucket_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.bucket
    }

    // required bytes key = 2;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.key
    }

    // optional uint32 r = 3;

    pub fn clear_r(&mut self) {
        self.r = ::std::option::Option::None;
    }

    pub fn has_r(&self) -> bool {
        self.r.is_some()
    }

    // Param is passed by value, moved
    pub fn set_r(&mut self, v: u32) {
        self.r = ::std::option::Option::Some(v);
    }

    pub fn get_r(&self) -> u32 {
        self.r.unwrap_or(0)
    }

    fn get_r_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.r
    }

    fn mut_r_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.r
    }

    // optional uint32 pr = 4;

    pub fn clear_pr(&mut self) {
        self.pr = ::std::option::Option::None;
    }

    pub fn has_pr(&self) -> bool {
        self.pr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pr(&mut self, v: u32) {
        self.pr = ::std::option::Option::Some(v);
    }

    pub fn get_pr(&self) -> u32 {
        self.pr.unwrap_or(0)
    }

    fn get_pr_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.pr
    }

    fn mut_pr_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.pr
    }

    // optional bool basic_quorum = 5;

    pub fn clear_basic_quorum(&mut self) {
        self.basic_quorum = ::std::option::Option::None;
    }

    pub fn has_basic_quorum(&self) -> bool {
        self.basic_quorum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_basic_quorum(&mut self, v: bool) {
        self.basic_quorum = ::std::option::Option::Some(v);
    }

    pub fn get_basic_quorum(&self) -> bool {
        self.basic_quorum.unwrap_or(false)
    }

    fn get_basic_quorum_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.basic_quorum
    }

    fn mut_basic_quorum_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.basic_quorum
    }

    // optional bool notfound_ok = 6;

    pub fn clear_notfound_ok(&mut self) {
        self.notfound_ok = ::std::option::Option::None;
    }

    pub fn has_notfound_ok(&self) -> bool {
        self.notfound_ok.is_some()
    }

    // Param is passed by value, moved
    pub fn set_notfound_ok(&mut self, v: bool) {
        self.notfound_ok = ::std::option::Option::Some(v);
    }

    pub fn get_notfound_ok(&self) -> bool {
        self.notfound_ok.unwrap_or(false)
    }

    fn get_notfound_ok_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.notfound_ok
    }

    fn mut_notfound_ok_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.notfound_ok
    }

    // optional bytes if_modified = 7;

    pub fn clear_if_modified(&mut self) {
        self.if_modified.clear();
    }

    pub fn has_if_modified(&self) -> bool {
        self.if_modified.is_some()
    }

    // Param is passed by value, moved
    pub fn set_if_modified(&mut self, v: ::std::vec::Vec<u8>) {
        self.if_modified = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_if_modified(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.if_modified.is_none() {
            self.if_modified.set_default();
        };
        self.if_modified.as_mut().unwrap()
    }

    // Take field
    pub fn take_if_modified(&mut self) -> ::std::vec::Vec<u8> {
        self.if_modified.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_if_modified(&self) -> &[u8] {
        match self.if_modified.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_if_modified_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.if_modified
    }

    fn mut_if_modified_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.if_modified
    }

    // optional bool head = 8;

    pub fn clear_head(&mut self) {
        self.head = ::std::option::Option::None;
    }

    pub fn has_head(&self) -> bool {
        self.head.is_some()
    }

    // Param is passed by value, moved
    pub fn set_head(&mut self, v: bool) {
        self.head = ::std::option::Option::Some(v);
    }

    pub fn get_head(&self) -> bool {
        self.head.unwrap_or(false)
    }

    fn get_head_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.head
    }

    fn mut_head_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.head
    }

    // optional bool deletedvclock = 9;

    pub fn clear_deletedvclock(&mut self) {
        self.deletedvclock = ::std::option::Option::None;
    }

    pub fn has_deletedvclock(&self) -> bool {
        self.deletedvclock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deletedvclock(&mut self, v: bool) {
        self.deletedvclock = ::std::option::Option::Some(v);
    }

    pub fn get_deletedvclock(&self) -> bool {
        self.deletedvclock.unwrap_or(false)
    }

    fn get_deletedvclock_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.deletedvclock
    }

    fn mut_deletedvclock_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.deletedvclock
    }

    // optional uint32 timeout = 10;

    pub fn clear_timeout(&mut self) {
        self.timeout = ::std::option::Option::None;
    }

    pub fn has_timeout(&self) -> bool {
        self.timeout.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timeout(&mut self, v: u32) {
        self.timeout = ::std::option::Option::Some(v);
    }

    pub fn get_timeout(&self) -> u32 {
        self.timeout.unwrap_or(0)
    }

    fn get_timeout_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.timeout
    }

    fn mut_timeout_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.timeout
    }

    // optional bool sloppy_quorum = 11;

    pub fn clear_sloppy_quorum(&mut self) {
        self.sloppy_quorum = ::std::option::Option::None;
    }

    pub fn has_sloppy_quorum(&self) -> bool {
        self.sloppy_quorum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sloppy_quorum(&mut self, v: bool) {
        self.sloppy_quorum = ::std::option::Option::Some(v);
    }

    pub fn get_sloppy_quorum(&self) -> bool {
        self.sloppy_quorum.unwrap_or(false)
    }

    fn get_sloppy_quorum_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.sloppy_quorum
    }

    fn mut_sloppy_quorum_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.sloppy_quorum
    }

    // optional uint32 n_val = 12;

    pub fn clear_n_val(&mut self) {
        self.n_val = ::std::option::Option::None;
    }

    pub fn has_n_val(&self) -> bool {
        self.n_val.is_some()
    }

    // Param is passed by value, moved
    pub fn set_n_val(&mut self, v: u32) {
        self.n_val = ::std::option::Option::Some(v);
    }

    pub fn get_n_val(&self) -> u32 {
        self.n_val.unwrap_or(0)
    }

    fn get_n_val_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.n_val
    }

    fn mut_n_val_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.n_val
    }

    // optional bytes type = 13;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::vec::Vec<u8>) {
        self.field_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.field_type.is_none() {
            self.field_type.set_default();
        };
        self.field_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::vec::Vec<u8> {
        self.field_type.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_field_type(&self) -> &[u8] {
        match self.field_type.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_field_type_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.field_type
    }
}

impl ::protobuf::Message for RpbGetReq {
    fn is_initialized(&self) -> bool {
        if self.bucket.is_none() {
            return false;
        };
        if self.key.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bucket)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.r = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.pr = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.basic_quorum = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.notfound_ok = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.if_modified)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.head = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.deletedvclock = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.timeout = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.sloppy_quorum = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.n_val = ::std::option::Option::Some(tmp);
                },
                13 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.field_type)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.bucket.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.r {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.pr {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.basic_quorum {
            my_size += 2;
        };
        if let Some(v) = self.notfound_ok {
            my_size += 2;
        };
        if let Some(v) = self.if_modified.as_ref() {
            my_size += ::protobuf::rt::bytes_size(7, &v);
        };
        if let Some(v) = self.head {
            my_size += 2;
        };
        if let Some(v) = self.deletedvclock {
            my_size += 2;
        };
        if let Some(v) = self.timeout {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.sloppy_quorum {
            my_size += 2;
        };
        if let Some(v) = self.n_val {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.field_type.as_ref() {
            my_size += ::protobuf::rt::bytes_size(13, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bucket.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.key.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.r {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.pr {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.basic_quorum {
            os.write_bool(5, v)?;
        };
        if let Some(v) = self.notfound_ok {
            os.write_bool(6, v)?;
        };
        if let Some(v) = self.if_modified.as_ref() {
            os.write_bytes(7, &v)?;
        };
        if let Some(v) = self.head {
            os.write_bool(8, v)?;
        };
        if let Some(v) = self.deletedvclock {
            os.write_bool(9, v)?;
        };
        if let Some(v) = self.timeout {
            os.write_uint32(10, v)?;
        };
        if let Some(v) = self.sloppy_quorum {
            os.write_bool(11, v)?;
        };
        if let Some(v) = self.n_val {
            os.write_uint32(12, v)?;
        };
        if let Some(v) = self.field_type.as_ref() {
            os.write_bytes(13, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbGetReq {
    fn new() -> RpbGetReq {
        RpbGetReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbGetReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "bucket",
                    RpbGetReq::get_bucket_for_reflect,
                    RpbGetReq::mut_bucket_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    RpbGetReq::get_key_for_reflect,
                    RpbGetReq::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "r",
                    RpbGetReq::get_r_for_reflect,
                    RpbGetReq::mut_r_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "pr",
                    RpbGetReq::get_pr_for_reflect,
                    RpbGetReq::mut_pr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "basic_quorum",
                    RpbGetReq::get_basic_quorum_for_reflect,
                    RpbGetReq::mut_basic_quorum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "notfound_ok",
                    RpbGetReq::get_notfound_ok_for_reflect,
                    RpbGetReq::mut_notfound_ok_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "if_modified",
                    RpbGetReq::get_if_modified_for_reflect,
                    RpbGetReq::mut_if_modified_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "head",
                    RpbGetReq::get_head_for_reflect,
                    RpbGetReq::mut_head_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "deletedvclock",
                    RpbGetReq::get_deletedvclock_for_reflect,
                    RpbGetReq::mut_deletedvclock_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "timeout",
                    RpbGetReq::get_timeout_for_reflect,
                    RpbGetReq::mut_timeout_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "sloppy_quorum",
                    RpbGetReq::get_sloppy_quorum_for_reflect,
                    RpbGetReq::mut_sloppy_quorum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "n_val",
                    RpbGetReq::get_n_val_for_reflect,
                    RpbGetReq::mut_n_val_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "type",
                    RpbGetReq::get_field_type_for_reflect,
                    RpbGetReq::mut_field_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbGetReq>(
                    "RpbGetReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbGetReq {
    fn clear(&mut self) {
        self.clear_bucket();
        self.clear_key();
        self.clear_r();
        self.clear_pr();
        self.clear_basic_quorum();
        self.clear_notfound_ok();
        self.clear_if_modified();
        self.clear_head();
        self.clear_deletedvclock();
        self.clear_timeout();
        self.clear_sloppy_quorum();
        self.clear_n_val();
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbGetReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbGetReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbGetResp {
    // message fields
    content: ::protobuf::RepeatedField<RpbContent>,
    vclock: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    unchanged: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbGetResp {}

impl RpbGetResp {
    pub fn new() -> RpbGetResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbGetResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbGetResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbGetResp,
        };
        unsafe {
            instance.get(RpbGetResp::new)
        }
    }

    // repeated .RpbContent content = 1;

    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ::protobuf::RepeatedField<RpbContent>) {
        self.content = v;
    }

    // Mutable pointer to the field.
    pub fn mut_content(&mut self) -> &mut ::protobuf::RepeatedField<RpbContent> {
        &mut self.content
    }

    // Take field
    pub fn take_content(&mut self) -> ::protobuf::RepeatedField<RpbContent> {
        ::std::mem::replace(&mut self.content, ::protobuf::RepeatedField::new())
    }

    pub fn get_content(&self) -> &[RpbContent] {
        &self.content
    }

    fn get_content_for_reflect(&self) -> &::protobuf::RepeatedField<RpbContent> {
        &self.content
    }

    fn mut_content_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RpbContent> {
        &mut self.content
    }

    // optional bytes vclock = 2;

    pub fn clear_vclock(&mut self) {
        self.vclock.clear();
    }

    pub fn has_vclock(&self) -> bool {
        self.vclock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vclock(&mut self, v: ::std::vec::Vec<u8>) {
        self.vclock = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_vclock(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.vclock.is_none() {
            self.vclock.set_default();
        };
        self.vclock.as_mut().unwrap()
    }

    // Take field
    pub fn take_vclock(&mut self) -> ::std::vec::Vec<u8> {
        self.vclock.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_vclock(&self) -> &[u8] {
        match self.vclock.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_vclock_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.vclock
    }

    fn mut_vclock_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.vclock
    }

    // optional bool unchanged = 3;

    pub fn clear_unchanged(&mut self) {
        self.unchanged = ::std::option::Option::None;
    }

    pub fn has_unchanged(&self) -> bool {
        self.unchanged.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unchanged(&mut self, v: bool) {
        self.unchanged = ::std::option::Option::Some(v);
    }

    pub fn get_unchanged(&self) -> bool {
        self.unchanged.unwrap_or(false)
    }

    fn get_unchanged_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.unchanged
    }

    fn mut_unchanged_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.unchanged
    }
}

impl ::protobuf::Message for RpbGetResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.content)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.vclock)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.unchanged = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.content {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.vclock.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.unchanged {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.content {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.vclock.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.unchanged {
            os.write_bool(3, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbGetResp {
    fn new() -> RpbGetResp {
        RpbGetResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbGetResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RpbContent>>(
                    "content",
                    RpbGetResp::get_content_for_reflect,
                    RpbGetResp::mut_content_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "vclock",
                    RpbGetResp::get_vclock_for_reflect,
                    RpbGetResp::mut_vclock_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "unchanged",
                    RpbGetResp::get_unchanged_for_reflect,
                    RpbGetResp::mut_unchanged_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbGetResp>(
                    "RpbGetResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbGetResp {
    fn clear(&mut self) {
        self.clear_content();
        self.clear_vclock();
        self.clear_unchanged();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbGetResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbGetResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbPutReq {
    // message fields
    bucket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    vclock: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    content: ::protobuf::SingularPtrField<RpbContent>,
    w: ::std::option::Option<u32>,
    dw: ::std::option::Option<u32>,
    return_body: ::std::option::Option<bool>,
    pw: ::std::option::Option<u32>,
    if_not_modified: ::std::option::Option<bool>,
    if_none_match: ::std::option::Option<bool>,
    return_head: ::std::option::Option<bool>,
    timeout: ::std::option::Option<u32>,
    asis: ::std::option::Option<bool>,
    sloppy_quorum: ::std::option::Option<bool>,
    n_val: ::std::option::Option<u32>,
    field_type: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbPutReq {}

impl RpbPutReq {
    pub fn new() -> RpbPutReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbPutReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbPutReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbPutReq,
        };
        unsafe {
            instance.get(RpbPutReq::new)
        }
    }

    // required bytes bucket = 1;

    pub fn clear_bucket(&mut self) {
        self.bucket.clear();
    }

    pub fn has_bucket(&self) -> bool {
        self.bucket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bucket(&mut self, v: ::std::vec::Vec<u8>) {
        self.bucket = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bucket(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.bucket.is_none() {
            self.bucket.set_default();
        };
        self.bucket.as_mut().unwrap()
    }

    // Take field
    pub fn take_bucket(&mut self) -> ::std::vec::Vec<u8> {
        self.bucket.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_bucket(&self) -> &[u8] {
        match self.bucket.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_bucket_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.bucket
    }

    fn mut_bucket_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.bucket
    }

    // optional bytes key = 2;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.key
    }

    // optional bytes vclock = 3;

    pub fn clear_vclock(&mut self) {
        self.vclock.clear();
    }

    pub fn has_vclock(&self) -> bool {
        self.vclock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vclock(&mut self, v: ::std::vec::Vec<u8>) {
        self.vclock = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_vclock(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.vclock.is_none() {
            self.vclock.set_default();
        };
        self.vclock.as_mut().unwrap()
    }

    // Take field
    pub fn take_vclock(&mut self) -> ::std::vec::Vec<u8> {
        self.vclock.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_vclock(&self) -> &[u8] {
        match self.vclock.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_vclock_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.vclock
    }

    fn mut_vclock_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.vclock
    }

    // required .RpbContent content = 4;

    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    pub fn has_content(&self) -> bool {
        self.content.is_some()
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: RpbContent) {
        self.content = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content(&mut self) -> &mut RpbContent {
        if self.content.is_none() {
            self.content.set_default();
        };
        self.content.as_mut().unwrap()
    }

    // Take field
    pub fn take_content(&mut self) -> RpbContent {
        self.content.take().unwrap_or_else(|| RpbContent::new())
    }

    pub fn get_content(&self) -> &RpbContent {
        self.content.as_ref().unwrap_or_else(|| RpbContent::default_instance())
    }

    fn get_content_for_reflect(&self) -> &::protobuf::SingularPtrField<RpbContent> {
        &self.content
    }

    fn mut_content_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RpbContent> {
        &mut self.content
    }

    // optional uint32 w = 5;

    pub fn clear_w(&mut self) {
        self.w = ::std::option::Option::None;
    }

    pub fn has_w(&self) -> bool {
        self.w.is_some()
    }

    // Param is passed by value, moved
    pub fn set_w(&mut self, v: u32) {
        self.w = ::std::option::Option::Some(v);
    }

    pub fn get_w(&self) -> u32 {
        self.w.unwrap_or(0)
    }

    fn get_w_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.w
    }

    fn mut_w_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.w
    }

    // optional uint32 dw = 6;

    pub fn clear_dw(&mut self) {
        self.dw = ::std::option::Option::None;
    }

    pub fn has_dw(&self) -> bool {
        self.dw.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dw(&mut self, v: u32) {
        self.dw = ::std::option::Option::Some(v);
    }

    pub fn get_dw(&self) -> u32 {
        self.dw.unwrap_or(0)
    }

    fn get_dw_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.dw
    }

    fn mut_dw_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.dw
    }

    // optional bool return_body = 7;

    pub fn clear_return_body(&mut self) {
        self.return_body = ::std::option::Option::None;
    }

    pub fn has_return_body(&self) -> bool {
        self.return_body.is_some()
    }

    // Param is passed by value, moved
    pub fn set_return_body(&mut self, v: bool) {
        self.return_body = ::std::option::Option::Some(v);
    }

    pub fn get_return_body(&self) -> bool {
        self.return_body.unwrap_or(false)
    }

    fn get_return_body_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.return_body
    }

    fn mut_return_body_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.return_body
    }

    // optional uint32 pw = 8;

    pub fn clear_pw(&mut self) {
        self.pw = ::std::option::Option::None;
    }

    pub fn has_pw(&self) -> bool {
        self.pw.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pw(&mut self, v: u32) {
        self.pw = ::std::option::Option::Some(v);
    }

    pub fn get_pw(&self) -> u32 {
        self.pw.unwrap_or(0)
    }

    fn get_pw_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.pw
    }

    fn mut_pw_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.pw
    }

    // optional bool if_not_modified = 9;

    pub fn clear_if_not_modified(&mut self) {
        self.if_not_modified = ::std::option::Option::None;
    }

    pub fn has_if_not_modified(&self) -> bool {
        self.if_not_modified.is_some()
    }

    // Param is passed by value, moved
    pub fn set_if_not_modified(&mut self, v: bool) {
        self.if_not_modified = ::std::option::Option::Some(v);
    }

    pub fn get_if_not_modified(&self) -> bool {
        self.if_not_modified.unwrap_or(false)
    }

    fn get_if_not_modified_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.if_not_modified
    }

    fn mut_if_not_modified_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.if_not_modified
    }

    // optional bool if_none_match = 10;

    pub fn clear_if_none_match(&mut self) {
        self.if_none_match = ::std::option::Option::None;
    }

    pub fn has_if_none_match(&self) -> bool {
        self.if_none_match.is_some()
    }

    // Param is passed by value, moved
    pub fn set_if_none_match(&mut self, v: bool) {
        self.if_none_match = ::std::option::Option::Some(v);
    }

    pub fn get_if_none_match(&self) -> bool {
        self.if_none_match.unwrap_or(false)
    }

    fn get_if_none_match_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.if_none_match
    }

    fn mut_if_none_match_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.if_none_match
    }

    // optional bool return_head = 11;

    pub fn clear_return_head(&mut self) {
        self.return_head = ::std::option::Option::None;
    }

    pub fn has_return_head(&self) -> bool {
        self.return_head.is_some()
    }

    // Param is passed by value, moved
    pub fn set_return_head(&mut self, v: bool) {
        self.return_head = ::std::option::Option::Some(v);
    }

    pub fn get_return_head(&self) -> bool {
        self.return_head.unwrap_or(false)
    }

    fn get_return_head_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.return_head
    }

    fn mut_return_head_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.return_head
    }

    // optional uint32 timeout = 12;

    pub fn clear_timeout(&mut self) {
        self.timeout = ::std::option::Option::None;
    }

    pub fn has_timeout(&self) -> bool {
        self.timeout.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timeout(&mut self, v: u32) {
        self.timeout = ::std::option::Option::Some(v);
    }

    pub fn get_timeout(&self) -> u32 {
        self.timeout.unwrap_or(0)
    }

    fn get_timeout_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.timeout
    }

    fn mut_timeout_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.timeout
    }

    // optional bool asis = 13;

    pub fn clear_asis(&mut self) {
        self.asis = ::std::option::Option::None;
    }

    pub fn has_asis(&self) -> bool {
        self.asis.is_some()
    }

    // Param is passed by value, moved
    pub fn set_asis(&mut self, v: bool) {
        self.asis = ::std::option::Option::Some(v);
    }

    pub fn get_asis(&self) -> bool {
        self.asis.unwrap_or(false)
    }

    fn get_asis_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.asis
    }

    fn mut_asis_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.asis
    }

    // optional bool sloppy_quorum = 14;

    pub fn clear_sloppy_quorum(&mut self) {
        self.sloppy_quorum = ::std::option::Option::None;
    }

    pub fn has_sloppy_quorum(&self) -> bool {
        self.sloppy_quorum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sloppy_quorum(&mut self, v: bool) {
        self.sloppy_quorum = ::std::option::Option::Some(v);
    }

    pub fn get_sloppy_quorum(&self) -> bool {
        self.sloppy_quorum.unwrap_or(false)
    }

    fn get_sloppy_quorum_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.sloppy_quorum
    }

    fn mut_sloppy_quorum_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.sloppy_quorum
    }

    // optional uint32 n_val = 15;

    pub fn clear_n_val(&mut self) {
        self.n_val = ::std::option::Option::None;
    }

    pub fn has_n_val(&self) -> bool {
        self.n_val.is_some()
    }

    // Param is passed by value, moved
    pub fn set_n_val(&mut self, v: u32) {
        self.n_val = ::std::option::Option::Some(v);
    }

    pub fn get_n_val(&self) -> u32 {
        self.n_val.unwrap_or(0)
    }

    fn get_n_val_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.n_val
    }

    fn mut_n_val_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.n_val
    }

    // optional bytes type = 16;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::vec::Vec<u8>) {
        self.field_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.field_type.is_none() {
            self.field_type.set_default();
        };
        self.field_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::vec::Vec<u8> {
        self.field_type.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_field_type(&self) -> &[u8] {
        match self.field_type.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_field_type_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.field_type
    }
}

impl ::protobuf::Message for RpbPutReq {
    fn is_initialized(&self) -> bool {
        if self.bucket.is_none() {
            return false;
        };
        if self.content.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bucket)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.vclock)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.content)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.w = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.dw = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.return_body = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.pw = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.if_not_modified = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.if_none_match = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.return_head = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.timeout = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.asis = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.sloppy_quorum = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.n_val = ::std::option::Option::Some(tmp);
                },
                16 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.field_type)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.bucket.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.vclock.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        if let Some(v) = self.content.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.w {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.dw {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.return_body {
            my_size += 2;
        };
        if let Some(v) = self.pw {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.if_not_modified {
            my_size += 2;
        };
        if let Some(v) = self.if_none_match {
            my_size += 2;
        };
        if let Some(v) = self.return_head {
            my_size += 2;
        };
        if let Some(v) = self.timeout {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.asis {
            my_size += 2;
        };
        if let Some(v) = self.sloppy_quorum {
            my_size += 2;
        };
        if let Some(v) = self.n_val {
            my_size += ::protobuf::rt::value_size(15, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.field_type.as_ref() {
            my_size += ::protobuf::rt::bytes_size(16, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bucket.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.key.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.vclock.as_ref() {
            os.write_bytes(3, &v)?;
        };
        if let Some(v) = self.content.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.w {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.dw {
            os.write_uint32(6, v)?;
        };
        if let Some(v) = self.return_body {
            os.write_bool(7, v)?;
        };
        if let Some(v) = self.pw {
            os.write_uint32(8, v)?;
        };
        if let Some(v) = self.if_not_modified {
            os.write_bool(9, v)?;
        };
        if let Some(v) = self.if_none_match {
            os.write_bool(10, v)?;
        };
        if let Some(v) = self.return_head {
            os.write_bool(11, v)?;
        };
        if let Some(v) = self.timeout {
            os.write_uint32(12, v)?;
        };
        if let Some(v) = self.asis {
            os.write_bool(13, v)?;
        };
        if let Some(v) = self.sloppy_quorum {
            os.write_bool(14, v)?;
        };
        if let Some(v) = self.n_val {
            os.write_uint32(15, v)?;
        };
        if let Some(v) = self.field_type.as_ref() {
            os.write_bytes(16, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbPutReq {
    fn new() -> RpbPutReq {
        RpbPutReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbPutReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "bucket",
                    RpbPutReq::get_bucket_for_reflect,
                    RpbPutReq::mut_bucket_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    RpbPutReq::get_key_for_reflect,
                    RpbPutReq::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "vclock",
                    RpbPutReq::get_vclock_for_reflect,
                    RpbPutReq::mut_vclock_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RpbContent>>(
                    "content",
                    RpbPutReq::get_content_for_reflect,
                    RpbPutReq::mut_content_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "w",
                    RpbPutReq::get_w_for_reflect,
                    RpbPutReq::mut_w_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "dw",
                    RpbPutReq::get_dw_for_reflect,
                    RpbPutReq::mut_dw_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "return_body",
                    RpbPutReq::get_return_body_for_reflect,
                    RpbPutReq::mut_return_body_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "pw",
                    RpbPutReq::get_pw_for_reflect,
                    RpbPutReq::mut_pw_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "if_not_modified",
                    RpbPutReq::get_if_not_modified_for_reflect,
                    RpbPutReq::mut_if_not_modified_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "if_none_match",
                    RpbPutReq::get_if_none_match_for_reflect,
                    RpbPutReq::mut_if_none_match_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "return_head",
                    RpbPutReq::get_return_head_for_reflect,
                    RpbPutReq::mut_return_head_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "timeout",
                    RpbPutReq::get_timeout_for_reflect,
                    RpbPutReq::mut_timeout_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "asis",
                    RpbPutReq::get_asis_for_reflect,
                    RpbPutReq::mut_asis_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "sloppy_quorum",
                    RpbPutReq::get_sloppy_quorum_for_reflect,
                    RpbPutReq::mut_sloppy_quorum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "n_val",
                    RpbPutReq::get_n_val_for_reflect,
                    RpbPutReq::mut_n_val_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "type",
                    RpbPutReq::get_field_type_for_reflect,
                    RpbPutReq::mut_field_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbPutReq>(
                    "RpbPutReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbPutReq {
    fn clear(&mut self) {
        self.clear_bucket();
        self.clear_key();
        self.clear_vclock();
        self.clear_content();
        self.clear_w();
        self.clear_dw();
        self.clear_return_body();
        self.clear_pw();
        self.clear_if_not_modified();
        self.clear_if_none_match();
        self.clear_return_head();
        self.clear_timeout();
        self.clear_asis();
        self.clear_sloppy_quorum();
        self.clear_n_val();
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbPutReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbPutReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbPutResp {
    // message fields
    content: ::protobuf::RepeatedField<RpbContent>,
    vclock: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbPutResp {}

impl RpbPutResp {
    pub fn new() -> RpbPutResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbPutResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbPutResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbPutResp,
        };
        unsafe {
            instance.get(RpbPutResp::new)
        }
    }

    // repeated .RpbContent content = 1;

    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ::protobuf::RepeatedField<RpbContent>) {
        self.content = v;
    }

    // Mutable pointer to the field.
    pub fn mut_content(&mut self) -> &mut ::protobuf::RepeatedField<RpbContent> {
        &mut self.content
    }

    // Take field
    pub fn take_content(&mut self) -> ::protobuf::RepeatedField<RpbContent> {
        ::std::mem::replace(&mut self.content, ::protobuf::RepeatedField::new())
    }

    pub fn get_content(&self) -> &[RpbContent] {
        &self.content
    }

    fn get_content_for_reflect(&self) -> &::protobuf::RepeatedField<RpbContent> {
        &self.content
    }

    fn mut_content_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RpbContent> {
        &mut self.content
    }

    // optional bytes vclock = 2;

    pub fn clear_vclock(&mut self) {
        self.vclock.clear();
    }

    pub fn has_vclock(&self) -> bool {
        self.vclock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vclock(&mut self, v: ::std::vec::Vec<u8>) {
        self.vclock = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_vclock(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.vclock.is_none() {
            self.vclock.set_default();
        };
        self.vclock.as_mut().unwrap()
    }

    // Take field
    pub fn take_vclock(&mut self) -> ::std::vec::Vec<u8> {
        self.vclock.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_vclock(&self) -> &[u8] {
        match self.vclock.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_vclock_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.vclock
    }

    fn mut_vclock_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.vclock
    }

    // optional bytes key = 3;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.key
    }
}

impl ::protobuf::Message for RpbPutResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.content)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.vclock)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.content {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.vclock.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.content {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.vclock.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.key.as_ref() {
            os.write_bytes(3, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbPutResp {
    fn new() -> RpbPutResp {
        RpbPutResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbPutResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RpbContent>>(
                    "content",
                    RpbPutResp::get_content_for_reflect,
                    RpbPutResp::mut_content_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "vclock",
                    RpbPutResp::get_vclock_for_reflect,
                    RpbPutResp::mut_vclock_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    RpbPutResp::get_key_for_reflect,
                    RpbPutResp::mut_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbPutResp>(
                    "RpbPutResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbPutResp {
    fn clear(&mut self) {
        self.clear_content();
        self.clear_vclock();
        self.clear_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbPutResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbPutResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbDelReq {
    // message fields
    bucket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    rw: ::std::option::Option<u32>,
    vclock: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    r: ::std::option::Option<u32>,
    w: ::std::option::Option<u32>,
    pr: ::std::option::Option<u32>,
    pw: ::std::option::Option<u32>,
    dw: ::std::option::Option<u32>,
    timeout: ::std::option::Option<u32>,
    sloppy_quorum: ::std::option::Option<bool>,
    n_val: ::std::option::Option<u32>,
    field_type: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbDelReq {}

impl RpbDelReq {
    pub fn new() -> RpbDelReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbDelReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbDelReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbDelReq,
        };
        unsafe {
            instance.get(RpbDelReq::new)
        }
    }

    // required bytes bucket = 1;

    pub fn clear_bucket(&mut self) {
        self.bucket.clear();
    }

    pub fn has_bucket(&self) -> bool {
        self.bucket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bucket(&mut self, v: ::std::vec::Vec<u8>) {
        self.bucket = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bucket(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.bucket.is_none() {
            self.bucket.set_default();
        };
        self.bucket.as_mut().unwrap()
    }

    // Take field
    pub fn take_bucket(&mut self) -> ::std::vec::Vec<u8> {
        self.bucket.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_bucket(&self) -> &[u8] {
        match self.bucket.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_bucket_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.bucket
    }

    fn mut_bucket_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.bucket
    }

    // required bytes key = 2;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.key
    }

    // optional uint32 rw = 3;

    pub fn clear_rw(&mut self) {
        self.rw = ::std::option::Option::None;
    }

    pub fn has_rw(&self) -> bool {
        self.rw.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rw(&mut self, v: u32) {
        self.rw = ::std::option::Option::Some(v);
    }

    pub fn get_rw(&self) -> u32 {
        self.rw.unwrap_or(0)
    }

    fn get_rw_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.rw
    }

    fn mut_rw_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.rw
    }

    // optional bytes vclock = 4;

    pub fn clear_vclock(&mut self) {
        self.vclock.clear();
    }

    pub fn has_vclock(&self) -> bool {
        self.vclock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vclock(&mut self, v: ::std::vec::Vec<u8>) {
        self.vclock = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_vclock(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.vclock.is_none() {
            self.vclock.set_default();
        };
        self.vclock.as_mut().unwrap()
    }

    // Take field
    pub fn take_vclock(&mut self) -> ::std::vec::Vec<u8> {
        self.vclock.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_vclock(&self) -> &[u8] {
        match self.vclock.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_vclock_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.vclock
    }

    fn mut_vclock_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.vclock
    }

    // optional uint32 r = 5;

    pub fn clear_r(&mut self) {
        self.r = ::std::option::Option::None;
    }

    pub fn has_r(&self) -> bool {
        self.r.is_some()
    }

    // Param is passed by value, moved
    pub fn set_r(&mut self, v: u32) {
        self.r = ::std::option::Option::Some(v);
    }

    pub fn get_r(&self) -> u32 {
        self.r.unwrap_or(0)
    }

    fn get_r_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.r
    }

    fn mut_r_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.r
    }

    // optional uint32 w = 6;

    pub fn clear_w(&mut self) {
        self.w = ::std::option::Option::None;
    }

    pub fn has_w(&self) -> bool {
        self.w.is_some()
    }

    // Param is passed by value, moved
    pub fn set_w(&mut self, v: u32) {
        self.w = ::std::option::Option::Some(v);
    }

    pub fn get_w(&self) -> u32 {
        self.w.unwrap_or(0)
    }

    fn get_w_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.w
    }

    fn mut_w_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.w
    }

    // optional uint32 pr = 7;

    pub fn clear_pr(&mut self) {
        self.pr = ::std::option::Option::None;
    }

    pub fn has_pr(&self) -> bool {
        self.pr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pr(&mut self, v: u32) {
        self.pr = ::std::option::Option::Some(v);
    }

    pub fn get_pr(&self) -> u32 {
        self.pr.unwrap_or(0)
    }

    fn get_pr_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.pr
    }

    fn mut_pr_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.pr
    }

    // optional uint32 pw = 8;

    pub fn clear_pw(&mut self) {
        self.pw = ::std::option::Option::None;
    }

    pub fn has_pw(&self) -> bool {
        self.pw.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pw(&mut self, v: u32) {
        self.pw = ::std::option::Option::Some(v);
    }

    pub fn get_pw(&self) -> u32 {
        self.pw.unwrap_or(0)
    }

    fn get_pw_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.pw
    }

    fn mut_pw_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.pw
    }

    // optional uint32 dw = 9;

    pub fn clear_dw(&mut self) {
        self.dw = ::std::option::Option::None;
    }

    pub fn has_dw(&self) -> bool {
        self.dw.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dw(&mut self, v: u32) {
        self.dw = ::std::option::Option::Some(v);
    }

    pub fn get_dw(&self) -> u32 {
        self.dw.unwrap_or(0)
    }

    fn get_dw_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.dw
    }

    fn mut_dw_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.dw
    }

    // optional uint32 timeout = 10;

    pub fn clear_timeout(&mut self) {
        self.timeout = ::std::option::Option::None;
    }

    pub fn has_timeout(&self) -> bool {
        self.timeout.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timeout(&mut self, v: u32) {
        self.timeout = ::std::option::Option::Some(v);
    }

    pub fn get_timeout(&self) -> u32 {
        self.timeout.unwrap_or(0)
    }

    fn get_timeout_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.timeout
    }

    fn mut_timeout_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.timeout
    }

    // optional bool sloppy_quorum = 11;

    pub fn clear_sloppy_quorum(&mut self) {
        self.sloppy_quorum = ::std::option::Option::None;
    }

    pub fn has_sloppy_quorum(&self) -> bool {
        self.sloppy_quorum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sloppy_quorum(&mut self, v: bool) {
        self.sloppy_quorum = ::std::option::Option::Some(v);
    }

    pub fn get_sloppy_quorum(&self) -> bool {
        self.sloppy_quorum.unwrap_or(false)
    }

    fn get_sloppy_quorum_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.sloppy_quorum
    }

    fn mut_sloppy_quorum_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.sloppy_quorum
    }

    // optional uint32 n_val = 12;

    pub fn clear_n_val(&mut self) {
        self.n_val = ::std::option::Option::None;
    }

    pub fn has_n_val(&self) -> bool {
        self.n_val.is_some()
    }

    // Param is passed by value, moved
    pub fn set_n_val(&mut self, v: u32) {
        self.n_val = ::std::option::Option::Some(v);
    }

    pub fn get_n_val(&self) -> u32 {
        self.n_val.unwrap_or(0)
    }

    fn get_n_val_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.n_val
    }

    fn mut_n_val_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.n_val
    }

    // optional bytes type = 13;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::vec::Vec<u8>) {
        self.field_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.field_type.is_none() {
            self.field_type.set_default();
        };
        self.field_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::vec::Vec<u8> {
        self.field_type.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_field_type(&self) -> &[u8] {
        match self.field_type.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_field_type_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.field_type
    }
}

impl ::protobuf::Message for RpbDelReq {
    fn is_initialized(&self) -> bool {
        if self.bucket.is_none() {
            return false;
        };
        if self.key.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bucket)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.rw = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.vclock)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.r = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.w = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.pr = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.pw = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.dw = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.timeout = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.sloppy_quorum = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.n_val = ::std::option::Option::Some(tmp);
                },
                13 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.field_type)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.bucket.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.rw {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.vclock.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        };
        if let Some(v) = self.r {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.w {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.pr {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.pw {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.dw {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.timeout {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.sloppy_quorum {
            my_size += 2;
        };
        if let Some(v) = self.n_val {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.field_type.as_ref() {
            my_size += ::protobuf::rt::bytes_size(13, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bucket.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.key.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.rw {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.vclock.as_ref() {
            os.write_bytes(4, &v)?;
        };
        if let Some(v) = self.r {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.w {
            os.write_uint32(6, v)?;
        };
        if let Some(v) = self.pr {
            os.write_uint32(7, v)?;
        };
        if let Some(v) = self.pw {
            os.write_uint32(8, v)?;
        };
        if let Some(v) = self.dw {
            os.write_uint32(9, v)?;
        };
        if let Some(v) = self.timeout {
            os.write_uint32(10, v)?;
        };
        if let Some(v) = self.sloppy_quorum {
            os.write_bool(11, v)?;
        };
        if let Some(v) = self.n_val {
            os.write_uint32(12, v)?;
        };
        if let Some(v) = self.field_type.as_ref() {
            os.write_bytes(13, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbDelReq {
    fn new() -> RpbDelReq {
        RpbDelReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbDelReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "bucket",
                    RpbDelReq::get_bucket_for_reflect,
                    RpbDelReq::mut_bucket_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    RpbDelReq::get_key_for_reflect,
                    RpbDelReq::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "rw",
                    RpbDelReq::get_rw_for_reflect,
                    RpbDelReq::mut_rw_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "vclock",
                    RpbDelReq::get_vclock_for_reflect,
                    RpbDelReq::mut_vclock_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "r",
                    RpbDelReq::get_r_for_reflect,
                    RpbDelReq::mut_r_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "w",
                    RpbDelReq::get_w_for_reflect,
                    RpbDelReq::mut_w_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "pr",
                    RpbDelReq::get_pr_for_reflect,
                    RpbDelReq::mut_pr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "pw",
                    RpbDelReq::get_pw_for_reflect,
                    RpbDelReq::mut_pw_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "dw",
                    RpbDelReq::get_dw_for_reflect,
                    RpbDelReq::mut_dw_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "timeout",
                    RpbDelReq::get_timeout_for_reflect,
                    RpbDelReq::mut_timeout_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "sloppy_quorum",
                    RpbDelReq::get_sloppy_quorum_for_reflect,
                    RpbDelReq::mut_sloppy_quorum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "n_val",
                    RpbDelReq::get_n_val_for_reflect,
                    RpbDelReq::mut_n_val_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "type",
                    RpbDelReq::get_field_type_for_reflect,
                    RpbDelReq::mut_field_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbDelReq>(
                    "RpbDelReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbDelReq {
    fn clear(&mut self) {
        self.clear_bucket();
        self.clear_key();
        self.clear_rw();
        self.clear_vclock();
        self.clear_r();
        self.clear_w();
        self.clear_pr();
        self.clear_pw();
        self.clear_dw();
        self.clear_timeout();
        self.clear_sloppy_quorum();
        self.clear_n_val();
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbDelReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbDelReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbListBucketsReq {
    // message fields
    timeout: ::std::option::Option<u32>,
    stream: ::std::option::Option<bool>,
    field_type: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbListBucketsReq {}

impl RpbListBucketsReq {
    pub fn new() -> RpbListBucketsReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbListBucketsReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbListBucketsReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbListBucketsReq,
        };
        unsafe {
            instance.get(RpbListBucketsReq::new)
        }
    }

    // optional uint32 timeout = 1;

    pub fn clear_timeout(&mut self) {
        self.timeout = ::std::option::Option::None;
    }

    pub fn has_timeout(&self) -> bool {
        self.timeout.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timeout(&mut self, v: u32) {
        self.timeout = ::std::option::Option::Some(v);
    }

    pub fn get_timeout(&self) -> u32 {
        self.timeout.unwrap_or(0)
    }

    fn get_timeout_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.timeout
    }

    fn mut_timeout_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.timeout
    }

    // optional bool stream = 2;

    pub fn clear_stream(&mut self) {
        self.stream = ::std::option::Option::None;
    }

    pub fn has_stream(&self) -> bool {
        self.stream.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stream(&mut self, v: bool) {
        self.stream = ::std::option::Option::Some(v);
    }

    pub fn get_stream(&self) -> bool {
        self.stream.unwrap_or(false)
    }

    fn get_stream_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.stream
    }

    fn mut_stream_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.stream
    }

    // optional bytes type = 3;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::vec::Vec<u8>) {
        self.field_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.field_type.is_none() {
            self.field_type.set_default();
        };
        self.field_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::vec::Vec<u8> {
        self.field_type.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_field_type(&self) -> &[u8] {
        match self.field_type.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_field_type_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.field_type
    }
}

impl ::protobuf::Message for RpbListBucketsReq {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.timeout = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.stream = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.field_type)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.timeout {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.stream {
            my_size += 2;
        };
        if let Some(v) = self.field_type.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.timeout {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.stream {
            os.write_bool(2, v)?;
        };
        if let Some(v) = self.field_type.as_ref() {
            os.write_bytes(3, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbListBucketsReq {
    fn new() -> RpbListBucketsReq {
        RpbListBucketsReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbListBucketsReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "timeout",
                    RpbListBucketsReq::get_timeout_for_reflect,
                    RpbListBucketsReq::mut_timeout_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "stream",
                    RpbListBucketsReq::get_stream_for_reflect,
                    RpbListBucketsReq::mut_stream_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "type",
                    RpbListBucketsReq::get_field_type_for_reflect,
                    RpbListBucketsReq::mut_field_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbListBucketsReq>(
                    "RpbListBucketsReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbListBucketsReq {
    fn clear(&mut self) {
        self.clear_timeout();
        self.clear_stream();
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbListBucketsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbListBucketsReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbListBucketsResp {
    // message fields
    buckets: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    done: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbListBucketsResp {}

impl RpbListBucketsResp {
    pub fn new() -> RpbListBucketsResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbListBucketsResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbListBucketsResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbListBucketsResp,
        };
        unsafe {
            instance.get(RpbListBucketsResp::new)
        }
    }

    // repeated bytes buckets = 1;

    pub fn clear_buckets(&mut self) {
        self.buckets.clear();
    }

    // Param is passed by value, moved
    pub fn set_buckets(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.buckets = v;
    }

    // Mutable pointer to the field.
    pub fn mut_buckets(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.buckets
    }

    // Take field
    pub fn take_buckets(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.buckets, ::protobuf::RepeatedField::new())
    }

    pub fn get_buckets(&self) -> &[::std::vec::Vec<u8>] {
        &self.buckets
    }

    fn get_buckets_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.buckets
    }

    fn mut_buckets_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.buckets
    }

    // optional bool done = 2;

    pub fn clear_done(&mut self) {
        self.done = ::std::option::Option::None;
    }

    pub fn has_done(&self) -> bool {
        self.done.is_some()
    }

    // Param is passed by value, moved
    pub fn set_done(&mut self, v: bool) {
        self.done = ::std::option::Option::Some(v);
    }

    pub fn get_done(&self) -> bool {
        self.done.unwrap_or(false)
    }

    fn get_done_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.done
    }

    fn mut_done_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.done
    }
}

impl ::protobuf::Message for RpbListBucketsResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.buckets)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.done = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.buckets {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        if let Some(v) = self.done {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.buckets {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.done {
            os.write_bool(2, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbListBucketsResp {
    fn new() -> RpbListBucketsResp {
        RpbListBucketsResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbListBucketsResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "buckets",
                    RpbListBucketsResp::get_buckets_for_reflect,
                    RpbListBucketsResp::mut_buckets_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "done",
                    RpbListBucketsResp::get_done_for_reflect,
                    RpbListBucketsResp::mut_done_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbListBucketsResp>(
                    "RpbListBucketsResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbListBucketsResp {
    fn clear(&mut self) {
        self.clear_buckets();
        self.clear_done();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbListBucketsResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbListBucketsResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbListKeysReq {
    // message fields
    bucket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    timeout: ::std::option::Option<u32>,
    field_type: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbListKeysReq {}

impl RpbListKeysReq {
    pub fn new() -> RpbListKeysReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbListKeysReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbListKeysReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbListKeysReq,
        };
        unsafe {
            instance.get(RpbListKeysReq::new)
        }
    }

    // required bytes bucket = 1;

    pub fn clear_bucket(&mut self) {
        self.bucket.clear();
    }

    pub fn has_bucket(&self) -> bool {
        self.bucket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bucket(&mut self, v: ::std::vec::Vec<u8>) {
        self.bucket = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bucket(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.bucket.is_none() {
            self.bucket.set_default();
        };
        self.bucket.as_mut().unwrap()
    }

    // Take field
    pub fn take_bucket(&mut self) -> ::std::vec::Vec<u8> {
        self.bucket.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_bucket(&self) -> &[u8] {
        match self.bucket.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_bucket_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.bucket
    }

    fn mut_bucket_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.bucket
    }

    // optional uint32 timeout = 2;

    pub fn clear_timeout(&mut self) {
        self.timeout = ::std::option::Option::None;
    }

    pub fn has_timeout(&self) -> bool {
        self.timeout.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timeout(&mut self, v: u32) {
        self.timeout = ::std::option::Option::Some(v);
    }

    pub fn get_timeout(&self) -> u32 {
        self.timeout.unwrap_or(0)
    }

    fn get_timeout_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.timeout
    }

    fn mut_timeout_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.timeout
    }

    // optional bytes type = 3;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::vec::Vec<u8>) {
        self.field_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.field_type.is_none() {
            self.field_type.set_default();
        };
        self.field_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::vec::Vec<u8> {
        self.field_type.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_field_type(&self) -> &[u8] {
        match self.field_type.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_field_type_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.field_type
    }
}

impl ::protobuf::Message for RpbListKeysReq {
    fn is_initialized(&self) -> bool {
        if self.bucket.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bucket)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.timeout = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.field_type)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.bucket.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.timeout {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.field_type.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bucket.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.timeout {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.field_type.as_ref() {
            os.write_bytes(3, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbListKeysReq {
    fn new() -> RpbListKeysReq {
        RpbListKeysReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbListKeysReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "bucket",
                    RpbListKeysReq::get_bucket_for_reflect,
                    RpbListKeysReq::mut_bucket_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "timeout",
                    RpbListKeysReq::get_timeout_for_reflect,
                    RpbListKeysReq::mut_timeout_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "type",
                    RpbListKeysReq::get_field_type_for_reflect,
                    RpbListKeysReq::mut_field_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbListKeysReq>(
                    "RpbListKeysReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbListKeysReq {
    fn clear(&mut self) {
        self.clear_bucket();
        self.clear_timeout();
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbListKeysReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbListKeysReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbListKeysResp {
    // message fields
    keys: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    done: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbListKeysResp {}

impl RpbListKeysResp {
    pub fn new() -> RpbListKeysResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbListKeysResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbListKeysResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbListKeysResp,
        };
        unsafe {
            instance.get(RpbListKeysResp::new)
        }
    }

    // repeated bytes keys = 1;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[::std::vec::Vec<u8>] {
        &self.keys
    }

    fn get_keys_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.keys
    }

    fn mut_keys_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.keys
    }

    // optional bool done = 2;

    pub fn clear_done(&mut self) {
        self.done = ::std::option::Option::None;
    }

    pub fn has_done(&self) -> bool {
        self.done.is_some()
    }

    // Param is passed by value, moved
    pub fn set_done(&mut self, v: bool) {
        self.done = ::std::option::Option::Some(v);
    }

    pub fn get_done(&self) -> bool {
        self.done.unwrap_or(false)
    }

    fn get_done_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.done
    }

    fn mut_done_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.done
    }
}

impl ::protobuf::Message for RpbListKeysResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.keys)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.done = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.keys {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        if let Some(v) = self.done {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.keys {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.done {
            os.write_bool(2, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbListKeysResp {
    fn new() -> RpbListKeysResp {
        RpbListKeysResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbListKeysResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "keys",
                    RpbListKeysResp::get_keys_for_reflect,
                    RpbListKeysResp::mut_keys_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "done",
                    RpbListKeysResp::get_done_for_reflect,
                    RpbListKeysResp::mut_done_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbListKeysResp>(
                    "RpbListKeysResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbListKeysResp {
    fn clear(&mut self) {
        self.clear_keys();
        self.clear_done();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbListKeysResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbListKeysResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbMapRedReq {
    // message fields
    request: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    content_type: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbMapRedReq {}

impl RpbMapRedReq {
    pub fn new() -> RpbMapRedReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbMapRedReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbMapRedReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbMapRedReq,
        };
        unsafe {
            instance.get(RpbMapRedReq::new)
        }
    }

    // required bytes request = 1;

    pub fn clear_request(&mut self) {
        self.request.clear();
    }

    pub fn has_request(&self) -> bool {
        self.request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request(&mut self, v: ::std::vec::Vec<u8>) {
        self.request = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_request(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.request.is_none() {
            self.request.set_default();
        };
        self.request.as_mut().unwrap()
    }

    // Take field
    pub fn take_request(&mut self) -> ::std::vec::Vec<u8> {
        self.request.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_request(&self) -> &[u8] {
        match self.request.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_request_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.request
    }

    fn mut_request_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.request
    }

    // required bytes content_type = 2;

    pub fn clear_content_type(&mut self) {
        self.content_type.clear();
    }

    pub fn has_content_type(&self) -> bool {
        self.content_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_content_type(&mut self, v: ::std::vec::Vec<u8>) {
        self.content_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content_type(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.content_type.is_none() {
            self.content_type.set_default();
        };
        self.content_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_content_type(&mut self) -> ::std::vec::Vec<u8> {
        self.content_type.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_content_type(&self) -> &[u8] {
        match self.content_type.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_content_type_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.content_type
    }

    fn mut_content_type_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.content_type
    }
}

impl ::protobuf::Message for RpbMapRedReq {
    fn is_initialized(&self) -> bool {
        if self.request.is_none() {
            return false;
        };
        if self.content_type.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.request)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.content_type)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.request.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.content_type.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.request.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.content_type.as_ref() {
            os.write_bytes(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbMapRedReq {
    fn new() -> RpbMapRedReq {
        RpbMapRedReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbMapRedReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "request",
                    RpbMapRedReq::get_request_for_reflect,
                    RpbMapRedReq::mut_request_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "content_type",
                    RpbMapRedReq::get_content_type_for_reflect,
                    RpbMapRedReq::mut_content_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbMapRedReq>(
                    "RpbMapRedReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbMapRedReq {
    fn clear(&mut self) {
        self.clear_request();
        self.clear_content_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbMapRedReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbMapRedReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbMapRedResp {
    // message fields
    phase: ::std::option::Option<u32>,
    response: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    done: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbMapRedResp {}

impl RpbMapRedResp {
    pub fn new() -> RpbMapRedResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbMapRedResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbMapRedResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbMapRedResp,
        };
        unsafe {
            instance.get(RpbMapRedResp::new)
        }
    }

    // optional uint32 phase = 1;

    pub fn clear_phase(&mut self) {
        self.phase = ::std::option::Option::None;
    }

    pub fn has_phase(&self) -> bool {
        self.phase.is_some()
    }

    // Param is passed by value, moved
    pub fn set_phase(&mut self, v: u32) {
        self.phase = ::std::option::Option::Some(v);
    }

    pub fn get_phase(&self) -> u32 {
        self.phase.unwrap_or(0)
    }

    fn get_phase_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.phase
    }

    fn mut_phase_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.phase
    }

    // optional bytes response = 2;

    pub fn clear_response(&mut self) {
        self.response.clear();
    }

    pub fn has_response(&self) -> bool {
        self.response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_response(&mut self, v: ::std::vec::Vec<u8>) {
        self.response = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_response(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.response.is_none() {
            self.response.set_default();
        };
        self.response.as_mut().unwrap()
    }

    // Take field
    pub fn take_response(&mut self) -> ::std::vec::Vec<u8> {
        self.response.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_response(&self) -> &[u8] {
        match self.response.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_response_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.response
    }

    fn mut_response_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.response
    }

    // optional bool done = 3;

    pub fn clear_done(&mut self) {
        self.done = ::std::option::Option::None;
    }

    pub fn has_done(&self) -> bool {
        self.done.is_some()
    }

    // Param is passed by value, moved
    pub fn set_done(&mut self, v: bool) {
        self.done = ::std::option::Option::Some(v);
    }

    pub fn get_done(&self) -> bool {
        self.done.unwrap_or(false)
    }

    fn get_done_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.done
    }

    fn mut_done_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.done
    }
}

impl ::protobuf::Message for RpbMapRedResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.phase = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.response)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.done = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.phase {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.response.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.done {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.phase {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.response.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.done {
            os.write_bool(3, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbMapRedResp {
    fn new() -> RpbMapRedResp {
        RpbMapRedResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbMapRedResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "phase",
                    RpbMapRedResp::get_phase_for_reflect,
                    RpbMapRedResp::mut_phase_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "response",
                    RpbMapRedResp::get_response_for_reflect,
                    RpbMapRedResp::mut_response_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "done",
                    RpbMapRedResp::get_done_for_reflect,
                    RpbMapRedResp::mut_done_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbMapRedResp>(
                    "RpbMapRedResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbMapRedResp {
    fn clear(&mut self) {
        self.clear_phase();
        self.clear_response();
        self.clear_done();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbMapRedResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbMapRedResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbIndexReq {
    // message fields
    bucket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    index: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    qtype: ::std::option::Option<RpbIndexReq_IndexQueryType>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    range_min: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    range_max: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    return_terms: ::std::option::Option<bool>,
    stream: ::std::option::Option<bool>,
    max_results: ::std::option::Option<u32>,
    continuation: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    timeout: ::std::option::Option<u32>,
    field_type: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    term_regex: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    pagination_sort: ::std::option::Option<bool>,
    cover_context: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    return_body: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbIndexReq {}

impl RpbIndexReq {
    pub fn new() -> RpbIndexReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbIndexReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbIndexReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbIndexReq,
        };
        unsafe {
            instance.get(RpbIndexReq::new)
        }
    }

    // required bytes bucket = 1;

    pub fn clear_bucket(&mut self) {
        self.bucket.clear();
    }

    pub fn has_bucket(&self) -> bool {
        self.bucket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bucket(&mut self, v: ::std::vec::Vec<u8>) {
        self.bucket = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bucket(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.bucket.is_none() {
            self.bucket.set_default();
        };
        self.bucket.as_mut().unwrap()
    }

    // Take field
    pub fn take_bucket(&mut self) -> ::std::vec::Vec<u8> {
        self.bucket.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_bucket(&self) -> &[u8] {
        match self.bucket.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_bucket_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.bucket
    }

    fn mut_bucket_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.bucket
    }

    // required bytes index = 2;

    pub fn clear_index(&mut self) {
        self.index.clear();
    }

    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: ::std::vec::Vec<u8>) {
        self.index = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_index(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.index.is_none() {
            self.index.set_default();
        };
        self.index.as_mut().unwrap()
    }

    // Take field
    pub fn take_index(&mut self) -> ::std::vec::Vec<u8> {
        self.index.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_index(&self) -> &[u8] {
        match self.index.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_index_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.index
    }

    fn mut_index_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.index
    }

    // required .RpbIndexReq.IndexQueryType qtype = 3;

    pub fn clear_qtype(&mut self) {
        self.qtype = ::std::option::Option::None;
    }

    pub fn has_qtype(&self) -> bool {
        self.qtype.is_some()
    }

    // Param is passed by value, moved
    pub fn set_qtype(&mut self, v: RpbIndexReq_IndexQueryType) {
        self.qtype = ::std::option::Option::Some(v);
    }

    pub fn get_qtype(&self) -> RpbIndexReq_IndexQueryType {
        self.qtype.unwrap_or(RpbIndexReq_IndexQueryType::eq)
    }

    fn get_qtype_for_reflect(&self) -> &::std::option::Option<RpbIndexReq_IndexQueryType> {
        &self.qtype
    }

    fn mut_qtype_for_reflect(&mut self) -> &mut ::std::option::Option<RpbIndexReq_IndexQueryType> {
        &mut self.qtype
    }

    // optional bytes key = 4;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.key
    }

    // optional bytes range_min = 5;

    pub fn clear_range_min(&mut self) {
        self.range_min.clear();
    }

    pub fn has_range_min(&self) -> bool {
        self.range_min.is_some()
    }

    // Param is passed by value, moved
    pub fn set_range_min(&mut self, v: ::std::vec::Vec<u8>) {
        self.range_min = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_range_min(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.range_min.is_none() {
            self.range_min.set_default();
        };
        self.range_min.as_mut().unwrap()
    }

    // Take field
    pub fn take_range_min(&mut self) -> ::std::vec::Vec<u8> {
        self.range_min.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_range_min(&self) -> &[u8] {
        match self.range_min.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_range_min_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.range_min
    }

    fn mut_range_min_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.range_min
    }

    // optional bytes range_max = 6;

    pub fn clear_range_max(&mut self) {
        self.range_max.clear();
    }

    pub fn has_range_max(&self) -> bool {
        self.range_max.is_some()
    }

    // Param is passed by value, moved
    pub fn set_range_max(&mut self, v: ::std::vec::Vec<u8>) {
        self.range_max = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_range_max(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.range_max.is_none() {
            self.range_max.set_default();
        };
        self.range_max.as_mut().unwrap()
    }

    // Take field
    pub fn take_range_max(&mut self) -> ::std::vec::Vec<u8> {
        self.range_max.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_range_max(&self) -> &[u8] {
        match self.range_max.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_range_max_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.range_max
    }

    fn mut_range_max_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.range_max
    }

    // optional bool return_terms = 7;

    pub fn clear_return_terms(&mut self) {
        self.return_terms = ::std::option::Option::None;
    }

    pub fn has_return_terms(&self) -> bool {
        self.return_terms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_return_terms(&mut self, v: bool) {
        self.return_terms = ::std::option::Option::Some(v);
    }

    pub fn get_return_terms(&self) -> bool {
        self.return_terms.unwrap_or(false)
    }

    fn get_return_terms_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.return_terms
    }

    fn mut_return_terms_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.return_terms
    }

    // optional bool stream = 8;

    pub fn clear_stream(&mut self) {
        self.stream = ::std::option::Option::None;
    }

    pub fn has_stream(&self) -> bool {
        self.stream.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stream(&mut self, v: bool) {
        self.stream = ::std::option::Option::Some(v);
    }

    pub fn get_stream(&self) -> bool {
        self.stream.unwrap_or(false)
    }

    fn get_stream_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.stream
    }

    fn mut_stream_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.stream
    }

    // optional uint32 max_results = 9;

    pub fn clear_max_results(&mut self) {
        self.max_results = ::std::option::Option::None;
    }

    pub fn has_max_results(&self) -> bool {
        self.max_results.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_results(&mut self, v: u32) {
        self.max_results = ::std::option::Option::Some(v);
    }

    pub fn get_max_results(&self) -> u32 {
        self.max_results.unwrap_or(0)
    }

    fn get_max_results_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.max_results
    }

    fn mut_max_results_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.max_results
    }

    // optional bytes continuation = 10;

    pub fn clear_continuation(&mut self) {
        self.continuation.clear();
    }

    pub fn has_continuation(&self) -> bool {
        self.continuation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_continuation(&mut self, v: ::std::vec::Vec<u8>) {
        self.continuation = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_continuation(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.continuation.is_none() {
            self.continuation.set_default();
        };
        self.continuation.as_mut().unwrap()
    }

    // Take field
    pub fn take_continuation(&mut self) -> ::std::vec::Vec<u8> {
        self.continuation.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_continuation(&self) -> &[u8] {
        match self.continuation.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_continuation_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.continuation
    }

    fn mut_continuation_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.continuation
    }

    // optional uint32 timeout = 11;

    pub fn clear_timeout(&mut self) {
        self.timeout = ::std::option::Option::None;
    }

    pub fn has_timeout(&self) -> bool {
        self.timeout.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timeout(&mut self, v: u32) {
        self.timeout = ::std::option::Option::Some(v);
    }

    pub fn get_timeout(&self) -> u32 {
        self.timeout.unwrap_or(0)
    }

    fn get_timeout_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.timeout
    }

    fn mut_timeout_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.timeout
    }

    // optional bytes type = 12;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::vec::Vec<u8>) {
        self.field_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.field_type.is_none() {
            self.field_type.set_default();
        };
        self.field_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::vec::Vec<u8> {
        self.field_type.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_field_type(&self) -> &[u8] {
        match self.field_type.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_field_type_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.field_type
    }

    // optional bytes term_regex = 13;

    pub fn clear_term_regex(&mut self) {
        self.term_regex.clear();
    }

    pub fn has_term_regex(&self) -> bool {
        self.term_regex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_term_regex(&mut self, v: ::std::vec::Vec<u8>) {
        self.term_regex = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_term_regex(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.term_regex.is_none() {
            self.term_regex.set_default();
        };
        self.term_regex.as_mut().unwrap()
    }

    // Take field
    pub fn take_term_regex(&mut self) -> ::std::vec::Vec<u8> {
        self.term_regex.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_term_regex(&self) -> &[u8] {
        match self.term_regex.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_term_regex_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.term_regex
    }

    fn mut_term_regex_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.term_regex
    }

    // optional bool pagination_sort = 14;

    pub fn clear_pagination_sort(&mut self) {
        self.pagination_sort = ::std::option::Option::None;
    }

    pub fn has_pagination_sort(&self) -> bool {
        self.pagination_sort.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pagination_sort(&mut self, v: bool) {
        self.pagination_sort = ::std::option::Option::Some(v);
    }

    pub fn get_pagination_sort(&self) -> bool {
        self.pagination_sort.unwrap_or(false)
    }

    fn get_pagination_sort_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.pagination_sort
    }

    fn mut_pagination_sort_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.pagination_sort
    }

    // optional bytes cover_context = 15;

    pub fn clear_cover_context(&mut self) {
        self.cover_context.clear();
    }

    pub fn has_cover_context(&self) -> bool {
        self.cover_context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cover_context(&mut self, v: ::std::vec::Vec<u8>) {
        self.cover_context = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cover_context(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.cover_context.is_none() {
            self.cover_context.set_default();
        };
        self.cover_context.as_mut().unwrap()
    }

    // Take field
    pub fn take_cover_context(&mut self) -> ::std::vec::Vec<u8> {
        self.cover_context.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_cover_context(&self) -> &[u8] {
        match self.cover_context.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_cover_context_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.cover_context
    }

    fn mut_cover_context_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.cover_context
    }

    // optional bool return_body = 16;

    pub fn clear_return_body(&mut self) {
        self.return_body = ::std::option::Option::None;
    }

    pub fn has_return_body(&self) -> bool {
        self.return_body.is_some()
    }

    // Param is passed by value, moved
    pub fn set_return_body(&mut self, v: bool) {
        self.return_body = ::std::option::Option::Some(v);
    }

    pub fn get_return_body(&self) -> bool {
        self.return_body.unwrap_or(false)
    }

    fn get_return_body_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.return_body
    }

    fn mut_return_body_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.return_body
    }
}

impl ::protobuf::Message for RpbIndexReq {
    fn is_initialized(&self) -> bool {
        if self.bucket.is_none() {
            return false;
        };
        if self.index.is_none() {
            return false;
        };
        if self.qtype.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bucket)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.index)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.qtype = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.range_min)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.range_max)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.return_terms = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.stream = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.max_results = ::std::option::Option::Some(tmp);
                },
                10 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.continuation)?;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.timeout = ::std::option::Option::Some(tmp);
                },
                12 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.field_type)?;
                },
                13 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.term_regex)?;
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.pagination_sort = ::std::option::Option::Some(tmp);
                },
                15 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.cover_context)?;
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.return_body = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.bucket.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.index.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.qtype {
            my_size += ::protobuf::rt::enum_size(3, v);
        };
        if let Some(v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        };
        if let Some(v) = self.range_min.as_ref() {
            my_size += ::protobuf::rt::bytes_size(5, &v);
        };
        if let Some(v) = self.range_max.as_ref() {
            my_size += ::protobuf::rt::bytes_size(6, &v);
        };
        if let Some(v) = self.return_terms {
            my_size += 2;
        };
        if let Some(v) = self.stream {
            my_size += 2;
        };
        if let Some(v) = self.max_results {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.continuation.as_ref() {
            my_size += ::protobuf::rt::bytes_size(10, &v);
        };
        if let Some(v) = self.timeout {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.field_type.as_ref() {
            my_size += ::protobuf::rt::bytes_size(12, &v);
        };
        if let Some(v) = self.term_regex.as_ref() {
            my_size += ::protobuf::rt::bytes_size(13, &v);
        };
        if let Some(v) = self.pagination_sort {
            my_size += 2;
        };
        if let Some(v) = self.cover_context.as_ref() {
            my_size += ::protobuf::rt::bytes_size(15, &v);
        };
        if let Some(v) = self.return_body {
            my_size += 3;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bucket.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.index.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.qtype {
            os.write_enum(3, v.value())?;
        };
        if let Some(v) = self.key.as_ref() {
            os.write_bytes(4, &v)?;
        };
        if let Some(v) = self.range_min.as_ref() {
            os.write_bytes(5, &v)?;
        };
        if let Some(v) = self.range_max.as_ref() {
            os.write_bytes(6, &v)?;
        };
        if let Some(v) = self.return_terms {
            os.write_bool(7, v)?;
        };
        if let Some(v) = self.stream {
            os.write_bool(8, v)?;
        };
        if let Some(v) = self.max_results {
            os.write_uint32(9, v)?;
        };
        if let Some(v) = self.continuation.as_ref() {
            os.write_bytes(10, &v)?;
        };
        if let Some(v) = self.timeout {
            os.write_uint32(11, v)?;
        };
        if let Some(v) = self.field_type.as_ref() {
            os.write_bytes(12, &v)?;
        };
        if let Some(v) = self.term_regex.as_ref() {
            os.write_bytes(13, &v)?;
        };
        if let Some(v) = self.pagination_sort {
            os.write_bool(14, v)?;
        };
        if let Some(v) = self.cover_context.as_ref() {
            os.write_bytes(15, &v)?;
        };
        if let Some(v) = self.return_body {
            os.write_bool(16, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbIndexReq {
    fn new() -> RpbIndexReq {
        RpbIndexReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbIndexReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "bucket",
                    RpbIndexReq::get_bucket_for_reflect,
                    RpbIndexReq::mut_bucket_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "index",
                    RpbIndexReq::get_index_for_reflect,
                    RpbIndexReq::mut_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<RpbIndexReq_IndexQueryType>>(
                    "qtype",
                    RpbIndexReq::get_qtype_for_reflect,
                    RpbIndexReq::mut_qtype_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    RpbIndexReq::get_key_for_reflect,
                    RpbIndexReq::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "range_min",
                    RpbIndexReq::get_range_min_for_reflect,
                    RpbIndexReq::mut_range_min_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "range_max",
                    RpbIndexReq::get_range_max_for_reflect,
                    RpbIndexReq::mut_range_max_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "return_terms",
                    RpbIndexReq::get_return_terms_for_reflect,
                    RpbIndexReq::mut_return_terms_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "stream",
                    RpbIndexReq::get_stream_for_reflect,
                    RpbIndexReq::mut_stream_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "max_results",
                    RpbIndexReq::get_max_results_for_reflect,
                    RpbIndexReq::mut_max_results_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "continuation",
                    RpbIndexReq::get_continuation_for_reflect,
                    RpbIndexReq::mut_continuation_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "timeout",
                    RpbIndexReq::get_timeout_for_reflect,
                    RpbIndexReq::mut_timeout_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "type",
                    RpbIndexReq::get_field_type_for_reflect,
                    RpbIndexReq::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "term_regex",
                    RpbIndexReq::get_term_regex_for_reflect,
                    RpbIndexReq::mut_term_regex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "pagination_sort",
                    RpbIndexReq::get_pagination_sort_for_reflect,
                    RpbIndexReq::mut_pagination_sort_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "cover_context",
                    RpbIndexReq::get_cover_context_for_reflect,
                    RpbIndexReq::mut_cover_context_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "return_body",
                    RpbIndexReq::get_return_body_for_reflect,
                    RpbIndexReq::mut_return_body_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbIndexReq>(
                    "RpbIndexReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbIndexReq {
    fn clear(&mut self) {
        self.clear_bucket();
        self.clear_index();
        self.clear_qtype();
        self.clear_key();
        self.clear_range_min();
        self.clear_range_max();
        self.clear_return_terms();
        self.clear_stream();
        self.clear_max_results();
        self.clear_continuation();
        self.clear_timeout();
        self.clear_field_type();
        self.clear_term_regex();
        self.clear_pagination_sort();
        self.clear_cover_context();
        self.clear_return_body();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbIndexReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbIndexReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum RpbIndexReq_IndexQueryType {
    eq = 0,
    range = 1,
}

impl ::protobuf::ProtobufEnum for RpbIndexReq_IndexQueryType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RpbIndexReq_IndexQueryType> {
        match value {
            0 => ::std::option::Option::Some(RpbIndexReq_IndexQueryType::eq),
            1 => ::std::option::Option::Some(RpbIndexReq_IndexQueryType::range),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [RpbIndexReq_IndexQueryType] = &[
            RpbIndexReq_IndexQueryType::eq,
            RpbIndexReq_IndexQueryType::range,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<RpbIndexReq_IndexQueryType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("RpbIndexReq_IndexQueryType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for RpbIndexReq_IndexQueryType {
}

impl ::protobuf::reflect::ProtobufValue for RpbIndexReq_IndexQueryType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbIndexResp {
    // message fields
    keys: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    results: ::protobuf::RepeatedField<super::riak::RpbPair>,
    continuation: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    done: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbIndexResp {}

impl RpbIndexResp {
    pub fn new() -> RpbIndexResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbIndexResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbIndexResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbIndexResp,
        };
        unsafe {
            instance.get(RpbIndexResp::new)
        }
    }

    // repeated bytes keys = 1;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[::std::vec::Vec<u8>] {
        &self.keys
    }

    fn get_keys_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.keys
    }

    fn mut_keys_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.keys
    }

    // repeated .RpbPair results = 2;

    pub fn clear_results(&mut self) {
        self.results.clear();
    }

    // Param is passed by value, moved
    pub fn set_results(&mut self, v: ::protobuf::RepeatedField<super::riak::RpbPair>) {
        self.results = v;
    }

    // Mutable pointer to the field.
    pub fn mut_results(&mut self) -> &mut ::protobuf::RepeatedField<super::riak::RpbPair> {
        &mut self.results
    }

    // Take field
    pub fn take_results(&mut self) -> ::protobuf::RepeatedField<super::riak::RpbPair> {
        ::std::mem::replace(&mut self.results, ::protobuf::RepeatedField::new())
    }

    pub fn get_results(&self) -> &[super::riak::RpbPair] {
        &self.results
    }

    fn get_results_for_reflect(&self) -> &::protobuf::RepeatedField<super::riak::RpbPair> {
        &self.results
    }

    fn mut_results_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::riak::RpbPair> {
        &mut self.results
    }

    // optional bytes continuation = 3;

    pub fn clear_continuation(&mut self) {
        self.continuation.clear();
    }

    pub fn has_continuation(&self) -> bool {
        self.continuation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_continuation(&mut self, v: ::std::vec::Vec<u8>) {
        self.continuation = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_continuation(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.continuation.is_none() {
            self.continuation.set_default();
        };
        self.continuation.as_mut().unwrap()
    }

    // Take field
    pub fn take_continuation(&mut self) -> ::std::vec::Vec<u8> {
        self.continuation.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_continuation(&self) -> &[u8] {
        match self.continuation.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_continuation_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.continuation
    }

    fn mut_continuation_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.continuation
    }

    // optional bool done = 4;

    pub fn clear_done(&mut self) {
        self.done = ::std::option::Option::None;
    }

    pub fn has_done(&self) -> bool {
        self.done.is_some()
    }

    // Param is passed by value, moved
    pub fn set_done(&mut self, v: bool) {
        self.done = ::std::option::Option::Some(v);
    }

    pub fn get_done(&self) -> bool {
        self.done.unwrap_or(false)
    }

    fn get_done_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.done
    }

    fn mut_done_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.done
    }
}

impl ::protobuf::Message for RpbIndexResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.keys)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.results)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.continuation)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.done = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.keys {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.results {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.continuation.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        if let Some(v) = self.done {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.keys {
            os.write_bytes(1, &v)?;
        };
        for v in &self.results {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.continuation.as_ref() {
            os.write_bytes(3, &v)?;
        };
        if let Some(v) = self.done {
            os.write_bool(4, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbIndexResp {
    fn new() -> RpbIndexResp {
        RpbIndexResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbIndexResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "keys",
                    RpbIndexResp::get_keys_for_reflect,
                    RpbIndexResp::mut_keys_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::riak::RpbPair>>(
                    "results",
                    RpbIndexResp::get_results_for_reflect,
                    RpbIndexResp::mut_results_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "continuation",
                    RpbIndexResp::get_continuation_for_reflect,
                    RpbIndexResp::mut_continuation_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "done",
                    RpbIndexResp::get_done_for_reflect,
                    RpbIndexResp::mut_done_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbIndexResp>(
                    "RpbIndexResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbIndexResp {
    fn clear(&mut self) {
        self.clear_keys();
        self.clear_results();
        self.clear_continuation();
        self.clear_done();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbIndexResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbIndexResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbIndexBodyResp {
    // message fields
    objects: ::protobuf::RepeatedField<RpbIndexObject>,
    continuation: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    done: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbIndexBodyResp {}

impl RpbIndexBodyResp {
    pub fn new() -> RpbIndexBodyResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbIndexBodyResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbIndexBodyResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbIndexBodyResp,
        };
        unsafe {
            instance.get(RpbIndexBodyResp::new)
        }
    }

    // repeated .RpbIndexObject objects = 1;

    pub fn clear_objects(&mut self) {
        self.objects.clear();
    }

    // Param is passed by value, moved
    pub fn set_objects(&mut self, v: ::protobuf::RepeatedField<RpbIndexObject>) {
        self.objects = v;
    }

    // Mutable pointer to the field.
    pub fn mut_objects(&mut self) -> &mut ::protobuf::RepeatedField<RpbIndexObject> {
        &mut self.objects
    }

    // Take field
    pub fn take_objects(&mut self) -> ::protobuf::RepeatedField<RpbIndexObject> {
        ::std::mem::replace(&mut self.objects, ::protobuf::RepeatedField::new())
    }

    pub fn get_objects(&self) -> &[RpbIndexObject] {
        &self.objects
    }

    fn get_objects_for_reflect(&self) -> &::protobuf::RepeatedField<RpbIndexObject> {
        &self.objects
    }

    fn mut_objects_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RpbIndexObject> {
        &mut self.objects
    }

    // optional bytes continuation = 2;

    pub fn clear_continuation(&mut self) {
        self.continuation.clear();
    }

    pub fn has_continuation(&self) -> bool {
        self.continuation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_continuation(&mut self, v: ::std::vec::Vec<u8>) {
        self.continuation = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_continuation(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.continuation.is_none() {
            self.continuation.set_default();
        };
        self.continuation.as_mut().unwrap()
    }

    // Take field
    pub fn take_continuation(&mut self) -> ::std::vec::Vec<u8> {
        self.continuation.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_continuation(&self) -> &[u8] {
        match self.continuation.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_continuation_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.continuation
    }

    fn mut_continuation_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.continuation
    }

    // optional bool done = 3;

    pub fn clear_done(&mut self) {
        self.done = ::std::option::Option::None;
    }

    pub fn has_done(&self) -> bool {
        self.done.is_some()
    }

    // Param is passed by value, moved
    pub fn set_done(&mut self, v: bool) {
        self.done = ::std::option::Option::Some(v);
    }

    pub fn get_done(&self) -> bool {
        self.done.unwrap_or(false)
    }

    fn get_done_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.done
    }

    fn mut_done_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.done
    }
}

impl ::protobuf::Message for RpbIndexBodyResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.objects)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.continuation)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.done = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.objects {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.continuation.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.done {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.objects {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.continuation.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.done {
            os.write_bool(3, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbIndexBodyResp {
    fn new() -> RpbIndexBodyResp {
        RpbIndexBodyResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbIndexBodyResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RpbIndexObject>>(
                    "objects",
                    RpbIndexBodyResp::get_objects_for_reflect,
                    RpbIndexBodyResp::mut_objects_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "continuation",
                    RpbIndexBodyResp::get_continuation_for_reflect,
                    RpbIndexBodyResp::mut_continuation_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "done",
                    RpbIndexBodyResp::get_done_for_reflect,
                    RpbIndexBodyResp::mut_done_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbIndexBodyResp>(
                    "RpbIndexBodyResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbIndexBodyResp {
    fn clear(&mut self) {
        self.clear_objects();
        self.clear_continuation();
        self.clear_done();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbIndexBodyResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbIndexBodyResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbCSBucketReq {
    // message fields
    bucket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    start_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    end_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    start_incl: ::std::option::Option<bool>,
    end_incl: ::std::option::Option<bool>,
    continuation: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    max_results: ::std::option::Option<u32>,
    timeout: ::std::option::Option<u32>,
    field_type: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    cover_context: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbCSBucketReq {}

impl RpbCSBucketReq {
    pub fn new() -> RpbCSBucketReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbCSBucketReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbCSBucketReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbCSBucketReq,
        };
        unsafe {
            instance.get(RpbCSBucketReq::new)
        }
    }

    // required bytes bucket = 1;

    pub fn clear_bucket(&mut self) {
        self.bucket.clear();
    }

    pub fn has_bucket(&self) -> bool {
        self.bucket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bucket(&mut self, v: ::std::vec::Vec<u8>) {
        self.bucket = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bucket(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.bucket.is_none() {
            self.bucket.set_default();
        };
        self.bucket.as_mut().unwrap()
    }

    // Take field
    pub fn take_bucket(&mut self) -> ::std::vec::Vec<u8> {
        self.bucket.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_bucket(&self) -> &[u8] {
        match self.bucket.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_bucket_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.bucket
    }

    fn mut_bucket_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.bucket
    }

    // required bytes start_key = 2;

    pub fn clear_start_key(&mut self) {
        self.start_key.clear();
    }

    pub fn has_start_key(&self) -> bool {
        self.start_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.start_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.start_key.is_none() {
            self.start_key.set_default();
        };
        self.start_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_start_key(&mut self) -> ::std::vec::Vec<u8> {
        self.start_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_start_key(&self) -> &[u8] {
        match self.start_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_start_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.start_key
    }

    fn mut_start_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.start_key
    }

    // optional bytes end_key = 3;

    pub fn clear_end_key(&mut self) {
        self.end_key.clear();
    }

    pub fn has_end_key(&self) -> bool {
        self.end_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.end_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_end_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.end_key.is_none() {
            self.end_key.set_default();
        };
        self.end_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_end_key(&mut self) -> ::std::vec::Vec<u8> {
        self.end_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_end_key(&self) -> &[u8] {
        match self.end_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_end_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.end_key
    }

    fn mut_end_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.end_key
    }

    // optional bool start_incl = 4;

    pub fn clear_start_incl(&mut self) {
        self.start_incl = ::std::option::Option::None;
    }

    pub fn has_start_incl(&self) -> bool {
        self.start_incl.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_incl(&mut self, v: bool) {
        self.start_incl = ::std::option::Option::Some(v);
    }

    pub fn get_start_incl(&self) -> bool {
        self.start_incl.unwrap_or(true)
    }

    fn get_start_incl_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.start_incl
    }

    fn mut_start_incl_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.start_incl
    }

    // optional bool end_incl = 5;

    pub fn clear_end_incl(&mut self) {
        self.end_incl = ::std::option::Option::None;
    }

    pub fn has_end_incl(&self) -> bool {
        self.end_incl.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end_incl(&mut self, v: bool) {
        self.end_incl = ::std::option::Option::Some(v);
    }

    pub fn get_end_incl(&self) -> bool {
        self.end_incl.unwrap_or(false)
    }

    fn get_end_incl_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.end_incl
    }

    fn mut_end_incl_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.end_incl
    }

    // optional bytes continuation = 6;

    pub fn clear_continuation(&mut self) {
        self.continuation.clear();
    }

    pub fn has_continuation(&self) -> bool {
        self.continuation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_continuation(&mut self, v: ::std::vec::Vec<u8>) {
        self.continuation = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_continuation(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.continuation.is_none() {
            self.continuation.set_default();
        };
        self.continuation.as_mut().unwrap()
    }

    // Take field
    pub fn take_continuation(&mut self) -> ::std::vec::Vec<u8> {
        self.continuation.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_continuation(&self) -> &[u8] {
        match self.continuation.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_continuation_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.continuation
    }

    fn mut_continuation_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.continuation
    }

    // optional uint32 max_results = 7;

    pub fn clear_max_results(&mut self) {
        self.max_results = ::std::option::Option::None;
    }

    pub fn has_max_results(&self) -> bool {
        self.max_results.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_results(&mut self, v: u32) {
        self.max_results = ::std::option::Option::Some(v);
    }

    pub fn get_max_results(&self) -> u32 {
        self.max_results.unwrap_or(0)
    }

    fn get_max_results_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.max_results
    }

    fn mut_max_results_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.max_results
    }

    // optional uint32 timeout = 8;

    pub fn clear_timeout(&mut self) {
        self.timeout = ::std::option::Option::None;
    }

    pub fn has_timeout(&self) -> bool {
        self.timeout.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timeout(&mut self, v: u32) {
        self.timeout = ::std::option::Option::Some(v);
    }

    pub fn get_timeout(&self) -> u32 {
        self.timeout.unwrap_or(0)
    }

    fn get_timeout_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.timeout
    }

    fn mut_timeout_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.timeout
    }

    // optional bytes type = 9;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::vec::Vec<u8>) {
        self.field_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.field_type.is_none() {
            self.field_type.set_default();
        };
        self.field_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::vec::Vec<u8> {
        self.field_type.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_field_type(&self) -> &[u8] {
        match self.field_type.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_field_type_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.field_type
    }

    // optional bytes cover_context = 10;

    pub fn clear_cover_context(&mut self) {
        self.cover_context.clear();
    }

    pub fn has_cover_context(&self) -> bool {
        self.cover_context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cover_context(&mut self, v: ::std::vec::Vec<u8>) {
        self.cover_context = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cover_context(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.cover_context.is_none() {
            self.cover_context.set_default();
        };
        self.cover_context.as_mut().unwrap()
    }

    // Take field
    pub fn take_cover_context(&mut self) -> ::std::vec::Vec<u8> {
        self.cover_context.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_cover_context(&self) -> &[u8] {
        match self.cover_context.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_cover_context_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.cover_context
    }

    fn mut_cover_context_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.cover_context
    }
}

impl ::protobuf::Message for RpbCSBucketReq {
    fn is_initialized(&self) -> bool {
        if self.bucket.is_none() {
            return false;
        };
        if self.start_key.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bucket)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.start_key)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.end_key)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.start_incl = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.end_incl = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.continuation)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.max_results = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.timeout = ::std::option::Option::Some(tmp);
                },
                9 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.field_type)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.cover_context)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.bucket.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.start_key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.end_key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        if let Some(v) = self.start_incl {
            my_size += 2;
        };
        if let Some(v) = self.end_incl {
            my_size += 2;
        };
        if let Some(v) = self.continuation.as_ref() {
            my_size += ::protobuf::rt::bytes_size(6, &v);
        };
        if let Some(v) = self.max_results {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.timeout {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.field_type.as_ref() {
            my_size += ::protobuf::rt::bytes_size(9, &v);
        };
        if let Some(v) = self.cover_context.as_ref() {
            my_size += ::protobuf::rt::bytes_size(10, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bucket.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.start_key.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.end_key.as_ref() {
            os.write_bytes(3, &v)?;
        };
        if let Some(v) = self.start_incl {
            os.write_bool(4, v)?;
        };
        if let Some(v) = self.end_incl {
            os.write_bool(5, v)?;
        };
        if let Some(v) = self.continuation.as_ref() {
            os.write_bytes(6, &v)?;
        };
        if let Some(v) = self.max_results {
            os.write_uint32(7, v)?;
        };
        if let Some(v) = self.timeout {
            os.write_uint32(8, v)?;
        };
        if let Some(v) = self.field_type.as_ref() {
            os.write_bytes(9, &v)?;
        };
        if let Some(v) = self.cover_context.as_ref() {
            os.write_bytes(10, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbCSBucketReq {
    fn new() -> RpbCSBucketReq {
        RpbCSBucketReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbCSBucketReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "bucket",
                    RpbCSBucketReq::get_bucket_for_reflect,
                    RpbCSBucketReq::mut_bucket_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "start_key",
                    RpbCSBucketReq::get_start_key_for_reflect,
                    RpbCSBucketReq::mut_start_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "end_key",
                    RpbCSBucketReq::get_end_key_for_reflect,
                    RpbCSBucketReq::mut_end_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "start_incl",
                    RpbCSBucketReq::get_start_incl_for_reflect,
                    RpbCSBucketReq::mut_start_incl_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "end_incl",
                    RpbCSBucketReq::get_end_incl_for_reflect,
                    RpbCSBucketReq::mut_end_incl_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "continuation",
                    RpbCSBucketReq::get_continuation_for_reflect,
                    RpbCSBucketReq::mut_continuation_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "max_results",
                    RpbCSBucketReq::get_max_results_for_reflect,
                    RpbCSBucketReq::mut_max_results_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "timeout",
                    RpbCSBucketReq::get_timeout_for_reflect,
                    RpbCSBucketReq::mut_timeout_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "type",
                    RpbCSBucketReq::get_field_type_for_reflect,
                    RpbCSBucketReq::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "cover_context",
                    RpbCSBucketReq::get_cover_context_for_reflect,
                    RpbCSBucketReq::mut_cover_context_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbCSBucketReq>(
                    "RpbCSBucketReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbCSBucketReq {
    fn clear(&mut self) {
        self.clear_bucket();
        self.clear_start_key();
        self.clear_end_key();
        self.clear_start_incl();
        self.clear_end_incl();
        self.clear_continuation();
        self.clear_max_results();
        self.clear_timeout();
        self.clear_field_type();
        self.clear_cover_context();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbCSBucketReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbCSBucketReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbCSBucketResp {
    // message fields
    objects: ::protobuf::RepeatedField<RpbIndexObject>,
    continuation: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    done: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbCSBucketResp {}

impl RpbCSBucketResp {
    pub fn new() -> RpbCSBucketResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbCSBucketResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbCSBucketResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbCSBucketResp,
        };
        unsafe {
            instance.get(RpbCSBucketResp::new)
        }
    }

    // repeated .RpbIndexObject objects = 1;

    pub fn clear_objects(&mut self) {
        self.objects.clear();
    }

    // Param is passed by value, moved
    pub fn set_objects(&mut self, v: ::protobuf::RepeatedField<RpbIndexObject>) {
        self.objects = v;
    }

    // Mutable pointer to the field.
    pub fn mut_objects(&mut self) -> &mut ::protobuf::RepeatedField<RpbIndexObject> {
        &mut self.objects
    }

    // Take field
    pub fn take_objects(&mut self) -> ::protobuf::RepeatedField<RpbIndexObject> {
        ::std::mem::replace(&mut self.objects, ::protobuf::RepeatedField::new())
    }

    pub fn get_objects(&self) -> &[RpbIndexObject] {
        &self.objects
    }

    fn get_objects_for_reflect(&self) -> &::protobuf::RepeatedField<RpbIndexObject> {
        &self.objects
    }

    fn mut_objects_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RpbIndexObject> {
        &mut self.objects
    }

    // optional bytes continuation = 2;

    pub fn clear_continuation(&mut self) {
        self.continuation.clear();
    }

    pub fn has_continuation(&self) -> bool {
        self.continuation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_continuation(&mut self, v: ::std::vec::Vec<u8>) {
        self.continuation = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_continuation(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.continuation.is_none() {
            self.continuation.set_default();
        };
        self.continuation.as_mut().unwrap()
    }

    // Take field
    pub fn take_continuation(&mut self) -> ::std::vec::Vec<u8> {
        self.continuation.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_continuation(&self) -> &[u8] {
        match self.continuation.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_continuation_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.continuation
    }

    fn mut_continuation_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.continuation
    }

    // optional bool done = 3;

    pub fn clear_done(&mut self) {
        self.done = ::std::option::Option::None;
    }

    pub fn has_done(&self) -> bool {
        self.done.is_some()
    }

    // Param is passed by value, moved
    pub fn set_done(&mut self, v: bool) {
        self.done = ::std::option::Option::Some(v);
    }

    pub fn get_done(&self) -> bool {
        self.done.unwrap_or(false)
    }

    fn get_done_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.done
    }

    fn mut_done_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.done
    }
}

impl ::protobuf::Message for RpbCSBucketResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.objects)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.continuation)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.done = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.objects {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.continuation.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.done {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.objects {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.continuation.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.done {
            os.write_bool(3, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbCSBucketResp {
    fn new() -> RpbCSBucketResp {
        RpbCSBucketResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbCSBucketResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RpbIndexObject>>(
                    "objects",
                    RpbCSBucketResp::get_objects_for_reflect,
                    RpbCSBucketResp::mut_objects_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "continuation",
                    RpbCSBucketResp::get_continuation_for_reflect,
                    RpbCSBucketResp::mut_continuation_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "done",
                    RpbCSBucketResp::get_done_for_reflect,
                    RpbCSBucketResp::mut_done_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbCSBucketResp>(
                    "RpbCSBucketResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbCSBucketResp {
    fn clear(&mut self) {
        self.clear_objects();
        self.clear_continuation();
        self.clear_done();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbCSBucketResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbCSBucketResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbIndexObject {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    object: ::protobuf::SingularPtrField<RpbGetResp>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbIndexObject {}

impl RpbIndexObject {
    pub fn new() -> RpbIndexObject {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbIndexObject {
        static mut instance: ::protobuf::lazy::Lazy<RpbIndexObject> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbIndexObject,
        };
        unsafe {
            instance.get(RpbIndexObject::new)
        }
    }

    // required bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.key
    }

    // required .RpbGetResp object = 2;

    pub fn clear_object(&mut self) {
        self.object.clear();
    }

    pub fn has_object(&self) -> bool {
        self.object.is_some()
    }

    // Param is passed by value, moved
    pub fn set_object(&mut self, v: RpbGetResp) {
        self.object = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_object(&mut self) -> &mut RpbGetResp {
        if self.object.is_none() {
            self.object.set_default();
        };
        self.object.as_mut().unwrap()
    }

    // Take field
    pub fn take_object(&mut self) -> RpbGetResp {
        self.object.take().unwrap_or_else(|| RpbGetResp::new())
    }

    pub fn get_object(&self) -> &RpbGetResp {
        self.object.as_ref().unwrap_or_else(|| RpbGetResp::default_instance())
    }

    fn get_object_for_reflect(&self) -> &::protobuf::SingularPtrField<RpbGetResp> {
        &self.object
    }

    fn mut_object_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RpbGetResp> {
        &mut self.object
    }
}

impl ::protobuf::Message for RpbIndexObject {
    fn is_initialized(&self) -> bool {
        if self.key.is_none() {
            return false;
        };
        if self.object.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.object)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.object.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.object.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbIndexObject {
    fn new() -> RpbIndexObject {
        RpbIndexObject::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbIndexObject>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    RpbIndexObject::get_key_for_reflect,
                    RpbIndexObject::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RpbGetResp>>(
                    "object",
                    RpbIndexObject::get_object_for_reflect,
                    RpbIndexObject::mut_object_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbIndexObject>(
                    "RpbIndexObject",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbIndexObject {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_object();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbIndexObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbIndexObject {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbContent {
    // message fields
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    content_type: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    charset: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    content_encoding: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    vtag: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    links: ::protobuf::RepeatedField<RpbLink>,
    last_mod: ::std::option::Option<u32>,
    last_mod_usecs: ::std::option::Option<u32>,
    usermeta: ::protobuf::RepeatedField<super::riak::RpbPair>,
    indexes: ::protobuf::RepeatedField<super::riak::RpbPair>,
    deleted: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbContent {}

impl RpbContent {
    pub fn new() -> RpbContent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbContent {
        static mut instance: ::protobuf::lazy::Lazy<RpbContent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbContent,
        };
        unsafe {
            instance.get(RpbContent::new)
        }
    }

    // required bytes value = 1;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.value
    }

    // optional bytes content_type = 2;

    pub fn clear_content_type(&mut self) {
        self.content_type.clear();
    }

    pub fn has_content_type(&self) -> bool {
        self.content_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_content_type(&mut self, v: ::std::vec::Vec<u8>) {
        self.content_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content_type(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.content_type.is_none() {
            self.content_type.set_default();
        };
        self.content_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_content_type(&mut self) -> ::std::vec::Vec<u8> {
        self.content_type.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_content_type(&self) -> &[u8] {
        match self.content_type.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_content_type_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.content_type
    }

    fn mut_content_type_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.content_type
    }

    // optional bytes charset = 3;

    pub fn clear_charset(&mut self) {
        self.charset.clear();
    }

    pub fn has_charset(&self) -> bool {
        self.charset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_charset(&mut self, v: ::std::vec::Vec<u8>) {
        self.charset = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_charset(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.charset.is_none() {
            self.charset.set_default();
        };
        self.charset.as_mut().unwrap()
    }

    // Take field
    pub fn take_charset(&mut self) -> ::std::vec::Vec<u8> {
        self.charset.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_charset(&self) -> &[u8] {
        match self.charset.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_charset_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.charset
    }

    fn mut_charset_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.charset
    }

    // optional bytes content_encoding = 4;

    pub fn clear_content_encoding(&mut self) {
        self.content_encoding.clear();
    }

    pub fn has_content_encoding(&self) -> bool {
        self.content_encoding.is_some()
    }

    // Param is passed by value, moved
    pub fn set_content_encoding(&mut self, v: ::std::vec::Vec<u8>) {
        self.content_encoding = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content_encoding(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.content_encoding.is_none() {
            self.content_encoding.set_default();
        };
        self.content_encoding.as_mut().unwrap()
    }

    // Take field
    pub fn take_content_encoding(&mut self) -> ::std::vec::Vec<u8> {
        self.content_encoding.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_content_encoding(&self) -> &[u8] {
        match self.content_encoding.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_content_encoding_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.content_encoding
    }

    fn mut_content_encoding_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.content_encoding
    }

    // optional bytes vtag = 5;

    pub fn clear_vtag(&mut self) {
        self.vtag.clear();
    }

    pub fn has_vtag(&self) -> bool {
        self.vtag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vtag(&mut self, v: ::std::vec::Vec<u8>) {
        self.vtag = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_vtag(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.vtag.is_none() {
            self.vtag.set_default();
        };
        self.vtag.as_mut().unwrap()
    }

    // Take field
    pub fn take_vtag(&mut self) -> ::std::vec::Vec<u8> {
        self.vtag.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_vtag(&self) -> &[u8] {
        match self.vtag.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_vtag_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.vtag
    }

    fn mut_vtag_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.vtag
    }

    // repeated .RpbLink links = 6;

    pub fn clear_links(&mut self) {
        self.links.clear();
    }

    // Param is passed by value, moved
    pub fn set_links(&mut self, v: ::protobuf::RepeatedField<RpbLink>) {
        self.links = v;
    }

    // Mutable pointer to the field.
    pub fn mut_links(&mut self) -> &mut ::protobuf::RepeatedField<RpbLink> {
        &mut self.links
    }

    // Take field
    pub fn take_links(&mut self) -> ::protobuf::RepeatedField<RpbLink> {
        ::std::mem::replace(&mut self.links, ::protobuf::RepeatedField::new())
    }

    pub fn get_links(&self) -> &[RpbLink] {
        &self.links
    }

    fn get_links_for_reflect(&self) -> &::protobuf::RepeatedField<RpbLink> {
        &self.links
    }

    fn mut_links_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RpbLink> {
        &mut self.links
    }

    // optional uint32 last_mod = 7;

    pub fn clear_last_mod(&mut self) {
        self.last_mod = ::std::option::Option::None;
    }

    pub fn has_last_mod(&self) -> bool {
        self.last_mod.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_mod(&mut self, v: u32) {
        self.last_mod = ::std::option::Option::Some(v);
    }

    pub fn get_last_mod(&self) -> u32 {
        self.last_mod.unwrap_or(0)
    }

    fn get_last_mod_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.last_mod
    }

    fn mut_last_mod_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.last_mod
    }

    // optional uint32 last_mod_usecs = 8;

    pub fn clear_last_mod_usecs(&mut self) {
        self.last_mod_usecs = ::std::option::Option::None;
    }

    pub fn has_last_mod_usecs(&self) -> bool {
        self.last_mod_usecs.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_mod_usecs(&mut self, v: u32) {
        self.last_mod_usecs = ::std::option::Option::Some(v);
    }

    pub fn get_last_mod_usecs(&self) -> u32 {
        self.last_mod_usecs.unwrap_or(0)
    }

    fn get_last_mod_usecs_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.last_mod_usecs
    }

    fn mut_last_mod_usecs_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.last_mod_usecs
    }

    // repeated .RpbPair usermeta = 9;

    pub fn clear_usermeta(&mut self) {
        self.usermeta.clear();
    }

    // Param is passed by value, moved
    pub fn set_usermeta(&mut self, v: ::protobuf::RepeatedField<super::riak::RpbPair>) {
        self.usermeta = v;
    }

    // Mutable pointer to the field.
    pub fn mut_usermeta(&mut self) -> &mut ::protobuf::RepeatedField<super::riak::RpbPair> {
        &mut self.usermeta
    }

    // Take field
    pub fn take_usermeta(&mut self) -> ::protobuf::RepeatedField<super::riak::RpbPair> {
        ::std::mem::replace(&mut self.usermeta, ::protobuf::RepeatedField::new())
    }

    pub fn get_usermeta(&self) -> &[super::riak::RpbPair] {
        &self.usermeta
    }

    fn get_usermeta_for_reflect(&self) -> &::protobuf::RepeatedField<super::riak::RpbPair> {
        &self.usermeta
    }

    fn mut_usermeta_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::riak::RpbPair> {
        &mut self.usermeta
    }

    // repeated .RpbPair indexes = 10;

    pub fn clear_indexes(&mut self) {
        self.indexes.clear();
    }

    // Param is passed by value, moved
    pub fn set_indexes(&mut self, v: ::protobuf::RepeatedField<super::riak::RpbPair>) {
        self.indexes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_indexes(&mut self) -> &mut ::protobuf::RepeatedField<super::riak::RpbPair> {
        &mut self.indexes
    }

    // Take field
    pub fn take_indexes(&mut self) -> ::protobuf::RepeatedField<super::riak::RpbPair> {
        ::std::mem::replace(&mut self.indexes, ::protobuf::RepeatedField::new())
    }

    pub fn get_indexes(&self) -> &[super::riak::RpbPair] {
        &self.indexes
    }

    fn get_indexes_for_reflect(&self) -> &::protobuf::RepeatedField<super::riak::RpbPair> {
        &self.indexes
    }

    fn mut_indexes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::riak::RpbPair> {
        &mut self.indexes
    }

    // optional bool deleted = 11;

    pub fn clear_deleted(&mut self) {
        self.deleted = ::std::option::Option::None;
    }

    pub fn has_deleted(&self) -> bool {
        self.deleted.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deleted(&mut self, v: bool) {
        self.deleted = ::std::option::Option::Some(v);
    }

    pub fn get_deleted(&self) -> bool {
        self.deleted.unwrap_or(false)
    }

    fn get_deleted_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.deleted
    }

    fn mut_deleted_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.deleted
    }
}

impl ::protobuf::Message for RpbContent {
    fn is_initialized(&self) -> bool {
        if self.value.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.content_type)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.charset)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.content_encoding)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.vtag)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.links)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.last_mod = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.last_mod_usecs = ::std::option::Option::Some(tmp);
                },
                9 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.usermeta)?;
                },
                10 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.indexes)?;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.deleted = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.value.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.content_type.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.charset.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        if let Some(v) = self.content_encoding.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        };
        if let Some(v) = self.vtag.as_ref() {
            my_size += ::protobuf::rt::bytes_size(5, &v);
        };
        for value in &self.links {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.last_mod {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.last_mod_usecs {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.usermeta {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.indexes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.deleted {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.value.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.content_type.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.charset.as_ref() {
            os.write_bytes(3, &v)?;
        };
        if let Some(v) = self.content_encoding.as_ref() {
            os.write_bytes(4, &v)?;
        };
        if let Some(v) = self.vtag.as_ref() {
            os.write_bytes(5, &v)?;
        };
        for v in &self.links {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.last_mod {
            os.write_uint32(7, v)?;
        };
        if let Some(v) = self.last_mod_usecs {
            os.write_uint32(8, v)?;
        };
        for v in &self.usermeta {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.indexes {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.deleted {
            os.write_bool(11, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbContent {
    fn new() -> RpbContent {
        RpbContent::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbContent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    RpbContent::get_value_for_reflect,
                    RpbContent::mut_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "content_type",
                    RpbContent::get_content_type_for_reflect,
                    RpbContent::mut_content_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "charset",
                    RpbContent::get_charset_for_reflect,
                    RpbContent::mut_charset_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "content_encoding",
                    RpbContent::get_content_encoding_for_reflect,
                    RpbContent::mut_content_encoding_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "vtag",
                    RpbContent::get_vtag_for_reflect,
                    RpbContent::mut_vtag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RpbLink>>(
                    "links",
                    RpbContent::get_links_for_reflect,
                    RpbContent::mut_links_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "last_mod",
                    RpbContent::get_last_mod_for_reflect,
                    RpbContent::mut_last_mod_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "last_mod_usecs",
                    RpbContent::get_last_mod_usecs_for_reflect,
                    RpbContent::mut_last_mod_usecs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::riak::RpbPair>>(
                    "usermeta",
                    RpbContent::get_usermeta_for_reflect,
                    RpbContent::mut_usermeta_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::riak::RpbPair>>(
                    "indexes",
                    RpbContent::get_indexes_for_reflect,
                    RpbContent::mut_indexes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "deleted",
                    RpbContent::get_deleted_for_reflect,
                    RpbContent::mut_deleted_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbContent>(
                    "RpbContent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbContent {
    fn clear(&mut self) {
        self.clear_value();
        self.clear_content_type();
        self.clear_charset();
        self.clear_content_encoding();
        self.clear_vtag();
        self.clear_links();
        self.clear_last_mod();
        self.clear_last_mod_usecs();
        self.clear_usermeta();
        self.clear_indexes();
        self.clear_deleted();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbContent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbContent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbLink {
    // message fields
    bucket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    tag: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbLink {}

impl RpbLink {
    pub fn new() -> RpbLink {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbLink {
        static mut instance: ::protobuf::lazy::Lazy<RpbLink> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbLink,
        };
        unsafe {
            instance.get(RpbLink::new)
        }
    }

    // optional bytes bucket = 1;

    pub fn clear_bucket(&mut self) {
        self.bucket.clear();
    }

    pub fn has_bucket(&self) -> bool {
        self.bucket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bucket(&mut self, v: ::std::vec::Vec<u8>) {
        self.bucket = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bucket(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.bucket.is_none() {
            self.bucket.set_default();
        };
        self.bucket.as_mut().unwrap()
    }

    // Take field
    pub fn take_bucket(&mut self) -> ::std::vec::Vec<u8> {
        self.bucket.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_bucket(&self) -> &[u8] {
        match self.bucket.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_bucket_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.bucket
    }

    fn mut_bucket_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.bucket
    }

    // optional bytes key = 2;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.key
    }

    // optional bytes tag = 3;

    pub fn clear_tag(&mut self) {
        self.tag.clear();
    }

    pub fn has_tag(&self) -> bool {
        self.tag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tag(&mut self, v: ::std::vec::Vec<u8>) {
        self.tag = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tag(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.tag.is_none() {
            self.tag.set_default();
        };
        self.tag.as_mut().unwrap()
    }

    // Take field
    pub fn take_tag(&mut self) -> ::std::vec::Vec<u8> {
        self.tag.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_tag(&self) -> &[u8] {
        match self.tag.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_tag_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.tag
    }

    fn mut_tag_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.tag
    }
}

impl ::protobuf::Message for RpbLink {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bucket)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.tag)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.bucket.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.tag.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bucket.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.key.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.tag.as_ref() {
            os.write_bytes(3, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbLink {
    fn new() -> RpbLink {
        RpbLink::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbLink>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "bucket",
                    RpbLink::get_bucket_for_reflect,
                    RpbLink::mut_bucket_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    RpbLink::get_key_for_reflect,
                    RpbLink::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "tag",
                    RpbLink::get_tag_for_reflect,
                    RpbLink::mut_tag_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbLink>(
                    "RpbLink",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbLink {
    fn clear(&mut self) {
        self.clear_bucket();
        self.clear_key();
        self.clear_tag();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbLink {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbLink {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbCounterUpdateReq {
    // message fields
    bucket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    amount: ::std::option::Option<i64>,
    w: ::std::option::Option<u32>,
    dw: ::std::option::Option<u32>,
    pw: ::std::option::Option<u32>,
    returnvalue: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbCounterUpdateReq {}

impl RpbCounterUpdateReq {
    pub fn new() -> RpbCounterUpdateReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbCounterUpdateReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbCounterUpdateReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbCounterUpdateReq,
        };
        unsafe {
            instance.get(RpbCounterUpdateReq::new)
        }
    }

    // required bytes bucket = 1;

    pub fn clear_bucket(&mut self) {
        self.bucket.clear();
    }

    pub fn has_bucket(&self) -> bool {
        self.bucket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bucket(&mut self, v: ::std::vec::Vec<u8>) {
        self.bucket = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bucket(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.bucket.is_none() {
            self.bucket.set_default();
        };
        self.bucket.as_mut().unwrap()
    }

    // Take field
    pub fn take_bucket(&mut self) -> ::std::vec::Vec<u8> {
        self.bucket.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_bucket(&self) -> &[u8] {
        match self.bucket.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_bucket_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.bucket
    }

    fn mut_bucket_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.bucket
    }

    // required bytes key = 2;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.key
    }

    // required sint64 amount = 3;

    pub fn clear_amount(&mut self) {
        self.amount = ::std::option::Option::None;
    }

    pub fn has_amount(&self) -> bool {
        self.amount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_amount(&mut self, v: i64) {
        self.amount = ::std::option::Option::Some(v);
    }

    pub fn get_amount(&self) -> i64 {
        self.amount.unwrap_or(0)
    }

    fn get_amount_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.amount
    }

    fn mut_amount_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.amount
    }

    // optional uint32 w = 4;

    pub fn clear_w(&mut self) {
        self.w = ::std::option::Option::None;
    }

    pub fn has_w(&self) -> bool {
        self.w.is_some()
    }

    // Param is passed by value, moved
    pub fn set_w(&mut self, v: u32) {
        self.w = ::std::option::Option::Some(v);
    }

    pub fn get_w(&self) -> u32 {
        self.w.unwrap_or(0)
    }

    fn get_w_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.w
    }

    fn mut_w_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.w
    }

    // optional uint32 dw = 5;

    pub fn clear_dw(&mut self) {
        self.dw = ::std::option::Option::None;
    }

    pub fn has_dw(&self) -> bool {
        self.dw.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dw(&mut self, v: u32) {
        self.dw = ::std::option::Option::Some(v);
    }

    pub fn get_dw(&self) -> u32 {
        self.dw.unwrap_or(0)
    }

    fn get_dw_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.dw
    }

    fn mut_dw_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.dw
    }

    // optional uint32 pw = 6;

    pub fn clear_pw(&mut self) {
        self.pw = ::std::option::Option::None;
    }

    pub fn has_pw(&self) -> bool {
        self.pw.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pw(&mut self, v: u32) {
        self.pw = ::std::option::Option::Some(v);
    }

    pub fn get_pw(&self) -> u32 {
        self.pw.unwrap_or(0)
    }

    fn get_pw_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.pw
    }

    fn mut_pw_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.pw
    }

    // optional bool returnvalue = 7;

    pub fn clear_returnvalue(&mut self) {
        self.returnvalue = ::std::option::Option::None;
    }

    pub fn has_returnvalue(&self) -> bool {
        self.returnvalue.is_some()
    }

    // Param is passed by value, moved
    pub fn set_returnvalue(&mut self, v: bool) {
        self.returnvalue = ::std::option::Option::Some(v);
    }

    pub fn get_returnvalue(&self) -> bool {
        self.returnvalue.unwrap_or(false)
    }

    fn get_returnvalue_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.returnvalue
    }

    fn mut_returnvalue_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.returnvalue
    }
}

impl ::protobuf::Message for RpbCounterUpdateReq {
    fn is_initialized(&self) -> bool {
        if self.bucket.is_none() {
            return false;
        };
        if self.key.is_none() {
            return false;
        };
        if self.amount.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bucket)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_sint64()?;
                    self.amount = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.w = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.dw = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.pw = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.returnvalue = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.bucket.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.amount {
            my_size += ::protobuf::rt::value_varint_zigzag_size(3, v);
        };
        if let Some(v) = self.w {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.dw {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.pw {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.returnvalue {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bucket.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.key.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.amount {
            os.write_sint64(3, v)?;
        };
        if let Some(v) = self.w {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.dw {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.pw {
            os.write_uint32(6, v)?;
        };
        if let Some(v) = self.returnvalue {
            os.write_bool(7, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbCounterUpdateReq {
    fn new() -> RpbCounterUpdateReq {
        RpbCounterUpdateReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbCounterUpdateReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "bucket",
                    RpbCounterUpdateReq::get_bucket_for_reflect,
                    RpbCounterUpdateReq::mut_bucket_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    RpbCounterUpdateReq::get_key_for_reflect,
                    RpbCounterUpdateReq::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint64>(
                    "amount",
                    RpbCounterUpdateReq::get_amount_for_reflect,
                    RpbCounterUpdateReq::mut_amount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "w",
                    RpbCounterUpdateReq::get_w_for_reflect,
                    RpbCounterUpdateReq::mut_w_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "dw",
                    RpbCounterUpdateReq::get_dw_for_reflect,
                    RpbCounterUpdateReq::mut_dw_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "pw",
                    RpbCounterUpdateReq::get_pw_for_reflect,
                    RpbCounterUpdateReq::mut_pw_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "returnvalue",
                    RpbCounterUpdateReq::get_returnvalue_for_reflect,
                    RpbCounterUpdateReq::mut_returnvalue_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbCounterUpdateReq>(
                    "RpbCounterUpdateReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbCounterUpdateReq {
    fn clear(&mut self) {
        self.clear_bucket();
        self.clear_key();
        self.clear_amount();
        self.clear_w();
        self.clear_dw();
        self.clear_pw();
        self.clear_returnvalue();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbCounterUpdateReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbCounterUpdateReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbCounterUpdateResp {
    // message fields
    value: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbCounterUpdateResp {}

impl RpbCounterUpdateResp {
    pub fn new() -> RpbCounterUpdateResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbCounterUpdateResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbCounterUpdateResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbCounterUpdateResp,
        };
        unsafe {
            instance.get(RpbCounterUpdateResp::new)
        }
    }

    // optional sint64 value = 1;

    pub fn clear_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: i64) {
        self.value = ::std::option::Option::Some(v);
    }

    pub fn get_value(&self) -> i64 {
        self.value.unwrap_or(0)
    }

    fn get_value_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.value
    }
}

impl ::protobuf::Message for RpbCounterUpdateResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_sint64()?;
                    self.value = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.value {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.value {
            os.write_sint64(1, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbCounterUpdateResp {
    fn new() -> RpbCounterUpdateResp {
        RpbCounterUpdateResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbCounterUpdateResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint64>(
                    "value",
                    RpbCounterUpdateResp::get_value_for_reflect,
                    RpbCounterUpdateResp::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbCounterUpdateResp>(
                    "RpbCounterUpdateResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbCounterUpdateResp {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbCounterUpdateResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbCounterUpdateResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbCounterGetReq {
    // message fields
    bucket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    r: ::std::option::Option<u32>,
    pr: ::std::option::Option<u32>,
    basic_quorum: ::std::option::Option<bool>,
    notfound_ok: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbCounterGetReq {}

impl RpbCounterGetReq {
    pub fn new() -> RpbCounterGetReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbCounterGetReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbCounterGetReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbCounterGetReq,
        };
        unsafe {
            instance.get(RpbCounterGetReq::new)
        }
    }

    // required bytes bucket = 1;

    pub fn clear_bucket(&mut self) {
        self.bucket.clear();
    }

    pub fn has_bucket(&self) -> bool {
        self.bucket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bucket(&mut self, v: ::std::vec::Vec<u8>) {
        self.bucket = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bucket(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.bucket.is_none() {
            self.bucket.set_default();
        };
        self.bucket.as_mut().unwrap()
    }

    // Take field
    pub fn take_bucket(&mut self) -> ::std::vec::Vec<u8> {
        self.bucket.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_bucket(&self) -> &[u8] {
        match self.bucket.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_bucket_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.bucket
    }

    fn mut_bucket_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.bucket
    }

    // required bytes key = 2;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.key
    }

    // optional uint32 r = 3;

    pub fn clear_r(&mut self) {
        self.r = ::std::option::Option::None;
    }

    pub fn has_r(&self) -> bool {
        self.r.is_some()
    }

    // Param is passed by value, moved
    pub fn set_r(&mut self, v: u32) {
        self.r = ::std::option::Option::Some(v);
    }

    pub fn get_r(&self) -> u32 {
        self.r.unwrap_or(0)
    }

    fn get_r_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.r
    }

    fn mut_r_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.r
    }

    // optional uint32 pr = 4;

    pub fn clear_pr(&mut self) {
        self.pr = ::std::option::Option::None;
    }

    pub fn has_pr(&self) -> bool {
        self.pr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pr(&mut self, v: u32) {
        self.pr = ::std::option::Option::Some(v);
    }

    pub fn get_pr(&self) -> u32 {
        self.pr.unwrap_or(0)
    }

    fn get_pr_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.pr
    }

    fn mut_pr_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.pr
    }

    // optional bool basic_quorum = 5;

    pub fn clear_basic_quorum(&mut self) {
        self.basic_quorum = ::std::option::Option::None;
    }

    pub fn has_basic_quorum(&self) -> bool {
        self.basic_quorum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_basic_quorum(&mut self, v: bool) {
        self.basic_quorum = ::std::option::Option::Some(v);
    }

    pub fn get_basic_quorum(&self) -> bool {
        self.basic_quorum.unwrap_or(false)
    }

    fn get_basic_quorum_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.basic_quorum
    }

    fn mut_basic_quorum_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.basic_quorum
    }

    // optional bool notfound_ok = 6;

    pub fn clear_notfound_ok(&mut self) {
        self.notfound_ok = ::std::option::Option::None;
    }

    pub fn has_notfound_ok(&self) -> bool {
        self.notfound_ok.is_some()
    }

    // Param is passed by value, moved
    pub fn set_notfound_ok(&mut self, v: bool) {
        self.notfound_ok = ::std::option::Option::Some(v);
    }

    pub fn get_notfound_ok(&self) -> bool {
        self.notfound_ok.unwrap_or(false)
    }

    fn get_notfound_ok_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.notfound_ok
    }

    fn mut_notfound_ok_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.notfound_ok
    }
}

impl ::protobuf::Message for RpbCounterGetReq {
    fn is_initialized(&self) -> bool {
        if self.bucket.is_none() {
            return false;
        };
        if self.key.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bucket)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.r = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.pr = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.basic_quorum = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.notfound_ok = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.bucket.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.r {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.pr {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.basic_quorum {
            my_size += 2;
        };
        if let Some(v) = self.notfound_ok {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bucket.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.key.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.r {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.pr {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.basic_quorum {
            os.write_bool(5, v)?;
        };
        if let Some(v) = self.notfound_ok {
            os.write_bool(6, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbCounterGetReq {
    fn new() -> RpbCounterGetReq {
        RpbCounterGetReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbCounterGetReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "bucket",
                    RpbCounterGetReq::get_bucket_for_reflect,
                    RpbCounterGetReq::mut_bucket_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    RpbCounterGetReq::get_key_for_reflect,
                    RpbCounterGetReq::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "r",
                    RpbCounterGetReq::get_r_for_reflect,
                    RpbCounterGetReq::mut_r_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "pr",
                    RpbCounterGetReq::get_pr_for_reflect,
                    RpbCounterGetReq::mut_pr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "basic_quorum",
                    RpbCounterGetReq::get_basic_quorum_for_reflect,
                    RpbCounterGetReq::mut_basic_quorum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "notfound_ok",
                    RpbCounterGetReq::get_notfound_ok_for_reflect,
                    RpbCounterGetReq::mut_notfound_ok_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbCounterGetReq>(
                    "RpbCounterGetReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbCounterGetReq {
    fn clear(&mut self) {
        self.clear_bucket();
        self.clear_key();
        self.clear_r();
        self.clear_pr();
        self.clear_basic_quorum();
        self.clear_notfound_ok();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbCounterGetReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbCounterGetReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbCounterGetResp {
    // message fields
    value: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbCounterGetResp {}

impl RpbCounterGetResp {
    pub fn new() -> RpbCounterGetResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbCounterGetResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbCounterGetResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbCounterGetResp,
        };
        unsafe {
            instance.get(RpbCounterGetResp::new)
        }
    }

    // optional sint64 value = 1;

    pub fn clear_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: i64) {
        self.value = ::std::option::Option::Some(v);
    }

    pub fn get_value(&self) -> i64 {
        self.value.unwrap_or(0)
    }

    fn get_value_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.value
    }
}

impl ::protobuf::Message for RpbCounterGetResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_sint64()?;
                    self.value = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.value {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.value {
            os.write_sint64(1, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbCounterGetResp {
    fn new() -> RpbCounterGetResp {
        RpbCounterGetResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbCounterGetResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint64>(
                    "value",
                    RpbCounterGetResp::get_value_for_reflect,
                    RpbCounterGetResp::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbCounterGetResp>(
                    "RpbCounterGetResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbCounterGetResp {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbCounterGetResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbCounterGetResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbGetBucketKeyPreflistReq {
    // message fields
    bucket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    field_type: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbGetBucketKeyPreflistReq {}

impl RpbGetBucketKeyPreflistReq {
    pub fn new() -> RpbGetBucketKeyPreflistReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbGetBucketKeyPreflistReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbGetBucketKeyPreflistReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbGetBucketKeyPreflistReq,
        };
        unsafe {
            instance.get(RpbGetBucketKeyPreflistReq::new)
        }
    }

    // required bytes bucket = 1;

    pub fn clear_bucket(&mut self) {
        self.bucket.clear();
    }

    pub fn has_bucket(&self) -> bool {
        self.bucket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bucket(&mut self, v: ::std::vec::Vec<u8>) {
        self.bucket = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bucket(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.bucket.is_none() {
            self.bucket.set_default();
        };
        self.bucket.as_mut().unwrap()
    }

    // Take field
    pub fn take_bucket(&mut self) -> ::std::vec::Vec<u8> {
        self.bucket.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_bucket(&self) -> &[u8] {
        match self.bucket.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_bucket_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.bucket
    }

    fn mut_bucket_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.bucket
    }

    // required bytes key = 2;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.key
    }

    // optional bytes type = 3;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::vec::Vec<u8>) {
        self.field_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.field_type.is_none() {
            self.field_type.set_default();
        };
        self.field_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::vec::Vec<u8> {
        self.field_type.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_field_type(&self) -> &[u8] {
        match self.field_type.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_field_type_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.field_type
    }
}

impl ::protobuf::Message for RpbGetBucketKeyPreflistReq {
    fn is_initialized(&self) -> bool {
        if self.bucket.is_none() {
            return false;
        };
        if self.key.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bucket)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.field_type)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.bucket.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.field_type.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bucket.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.key.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.field_type.as_ref() {
            os.write_bytes(3, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbGetBucketKeyPreflistReq {
    fn new() -> RpbGetBucketKeyPreflistReq {
        RpbGetBucketKeyPreflistReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbGetBucketKeyPreflistReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "bucket",
                    RpbGetBucketKeyPreflistReq::get_bucket_for_reflect,
                    RpbGetBucketKeyPreflistReq::mut_bucket_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    RpbGetBucketKeyPreflistReq::get_key_for_reflect,
                    RpbGetBucketKeyPreflistReq::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "type",
                    RpbGetBucketKeyPreflistReq::get_field_type_for_reflect,
                    RpbGetBucketKeyPreflistReq::mut_field_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbGetBucketKeyPreflistReq>(
                    "RpbGetBucketKeyPreflistReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbGetBucketKeyPreflistReq {
    fn clear(&mut self) {
        self.clear_bucket();
        self.clear_key();
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbGetBucketKeyPreflistReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbGetBucketKeyPreflistReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbGetBucketKeyPreflistResp {
    // message fields
    preflist: ::protobuf::RepeatedField<RpbBucketKeyPreflistItem>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbGetBucketKeyPreflistResp {}

impl RpbGetBucketKeyPreflistResp {
    pub fn new() -> RpbGetBucketKeyPreflistResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbGetBucketKeyPreflistResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbGetBucketKeyPreflistResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbGetBucketKeyPreflistResp,
        };
        unsafe {
            instance.get(RpbGetBucketKeyPreflistResp::new)
        }
    }

    // repeated .RpbBucketKeyPreflistItem preflist = 1;

    pub fn clear_preflist(&mut self) {
        self.preflist.clear();
    }

    // Param is passed by value, moved
    pub fn set_preflist(&mut self, v: ::protobuf::RepeatedField<RpbBucketKeyPreflistItem>) {
        self.preflist = v;
    }

    // Mutable pointer to the field.
    pub fn mut_preflist(&mut self) -> &mut ::protobuf::RepeatedField<RpbBucketKeyPreflistItem> {
        &mut self.preflist
    }

    // Take field
    pub fn take_preflist(&mut self) -> ::protobuf::RepeatedField<RpbBucketKeyPreflistItem> {
        ::std::mem::replace(&mut self.preflist, ::protobuf::RepeatedField::new())
    }

    pub fn get_preflist(&self) -> &[RpbBucketKeyPreflistItem] {
        &self.preflist
    }

    fn get_preflist_for_reflect(&self) -> &::protobuf::RepeatedField<RpbBucketKeyPreflistItem> {
        &self.preflist
    }

    fn mut_preflist_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RpbBucketKeyPreflistItem> {
        &mut self.preflist
    }
}

impl ::protobuf::Message for RpbGetBucketKeyPreflistResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.preflist)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.preflist {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.preflist {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbGetBucketKeyPreflistResp {
    fn new() -> RpbGetBucketKeyPreflistResp {
        RpbGetBucketKeyPreflistResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbGetBucketKeyPreflistResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RpbBucketKeyPreflistItem>>(
                    "preflist",
                    RpbGetBucketKeyPreflistResp::get_preflist_for_reflect,
                    RpbGetBucketKeyPreflistResp::mut_preflist_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbGetBucketKeyPreflistResp>(
                    "RpbGetBucketKeyPreflistResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbGetBucketKeyPreflistResp {
    fn clear(&mut self) {
        self.clear_preflist();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbGetBucketKeyPreflistResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbGetBucketKeyPreflistResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbBucketKeyPreflistItem {
    // message fields
    partition: ::std::option::Option<i64>,
    node: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    primary: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbBucketKeyPreflistItem {}

impl RpbBucketKeyPreflistItem {
    pub fn new() -> RpbBucketKeyPreflistItem {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbBucketKeyPreflistItem {
        static mut instance: ::protobuf::lazy::Lazy<RpbBucketKeyPreflistItem> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbBucketKeyPreflistItem,
        };
        unsafe {
            instance.get(RpbBucketKeyPreflistItem::new)
        }
    }

    // required int64 partition = 1;

    pub fn clear_partition(&mut self) {
        self.partition = ::std::option::Option::None;
    }

    pub fn has_partition(&self) -> bool {
        self.partition.is_some()
    }

    // Param is passed by value, moved
    pub fn set_partition(&mut self, v: i64) {
        self.partition = ::std::option::Option::Some(v);
    }

    pub fn get_partition(&self) -> i64 {
        self.partition.unwrap_or(0)
    }

    fn get_partition_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.partition
    }

    fn mut_partition_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.partition
    }

    // required bytes node = 2;

    pub fn clear_node(&mut self) {
        self.node.clear();
    }

    pub fn has_node(&self) -> bool {
        self.node.is_some()
    }

    // Param is passed by value, moved
    pub fn set_node(&mut self, v: ::std::vec::Vec<u8>) {
        self.node = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_node(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.node.is_none() {
            self.node.set_default();
        };
        self.node.as_mut().unwrap()
    }

    // Take field
    pub fn take_node(&mut self) -> ::std::vec::Vec<u8> {
        self.node.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_node(&self) -> &[u8] {
        match self.node.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_node_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.node
    }

    fn mut_node_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.node
    }

    // required bool primary = 3;

    pub fn clear_primary(&mut self) {
        self.primary = ::std::option::Option::None;
    }

    pub fn has_primary(&self) -> bool {
        self.primary.is_some()
    }

    // Param is passed by value, moved
    pub fn set_primary(&mut self, v: bool) {
        self.primary = ::std::option::Option::Some(v);
    }

    pub fn get_primary(&self) -> bool {
        self.primary.unwrap_or(false)
    }

    fn get_primary_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.primary
    }

    fn mut_primary_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.primary
    }
}

impl ::protobuf::Message for RpbBucketKeyPreflistItem {
    fn is_initialized(&self) -> bool {
        if self.partition.is_none() {
            return false;
        };
        if self.node.is_none() {
            return false;
        };
        if self.primary.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int64()?;
                    self.partition = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.node)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.primary = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.partition {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.node.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.primary {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.partition {
            os.write_int64(1, v)?;
        };
        if let Some(v) = self.node.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.primary {
            os.write_bool(3, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbBucketKeyPreflistItem {
    fn new() -> RpbBucketKeyPreflistItem {
        RpbBucketKeyPreflistItem::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbBucketKeyPreflistItem>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "partition",
                    RpbBucketKeyPreflistItem::get_partition_for_reflect,
                    RpbBucketKeyPreflistItem::mut_partition_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "node",
                    RpbBucketKeyPreflistItem::get_node_for_reflect,
                    RpbBucketKeyPreflistItem::mut_node_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "primary",
                    RpbBucketKeyPreflistItem::get_primary_for_reflect,
                    RpbBucketKeyPreflistItem::mut_primary_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbBucketKeyPreflistItem>(
                    "RpbBucketKeyPreflistItem",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbBucketKeyPreflistItem {
    fn clear(&mut self) {
        self.clear_partition();
        self.clear_node();
        self.clear_primary();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbBucketKeyPreflistItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbBucketKeyPreflistItem {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbCoverageReq {
    // message fields
    field_type: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    bucket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    min_partitions: ::std::option::Option<u32>,
    replace_cover: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    unavailable_cover: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbCoverageReq {}

impl RpbCoverageReq {
    pub fn new() -> RpbCoverageReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbCoverageReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbCoverageReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbCoverageReq,
        };
        unsafe {
            instance.get(RpbCoverageReq::new)
        }
    }

    // optional bytes type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::vec::Vec<u8>) {
        self.field_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.field_type.is_none() {
            self.field_type.set_default();
        };
        self.field_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::vec::Vec<u8> {
        self.field_type.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_field_type(&self) -> &[u8] {
        match self.field_type.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_field_type_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.field_type
    }

    // required bytes bucket = 2;

    pub fn clear_bucket(&mut self) {
        self.bucket.clear();
    }

    pub fn has_bucket(&self) -> bool {
        self.bucket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bucket(&mut self, v: ::std::vec::Vec<u8>) {
        self.bucket = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bucket(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.bucket.is_none() {
            self.bucket.set_default();
        };
        self.bucket.as_mut().unwrap()
    }

    // Take field
    pub fn take_bucket(&mut self) -> ::std::vec::Vec<u8> {
        self.bucket.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_bucket(&self) -> &[u8] {
        match self.bucket.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_bucket_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.bucket
    }

    fn mut_bucket_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.bucket
    }

    // optional uint32 min_partitions = 3;

    pub fn clear_min_partitions(&mut self) {
        self.min_partitions = ::std::option::Option::None;
    }

    pub fn has_min_partitions(&self) -> bool {
        self.min_partitions.is_some()
    }

    // Param is passed by value, moved
    pub fn set_min_partitions(&mut self, v: u32) {
        self.min_partitions = ::std::option::Option::Some(v);
    }

    pub fn get_min_partitions(&self) -> u32 {
        self.min_partitions.unwrap_or(0)
    }

    fn get_min_partitions_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.min_partitions
    }

    fn mut_min_partitions_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.min_partitions
    }

    // optional bytes replace_cover = 4;

    pub fn clear_replace_cover(&mut self) {
        self.replace_cover.clear();
    }

    pub fn has_replace_cover(&self) -> bool {
        self.replace_cover.is_some()
    }

    // Param is passed by value, moved
    pub fn set_replace_cover(&mut self, v: ::std::vec::Vec<u8>) {
        self.replace_cover = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_replace_cover(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.replace_cover.is_none() {
            self.replace_cover.set_default();
        };
        self.replace_cover.as_mut().unwrap()
    }

    // Take field
    pub fn take_replace_cover(&mut self) -> ::std::vec::Vec<u8> {
        self.replace_cover.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_replace_cover(&self) -> &[u8] {
        match self.replace_cover.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_replace_cover_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.replace_cover
    }

    fn mut_replace_cover_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.replace_cover
    }

    // repeated bytes unavailable_cover = 5;

    pub fn clear_unavailable_cover(&mut self) {
        self.unavailable_cover.clear();
    }

    // Param is passed by value, moved
    pub fn set_unavailable_cover(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.unavailable_cover = v;
    }

    // Mutable pointer to the field.
    pub fn mut_unavailable_cover(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.unavailable_cover
    }

    // Take field
    pub fn take_unavailable_cover(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.unavailable_cover, ::protobuf::RepeatedField::new())
    }

    pub fn get_unavailable_cover(&self) -> &[::std::vec::Vec<u8>] {
        &self.unavailable_cover
    }

    fn get_unavailable_cover_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.unavailable_cover
    }

    fn mut_unavailable_cover_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.unavailable_cover
    }
}

impl ::protobuf::Message for RpbCoverageReq {
    fn is_initialized(&self) -> bool {
        if self.bucket.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.field_type)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bucket)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.min_partitions = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.replace_cover)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.unavailable_cover)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.field_type.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.bucket.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.min_partitions {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.replace_cover.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        };
        for value in &self.unavailable_cover {
            my_size += ::protobuf::rt::bytes_size(5, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.bucket.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.min_partitions {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.replace_cover.as_ref() {
            os.write_bytes(4, &v)?;
        };
        for v in &self.unavailable_cover {
            os.write_bytes(5, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbCoverageReq {
    fn new() -> RpbCoverageReq {
        RpbCoverageReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbCoverageReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "type",
                    RpbCoverageReq::get_field_type_for_reflect,
                    RpbCoverageReq::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "bucket",
                    RpbCoverageReq::get_bucket_for_reflect,
                    RpbCoverageReq::mut_bucket_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "min_partitions",
                    RpbCoverageReq::get_min_partitions_for_reflect,
                    RpbCoverageReq::mut_min_partitions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "replace_cover",
                    RpbCoverageReq::get_replace_cover_for_reflect,
                    RpbCoverageReq::mut_replace_cover_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "unavailable_cover",
                    RpbCoverageReq::get_unavailable_cover_for_reflect,
                    RpbCoverageReq::mut_unavailable_cover_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbCoverageReq>(
                    "RpbCoverageReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbCoverageReq {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_bucket();
        self.clear_min_partitions();
        self.clear_replace_cover();
        self.clear_unavailable_cover();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbCoverageReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbCoverageReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbCoverageResp {
    // message fields
    entries: ::protobuf::RepeatedField<RpbCoverageEntry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbCoverageResp {}

impl RpbCoverageResp {
    pub fn new() -> RpbCoverageResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbCoverageResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbCoverageResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbCoverageResp,
        };
        unsafe {
            instance.get(RpbCoverageResp::new)
        }
    }

    // repeated .RpbCoverageEntry entries = 1;

    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }

    // Param is passed by value, moved
    pub fn set_entries(&mut self, v: ::protobuf::RepeatedField<RpbCoverageEntry>) {
        self.entries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_entries(&mut self) -> &mut ::protobuf::RepeatedField<RpbCoverageEntry> {
        &mut self.entries
    }

    // Take field
    pub fn take_entries(&mut self) -> ::protobuf::RepeatedField<RpbCoverageEntry> {
        ::std::mem::replace(&mut self.entries, ::protobuf::RepeatedField::new())
    }

    pub fn get_entries(&self) -> &[RpbCoverageEntry] {
        &self.entries
    }

    fn get_entries_for_reflect(&self) -> &::protobuf::RepeatedField<RpbCoverageEntry> {
        &self.entries
    }

    fn mut_entries_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RpbCoverageEntry> {
        &mut self.entries
    }
}

impl ::protobuf::Message for RpbCoverageResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.entries)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.entries {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.entries {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbCoverageResp {
    fn new() -> RpbCoverageResp {
        RpbCoverageResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbCoverageResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RpbCoverageEntry>>(
                    "entries",
                    RpbCoverageResp::get_entries_for_reflect,
                    RpbCoverageResp::mut_entries_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbCoverageResp>(
                    "RpbCoverageResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbCoverageResp {
    fn clear(&mut self) {
        self.clear_entries();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbCoverageResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbCoverageResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbCoverageEntry {
    // message fields
    ip: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    port: ::std::option::Option<u32>,
    keyspace_desc: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    cover_context: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbCoverageEntry {}

impl RpbCoverageEntry {
    pub fn new() -> RpbCoverageEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbCoverageEntry {
        static mut instance: ::protobuf::lazy::Lazy<RpbCoverageEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbCoverageEntry,
        };
        unsafe {
            instance.get(RpbCoverageEntry::new)
        }
    }

    // required bytes ip = 1;

    pub fn clear_ip(&mut self) {
        self.ip.clear();
    }

    pub fn has_ip(&self) -> bool {
        self.ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ip(&mut self, v: ::std::vec::Vec<u8>) {
        self.ip = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ip(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.ip.is_none() {
            self.ip.set_default();
        };
        self.ip.as_mut().unwrap()
    }

    // Take field
    pub fn take_ip(&mut self) -> ::std::vec::Vec<u8> {
        self.ip.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_ip(&self) -> &[u8] {
        match self.ip.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_ip_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.ip
    }

    fn mut_ip_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.ip
    }

    // required uint32 port = 2;

    pub fn clear_port(&mut self) {
        self.port = ::std::option::Option::None;
    }

    pub fn has_port(&self) -> bool {
        self.port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_port(&mut self, v: u32) {
        self.port = ::std::option::Option::Some(v);
    }

    pub fn get_port(&self) -> u32 {
        self.port.unwrap_or(0)
    }

    fn get_port_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.port
    }

    fn mut_port_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.port
    }

    // optional bytes keyspace_desc = 3;

    pub fn clear_keyspace_desc(&mut self) {
        self.keyspace_desc.clear();
    }

    pub fn has_keyspace_desc(&self) -> bool {
        self.keyspace_desc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_keyspace_desc(&mut self, v: ::std::vec::Vec<u8>) {
        self.keyspace_desc = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_keyspace_desc(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.keyspace_desc.is_none() {
            self.keyspace_desc.set_default();
        };
        self.keyspace_desc.as_mut().unwrap()
    }

    // Take field
    pub fn take_keyspace_desc(&mut self) -> ::std::vec::Vec<u8> {
        self.keyspace_desc.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_keyspace_desc(&self) -> &[u8] {
        match self.keyspace_desc.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_keyspace_desc_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.keyspace_desc
    }

    fn mut_keyspace_desc_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.keyspace_desc
    }

    // required bytes cover_context = 4;

    pub fn clear_cover_context(&mut self) {
        self.cover_context.clear();
    }

    pub fn has_cover_context(&self) -> bool {
        self.cover_context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cover_context(&mut self, v: ::std::vec::Vec<u8>) {
        self.cover_context = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cover_context(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.cover_context.is_none() {
            self.cover_context.set_default();
        };
        self.cover_context.as_mut().unwrap()
    }

    // Take field
    pub fn take_cover_context(&mut self) -> ::std::vec::Vec<u8> {
        self.cover_context.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_cover_context(&self) -> &[u8] {
        match self.cover_context.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_cover_context_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.cover_context
    }

    fn mut_cover_context_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.cover_context
    }
}

impl ::protobuf::Message for RpbCoverageEntry {
    fn is_initialized(&self) -> bool {
        if self.ip.is_none() {
            return false;
        };
        if self.port.is_none() {
            return false;
        };
        if self.cover_context.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.ip)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.port = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.keyspace_desc)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.cover_context)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.ip.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.port {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.keyspace_desc.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        if let Some(v) = self.cover_context.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ip.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.port {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.keyspace_desc.as_ref() {
            os.write_bytes(3, &v)?;
        };
        if let Some(v) = self.cover_context.as_ref() {
            os.write_bytes(4, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbCoverageEntry {
    fn new() -> RpbCoverageEntry {
        RpbCoverageEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbCoverageEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "ip",
                    RpbCoverageEntry::get_ip_for_reflect,
                    RpbCoverageEntry::mut_ip_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "port",
                    RpbCoverageEntry::get_port_for_reflect,
                    RpbCoverageEntry::mut_port_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "keyspace_desc",
                    RpbCoverageEntry::get_keyspace_desc_for_reflect,
                    RpbCoverageEntry::mut_keyspace_desc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "cover_context",
                    RpbCoverageEntry::get_cover_context_for_reflect,
                    RpbCoverageEntry::mut_cover_context_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbCoverageEntry>(
                    "RpbCoverageEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbCoverageEntry {
    fn clear(&mut self) {
        self.clear_ip();
        self.clear_port();
        self.clear_keyspace_desc();
        self.clear_cover_context();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbCoverageEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbCoverageEntry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0d, 0x72, 0x69, 0x61, 0x6b, 0x5f, 0x6b, 0x76, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a,
    0x0a, 0x72, 0x69, 0x61, 0x6b, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x27, 0x0a, 0x12, 0x52,
    0x70, 0x62, 0x47, 0x65, 0x74, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x52, 0x65, 0x73,
    0x70, 0x12, 0x11, 0x0a, 0x09, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x0c, 0x22, 0x26, 0x0a, 0x11, 0x52, 0x70, 0x62, 0x53, 0x65, 0x74, 0x43, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x52, 0x65, 0x71, 0x12, 0x11, 0x0a, 0x09, 0x63, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x22, 0xe9, 0x01, 0x0a,
    0x09, 0x52, 0x70, 0x62, 0x47, 0x65, 0x74, 0x52, 0x65, 0x71, 0x12, 0x0e, 0x0a, 0x06, 0x62, 0x75,
    0x63, 0x6b, 0x65, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65,
    0x79, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x09, 0x0a, 0x01, 0x72, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0d, 0x12, 0x0a, 0x0a, 0x02, 0x70, 0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x14,
    0x0a, 0x0c, 0x62, 0x61, 0x73, 0x69, 0x63, 0x5f, 0x71, 0x75, 0x6f, 0x72, 0x75, 0x6d, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x08, 0x12, 0x13, 0x0a, 0x0b, 0x6e, 0x6f, 0x74, 0x66, 0x6f, 0x75, 0x6e, 0x64,
    0x5f, 0x6f, 0x6b, 0x18, 0x06, 0x20, 0x01, 0x28, 0x08, 0x12, 0x13, 0x0a, 0x0b, 0x69, 0x66, 0x5f,
    0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0c,
    0x0a, 0x04, 0x68, 0x65, 0x61, 0x64, 0x18, 0x08, 0x20, 0x01, 0x28, 0x08, 0x12, 0x15, 0x0a, 0x0d,
    0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x76, 0x63, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x09, 0x20,
    0x01, 0x28, 0x08, 0x12, 0x0f, 0x0a, 0x07, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x18, 0x0a,
    0x20, 0x01, 0x28, 0x0d, 0x12, 0x15, 0x0a, 0x0d, 0x73, 0x6c, 0x6f, 0x70, 0x70, 0x79, 0x5f, 0x71,
    0x75, 0x6f, 0x72, 0x75, 0x6d, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x08, 0x12, 0x0d, 0x0a, 0x05, 0x6e,
    0x5f, 0x76, 0x61, 0x6c, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x79,
    0x70, 0x65, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x4d, 0x0a, 0x0a, 0x52, 0x70, 0x62, 0x47,
    0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x12, 0x1c, 0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e,
    0x74, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x52, 0x70, 0x62, 0x43, 0x6f, 0x6e,
    0x74, 0x65, 0x6e, 0x74, 0x12, 0x0e, 0x0a, 0x06, 0x76, 0x63, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0c, 0x12, 0x11, 0x0a, 0x09, 0x75, 0x6e, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65,
    0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x22, 0xa6, 0x02, 0x0a, 0x09, 0x52, 0x70, 0x62, 0x50,
    0x75, 0x74, 0x52, 0x65, 0x71, 0x12, 0x0e, 0x0a, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0c, 0x12, 0x0e, 0x0a, 0x06, 0x76, 0x63, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0c, 0x12, 0x1c, 0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x18, 0x04, 0x20,
    0x02, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x52, 0x70, 0x62, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74,
    0x12, 0x09, 0x0a, 0x01, 0x77, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0a, 0x0a, 0x02, 0x64,
    0x77, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x13, 0x0a, 0x0b, 0x72, 0x65, 0x74, 0x75, 0x72,
    0x6e, 0x5f, 0x62, 0x6f, 0x64, 0x79, 0x18, 0x07, 0x20, 0x01, 0x28, 0x08, 0x12, 0x0a, 0x0a, 0x02,
    0x70, 0x77, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x17, 0x0a, 0x0f, 0x69, 0x66, 0x5f, 0x6e,
    0x6f, 0x74, 0x5f, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64, 0x18, 0x09, 0x20, 0x01, 0x28,
    0x08, 0x12, 0x15, 0x0a, 0x0d, 0x69, 0x66, 0x5f, 0x6e, 0x6f, 0x6e, 0x65, 0x5f, 0x6d, 0x61, 0x74,
    0x63, 0x68, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x08, 0x12, 0x13, 0x0a, 0x0b, 0x72, 0x65, 0x74, 0x75,
    0x72, 0x6e, 0x5f, 0x68, 0x65, 0x61, 0x64, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x08, 0x12, 0x0f, 0x0a,
    0x07, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0c,
    0x0a, 0x04, 0x61, 0x73, 0x69, 0x73, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x08, 0x12, 0x15, 0x0a, 0x0d,
    0x73, 0x6c, 0x6f, 0x70, 0x70, 0x79, 0x5f, 0x71, 0x75, 0x6f, 0x72, 0x75, 0x6d, 0x18, 0x0e, 0x20,
    0x01, 0x28, 0x08, 0x12, 0x0d, 0x0a, 0x05, 0x6e, 0x5f, 0x76, 0x61, 0x6c, 0x18, 0x0f, 0x20, 0x01,
    0x28, 0x0d, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x10, 0x20, 0x01, 0x28, 0x0c,
    0x22, 0x47, 0x0a, 0x0a, 0x52, 0x70, 0x62, 0x50, 0x75, 0x74, 0x52, 0x65, 0x73, 0x70, 0x12, 0x1c,
    0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x0b, 0x2e, 0x52, 0x70, 0x62, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x12, 0x0e, 0x0a, 0x06,
    0x76, 0x63, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0b, 0x0a, 0x03,
    0x6b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x22, 0xc3, 0x01, 0x0a, 0x09, 0x52, 0x70,
    0x62, 0x44, 0x65, 0x6c, 0x52, 0x65, 0x71, 0x12, 0x0e, 0x0a, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65,
    0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x0c, 0x12, 0x0a, 0x0a, 0x02, 0x72, 0x77, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d,
    0x12, 0x0e, 0x0a, 0x06, 0x76, 0x63, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c,
    0x12, 0x09, 0x0a, 0x01, 0x72, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x09, 0x0a, 0x01, 0x77,
    0x18, 0x06, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0a, 0x0a, 0x02, 0x70, 0x72, 0x18, 0x07, 0x20, 0x01,
    0x28, 0x0d, 0x12, 0x0a, 0x0a, 0x02, 0x70, 0x77, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0a,
    0x0a, 0x02, 0x64, 0x77, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0f, 0x0a, 0x07, 0x74, 0x69,
    0x6d, 0x65, 0x6f, 0x75, 0x74, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x15, 0x0a, 0x0d, 0x73,
    0x6c, 0x6f, 0x70, 0x70, 0x79, 0x5f, 0x71, 0x75, 0x6f, 0x72, 0x75, 0x6d, 0x18, 0x0b, 0x20, 0x01,
    0x28, 0x08, 0x12, 0x0d, 0x0a, 0x05, 0x6e, 0x5f, 0x76, 0x61, 0x6c, 0x18, 0x0c, 0x20, 0x01, 0x28,
    0x0d, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x0c, 0x22,
    0x42, 0x0a, 0x11, 0x52, 0x70, 0x62, 0x4c, 0x69, 0x73, 0x74, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74,
    0x73, 0x52, 0x65, 0x71, 0x12, 0x0f, 0x0a, 0x07, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0e, 0x0a, 0x06, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x08, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0c, 0x22, 0x33, 0x0a, 0x12, 0x52, 0x70, 0x62, 0x4c, 0x69, 0x73, 0x74, 0x42, 0x75,
    0x63, 0x6b, 0x65, 0x74, 0x73, 0x52, 0x65, 0x73, 0x70, 0x12, 0x0f, 0x0a, 0x07, 0x62, 0x75, 0x63,
    0x6b, 0x65, 0x74, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0c, 0x12, 0x0c, 0x0a, 0x04, 0x64, 0x6f,
    0x6e, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x22, 0x3f, 0x0a, 0x0e, 0x52, 0x70, 0x62, 0x4c,
    0x69, 0x73, 0x74, 0x4b, 0x65, 0x79, 0x73, 0x52, 0x65, 0x71, 0x12, 0x0e, 0x0a, 0x06, 0x62, 0x75,
    0x63, 0x6b, 0x65, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0f, 0x0a, 0x07, 0x74, 0x69,
    0x6d, 0x65, 0x6f, 0x75, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0c, 0x0a, 0x04, 0x74,
    0x79, 0x70, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x2d, 0x0a, 0x0f, 0x52, 0x70, 0x62,
    0x4c, 0x69, 0x73, 0x74, 0x4b, 0x65, 0x79, 0x73, 0x52, 0x65, 0x73, 0x70, 0x12, 0x0c, 0x0a, 0x04,
    0x6b, 0x65, 0x79, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0c, 0x12, 0x0c, 0x0a, 0x04, 0x64, 0x6f,
    0x6e, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x22, 0x35, 0x0a, 0x0c, 0x52, 0x70, 0x62, 0x4d,
    0x61, 0x70, 0x52, 0x65, 0x64, 0x52, 0x65, 0x71, 0x12, 0x0f, 0x0a, 0x07, 0x72, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x14, 0x0a, 0x0c, 0x63, 0x6f, 0x6e,
    0x74, 0x65, 0x6e, 0x74, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x22,
    0x3e, 0x0a, 0x0d, 0x52, 0x70, 0x62, 0x4d, 0x61, 0x70, 0x52, 0x65, 0x64, 0x52, 0x65, 0x73, 0x70,
    0x12, 0x0d, 0x0a, 0x05, 0x70, 0x68, 0x61, 0x73, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12,
    0x10, 0x0a, 0x08, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0c, 0x12, 0x0c, 0x0a, 0x04, 0x64, 0x6f, 0x6e, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x22,
    0xf9, 0x02, 0x0a, 0x0b, 0x52, 0x70, 0x62, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x52, 0x65, 0x71, 0x12,
    0x0e, 0x0a, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12,
    0x0d, 0x0a, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x2a,
    0x0a, 0x05, 0x71, 0x74, 0x79, 0x70, 0x65, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x1b, 0x2e,
    0x52, 0x70, 0x62, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x52, 0x65, 0x71, 0x2e, 0x49, 0x6e, 0x64, 0x65,
    0x78, 0x51, 0x75, 0x65, 0x72, 0x79, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65,
    0x79, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x11, 0x0a, 0x09, 0x72, 0x61, 0x6e, 0x67, 0x65,
    0x5f, 0x6d, 0x69, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x11, 0x0a, 0x09, 0x72, 0x61,
    0x6e, 0x67, 0x65, 0x5f, 0x6d, 0x61, 0x78, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x14, 0x0a,
    0x0c, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x5f, 0x74, 0x65, 0x72, 0x6d, 0x73, 0x18, 0x07, 0x20,
    0x01, 0x28, 0x08, 0x12, 0x0e, 0x0a, 0x06, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x18, 0x08, 0x20,
    0x01, 0x28, 0x08, 0x12, 0x13, 0x0a, 0x0b, 0x6d, 0x61, 0x78, 0x5f, 0x72, 0x65, 0x73, 0x75, 0x6c,
    0x74, 0x73, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x14, 0x0a, 0x0c, 0x63, 0x6f, 0x6e, 0x74,
    0x69, 0x6e, 0x75, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0f,
    0x0a, 0x07, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0d, 0x12,
    0x0c, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x12, 0x0a,
    0x0a, 0x74, 0x65, 0x72, 0x6d, 0x5f, 0x72, 0x65, 0x67, 0x65, 0x78, 0x18, 0x0d, 0x20, 0x01, 0x28,
    0x0c, 0x12, 0x17, 0x0a, 0x0f, 0x70, 0x61, 0x67, 0x69, 0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f,
    0x73, 0x6f, 0x72, 0x74, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x08, 0x12, 0x15, 0x0a, 0x0d, 0x63, 0x6f,
    0x76, 0x65, 0x72, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x18, 0x0f, 0x20, 0x01, 0x28,
    0x0c, 0x12, 0x13, 0x0a, 0x0b, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x5f, 0x62, 0x6f, 0x64, 0x79,
    0x18, 0x10, 0x20, 0x01, 0x28, 0x08, 0x22, 0x23, 0x0a, 0x0e, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x51,
    0x75, 0x65, 0x72, 0x79, 0x54, 0x79, 0x70, 0x65, 0x12, 0x06, 0x0a, 0x02, 0x65, 0x71, 0x10, 0x00,
    0x12, 0x09, 0x0a, 0x05, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x10, 0x01, 0x22, 0x5b, 0x0a, 0x0c, 0x52,
    0x70, 0x62, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x52, 0x65, 0x73, 0x70, 0x12, 0x0c, 0x0a, 0x04, 0x6b,
    0x65, 0x79, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0c, 0x12, 0x19, 0x0a, 0x07, 0x72, 0x65, 0x73,
    0x75, 0x6c, 0x74, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x08, 0x2e, 0x52, 0x70, 0x62,
    0x50, 0x61, 0x69, 0x72, 0x12, 0x14, 0x0a, 0x0c, 0x63, 0x6f, 0x6e, 0x74, 0x69, 0x6e, 0x75, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0c, 0x0a, 0x04, 0x64, 0x6f,
    0x6e, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x22, 0x58, 0x0a, 0x10, 0x52, 0x70, 0x62, 0x49,
    0x6e, 0x64, 0x65, 0x78, 0x42, 0x6f, 0x64, 0x79, 0x52, 0x65, 0x73, 0x70, 0x12, 0x20, 0x0a, 0x07,
    0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0f, 0x2e,
    0x52, 0x70, 0x62, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x12, 0x14,
    0x0a, 0x0c, 0x63, 0x6f, 0x6e, 0x74, 0x69, 0x6e, 0x75, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0c, 0x12, 0x0c, 0x0a, 0x04, 0x64, 0x6f, 0x6e, 0x65, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x08, 0x22, 0xd8, 0x01, 0x0a, 0x0e, 0x52, 0x70, 0x62, 0x43, 0x53, 0x42, 0x75, 0x63, 0x6b,
    0x65, 0x74, 0x52, 0x65, 0x71, 0x12, 0x0e, 0x0a, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x11, 0x0a, 0x09, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x6b,
    0x65, 0x79, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0f, 0x0a, 0x07, 0x65, 0x6e, 0x64, 0x5f,
    0x6b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x18, 0x0a, 0x0a, 0x73, 0x74, 0x61,
    0x72, 0x74, 0x5f, 0x69, 0x6e, 0x63, 0x6c, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x3a, 0x04, 0x74,
    0x72, 0x75, 0x65, 0x12, 0x17, 0x0a, 0x08, 0x65, 0x6e, 0x64, 0x5f, 0x69, 0x6e, 0x63, 0x6c, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x08, 0x3a, 0x05, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x12, 0x14, 0x0a, 0x0c,
    0x63, 0x6f, 0x6e, 0x74, 0x69, 0x6e, 0x75, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x0c, 0x12, 0x13, 0x0a, 0x0b, 0x6d, 0x61, 0x78, 0x5f, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x73, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0f, 0x0a, 0x07, 0x74, 0x69, 0x6d, 0x65, 0x6f,
    0x75, 0x74, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65,
    0x18, 0x09, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x15, 0x0a, 0x0d, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x5f,
    0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x57, 0x0a,
    0x0f, 0x52, 0x70, 0x62, 0x43, 0x53, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70,
    0x12, 0x20, 0x0a, 0x07, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x0f, 0x2e, 0x52, 0x70, 0x62, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x4f, 0x62, 0x6a, 0x65,
    0x63, 0x74, 0x12, 0x14, 0x0a, 0x0c, 0x63, 0x6f, 0x6e, 0x74, 0x69, 0x6e, 0x75, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0c, 0x0a, 0x04, 0x64, 0x6f, 0x6e, 0x65,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x22, 0x3a, 0x0a, 0x0e, 0x52, 0x70, 0x62, 0x49, 0x6e, 0x64,
    0x65, 0x78, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x1b, 0x0a, 0x06, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x52, 0x70, 0x62, 0x47, 0x65, 0x74, 0x52, 0x65,
    0x73, 0x70, 0x22, 0xf5, 0x01, 0x0a, 0x0a, 0x52, 0x70, 0x62, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e,
    0x74, 0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c,
    0x12, 0x14, 0x0a, 0x0c, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x5f, 0x74, 0x79, 0x70, 0x65,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0f, 0x0a, 0x07, 0x63, 0x68, 0x61, 0x72, 0x73, 0x65,
    0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x18, 0x0a, 0x10, 0x63, 0x6f, 0x6e, 0x74, 0x65,
    0x6e, 0x74, 0x5f, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e, 0x67, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0c, 0x12, 0x0c, 0x0a, 0x04, 0x76, 0x74, 0x61, 0x67, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0c, 0x12,
    0x17, 0x0a, 0x05, 0x6c, 0x69, 0x6e, 0x6b, 0x73, 0x18, 0x06, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x08,
    0x2e, 0x52, 0x70, 0x62, 0x4c, 0x69, 0x6e, 0x6b, 0x12, 0x10, 0x0a, 0x08, 0x6c, 0x61, 0x73, 0x74,
    0x5f, 0x6d, 0x6f, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x16, 0x0a, 0x0e, 0x6c, 0x61,
    0x73, 0x74, 0x5f, 0x6d, 0x6f, 0x64, 0x5f, 0x75, 0x73, 0x65, 0x63, 0x73, 0x18, 0x08, 0x20, 0x01,
    0x28, 0x0d, 0x12, 0x1a, 0x0a, 0x08, 0x75, 0x73, 0x65, 0x72, 0x6d, 0x65, 0x74, 0x61, 0x18, 0x09,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x08, 0x2e, 0x52, 0x70, 0x62, 0x50, 0x61, 0x69, 0x72, 0x12, 0x19,
    0x0a, 0x07, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x65, 0x73, 0x18, 0x0a, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x08, 0x2e, 0x52, 0x70, 0x62, 0x50, 0x61, 0x69, 0x72, 0x12, 0x0f, 0x0a, 0x07, 0x64, 0x65, 0x6c,
    0x65, 0x74, 0x65, 0x64, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x08, 0x22, 0x33, 0x0a, 0x07, 0x52, 0x70,
    0x62, 0x4c, 0x69, 0x6e, 0x6b, 0x12, 0x0e, 0x0a, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0c, 0x12, 0x0b, 0x0a, 0x03, 0x74, 0x61, 0x67, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x22,
    0x7a, 0x0a, 0x13, 0x52, 0x70, 0x62, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x55, 0x70, 0x64,
    0x61, 0x74, 0x65, 0x52, 0x65, 0x71, 0x12, 0x0e, 0x0a, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20,
    0x02, 0x28, 0x0c, 0x12, 0x0e, 0x0a, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x03, 0x20,
    0x02, 0x28, 0x12, 0x12, 0x09, 0x0a, 0x01, 0x77, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0a,
    0x0a, 0x02, 0x64, 0x77, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0a, 0x0a, 0x02, 0x70, 0x77,
    0x18, 0x06, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x13, 0x0a, 0x0b, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x08, 0x22, 0x25, 0x0a, 0x14, 0x52,
    0x70, 0x62, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x52,
    0x65, 0x73, 0x70, 0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x12, 0x22, 0x71, 0x0a, 0x10, 0x52, 0x70, 0x62, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72,
    0x47, 0x65, 0x74, 0x52, 0x65, 0x71, 0x12, 0x0e, 0x0a, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20,
    0x02, 0x28, 0x0c, 0x12, 0x09, 0x0a, 0x01, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0a,
    0x0a, 0x02, 0x70, 0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x14, 0x0a, 0x0c, 0x62, 0x61,
    0x73, 0x69, 0x63, 0x5f, 0x71, 0x75, 0x6f, 0x72, 0x75, 0x6d, 0x18, 0x05, 0x20, 0x01, 0x28, 0x08,
    0x12, 0x13, 0x0a, 0x0b, 0x6e, 0x6f, 0x74, 0x66, 0x6f, 0x75, 0x6e, 0x64, 0x5f, 0x6f, 0x6b, 0x18,
    0x06, 0x20, 0x01, 0x28, 0x08, 0x22, 0x22, 0x0a, 0x11, 0x52, 0x70, 0x62, 0x43, 0x6f, 0x75, 0x6e,
    0x74, 0x65, 0x72, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x12, 0x22, 0x47, 0x0a, 0x1a, 0x52, 0x70, 0x62,
    0x47, 0x65, 0x74, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x4b, 0x65, 0x79, 0x50, 0x72, 0x65, 0x66,
    0x6c, 0x69, 0x73, 0x74, 0x52, 0x65, 0x71, 0x12, 0x0e, 0x0a, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65,
    0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x0c, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0c, 0x22, 0x4a, 0x0a, 0x1b, 0x52, 0x70, 0x62, 0x47, 0x65, 0x74, 0x42, 0x75, 0x63, 0x6b,
    0x65, 0x74, 0x4b, 0x65, 0x79, 0x50, 0x72, 0x65, 0x66, 0x6c, 0x69, 0x73, 0x74, 0x52, 0x65, 0x73,
    0x70, 0x12, 0x2b, 0x0a, 0x08, 0x70, 0x72, 0x65, 0x66, 0x6c, 0x69, 0x73, 0x74, 0x18, 0x01, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x52, 0x70, 0x62, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x4b,
    0x65, 0x79, 0x50, 0x72, 0x65, 0x66, 0x6c, 0x69, 0x73, 0x74, 0x49, 0x74, 0x65, 0x6d, 0x22, 0x4c,
    0x0a, 0x18, 0x52, 0x70, 0x62, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x4b, 0x65, 0x79, 0x50, 0x72,
    0x65, 0x66, 0x6c, 0x69, 0x73, 0x74, 0x49, 0x74, 0x65, 0x6d, 0x12, 0x11, 0x0a, 0x09, 0x70, 0x61,
    0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x02, 0x28, 0x03, 0x12, 0x0c, 0x0a,
    0x04, 0x6e, 0x6f, 0x64, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0f, 0x0a, 0x07, 0x70,
    0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x18, 0x03, 0x20, 0x02, 0x28, 0x08, 0x22, 0x78, 0x0a, 0x0e,
    0x52, 0x70, 0x62, 0x43, 0x6f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65, 0x52, 0x65, 0x71, 0x12, 0x0c,
    0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0e, 0x0a, 0x06,
    0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x16, 0x0a, 0x0e,
    0x6d, 0x69, 0x6e, 0x5f, 0x70, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0d, 0x12, 0x15, 0x0a, 0x0d, 0x72, 0x65, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x5f,
    0x63, 0x6f, 0x76, 0x65, 0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x19, 0x0a, 0x11, 0x75,
    0x6e, 0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x63, 0x6f, 0x76, 0x65, 0x72,
    0x18, 0x05, 0x20, 0x03, 0x28, 0x0c, 0x22, 0x35, 0x0a, 0x0f, 0x52, 0x70, 0x62, 0x43, 0x6f, 0x76,
    0x65, 0x72, 0x61, 0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x12, 0x22, 0x0a, 0x07, 0x65, 0x6e, 0x74,
    0x72, 0x69, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x52, 0x70, 0x62,
    0x43, 0x6f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x22, 0x5a, 0x0a,
    0x10, 0x52, 0x70, 0x62, 0x43, 0x6f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65, 0x45, 0x6e, 0x74, 0x72,
    0x79, 0x12, 0x0a, 0x0a, 0x02, 0x69, 0x70, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0c, 0x0a,
    0x04, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x15, 0x0a, 0x0d, 0x6b,
    0x65, 0x79, 0x73, 0x70, 0x61, 0x63, 0x65, 0x5f, 0x64, 0x65, 0x73, 0x63, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0c, 0x12, 0x15, 0x0a, 0x0d, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x5f, 0x63, 0x6f, 0x6e, 0x74,
    0x65, 0x78, 0x74, 0x18, 0x04, 0x20, 0x02, 0x28, 0x0c, 0x42, 0x23, 0x0a, 0x17, 0x63, 0x6f, 0x6d,
    0x2e, 0x62, 0x61, 0x73, 0x68, 0x6f, 0x2e, 0x72, 0x69, 0x61, 0x6b, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x62, 0x75, 0x66, 0x42, 0x08, 0x52, 0x69, 0x61, 0x6b, 0x4b, 0x76, 0x50, 0x42, 0x4a, 0x86,
    0x79, 0x0a, 0x07, 0x12, 0x05, 0x1c, 0x00, 0xc8, 0x02, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12,
    0x03, 0x1c, 0x00, 0x30, 0x0a, 0x26, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x1c, 0x00,
    0x30, 0x1a, 0x19, 0x20, 0x4a, 0x61, 0x76, 0x61, 0x20, 0x70, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65,
    0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x72, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x1c, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1c, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07,
    0x00, 0x07, 0x12, 0x03, 0x1c, 0x16, 0x2f, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x1d, 0x00,
    0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01, 0x12, 0x03, 0x1d, 0x00, 0x29, 0x0a, 0x0c,
    0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03, 0x1d, 0x07, 0x1b, 0x0a, 0x0d, 0x0a, 0x06,
    0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x1d, 0x07, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x08,
    0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x07, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x08,
    0xe7, 0x07, 0x01, 0x07, 0x12, 0x03, 0x1d, 0x1e, 0x28, 0x0a, 0x18, 0x0a, 0x02, 0x03, 0x00, 0x12,
    0x03, 0x1f, 0x07, 0x13, 0x22, 0x0d, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x52, 0x70, 0x62, 0x50, 0x61,
    0x69, 0x72, 0x0a, 0x0a, 0x61, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x22, 0x00, 0x24, 0x01, 0x1a,
    0x55, 0x20, 0x47, 0x65, 0x74, 0x20, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x20, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x2d, 0x20, 0x6e, 0x6f, 0x20, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x64, 0x2c, 0x20, 0x6a, 0x75, 0x73,
    0x74, 0x20, 0x73, 0x65, 0x6e, 0x64, 0x20, 0x52, 0x70, 0x62, 0x47, 0x65, 0x74, 0x43, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x49, 0x64, 0x52, 0x65, 0x71, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x20, 0x63, 0x6f, 0x64, 0x65, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x22,
    0x08, 0x1a, 0x0a, 0x33, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x23, 0x04, 0x21, 0x22,
    0x26, 0x20, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x69, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x75,
    0x73, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x63, 0x6f, 0x6e, 0x6e,
    0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x23, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x23, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x23, 0x13,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x23, 0x1f, 0x20, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x26, 0x00, 0x28, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x01, 0x01, 0x12, 0x03, 0x26, 0x08, 0x19, 0x0a, 0x33, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12,
    0x03, 0x27, 0x04, 0x21, 0x22, 0x26, 0x20, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x69, 0x64,
    0x20, 0x74, 0x6f, 0x20, 0x75, 0x73, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x69, 0x73,
    0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x27, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x27, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x27, 0x13, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x27, 0x1f, 0x20, 0x0a, 0x2f, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x2d, 0x00, 0x3b, 0x01,
    0x1a, 0x23, 0x20, 0x47, 0x65, 0x74, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x2d,
    0x20, 0x72, 0x65, 0x74, 0x72, 0x69, 0x65, 0x76, 0x65, 0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74,
    0x2f, 0x6b, 0x65, 0x79, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x2d, 0x08,
    0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x2e, 0x04, 0x1e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2e, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x2e, 0x13, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x2e, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x2f, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2f, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2f, 0x0d, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2f, 0x13, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2f, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x02, 0x12, 0x03, 0x30, 0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x30, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x30, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x30,
    0x14, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x30, 0x18, 0x19,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x31, 0x04, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x31, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x31, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x31, 0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x31, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x32,
    0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12, 0x03, 0x32, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x05, 0x12, 0x03, 0x32, 0x0d, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x32, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x32, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x33, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x04,
    0x12, 0x03, 0x33, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x05, 0x12, 0x03,
    0x33, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x01, 0x12, 0x03, 0x33, 0x12,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x03, 0x12, 0x03, 0x33, 0x20, 0x21, 0x0a,
    0x39, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x34, 0x04, 0x23, 0x22, 0x2c, 0x20, 0x66,
    0x61, 0x69, 0x6c, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6c,
    0x69, 0x65, 0x64, 0x20, 0x76, 0x63, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x64, 0x6f, 0x65, 0x73, 0x20,
    0x6e, 0x6f, 0x74, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x06, 0x04, 0x12, 0x03, 0x34, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06,
    0x05, 0x12, 0x03, 0x34, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x01, 0x12,
    0x03, 0x34, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x03, 0x12, 0x03, 0x34,
    0x21, 0x22, 0x0a, 0x2e, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x07, 0x12, 0x03, 0x35, 0x04, 0x1b, 0x22,
    0x21, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x65, 0x76, 0x65, 0x72, 0x79, 0x74, 0x68,
    0x69, 0x6e, 0x67, 0x20, 0x62, 0x75, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x04, 0x12, 0x03, 0x35, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x05, 0x12, 0x03, 0x35, 0x0d, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x01, 0x12, 0x03, 0x35, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x07, 0x03, 0x12, 0x03, 0x35, 0x19, 0x1a, 0x0a, 0x3b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x08, 0x12, 0x03, 0x36, 0x04, 0x24, 0x22, 0x2e, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x6f, 0x6d, 0x62, 0x73, 0x74, 0x6f, 0x6e, 0x65, 0x27, 0x73,
    0x20, 0x76, 0x63, 0x6c, 0x6f, 0x63, 0x6b, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x61, 0x70, 0x70, 0x6c,
    0x69, 0x63, 0x61, 0x62, 0x6c, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x04,
    0x12, 0x03, 0x36, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x05, 0x12, 0x03,
    0x36, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x01, 0x12, 0x03, 0x36, 0x12,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x03, 0x12, 0x03, 0x36, 0x22, 0x23, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x09, 0x12, 0x03, 0x37, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x09, 0x04, 0x12, 0x03, 0x37, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x09, 0x05, 0x12, 0x03, 0x37, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x09,
    0x01, 0x12, 0x03, 0x37, 0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x09, 0x03, 0x12,
    0x03, 0x37, 0x1e, 0x20, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x0a, 0x12, 0x03, 0x38, 0x04,
    0x25, 0x22, 0x24, 0x20, 0x45, 0x78, 0x70, 0x65, 0x72, 0x69, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x6c,
    0x2c, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x2f, 0x64, 0x69, 0x73,
    0x61, 0x70, 0x70, 0x65, 0x61, 0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0a, 0x04,
    0x12, 0x03, 0x38, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0a, 0x05, 0x12, 0x03,
    0x38, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x38, 0x12,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x38, 0x22, 0x24, 0x0a,
    0x31, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x0b, 0x12, 0x03, 0x39, 0x04, 0x1f, 0x22, 0x24, 0x20, 0x45,
    0x78, 0x70, 0x65, 0x72, 0x69, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x6c, 0x2c, 0x20, 0x6d, 0x61, 0x79,
    0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x2f, 0x64, 0x69, 0x73, 0x61, 0x70, 0x70, 0x65, 0x61,
    0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x39, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x39, 0x0d, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x39, 0x14, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x39, 0x1c, 0x1e, 0x0a, 0x43, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x0c, 0x12, 0x03, 0x3a, 0x04, 0x1d, 0x22, 0x36, 0x20, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74,
    0x20, 0x74, 0x79, 0x70, 0x65, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x65,
    0x74, 0x20, 0x77, 0x65, 0x20, 0x61, 0x73, 0x73, 0x75, 0x6d, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x27, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x27, 0x20, 0x74, 0x79, 0x70, 0x65, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x3a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x3a, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x3a, 0x13, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x0c, 0x03, 0x12, 0x03, 0x3a, 0x1a, 0x1c, 0x0a, 0x58, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x3e,
    0x00, 0x42, 0x01, 0x1a, 0x4c, 0x20, 0x47, 0x65, 0x74, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x20, 0x2d, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x63, 0x6f,
    0x72, 0x64, 0x20, 0x77, 0x61, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x66, 0x6f, 0x75, 0x6e, 0x64,
    0x20, 0x74, 0x68, 0x65, 0x72, 0x65, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x6e,
    0x6f, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x2f, 0x76, 0x63, 0x6c, 0x6f, 0x63, 0x6b,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x3e, 0x08, 0x12, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x3f, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x3f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x3f, 0x0d, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x3f, 0x18, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3f,
    0x22, 0x23, 0x0a, 0x35, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x40, 0x04, 0x1e, 0x22,
    0x28, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x70, 0x61, 0x71, 0x75, 0x65, 0x20, 0x76, 0x65, 0x63,
    0x74, 0x6f, 0x72, 0x20, 0x63, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x40, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x40, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x40, 0x13, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x40, 0x1c,
    0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x41, 0x04, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x41, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x41, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x41, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x41, 0x1e, 0x1f, 0x0a, 0x85, 0x01, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x47,
    0x00, 0x58, 0x01, 0x1a, 0x79, 0x20, 0x50, 0x75, 0x74, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x20, 0x2d, 0x20, 0x69, 0x66, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x72,
    0x65, 0x74, 0x75, 0x72, 0x6e, 0x5f, 0x62, 0x6f, 0x64, 0x79, 0x20, 0x69, 0x73, 0x20, 0x73, 0x65,
    0x74, 0x20, 0x74, 0x68, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74,
    0x65, 0x64, 0x20, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2f, 0x64, 0x61, 0x74, 0x61,
    0x20, 0x66, 0x6f, 0x72, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x77, 0x69, 0x6c, 0x6c,
    0x20, 0x62, 0x65, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x47, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x00, 0x12, 0x03, 0x48, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x48, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x48, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x48, 0x13,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x48, 0x1c, 0x1d, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x49, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x49, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x49, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x49, 0x13, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x49, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x4a, 0x04,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x4a, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x4a, 0x0d, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4a, 0x13, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x4a, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x03, 0x12, 0x03, 0x4b, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x04, 0x12,
    0x03, 0x4b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x06, 0x12, 0x03, 0x4b,
    0x0d, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01, 0x12, 0x03, 0x4b, 0x18, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03, 0x4b, 0x22, 0x23, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x04, 0x12, 0x03, 0x4c, 0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x04, 0x04, 0x12, 0x03, 0x4c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x04, 0x05, 0x12, 0x03, 0x4c, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x4c, 0x14, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x03, 0x12, 0x03,
    0x4c, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x05, 0x12, 0x03, 0x4d, 0x04, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x04, 0x12, 0x03, 0x4d, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x05, 0x12, 0x03, 0x4d, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x05, 0x01, 0x12, 0x03, 0x4d, 0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x05, 0x03, 0x12, 0x03, 0x4d, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x06,
    0x12, 0x03, 0x4e, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x04, 0x12, 0x03,
    0x4e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x05, 0x12, 0x03, 0x4e, 0x0d,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x01, 0x12, 0x03, 0x4e, 0x12, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x03, 0x12, 0x03, 0x4e, 0x20, 0x21, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x04, 0x02, 0x07, 0x12, 0x03, 0x4f, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x07, 0x04, 0x12, 0x03, 0x4f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07,
    0x05, 0x12, 0x03, 0x4f, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x01, 0x12,
    0x03, 0x4f, 0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x03, 0x12, 0x03, 0x4f,
    0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x08, 0x12, 0x03, 0x50, 0x04, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x08, 0x04, 0x12, 0x03, 0x50, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x08, 0x05, 0x12, 0x03, 0x50, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x08, 0x01, 0x12, 0x03, 0x50, 0x12, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x08, 0x03, 0x12, 0x03, 0x50, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x09, 0x12,
    0x03, 0x51, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x09, 0x04, 0x12, 0x03, 0x51,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x09, 0x05, 0x12, 0x03, 0x51, 0x0d, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x09, 0x01, 0x12, 0x03, 0x51, 0x12, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x09, 0x03, 0x12, 0x03, 0x51, 0x22, 0x24, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x0a, 0x12, 0x03, 0x52, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x0a, 0x04, 0x12, 0x03, 0x52, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0a, 0x05,
    0x12, 0x03, 0x52, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0a, 0x01, 0x12, 0x03,
    0x52, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x52, 0x20,
    0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x0b, 0x12, 0x03, 0x53, 0x04, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x53, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x53, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x0b, 0x01, 0x12, 0x03, 0x53, 0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0b,
    0x03, 0x12, 0x03, 0x53, 0x1e, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x0c, 0x12, 0x03,
    0x54, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x54, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x54, 0x0d, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x54, 0x12, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x54, 0x19, 0x1b, 0x0a, 0x31, 0x0a, 0x04, 0x04,
    0x04, 0x02, 0x0d, 0x12, 0x03, 0x55, 0x04, 0x25, 0x22, 0x24, 0x20, 0x45, 0x78, 0x70, 0x65, 0x72,
    0x69, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x6c, 0x2c, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x63, 0x68, 0x61,
    0x6e, 0x67, 0x65, 0x2f, 0x64, 0x69, 0x73, 0x61, 0x70, 0x70, 0x65, 0x61, 0x72, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x55, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x55, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x0d, 0x01, 0x12, 0x03, 0x55, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0d,
    0x03, 0x12, 0x03, 0x55, 0x22, 0x24, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x0e, 0x12, 0x03,
    0x56, 0x04, 0x1f, 0x22, 0x24, 0x20, 0x45, 0x78, 0x70, 0x65, 0x72, 0x69, 0x6d, 0x65, 0x6e, 0x74,
    0x61, 0x6c, 0x2c, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x2f, 0x64,
    0x69, 0x73, 0x61, 0x70, 0x70, 0x65, 0x61, 0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x0e, 0x04, 0x12, 0x03, 0x56, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0e, 0x05,
    0x12, 0x03, 0x56, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0e, 0x01, 0x12, 0x03,
    0x56, 0x14, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x56, 0x1c,
    0x1e, 0x0a, 0x43, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x0f, 0x12, 0x03, 0x57, 0x04, 0x1d, 0x22, 0x36,
    0x20, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x74, 0x79, 0x70, 0x65, 0x2c, 0x20, 0x69, 0x66,
    0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x65, 0x74, 0x20, 0x77, 0x65, 0x20, 0x61, 0x73, 0x73, 0x75,
    0x6d, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x27, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x27,
    0x20, 0x74, 0x79, 0x70, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0f, 0x04, 0x12,
    0x03, 0x57, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0f, 0x05, 0x12, 0x03, 0x57,
    0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x57, 0x13, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0f, 0x03, 0x12, 0x03, 0x57, 0x1a, 0x1c, 0x0a, 0x58,
    0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x5b, 0x00, 0x5f, 0x01, 0x1a, 0x4c, 0x20, 0x50, 0x75, 0x74,
    0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x2d, 0x20, 0x73, 0x61, 0x6d, 0x65,
    0x20, 0x61, 0x73, 0x20, 0x67, 0x65, 0x74, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x6b,
    0x65, 0x79, 0x20, 0x69, 0x66, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x77, 0x61, 0x73, 0x20, 0x67, 0x65,
    0x6e, 0x65, 0x72, 0x61, 0x74, 0x65, 0x64, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12,
    0x03, 0x5b, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x5c, 0x04,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5c, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12, 0x03, 0x5c, 0x0d, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5c, 0x18, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5c, 0x22, 0x23, 0x0a, 0x35, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x01, 0x12, 0x03, 0x5d, 0x04, 0x1e, 0x22, 0x28, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x70, 0x61,
    0x71, 0x75, 0x65, 0x20, 0x76, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x20, 0x63, 0x6c, 0x6f, 0x63, 0x6b,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x5d, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x05, 0x12, 0x03, 0x5d, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5d, 0x13, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x5d, 0x1c, 0x1d, 0x0a, 0x28, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02,
    0x12, 0x03, 0x5e, 0x04, 0x1b, 0x22, 0x1b, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6b, 0x65, 0x79, 0x20,
    0x67, 0x65, 0x6e, 0x65, 0x72, 0x61, 0x74, 0x65, 0x64, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x61, 0x6e,
    0x79, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x5e, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x5e, 0x0d, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x5e, 0x13, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x5e, 0x19, 0x1a, 0x0a, 0x1c, 0x0a, 0x02, 0x04, 0x06,
    0x12, 0x04, 0x63, 0x00, 0x71, 0x01, 0x1a, 0x10, 0x20, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x20,
    0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12,
    0x03, 0x63, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x64, 0x04,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x64, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x64, 0x0d, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x64, 0x13, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x64, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02,
    0x01, 0x12, 0x03, 0x65, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x65, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x03, 0x65,
    0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x65, 0x13, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x65, 0x19, 0x1a, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x66, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x02, 0x04, 0x12, 0x03, 0x66, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x66, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x66, 0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x66, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x03, 0x67, 0x04, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x04, 0x12, 0x03, 0x67, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x05, 0x12, 0x03, 0x67, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x03, 0x01, 0x12, 0x03, 0x67, 0x13, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x67, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x04,
    0x12, 0x03, 0x68, 0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x04, 0x12, 0x03,
    0x68, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x05, 0x12, 0x03, 0x68, 0x0d,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x01, 0x12, 0x03, 0x68, 0x14, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x03, 0x12, 0x03, 0x68, 0x18, 0x19, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x06, 0x02, 0x05, 0x12, 0x03, 0x69, 0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x05, 0x04, 0x12, 0x03, 0x69, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05,
    0x05, 0x12, 0x03, 0x69, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x01, 0x12,
    0x03, 0x69, 0x14, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x03, 0x12, 0x03, 0x69,
    0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x06, 0x12, 0x03, 0x6a, 0x04, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x04, 0x12, 0x03, 0x6a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x06, 0x05, 0x12, 0x03, 0x6a, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x06, 0x01, 0x12, 0x03, 0x6a, 0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x06, 0x03, 0x12, 0x03, 0x6a, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x07, 0x12,
    0x03, 0x6b, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x07, 0x04, 0x12, 0x03, 0x6b,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x07, 0x05, 0x12, 0x03, 0x6b, 0x0d, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x07, 0x01, 0x12, 0x03, 0x6b, 0x14, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x07, 0x03, 0x12, 0x03, 0x6b, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x06, 0x02, 0x08, 0x12, 0x03, 0x6c, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x08, 0x04, 0x12, 0x03, 0x6c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x08, 0x05,
    0x12, 0x03, 0x6c, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x08, 0x01, 0x12, 0x03,
    0x6c, 0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x08, 0x03, 0x12, 0x03, 0x6c, 0x19,
    0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x09, 0x12, 0x03, 0x6d, 0x04, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x09, 0x04, 0x12, 0x03, 0x6d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x09, 0x05, 0x12, 0x03, 0x6d, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x09, 0x01, 0x12, 0x03, 0x6d, 0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x09,
    0x03, 0x12, 0x03, 0x6d, 0x1e, 0x20, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x0a, 0x12, 0x03,
    0x6e, 0x04, 0x25, 0x22, 0x24, 0x20, 0x45, 0x78, 0x70, 0x65, 0x72, 0x69, 0x6d, 0x65, 0x6e, 0x74,
    0x61, 0x6c, 0x2c, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x2f, 0x64,
    0x69, 0x73, 0x61, 0x70, 0x70, 0x65, 0x61, 0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x0a, 0x04, 0x12, 0x03, 0x6e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0a, 0x05,
    0x12, 0x03, 0x6e, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0a, 0x01, 0x12, 0x03,
    0x6e, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x6e, 0x22,
    0x24, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x0b, 0x12, 0x03, 0x6f, 0x04, 0x1f, 0x22, 0x24,
    0x20, 0x45, 0x78, 0x70, 0x65, 0x72, 0x69, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x6c, 0x2c, 0x20, 0x6d,
    0x61, 0x79, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x2f, 0x64, 0x69, 0x73, 0x61, 0x70, 0x70,
    0x65, 0x61, 0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x6f,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x6f, 0x0d, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x6f, 0x14, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x6f, 0x1c, 0x1e, 0x0a, 0x43, 0x0a, 0x04,
    0x04, 0x06, 0x02, 0x0c, 0x12, 0x03, 0x70, 0x04, 0x1d, 0x22, 0x36, 0x20, 0x42, 0x75, 0x63, 0x6b,
    0x65, 0x74, 0x20, 0x74, 0x79, 0x70, 0x65, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x6e, 0x6f, 0x74, 0x20,
    0x73, 0x65, 0x74, 0x20, 0x77, 0x65, 0x20, 0x61, 0x73, 0x73, 0x75, 0x6d, 0x65, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x27, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x27, 0x20, 0x74, 0x79, 0x70, 0x65,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x70, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x70, 0x0d, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x70, 0x13, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x70, 0x1a, 0x1c, 0x0a, 0x22, 0x0a, 0x02, 0x04, 0x07, 0x12,
    0x04, 0x76, 0x00, 0x7a, 0x01, 0x1a, 0x16, 0x20, 0x4c, 0x69, 0x73, 0x74, 0x20, 0x62, 0x75, 0x63,
    0x6b, 0x65, 0x74, 0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x76, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02,
    0x00, 0x12, 0x03, 0x77, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x77, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x77,
    0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x77, 0x14, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x77, 0x1e, 0x1f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x78, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x01, 0x04, 0x12, 0x03, 0x78, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x78, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x78, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x78, 0x1b, 0x1c, 0x0a, 0x43, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x02, 0x12, 0x03, 0x79, 0x04, 0x1c,
    0x22, 0x36, 0x20, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x74, 0x79, 0x70, 0x65, 0x2c, 0x20,
    0x69, 0x66, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x65, 0x74, 0x20, 0x77, 0x65, 0x20, 0x61, 0x73,
    0x73, 0x75, 0x6d, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x27, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c,
    0x74, 0x27, 0x20, 0x74, 0x79, 0x70, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x79, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x79, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12, 0x03, 0x79,
    0x13, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x03, 0x79, 0x1a, 0x1b,
    0x0a, 0x9c, 0x01, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x05, 0x7e, 0x00, 0x81, 0x01, 0x01, 0x1a, 0x8e,
    0x01, 0x20, 0x4c, 0x69, 0x73, 0x74, 0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x20, 0x72,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x2d, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x6f, 0x72,
    0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x73, 0x65, 0x20, 0x70,
    0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x73,
    0x65, 0x6e, 0x74, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x73, 0x74, 0x20, 0x6f, 0x6e,
    0x65, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x64, 0x6f, 0x6e, 0x65,
    0x20, 0x73, 0x65, 0x74, 0x20, 0x74, 0x72, 0x75, 0x65, 0x20, 0x28, 0x61, 0x6e, 0x64, 0x20, 0x6d,
    0x61, 0x79, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x61, 0x6e, 0x79, 0x20,
    0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x69, 0x74, 0x29, 0x0a, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x7e, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x08, 0x02, 0x00, 0x12, 0x03, 0x7f, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x7f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x7f, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x7f,
    0x13, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x7f, 0x1d, 0x1e,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x04, 0x80, 0x01, 0x04, 0x1b, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x04, 0x80, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12, 0x04, 0x80, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x04, 0x80, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x01, 0x03, 0x12, 0x04, 0x80, 0x01, 0x19, 0x1a, 0x0a, 0x2b, 0x0a, 0x02, 0x04, 0x09,
    0x12, 0x06, 0x85, 0x01, 0x00, 0x89, 0x01, 0x01, 0x1a, 0x1d, 0x20, 0x4c, 0x69, 0x73, 0x74, 0x20,
    0x6b, 0x65, 0x79, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x72,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x04,
    0x85, 0x01, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x04, 0x86, 0x01,
    0x04, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x04, 0x86, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x04, 0x86, 0x01, 0x0d, 0x12,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x04, 0x86, 0x01, 0x13, 0x19, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x04, 0x86, 0x01, 0x1c, 0x1d, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x04, 0x87, 0x01, 0x04, 0x20, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x04, 0x87, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x01, 0x05, 0x12, 0x04, 0x87, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x01, 0x01, 0x12, 0x04, 0x87, 0x01, 0x14, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x01, 0x03, 0x12, 0x04, 0x87, 0x01, 0x1e, 0x1f, 0x0a, 0x44, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x02,
    0x12, 0x04, 0x88, 0x01, 0x04, 0x1c, 0x22, 0x36, 0x20, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20,
    0x74, 0x79, 0x70, 0x65, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x65, 0x74,
    0x20, 0x77, 0x65, 0x20, 0x61, 0x73, 0x73, 0x75, 0x6d, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x27,
    0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x27, 0x20, 0x74, 0x79, 0x70, 0x65, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x04, 0x12, 0x04, 0x88, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x02, 0x05, 0x12, 0x04, 0x88, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x02, 0x01, 0x12, 0x04, 0x88, 0x01, 0x13, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x02, 0x03, 0x12, 0x04, 0x88, 0x01, 0x1a, 0x1b, 0x0a, 0xa1, 0x01, 0x0a, 0x02, 0x04,
    0x0a, 0x12, 0x06, 0x8d, 0x01, 0x00, 0x90, 0x01, 0x01, 0x1a, 0x92, 0x01, 0x20, 0x4c, 0x69, 0x73,
    0x74, 0x20, 0x6b, 0x65, 0x79, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74,
    0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x2d, 0x20, 0x6f, 0x6e, 0x65, 0x20,
    0x6f, 0x72, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x73, 0x65,
    0x20, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65,
    0x20, 0x73, 0x65, 0x6e, 0x74, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x73, 0x74, 0x20,
    0x6f, 0x6e, 0x65, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x64, 0x6f,
    0x6e, 0x65, 0x20, 0x73, 0x65, 0x74, 0x20, 0x74, 0x72, 0x75, 0x65, 0x20, 0x28, 0x61, 0x6e, 0x64,
    0x20, 0x6d, 0x61, 0x79, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x61, 0x6e,
    0x79, 0x20, 0x6b, 0x65, 0x79, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x69, 0x74, 0x29, 0x0a, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x04, 0x8d, 0x01, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0a, 0x02, 0x00, 0x12, 0x04, 0x8e, 0x01, 0x04, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x8e, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00,
    0x05, 0x12, 0x04, 0x8e, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x8e, 0x01, 0x13, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12,
    0x04, 0x8e, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x01, 0x12, 0x04, 0x8f,
    0x01, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x04, 0x12, 0x04, 0x8f, 0x01,
    0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x05, 0x12, 0x04, 0x8f, 0x01, 0x0d,
    0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8f, 0x01, 0x12, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8f, 0x01, 0x19, 0x1a, 0x0a,
    0x22, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x06, 0x94, 0x01, 0x00, 0x97, 0x01, 0x01, 0x1a, 0x14, 0x20,
    0x4d, 0x61, 0x70, 0x2f, 0x52, 0x65, 0x64, 0x75, 0x63, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x04, 0x94, 0x01, 0x08, 0x14,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x04, 0x95, 0x01, 0x04, 0x1f, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x04, 0x95, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x00, 0x05, 0x12, 0x04, 0x95, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x04, 0x95, 0x01, 0x13, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x00, 0x03, 0x12, 0x04, 0x95, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b,
    0x02, 0x01, 0x12, 0x04, 0x96, 0x01, 0x04, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01,
    0x04, 0x12, 0x04, 0x96, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x05,
    0x12, 0x04, 0x96, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12,
    0x04, 0x96, 0x01, 0x13, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x04,
    0x96, 0x01, 0x22, 0x23, 0x0a, 0x99, 0x01, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x06, 0x9c, 0x01, 0x00,
    0xa0, 0x01, 0x01, 0x1a, 0x8a, 0x01, 0x20, 0x4d, 0x61, 0x70, 0x2f, 0x52, 0x65, 0x64, 0x75, 0x63,
    0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x0a, 0x20, 0x6f, 0x6e, 0x65, 0x20,
    0x6f, 0x72, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x73, 0x65,
    0x20, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65,
    0x20, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x73, 0x74, 0x20, 0x6f,
    0x6e, 0x65, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x64, 0x6f, 0x6e,
    0x65, 0x20, 0x73, 0x65, 0x74, 0x0a, 0x20, 0x74, 0x72, 0x75, 0x65, 0x20, 0x28, 0x61, 0x6e, 0x64,
    0x20, 0x6d, 0x61, 0x79, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x70, 0x68,
    0x61, 0x73, 0x65, 0x2f, 0x64, 0x61, 0x74, 0x61, 0x20, 0x69, 0x6e, 0x20, 0x69, 0x74, 0x29, 0x0a,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x04, 0x9c, 0x01, 0x08, 0x15, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x04, 0x9d, 0x01, 0x04, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9d, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x00, 0x05, 0x12, 0x04, 0x9d, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x00, 0x01, 0x12, 0x04, 0x9d, 0x01, 0x14, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00,
    0x03, 0x12, 0x04, 0x9d, 0x01, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12,
    0x04, 0x9e, 0x01, 0x04, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x9e, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x05, 0x12, 0x04, 0x9e,
    0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9e, 0x01,
    0x13, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x03, 0x12, 0x04, 0x9e, 0x01, 0x1e,
    0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x02, 0x12, 0x04, 0x9f, 0x01, 0x04, 0x1b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x04, 0x12, 0x04, 0x9f, 0x01, 0x04, 0x0c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x05, 0x12, 0x04, 0x9f, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x02, 0x01, 0x12, 0x04, 0x9f, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x02, 0x03, 0x12, 0x04, 0x9f, 0x01, 0x19, 0x1a, 0x0a, 0x2d, 0x0a, 0x02, 0x04,
    0x0d, 0x12, 0x06, 0xa3, 0x01, 0x00, 0xbc, 0x01, 0x01, 0x1a, 0x1f, 0x20, 0x53, 0x65, 0x63, 0x6f,
    0x6e, 0x64, 0x61, 0x72, 0x79, 0x20, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x20, 0x71, 0x75, 0x65, 0x72,
    0x79, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0d,
    0x01, 0x12, 0x04, 0xa3, 0x01, 0x08, 0x13, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x0d, 0x04, 0x00, 0x12,
    0x06, 0xa4, 0x01, 0x04, 0xa7, 0x01, 0x05, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x04, 0x00, 0x01,
    0x12, 0x04, 0xa4, 0x01, 0x09, 0x17, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x04, 0xa5, 0x01, 0x08, 0x0f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xa5, 0x01, 0x08, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00, 0x02,
    0x00, 0x02, 0x12, 0x04, 0xa5, 0x01, 0x0d, 0x0e, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0d, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x04, 0xa6, 0x01, 0x08, 0x12, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xa6, 0x01, 0x08, 0x0d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x04,
    0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xa6, 0x01, 0x10, 0x11, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d,
    0x02, 0x00, 0x12, 0x04, 0xa9, 0x01, 0x04, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xa9, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x05,
    0x12, 0x04, 0xa9, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xa9, 0x01, 0x13, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xa9, 0x01, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x01, 0x12, 0x04, 0xaa, 0x01,
    0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x04, 0x12, 0x04, 0xaa, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x05, 0x12, 0x04, 0xaa, 0x01, 0x0d, 0x12,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x01, 0x12, 0x04, 0xaa, 0x01, 0x13, 0x18, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x03, 0x12, 0x04, 0xaa, 0x01, 0x1b, 0x1c, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x0d, 0x02, 0x02, 0x12, 0x04, 0xab, 0x01, 0x04, 0x26, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x02, 0x04, 0x12, 0x04, 0xab, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x02, 0x06, 0x12, 0x04, 0xab, 0x01, 0x0d, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x02, 0x01, 0x12, 0x04, 0xab, 0x01, 0x1c, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x02, 0x03, 0x12, 0x04, 0xab, 0x01, 0x24, 0x25, 0x0a, 0x36, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x03,
    0x12, 0x04, 0xac, 0x01, 0x04, 0x1b, 0x22, 0x28, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x68, 0x65, 0x72,
    0x65, 0x20, 0x6d, 0x65, 0x61, 0x6e, 0x73, 0x20, 0x65, 0x71, 0x75, 0x61, 0x6c, 0x73, 0x20, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x3f, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x04, 0x12, 0x04, 0xac, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x05, 0x12, 0x04, 0xac, 0x01, 0x0d, 0x12, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x01, 0x12, 0x04, 0xac, 0x01, 0x13, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x03, 0x03, 0x12, 0x04, 0xac, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x0d, 0x02, 0x04, 0x12, 0x04, 0xad, 0x01, 0x04, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x04, 0x04, 0x12, 0x04, 0xad, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x04, 0x05, 0x12, 0x04, 0xad, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04,
    0x01, 0x12, 0x04, 0xad, 0x01, 0x13, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x03,
    0x12, 0x04, 0xad, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x05, 0x12, 0x04,
    0xae, 0x01, 0x04, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x04, 0x12, 0x04, 0xae,
    0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x05, 0x12, 0x04, 0xae, 0x01,
    0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x01, 0x12, 0x04, 0xae, 0x01, 0x13,
    0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x03, 0x12, 0x04, 0xae, 0x01, 0x1f, 0x20,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x06, 0x12, 0x04, 0xaf, 0x01, 0x04, 0x23, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06, 0x04, 0x12, 0x04, 0xaf, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x06, 0x05, 0x12, 0x04, 0xaf, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x06, 0x01, 0x12, 0x04, 0xaf, 0x01, 0x12, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x06, 0x03, 0x12, 0x04, 0xaf, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d,
    0x02, 0x07, 0x12, 0x04, 0xb0, 0x01, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x07,
    0x04, 0x12, 0x04, 0xb0, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x07, 0x05,
    0x12, 0x04, 0xb0, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x07, 0x01, 0x12,
    0x04, 0xb0, 0x01, 0x12, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x07, 0x03, 0x12, 0x04,
    0xb0, 0x01, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x08, 0x12, 0x04, 0xb1, 0x01,
    0x04, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x08, 0x04, 0x12, 0x04, 0xb1, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x08, 0x05, 0x12, 0x04, 0xb1, 0x01, 0x0d, 0x13,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x08, 0x01, 0x12, 0x04, 0xb1, 0x01, 0x14, 0x1f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x08, 0x03, 0x12, 0x04, 0xb1, 0x01, 0x22, 0x23, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x0d, 0x02, 0x09, 0x12, 0x04, 0xb2, 0x01, 0x04, 0x25, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x09, 0x04, 0x12, 0x04, 0xb2, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x09, 0x05, 0x12, 0x04, 0xb2, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x09, 0x01, 0x12, 0x04, 0xb2, 0x01, 0x13, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x09, 0x03, 0x12, 0x04, 0xb2, 0x01, 0x22, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0a,
    0x12, 0x04, 0xb3, 0x01, 0x04, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0a, 0x04, 0x12,
    0x04, 0xb3, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0a, 0x05, 0x12, 0x04,
    0xb3, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0a, 0x01, 0x12, 0x04, 0xb3,
    0x01, 0x14, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0a, 0x03, 0x12, 0x04, 0xb3, 0x01,
    0x1e, 0x20, 0x0a, 0x44, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0b, 0x12, 0x04, 0xb4, 0x01, 0x04, 0x1d,
    0x22, 0x36, 0x20, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x74, 0x79, 0x70, 0x65, 0x2c, 0x20,
    0x69, 0x66, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x65, 0x74, 0x20, 0x77, 0x65, 0x20, 0x61, 0x73,
    0x73, 0x75, 0x6d, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x27, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c,
    0x74, 0x27, 0x20, 0x74, 0x79, 0x70, 0x65, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0b,
    0x04, 0x12, 0x04, 0xb4, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0b, 0x05,
    0x12, 0x04, 0xb4, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0b, 0x01, 0x12,
    0x04, 0xb4, 0x01, 0x13, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0b, 0x03, 0x12, 0x04,
    0xb4, 0x01, 0x1a, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0c, 0x12, 0x04, 0xb5, 0x01,
    0x04, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0c, 0x04, 0x12, 0x04, 0xb5, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0c, 0x05, 0x12, 0x04, 0xb5, 0x01, 0x0d, 0x12,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0c, 0x01, 0x12, 0x04, 0xb5, 0x01, 0x13, 0x1d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0c, 0x03, 0x12, 0x04, 0xb5, 0x01, 0x20, 0x22, 0x0a, 0x48,
    0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0d, 0x12, 0x04, 0xb7, 0x01, 0x04, 0x27, 0x1a, 0x3a, 0x20, 0x57,
    0x68, 0x65, 0x74, 0x68, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x75, 0x73, 0x65, 0x20, 0x70, 0x61,
    0x67, 0x69, 0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x73, 0x6f, 0x72, 0x74, 0x20, 0x66, 0x6f,
    0x72, 0x20, 0x6e, 0x6f, 0x6e, 0x2d, 0x70, 0x61, 0x67, 0x69, 0x6e, 0x61, 0x74, 0x65, 0x64, 0x20,
    0x71, 0x75, 0x65, 0x72, 0x69, 0x65, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0d,
    0x04, 0x12, 0x04, 0xb7, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0d, 0x05,
    0x12, 0x04, 0xb7, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0d, 0x01, 0x12,
    0x04, 0xb7, 0x01, 0x12, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0d, 0x03, 0x12, 0x04,
    0xb7, 0x01, 0x24, 0x26, 0x0a, 0x51, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0e, 0x12, 0x04, 0xb9, 0x01,
    0x04, 0x26, 0x1a, 0x1f, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6c, 0x6c, 0x65, 0x6c, 0x20, 0x65, 0x78,
    0x74, 0x72, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69,
    0x6f, 0x6e, 0x0a, 0x22, 0x22, 0x20, 0x63, 0x68, 0x6f, 0x70, 0x70, 0x65, 0x64, 0x20, 0x75, 0x70,
    0x20, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65, 0x20, 0x70, 0x6c, 0x61, 0x6e, 0x20, 0x70,
    0x65, 0x72, 0x2d, 0x72, 0x65, 0x71, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0e, 0x04,
    0x12, 0x04, 0xb9, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0e, 0x05, 0x12,
    0x04, 0xb9, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0e, 0x01, 0x12, 0x04,
    0xb9, 0x01, 0x13, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0e, 0x03, 0x12, 0x04, 0xb9,
    0x01, 0x23, 0x25, 0x0a, 0x53, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0f, 0x12, 0x04, 0xba, 0x01, 0x04,
    0x23, 0x22, 0x45, 0x20, 0x52, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x6b, 0x65, 0x79, 0x73, 0x2c, 0x20, 0x6f, 0x6e, 0x6c,
    0x79, 0x20, 0x77, 0x6f, 0x72, 0x6b, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x24, 0x62, 0x75,
    0x63, 0x6b, 0x65, 0x74, 0x2f, 0x24, 0x6b, 0x65, 0x79, 0x20, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x20,
    0x71, 0x75, 0x65, 0x72, 0x69, 0x65, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0f,
    0x04, 0x12, 0x04, 0xba, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0f, 0x05,
    0x12, 0x04, 0xba, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0f, 0x01, 0x12,
    0x04, 0xba, 0x01, 0x12, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0f, 0x03, 0x12, 0x04,
    0xba, 0x01, 0x20, 0x22, 0x0a, 0x2e, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x06, 0xbf, 0x01, 0x00, 0xc4,
    0x01, 0x01, 0x1a, 0x20, 0x20, 0x53, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x61, 0x72, 0x79, 0x20, 0x49,
    0x6e, 0x64, 0x65, 0x78, 0x20, 0x71, 0x75, 0x65, 0x72, 0x79, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x04, 0xbf, 0x01, 0x08,
    0x14, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x04, 0xc0, 0x01, 0x04, 0x1c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc0, 0x01, 0x04, 0x0c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x05, 0x12, 0x04, 0xc0, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc0, 0x01, 0x13, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc0, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0e, 0x02, 0x01, 0x12, 0x04, 0xc1, 0x01, 0x04, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xc1, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01,
    0x06, 0x12, 0x04, 0xc1, 0x01, 0x0d, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xc1, 0x01, 0x15, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xc1, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x02, 0x12, 0x04, 0xc2,
    0x01, 0x04, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x04, 0x12, 0x04, 0xc2, 0x01,
    0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x05, 0x12, 0x04, 0xc2, 0x01, 0x0d,
    0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x01, 0x12, 0x04, 0xc2, 0x01, 0x13, 0x1f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x03, 0x12, 0x04, 0xc2, 0x01, 0x22, 0x23, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x03, 0x12, 0x04, 0xc3, 0x01, 0x04, 0x1b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x03, 0x04, 0x12, 0x04, 0xc3, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x03, 0x05, 0x12, 0x04, 0xc3, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x03, 0x01, 0x12, 0x04, 0xc3, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x03, 0x03, 0x12, 0x04, 0xc3, 0x01, 0x19, 0x1a, 0x0a, 0x50, 0x0a, 0x02, 0x04, 0x0f, 0x12,
    0x06, 0xc7, 0x01, 0x00, 0xcb, 0x01, 0x01, 0x1a, 0x42, 0x20, 0x53, 0x74, 0x6f, 0x6c, 0x65, 0x6e,
    0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x43, 0x53, 0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20,
    0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2c, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20,
    0x75, 0x73, 0x65, 0x64, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e,
    0x5f, 0x62, 0x6f, 0x64, 0x79, 0x3d, 0x74, 0x72, 0x75, 0x65, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x0f, 0x01, 0x12, 0x04, 0xc7, 0x01, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00,
    0x12, 0x04, 0xc8, 0x01, 0x04, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xc8, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x06, 0x12, 0x04,
    0xc8, 0x01, 0x0d, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc8,
    0x01, 0x1c, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc8, 0x01,
    0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x01, 0x12, 0x04, 0xc9, 0x01, 0x04, 0x24,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc9, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x05, 0x12, 0x04, 0xc9, 0x01, 0x0d, 0x12, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc9, 0x01, 0x13, 0x1f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x01, 0x03, 0x12, 0x04, 0xc9, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x0f, 0x02, 0x02, 0x12, 0x04, 0xca, 0x01, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x02, 0x04, 0x12, 0x04, 0xca, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x02, 0x05, 0x12, 0x04, 0xca, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02,
    0x01, 0x12, 0x04, 0xca, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x03,
    0x12, 0x04, 0xca, 0x01, 0x19, 0x1a, 0x0a, 0x65, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x06, 0xd0, 0x01,
    0x00, 0xdc, 0x01, 0x01, 0x1a, 0x57, 0x20, 0x61, 0x64, 0x64, 0x65, 0x64, 0x20, 0x73, 0x6f, 0x6c,
    0x65, 0x6c, 0x79, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x72, 0x69, 0x61, 0x6b, 0x5f, 0x63, 0x73, 0x20,
    0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x6c, 0x79, 0x0a, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x66,
    0x6f, 0x6c, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x20, 0x61, 0x20, 0x62, 0x75,
    0x63, 0x6b, 0x65, 0x74, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x69,
    0x6e, 0x67, 0x0a, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x10, 0x01, 0x12, 0x04, 0xd0, 0x01, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10,
    0x02, 0x00, 0x12, 0x04, 0xd1, 0x01, 0x04, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xd1, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x05,
    0x12, 0x04, 0xd1, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xd1, 0x01, 0x13, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xd1, 0x01, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x01, 0x12, 0x04, 0xd2, 0x01,
    0x04, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd2, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x05, 0x12, 0x04, 0xd2, 0x01, 0x0d, 0x12,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd2, 0x01, 0x13, 0x1c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd2, 0x01, 0x1f, 0x20, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x10, 0x02, 0x02, 0x12, 0x04, 0xd3, 0x01, 0x04, 0x1f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x02, 0x04, 0x12, 0x04, 0xd3, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x02, 0x05, 0x12, 0x04, 0xd3, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x02, 0x01, 0x12, 0x04, 0xd3, 0x01, 0x13, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x02, 0x03, 0x12, 0x04, 0xd3, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x03,
    0x12, 0x04, 0xd4, 0x01, 0x04, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x04, 0x12,
    0x04, 0xd4, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x05, 0x12, 0x04,
    0xd4, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x01, 0x12, 0x04, 0xd4,
    0x01, 0x12, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x03, 0x12, 0x04, 0xd4, 0x01,
    0x1f, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x08, 0x12, 0x04, 0xd4, 0x01, 0x21,
    0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x07, 0x12, 0x04, 0xd4, 0x01, 0x2c, 0x30,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x04, 0x12, 0x04, 0xd5, 0x01, 0x04, 0x31, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x04, 0x04, 0x12, 0x04, 0xd5, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x04, 0x05, 0x12, 0x04, 0xd5, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x04, 0x01, 0x12, 0x04, 0xd5, 0x01, 0x12, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x04, 0x03, 0x12, 0x04, 0xd5, 0x01, 0x1d, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x04, 0x08, 0x12, 0x04, 0xd5, 0x01, 0x1f, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x04, 0x07, 0x12, 0x04, 0xd5, 0x01, 0x2a, 0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x05,
    0x12, 0x04, 0xd6, 0x01, 0x04, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x05, 0x04, 0x12,
    0x04, 0xd6, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x05, 0x05, 0x12, 0x04,
    0xd6, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x05, 0x01, 0x12, 0x04, 0xd6,
    0x01, 0x13, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x05, 0x03, 0x12, 0x04, 0xd6, 0x01,
    0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x06, 0x12, 0x04, 0xd7, 0x01, 0x04, 0x24,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x06, 0x04, 0x12, 0x04, 0xd7, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x06, 0x05, 0x12, 0x04, 0xd7, 0x01, 0x0d, 0x13, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x06, 0x01, 0x12, 0x04, 0xd7, 0x01, 0x14, 0x1f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x06, 0x03, 0x12, 0x04, 0xd7, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x10, 0x02, 0x07, 0x12, 0x04, 0xd8, 0x01, 0x04, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x07, 0x04, 0x12, 0x04, 0xd8, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x07, 0x05, 0x12, 0x04, 0xd8, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x07,
    0x01, 0x12, 0x04, 0xd8, 0x01, 0x14, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x07, 0x03,
    0x12, 0x04, 0xd8, 0x01, 0x1e, 0x1f, 0x0a, 0x44, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x08, 0x12, 0x04,
    0xd9, 0x01, 0x04, 0x1c, 0x22, 0x36, 0x20, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x74, 0x79,
    0x70, 0x65, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x65, 0x74, 0x20, 0x77,
    0x65, 0x20, 0x61, 0x73, 0x73, 0x75, 0x6d, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x27, 0x64, 0x65,
    0x66, 0x61, 0x75, 0x6c, 0x74, 0x27, 0x20, 0x74, 0x79, 0x70, 0x65, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x08, 0x04, 0x12, 0x04, 0xd9, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x08, 0x05, 0x12, 0x04, 0xd9, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x08, 0x01, 0x12, 0x04, 0xd9, 0x01, 0x13, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x08, 0x03, 0x12, 0x04, 0xd9, 0x01, 0x1a, 0x1b, 0x0a, 0x51, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x09,
    0x12, 0x04, 0xdb, 0x01, 0x04, 0x26, 0x1a, 0x1f, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6c, 0x6c, 0x65,
    0x6c, 0x20, 0x65, 0x78, 0x74, 0x72, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x65, 0x78, 0x74,
    0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x0a, 0x22, 0x22, 0x20, 0x63, 0x68, 0x6f, 0x70, 0x70, 0x65,
    0x64, 0x20, 0x75, 0x70, 0x20, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65, 0x20, 0x70, 0x6c,
    0x61, 0x6e, 0x20, 0x70, 0x65, 0x72, 0x2d, 0x72, 0x65, 0x71, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x09, 0x04, 0x12, 0x04, 0xdb, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x09, 0x05, 0x12, 0x04, 0xdb, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x09, 0x01, 0x12, 0x04, 0xdb, 0x01, 0x13, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x09,
    0x03, 0x12, 0x04, 0xdb, 0x01, 0x23, 0x25, 0x0a, 0x29, 0x0a, 0x02, 0x04, 0x11, 0x12, 0x06, 0xdf,
    0x01, 0x00, 0xe3, 0x01, 0x01, 0x1a, 0x1b, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x43, 0x53, 0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x66, 0x6f, 0x6c,
    0x64, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x11, 0x01, 0x12, 0x04, 0xdf, 0x01, 0x08, 0x17, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12, 0x04, 0xe0, 0x01, 0x04, 0x28, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe0, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x00, 0x06, 0x12, 0x04, 0xe0, 0x01, 0x0d, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe0, 0x01, 0x1c, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xe0, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02,
    0x01, 0x12, 0x04, 0xe1, 0x01, 0x04, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xe1, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x05, 0x12,
    0x04, 0xe1, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xe1, 0x01, 0x13, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe1,
    0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x02, 0x12, 0x04, 0xe2, 0x01, 0x04,
    0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x04, 0x12, 0x04, 0xe2, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x05, 0x12, 0x04, 0xe2, 0x01, 0x0d, 0x11, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x01, 0x12, 0x04, 0xe2, 0x01, 0x12, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x03, 0x12, 0x04, 0xe2, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x12, 0x12, 0x06, 0xe5, 0x01, 0x00, 0xe8, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x12, 0x01, 0x12, 0x04, 0xe5, 0x01, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x00,
    0x12, 0x04, 0xe6, 0x01, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xe6, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x05, 0x12, 0x04,
    0xe6, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe6,
    0x01, 0x13, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe6, 0x01,
    0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x01, 0x12, 0x04, 0xe7, 0x01, 0x04, 0x23,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x04, 0x12, 0x04, 0xe7, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x06, 0x12, 0x04, 0xe7, 0x01, 0x0d, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe7, 0x01, 0x18, 0x1e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x12, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe7, 0x01, 0x21, 0x22, 0x0a, 0x66, 0x0a, 0x02,
    0x04, 0x13, 0x12, 0x06, 0xec, 0x01, 0x00, 0xf8, 0x01, 0x01, 0x1a, 0x58, 0x20, 0x43, 0x6f, 0x6e,
    0x74, 0x65, 0x6e, 0x74, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x69, 0x6e, 0x63,
    0x6c, 0x75, 0x64, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x67, 0x65, 0x74, 0x2f, 0x70, 0x75, 0x74,
    0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x0a, 0x20, 0x48, 0x6f, 0x6c, 0x64,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x61, 0x6e, 0x64, 0x20,
    0x61, 0x73, 0x73, 0x6f, 0x63, 0x69, 0x61, 0x74, 0x65, 0x64, 0x20, 0x6d, 0x65, 0x74, 0x61, 0x64,
    0x61, 0x74, 0x61, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13, 0x01, 0x12, 0x04, 0xec, 0x01, 0x08,
    0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12, 0x04, 0xed, 0x01, 0x04, 0x1d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x04, 0x12, 0x04, 0xed, 0x01, 0x04, 0x0c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x05, 0x12, 0x04, 0xed, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x00, 0x01, 0x12, 0x04, 0xed, 0x01, 0x13, 0x18, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x00, 0x03, 0x12, 0x04, 0xed, 0x01, 0x1b, 0x1c, 0x0a, 0x25, 0x0a, 0x04, 0x04,
    0x13, 0x02, 0x01, 0x12, 0x04, 0xee, 0x01, 0x04, 0x24, 0x22, 0x17, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x6d, 0x65, 0x64, 0x69, 0x61, 0x20, 0x74, 0x79, 0x70, 0x65, 0x2f, 0x66, 0x6f, 0x72, 0x6d, 0x61,
    0x74, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x04, 0x12, 0x04, 0xee, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x05, 0x12, 0x04, 0xee, 0x01, 0x0d, 0x12,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x01, 0x12, 0x04, 0xee, 0x01, 0x13, 0x1f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x03, 0x12, 0x04, 0xee, 0x01, 0x22, 0x23, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x13, 0x02, 0x02, 0x12, 0x04, 0xef, 0x01, 0x04, 0x1f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x02, 0x04, 0x12, 0x04, 0xef, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x02, 0x05, 0x12, 0x04, 0xef, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x02, 0x01, 0x12, 0x04, 0xef, 0x01, 0x13, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x02, 0x03, 0x12, 0x04, 0xef, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x03,
    0x12, 0x04, 0xf0, 0x01, 0x04, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x04, 0x12,
    0x04, 0xf0, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x05, 0x12, 0x04,
    0xf0, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x01, 0x12, 0x04, 0xf0,
    0x01, 0x13, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x03, 0x12, 0x04, 0xf0, 0x01,
    0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x04, 0x12, 0x04, 0xf1, 0x01, 0x04, 0x1c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x04, 0x04, 0x12, 0x04, 0xf1, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x04, 0x05, 0x12, 0x04, 0xf1, 0x01, 0x0d, 0x12, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x04, 0x01, 0x12, 0x04, 0xf1, 0x01, 0x13, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x04, 0x03, 0x12, 0x04, 0xf1, 0x01, 0x1a, 0x1b, 0x0a, 0x28, 0x0a, 0x04,
    0x04, 0x13, 0x02, 0x05, 0x12, 0x04, 0xf2, 0x01, 0x04, 0x1f, 0x22, 0x1a, 0x20, 0x6c, 0x69, 0x6e,
    0x6b, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x20, 0x72, 0x65, 0x73, 0x6f,
    0x75, 0x72, 0x63, 0x65, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x05, 0x04, 0x12,
    0x04, 0xf2, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x05, 0x06, 0x12, 0x04,
    0xf2, 0x01, 0x0d, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x05, 0x01, 0x12, 0x04, 0xf2,
    0x01, 0x15, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x05, 0x03, 0x12, 0x04, 0xf2, 0x01,
    0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x06, 0x12, 0x04, 0xf3, 0x01, 0x04, 0x21,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x06, 0x04, 0x12, 0x04, 0xf3, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x06, 0x05, 0x12, 0x04, 0xf3, 0x01, 0x0d, 0x13, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x06, 0x01, 0x12, 0x04, 0xf3, 0x01, 0x14, 0x1c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x06, 0x03, 0x12, 0x04, 0xf3, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x13, 0x02, 0x07, 0x12, 0x04, 0xf4, 0x01, 0x04, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x07, 0x04, 0x12, 0x04, 0xf4, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x07, 0x05, 0x12, 0x04, 0xf4, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x07,
    0x01, 0x12, 0x04, 0xf4, 0x01, 0x14, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x07, 0x03,
    0x12, 0x04, 0xf4, 0x01, 0x25, 0x26, 0x0a, 0x34, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x08, 0x12, 0x04,
    0xf5, 0x01, 0x04, 0x22, 0x22, 0x26, 0x20, 0x75, 0x73, 0x65, 0x72, 0x20, 0x6d, 0x65, 0x74, 0x61,
    0x64, 0x61, 0x74, 0x61, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x08, 0x04, 0x12, 0x04, 0xf5, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x08, 0x06, 0x12, 0x04, 0xf5, 0x01, 0x0d, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x08, 0x01, 0x12, 0x04, 0xf5, 0x01, 0x15, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x08, 0x03, 0x12, 0x04, 0xf5, 0x01, 0x20, 0x21, 0x0a, 0x34, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x09,
    0x12, 0x04, 0xf6, 0x01, 0x04, 0x22, 0x22, 0x26, 0x20, 0x75, 0x73, 0x65, 0x72, 0x20, 0x6d, 0x65,
    0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x64, 0x20, 0x77, 0x69,
    0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x09, 0x04, 0x12, 0x04, 0xf6, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x09, 0x06, 0x12, 0x04, 0xf6, 0x01, 0x0d, 0x14, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x09, 0x01, 0x12, 0x04, 0xf6, 0x01, 0x15, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x09, 0x03, 0x12, 0x04, 0xf6, 0x01, 0x1f, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13,
    0x02, 0x0a, 0x12, 0x04, 0xf7, 0x01, 0x04, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x0a,
    0x04, 0x12, 0x04, 0xf7, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x0a, 0x05,
    0x12, 0x04, 0xf7, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x0a, 0x01, 0x12,
    0x04, 0xf7, 0x01, 0x12, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x0a, 0x03, 0x12, 0x04,
    0xf7, 0x01, 0x1c, 0x1e, 0x0a, 0x1d, 0x0a, 0x02, 0x04, 0x14, 0x12, 0x06, 0xfb, 0x01, 0x00, 0xff,
    0x01, 0x01, 0x1a, 0x0f, 0x20, 0x4c, 0x69, 0x6e, 0x6b, 0x20, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61,
    0x74, 0x61, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x14, 0x01, 0x12, 0x04, 0xfb, 0x01, 0x08, 0x0f,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x00, 0x12, 0x04, 0xfc, 0x01, 0x04, 0x1e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x04, 0x12, 0x04, 0xfc, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x00, 0x05, 0x12, 0x04, 0xfc, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x00, 0x01, 0x12, 0x04, 0xfc, 0x01, 0x13, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x00, 0x03, 0x12, 0x04, 0xfc, 0x01, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14,
    0x02, 0x01, 0x12, 0x04, 0xfd, 0x01, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01,
    0x04, 0x12, 0x04, 0xfd, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x05,
    0x12, 0x04, 0xfd, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xfd, 0x01, 0x13, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x03, 0x12, 0x04,
    0xfd, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x02, 0x12, 0x04, 0xfe, 0x01,
    0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x04, 0x12, 0x04, 0xfe, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x05, 0x12, 0x04, 0xfe, 0x01, 0x0d, 0x12,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x01, 0x12, 0x04, 0xfe, 0x01, 0x13, 0x16, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x03, 0x12, 0x04, 0xfe, 0x01, 0x19, 0x1a, 0x0a, 0x26,
    0x0a, 0x02, 0x04, 0x15, 0x12, 0x06, 0x82, 0x02, 0x00, 0x8a, 0x02, 0x01, 0x1a, 0x18, 0x20, 0x43,
    0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x20, 0x72, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x15, 0x01, 0x12, 0x04, 0x82,
    0x02, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x00, 0x12, 0x04, 0x83, 0x02, 0x04,
    0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x04, 0x12, 0x04, 0x83, 0x02, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x05, 0x12, 0x04, 0x83, 0x02, 0x0d, 0x12, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x01, 0x12, 0x04, 0x83, 0x02, 0x13, 0x19, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x03, 0x12, 0x04, 0x83, 0x02, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x15, 0x02, 0x01, 0x12, 0x04, 0x84, 0x02, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x01, 0x04, 0x12, 0x04, 0x84, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x01, 0x05, 0x12, 0x04, 0x84, 0x02, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x01, 0x01, 0x12, 0x04, 0x84, 0x02, 0x13, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01,
    0x03, 0x12, 0x04, 0x84, 0x02, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x02, 0x12,
    0x04, 0x85, 0x02, 0x04, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x04, 0x12, 0x04,
    0x85, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x05, 0x12, 0x04, 0x85,
    0x02, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x01, 0x12, 0x04, 0x85, 0x02,
    0x14, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x03, 0x12, 0x04, 0x85, 0x02, 0x1d,
    0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x03, 0x12, 0x04, 0x86, 0x02, 0x04, 0x1a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x04, 0x12, 0x04, 0x86, 0x02, 0x04, 0x0c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x05, 0x12, 0x04, 0x86, 0x02, 0x0d, 0x13, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x03, 0x01, 0x12, 0x04, 0x86, 0x02, 0x14, 0x15, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x03, 0x03, 0x12, 0x04, 0x86, 0x02, 0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x15, 0x02, 0x04, 0x12, 0x04, 0x87, 0x02, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x04, 0x04, 0x12, 0x04, 0x87, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x04,
    0x05, 0x12, 0x04, 0x87, 0x02, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x04, 0x01,
    0x12, 0x04, 0x87, 0x02, 0x14, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x04, 0x03, 0x12,
    0x04, 0x87, 0x02, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x05, 0x12, 0x04, 0x88,
    0x02, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x05, 0x04, 0x12, 0x04, 0x88, 0x02,
    0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x05, 0x05, 0x12, 0x04, 0x88, 0x02, 0x0d,
    0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x05, 0x01, 0x12, 0x04, 0x88, 0x02, 0x14, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x05, 0x03, 0x12, 0x04, 0x88, 0x02, 0x19, 0x1a, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x06, 0x12, 0x04, 0x89, 0x02, 0x04, 0x22, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x06, 0x04, 0x12, 0x04, 0x89, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x06, 0x05, 0x12, 0x04, 0x89, 0x02, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x06, 0x01, 0x12, 0x04, 0x89, 0x02, 0x12, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x06, 0x03, 0x12, 0x04, 0x89, 0x02, 0x20, 0x21, 0x0a, 0x44, 0x0a, 0x02, 0x04, 0x16, 0x12,
    0x06, 0x8d, 0x02, 0x00, 0x8f, 0x02, 0x01, 0x1a, 0x36, 0x20, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x65,
    0x72, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x3f, 0x20, 0x4e, 0x6f, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x7c, 0x20,
    0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x0a, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x16, 0x01, 0x12, 0x04, 0x8d, 0x02, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x16, 0x02, 0x00, 0x12, 0x04, 0x8e, 0x02, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x8e, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x00, 0x05, 0x12, 0x04, 0x8e, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00,
    0x01, 0x12, 0x04, 0x8e, 0x02, 0x18, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x03,
    0x12, 0x04, 0x8e, 0x02, 0x20, 0x21, 0x0a, 0x1d, 0x0a, 0x02, 0x04, 0x17, 0x12, 0x06, 0x92, 0x02,
    0x00, 0x99, 0x02, 0x01, 0x1a, 0x0f, 0x20, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x20, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x17, 0x01, 0x12, 0x04, 0x92, 0x02,
    0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x00, 0x12, 0x04, 0x93, 0x02, 0x04, 0x1e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x04, 0x12, 0x04, 0x93, 0x02, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x05, 0x12, 0x04, 0x93, 0x02, 0x0d, 0x12, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x01, 0x12, 0x04, 0x93, 0x02, 0x13, 0x19, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x00, 0x03, 0x12, 0x04, 0x93, 0x02, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x17, 0x02, 0x01, 0x12, 0x04, 0x94, 0x02, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x94, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02,
    0x01, 0x05, 0x12, 0x04, 0x94, 0x02, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01,
    0x01, 0x12, 0x04, 0x94, 0x02, 0x13, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x03,
    0x12, 0x04, 0x94, 0x02, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x02, 0x12, 0x04,
    0x95, 0x02, 0x04, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x04, 0x12, 0x04, 0x95,
    0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x05, 0x12, 0x04, 0x95, 0x02,
    0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x01, 0x12, 0x04, 0x95, 0x02, 0x14,
    0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x03, 0x12, 0x04, 0x95, 0x02, 0x18, 0x19,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x03, 0x12, 0x04, 0x96, 0x02, 0x04, 0x1b, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x17, 0x02, 0x03, 0x04, 0x12, 0x04, 0x96, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x03, 0x05, 0x12, 0x04, 0x96, 0x02, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x17, 0x02, 0x03, 0x01, 0x12, 0x04, 0x96, 0x02, 0x14, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x17, 0x02, 0x03, 0x03, 0x12, 0x04, 0x96, 0x02, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17,
    0x02, 0x04, 0x12, 0x04, 0x97, 0x02, 0x04, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x04,
    0x04, 0x12, 0x04, 0x97, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x04, 0x05,
    0x12, 0x04, 0x97, 0x02, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x04, 0x01, 0x12,
    0x04, 0x97, 0x02, 0x12, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x04, 0x03, 0x12, 0x04,
    0x97, 0x02, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x05, 0x12, 0x04, 0x98, 0x02,
    0x04, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x05, 0x04, 0x12, 0x04, 0x98, 0x02, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x05, 0x05, 0x12, 0x04, 0x98, 0x02, 0x0d, 0x11,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x05, 0x01, 0x12, 0x04, 0x98, 0x02, 0x12, 0x1d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x05, 0x03, 0x12, 0x04, 0x98, 0x02, 0x20, 0x21, 0x0a, 0x26,
    0x0a, 0x02, 0x04, 0x18, 0x12, 0x06, 0x9c, 0x02, 0x00, 0x9e, 0x02, 0x01, 0x1a, 0x18, 0x20, 0x43,
    0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x72, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x18, 0x01, 0x12, 0x04, 0x9c,
    0x02, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x00, 0x12, 0x04, 0x9d, 0x02, 0x04,
    0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9d, 0x02, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x05, 0x12, 0x04, 0x9d, 0x02, 0x0d, 0x13, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9d, 0x02, 0x14, 0x19, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9d, 0x02, 0x1c, 0x1d, 0x0a, 0x2f, 0x0a,
    0x02, 0x04, 0x19, 0x12, 0x06, 0xa1, 0x02, 0x00, 0xa5, 0x02, 0x01, 0x1a, 0x21, 0x20, 0x47, 0x65,
    0x74, 0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x2d, 0x6b, 0x65, 0x79, 0x20, 0x70, 0x72, 0x65,
    0x66, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x19, 0x01, 0x12, 0x04, 0xa1, 0x02, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x19, 0x02, 0x00, 0x12, 0x04, 0xa2, 0x02, 0x04, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xa2, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00,
    0x05, 0x12, 0x04, 0xa2, 0x02, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xa2, 0x02, 0x13, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xa2, 0x02, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x01, 0x12, 0x04, 0xa3,
    0x02, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa3, 0x02,
    0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x05, 0x12, 0x04, 0xa3, 0x02, 0x0d,
    0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa3, 0x02, 0x13, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa3, 0x02, 0x19, 0x1a, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x02, 0x12, 0x04, 0xa4, 0x02, 0x04, 0x1c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x19, 0x02, 0x02, 0x04, 0x12, 0x04, 0xa4, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x19, 0x02, 0x02, 0x05, 0x12, 0x04, 0xa4, 0x02, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x19, 0x02, 0x02, 0x01, 0x12, 0x04, 0xa4, 0x02, 0x13, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19,
    0x02, 0x02, 0x03, 0x12, 0x04, 0xa4, 0x02, 0x1a, 0x1b, 0x0a, 0x30, 0x0a, 0x02, 0x04, 0x1a, 0x12,
    0x06, 0xa8, 0x02, 0x00, 0xaa, 0x02, 0x01, 0x1a, 0x22, 0x20, 0x47, 0x65, 0x74, 0x20, 0x62, 0x75,
    0x63, 0x6b, 0x65, 0x74, 0x2d, 0x6b, 0x65, 0x79, 0x20, 0x70, 0x72, 0x65, 0x66, 0x6c, 0x69, 0x73,
    0x74, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x1a, 0x01, 0x12, 0x04, 0xa8, 0x02, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x00,
    0x12, 0x04, 0xa9, 0x02, 0x04, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xa9, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x06, 0x12, 0x04,
    0xa9, 0x02, 0x0d, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa9,
    0x02, 0x26, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa9, 0x02,
    0x31, 0x32, 0x0a, 0x1d, 0x0a, 0x02, 0x04, 0x1b, 0x12, 0x06, 0xad, 0x02, 0x00, 0xb1, 0x02, 0x01,
    0x1a, 0x0f, 0x20, 0x50, 0x72, 0x65, 0x66, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x69, 0x74, 0x65, 0x6d,
    0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1b, 0x01, 0x12, 0x04, 0xad, 0x02, 0x08, 0x20, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x1b, 0x02, 0x00, 0x12, 0x04, 0xae, 0x02, 0x04, 0x21, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1b, 0x02, 0x00, 0x04, 0x12, 0x04, 0xae, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1b, 0x02, 0x00, 0x05, 0x12, 0x04, 0xae, 0x02, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xae, 0x02, 0x13, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xae, 0x02, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x01,
    0x12, 0x04, 0xaf, 0x02, 0x04, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xaf, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x05, 0x12, 0x04,
    0xaf, 0x02, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x01, 0x12, 0x04, 0xaf,
    0x02, 0x13, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x03, 0x12, 0x04, 0xaf, 0x02,
    0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x02, 0x12, 0x04, 0xb0, 0x02, 0x04, 0x1f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x02, 0x04, 0x12, 0x04, 0xb0, 0x02, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x02, 0x05, 0x12, 0x04, 0xb0, 0x02, 0x0d, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1b, 0x02, 0x02, 0x01, 0x12, 0x04, 0xb0, 0x02, 0x13, 0x1a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1b, 0x02, 0x02, 0x03, 0x12, 0x04, 0xb0, 0x02, 0x1d, 0x1e, 0x0a, 0x4a, 0x0a, 0x02,
    0x04, 0x1c, 0x12, 0x06, 0xb5, 0x02, 0x00, 0xbb, 0x02, 0x01, 0x1a, 0x3c, 0x20, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x20, 0x61, 0x20, 0x73, 0x65, 0x67, 0x6d, 0x65, 0x6e, 0x74, 0x65, 0x64,
    0x20, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65, 0x20, 0x70, 0x6c, 0x61, 0x6e, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64,
    0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1c, 0x01, 0x12,
    0x04, 0xb5, 0x02, 0x08, 0x16, 0x0a, 0x44, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x00, 0x12, 0x04, 0xb6,
    0x02, 0x04, 0x1c, 0x22, 0x36, 0x20, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x74, 0x79, 0x70,
    0x65, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x65, 0x74, 0x20, 0x77, 0x65,
    0x20, 0x61, 0x73, 0x73, 0x75, 0x6d, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x27, 0x64, 0x65, 0x66,
    0x61, 0x75, 0x6c, 0x74, 0x27, 0x20, 0x74, 0x79, 0x70, 0x65, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1c, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb6, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xb6, 0x02, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xb6, 0x02, 0x13, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xb6, 0x02, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x01, 0x12,
    0x04, 0xb7, 0x02, 0x04, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x04, 0x12, 0x04,
    0xb7, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x05, 0x12, 0x04, 0xb7,
    0x02, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb7, 0x02,
    0x13, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x03, 0x12, 0x04, 0xb7, 0x02, 0x1c,
    0x1d, 0x0a, 0xe2, 0x01, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x02, 0x12, 0x04, 0xb8, 0x02, 0x04, 0x27,
    0x22, 0xd3, 0x01, 0x20, 0x49, 0x66, 0x20, 0x75, 0x6e, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x64,
    0x2c, 0x20, 0x77, 0x65, 0x20, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x20, 0x61, 0x20, 0x6e, 0x6f, 0x72,
    0x6d, 0x61, 0x6c, 0x20, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65, 0x20, 0x70, 0x6c, 0x61,
    0x6e, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x3c, 0x72, 0x69, 0x6e, 0x67, 0x5f, 0x73, 0x69, 0x7a, 0x65,
    0x2c, 0x20, 0x77, 0x65, 0x20, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x76,
    0x65, 0x72, 0x61, 0x67, 0x65, 0x20, 0x70, 0x6c, 0x61, 0x6e, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20,
    0x65, 0x78, 0x61, 0x63, 0x74, 0x6c, 0x79, 0x20, 0x72, 0x69, 0x6e, 0x67, 0x5f, 0x73, 0x69, 0x7a,
    0x65, 0x20, 0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x2c, 0x20, 0x61, 0x6e, 0x79, 0x74, 0x68,
    0x69, 0x6e, 0x67, 0x20, 0x6c, 0x61, 0x72, 0x67, 0x65, 0x72, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20,
    0x68, 0x61, 0x76, 0x65, 0x20, 0x61, 0x20, 0x70, 0x6f, 0x77, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20,
    0x32, 0x20, 0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x20, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x69,
    0x6e, 0x67, 0x20, 0x6b, 0x65, 0x79, 0x73, 0x70, 0x61, 0x63, 0x65, 0x73, 0x20, 0x73, 0x6d, 0x61,
    0x6c, 0x6c, 0x65, 0x72, 0x20, 0x74, 0x68, 0x61, 0x6e, 0x20, 0x61, 0x20, 0x70, 0x61, 0x72, 0x74,
    0x69, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x02, 0x04, 0x12,
    0x04, 0xb8, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x02, 0x05, 0x12, 0x04,
    0xb8, 0x02, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x02, 0x01, 0x12, 0x04, 0xb8,
    0x02, 0x14, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x02, 0x03, 0x12, 0x04, 0xb8, 0x02,
    0x25, 0x26, 0x0a, 0x24, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x03, 0x12, 0x04, 0xb9, 0x02, 0x04, 0x25,
    0x22, 0x16, 0x20, 0x46, 0x6f, 0x72, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x75, 0x72, 0x65, 0x20, 0x72,
    0x65, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x03,
    0x04, 0x12, 0x04, 0xb9, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x03, 0x05,
    0x12, 0x04, 0xb9, 0x02, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x03, 0x01, 0x12,
    0x04, 0xb9, 0x02, 0x13, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x03, 0x03, 0x12, 0x04,
    0xb9, 0x02, 0x23, 0x24, 0x0a, 0x67, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x04, 0x12, 0x04, 0xba, 0x02,
    0x04, 0x29, 0x22, 0x59, 0x20, 0x4f, 0x74, 0x68, 0x65, 0x72, 0x20, 0x63, 0x6f, 0x76, 0x65, 0x72,
    0x61, 0x67, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x73, 0x20, 0x74, 0x68, 0x61,
    0x74, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x20, 0x74, 0x6f,
    0x20, 0x61, 0x73, 0x73, 0x69, 0x73, 0x74, 0x20, 0x52, 0x69, 0x61, 0x6b, 0x20, 0x69, 0x6e, 0x20,
    0x64, 0x65, 0x63, 0x69, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x77, 0x68, 0x61, 0x74, 0x20, 0x6e, 0x6f,
    0x64, 0x65, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x76, 0x6f, 0x69, 0x64, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1c, 0x02, 0x04, 0x04, 0x12, 0x04, 0xba, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1c, 0x02, 0x04, 0x05, 0x12, 0x04, 0xba, 0x02, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1c, 0x02, 0x04, 0x01, 0x12, 0x04, 0xba, 0x02, 0x13, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c,
    0x02, 0x04, 0x03, 0x12, 0x04, 0xba, 0x02, 0x27, 0x28, 0x0a, 0x30, 0x0a, 0x02, 0x04, 0x1d, 0x12,
    0x06, 0xbe, 0x02, 0x00, 0xc0, 0x02, 0x01, 0x1a, 0x22, 0x20, 0x53, 0x65, 0x67, 0x6d, 0x65, 0x6e,
    0x74, 0x65, 0x64, 0x20, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65, 0x20, 0x70, 0x6c, 0x61,
    0x6e, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x1d, 0x01, 0x12, 0x04, 0xbe, 0x02, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x00,
    0x12, 0x04, 0xbf, 0x02, 0x03, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xbf, 0x02, 0x03, 0x0b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x06, 0x12, 0x04,
    0xbf, 0x02, 0x0c, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbf,
    0x02, 0x1d, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x03, 0x12, 0x04, 0xbf, 0x02,
    0x27, 0x28, 0x0a, 0x2a, 0x0a, 0x02, 0x04, 0x1e, 0x12, 0x06, 0xc3, 0x02, 0x00, 0xc8, 0x02, 0x01,
    0x1a, 0x1c, 0x20, 0x53, 0x65, 0x67, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20,
    0x63, 0x6f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65, 0x20, 0x70, 0x6c, 0x61, 0x6e, 0x0a, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x1e, 0x01, 0x12, 0x04, 0xc3, 0x02, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x1e, 0x02, 0x00, 0x12, 0x04, 0xc4, 0x02, 0x04, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xc4, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00,
    0x05, 0x12, 0x04, 0xc4, 0x02, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xc4, 0x02, 0x13, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xc4, 0x02, 0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1e, 0x02, 0x01, 0x12, 0x04, 0xc5,
    0x02, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc5, 0x02,
    0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x05, 0x12, 0x04, 0xc5, 0x02, 0x0d,
    0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc5, 0x02, 0x14, 0x18,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x03, 0x12, 0x04, 0xc5, 0x02, 0x1b, 0x1c, 0x0a,
    0x47, 0x0a, 0x04, 0x04, 0x1e, 0x02, 0x02, 0x12, 0x04, 0xc6, 0x02, 0x04, 0x25, 0x22, 0x39, 0x20,
    0x53, 0x6f, 0x6d, 0x65, 0x20, 0x68, 0x75, 0x6d, 0x61, 0x6e, 0x20, 0x72, 0x65, 0x61, 0x64, 0x61,
    0x62, 0x6c, 0x65, 0x20, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6b, 0x65, 0x79, 0x73, 0x70, 0x61, 0x63, 0x65, 0x20,
    0x63, 0x6f, 0x76, 0x65, 0x72, 0x65, 0x64, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x02,
    0x04, 0x12, 0x04, 0xc6, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x02, 0x05,
    0x12, 0x04, 0xc6, 0x02, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x02, 0x01, 0x12,
    0x04, 0xc6, 0x02, 0x13, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x02, 0x03, 0x12, 0x04,
    0xc6, 0x02, 0x23, 0x24, 0x0a, 0x35, 0x0a, 0x04, 0x04, 0x1e, 0x02, 0x03, 0x12, 0x04, 0xc7, 0x02,
    0x04, 0x25, 0x22, 0x27, 0x20, 0x4f, 0x70, 0x61, 0x71, 0x75, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74,
    0x65, 0x78, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x61, 0x73, 0x73, 0x20, 0x69, 0x6e, 0x74, 0x6f,
    0x20, 0x32, 0x49, 0x20, 0x71, 0x75, 0x65, 0x72, 0x79, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1e, 0x02, 0x03, 0x04, 0x12, 0x04, 0xc7, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e,
    0x02, 0x03, 0x05, 0x12, 0x04, 0xc7, 0x02, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02,
    0x03, 0x01, 0x12, 0x04, 0xc7, 0x02, 0x13, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x03,
    0x03, 0x12, 0x04, 0xc7, 0x02, 0x23, 0x24,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
