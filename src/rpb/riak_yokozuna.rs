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
pub struct RpbYokozunaIndex {
    // message fields
    name: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    schema: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    n_val: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbYokozunaIndex {}

impl RpbYokozunaIndex {
    pub fn new() -> RpbYokozunaIndex {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbYokozunaIndex {
        static mut instance: ::protobuf::lazy::Lazy<RpbYokozunaIndex> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbYokozunaIndex,
        };
        unsafe {
            instance.get(RpbYokozunaIndex::new)
        }
    }

    // required bytes name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::vec::Vec<u8>) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::vec::Vec<u8> {
        self.name.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_name(&self) -> &[u8] {
        match self.name.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.name
    }

    // optional bytes schema = 2;

    pub fn clear_schema(&mut self) {
        self.schema.clear();
    }

    pub fn has_schema(&self) -> bool {
        self.schema.is_some()
    }

    // Param is passed by value, moved
    pub fn set_schema(&mut self, v: ::std::vec::Vec<u8>) {
        self.schema = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_schema(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.schema.is_none() {
            self.schema.set_default();
        };
        self.schema.as_mut().unwrap()
    }

    // Take field
    pub fn take_schema(&mut self) -> ::std::vec::Vec<u8> {
        self.schema.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_schema(&self) -> &[u8] {
        match self.schema.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_schema_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.schema
    }

    fn mut_schema_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.schema
    }

    // optional uint32 n_val = 3;

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
}

impl ::protobuf::Message for RpbYokozunaIndex {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.schema)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.n_val = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.schema.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.n_val {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.schema.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.n_val {
            os.write_uint32(3, v)?;
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

impl ::protobuf::MessageStatic for RpbYokozunaIndex {
    fn new() -> RpbYokozunaIndex {
        RpbYokozunaIndex::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbYokozunaIndex>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "name",
                    RpbYokozunaIndex::get_name_for_reflect,
                    RpbYokozunaIndex::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "schema",
                    RpbYokozunaIndex::get_schema_for_reflect,
                    RpbYokozunaIndex::mut_schema_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "n_val",
                    RpbYokozunaIndex::get_n_val_for_reflect,
                    RpbYokozunaIndex::mut_n_val_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbYokozunaIndex>(
                    "RpbYokozunaIndex",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbYokozunaIndex {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_schema();
        self.clear_n_val();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbYokozunaIndex {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbYokozunaIndex {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbYokozunaIndexGetReq {
    // message fields
    name: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbYokozunaIndexGetReq {}

impl RpbYokozunaIndexGetReq {
    pub fn new() -> RpbYokozunaIndexGetReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbYokozunaIndexGetReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbYokozunaIndexGetReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbYokozunaIndexGetReq,
        };
        unsafe {
            instance.get(RpbYokozunaIndexGetReq::new)
        }
    }

    // optional bytes name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::vec::Vec<u8>) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::vec::Vec<u8> {
        self.name.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_name(&self) -> &[u8] {
        match self.name.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.name
    }
}

impl ::protobuf::Message for RpbYokozunaIndexGetReq {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.name)?;
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
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
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

impl ::protobuf::MessageStatic for RpbYokozunaIndexGetReq {
    fn new() -> RpbYokozunaIndexGetReq {
        RpbYokozunaIndexGetReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbYokozunaIndexGetReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "name",
                    RpbYokozunaIndexGetReq::get_name_for_reflect,
                    RpbYokozunaIndexGetReq::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbYokozunaIndexGetReq>(
                    "RpbYokozunaIndexGetReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbYokozunaIndexGetReq {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbYokozunaIndexGetReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbYokozunaIndexGetReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbYokozunaIndexGetResp {
    // message fields
    index: ::protobuf::RepeatedField<RpbYokozunaIndex>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbYokozunaIndexGetResp {}

impl RpbYokozunaIndexGetResp {
    pub fn new() -> RpbYokozunaIndexGetResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbYokozunaIndexGetResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbYokozunaIndexGetResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbYokozunaIndexGetResp,
        };
        unsafe {
            instance.get(RpbYokozunaIndexGetResp::new)
        }
    }

    // repeated .RpbYokozunaIndex index = 1;

    pub fn clear_index(&mut self) {
        self.index.clear();
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: ::protobuf::RepeatedField<RpbYokozunaIndex>) {
        self.index = v;
    }

    // Mutable pointer to the field.
    pub fn mut_index(&mut self) -> &mut ::protobuf::RepeatedField<RpbYokozunaIndex> {
        &mut self.index
    }

    // Take field
    pub fn take_index(&mut self) -> ::protobuf::RepeatedField<RpbYokozunaIndex> {
        ::std::mem::replace(&mut self.index, ::protobuf::RepeatedField::new())
    }

    pub fn get_index(&self) -> &[RpbYokozunaIndex] {
        &self.index
    }

    fn get_index_for_reflect(&self) -> &::protobuf::RepeatedField<RpbYokozunaIndex> {
        &self.index
    }

    fn mut_index_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RpbYokozunaIndex> {
        &mut self.index
    }
}

impl ::protobuf::Message for RpbYokozunaIndexGetResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.index)?;
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
        for value in &self.index {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.index {
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

impl ::protobuf::MessageStatic for RpbYokozunaIndexGetResp {
    fn new() -> RpbYokozunaIndexGetResp {
        RpbYokozunaIndexGetResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbYokozunaIndexGetResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RpbYokozunaIndex>>(
                    "index",
                    RpbYokozunaIndexGetResp::get_index_for_reflect,
                    RpbYokozunaIndexGetResp::mut_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbYokozunaIndexGetResp>(
                    "RpbYokozunaIndexGetResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbYokozunaIndexGetResp {
    fn clear(&mut self) {
        self.clear_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbYokozunaIndexGetResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbYokozunaIndexGetResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbYokozunaIndexPutReq {
    // message fields
    index: ::protobuf::SingularPtrField<RpbYokozunaIndex>,
    timeout: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbYokozunaIndexPutReq {}

impl RpbYokozunaIndexPutReq {
    pub fn new() -> RpbYokozunaIndexPutReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbYokozunaIndexPutReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbYokozunaIndexPutReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbYokozunaIndexPutReq,
        };
        unsafe {
            instance.get(RpbYokozunaIndexPutReq::new)
        }
    }

    // required .RpbYokozunaIndex index = 1;

    pub fn clear_index(&mut self) {
        self.index.clear();
    }

    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: RpbYokozunaIndex) {
        self.index = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_index(&mut self) -> &mut RpbYokozunaIndex {
        if self.index.is_none() {
            self.index.set_default();
        };
        self.index.as_mut().unwrap()
    }

    // Take field
    pub fn take_index(&mut self) -> RpbYokozunaIndex {
        self.index.take().unwrap_or_else(|| RpbYokozunaIndex::new())
    }

    pub fn get_index(&self) -> &RpbYokozunaIndex {
        self.index.as_ref().unwrap_or_else(|| RpbYokozunaIndex::default_instance())
    }

    fn get_index_for_reflect(&self) -> &::protobuf::SingularPtrField<RpbYokozunaIndex> {
        &self.index
    }

    fn mut_index_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RpbYokozunaIndex> {
        &mut self.index
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
}

impl ::protobuf::Message for RpbYokozunaIndexPutReq {
    fn is_initialized(&self) -> bool {
        if self.index.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.index)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.timeout = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.index.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.timeout {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.index.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.timeout {
            os.write_uint32(2, v)?;
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

impl ::protobuf::MessageStatic for RpbYokozunaIndexPutReq {
    fn new() -> RpbYokozunaIndexPutReq {
        RpbYokozunaIndexPutReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbYokozunaIndexPutReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RpbYokozunaIndex>>(
                    "index",
                    RpbYokozunaIndexPutReq::get_index_for_reflect,
                    RpbYokozunaIndexPutReq::mut_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "timeout",
                    RpbYokozunaIndexPutReq::get_timeout_for_reflect,
                    RpbYokozunaIndexPutReq::mut_timeout_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbYokozunaIndexPutReq>(
                    "RpbYokozunaIndexPutReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbYokozunaIndexPutReq {
    fn clear(&mut self) {
        self.clear_index();
        self.clear_timeout();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbYokozunaIndexPutReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbYokozunaIndexPutReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbYokozunaIndexDeleteReq {
    // message fields
    name: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbYokozunaIndexDeleteReq {}

impl RpbYokozunaIndexDeleteReq {
    pub fn new() -> RpbYokozunaIndexDeleteReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbYokozunaIndexDeleteReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbYokozunaIndexDeleteReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbYokozunaIndexDeleteReq,
        };
        unsafe {
            instance.get(RpbYokozunaIndexDeleteReq::new)
        }
    }

    // required bytes name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::vec::Vec<u8>) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::vec::Vec<u8> {
        self.name.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_name(&self) -> &[u8] {
        match self.name.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.name
    }
}

impl ::protobuf::Message for RpbYokozunaIndexDeleteReq {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.name)?;
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
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
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

impl ::protobuf::MessageStatic for RpbYokozunaIndexDeleteReq {
    fn new() -> RpbYokozunaIndexDeleteReq {
        RpbYokozunaIndexDeleteReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbYokozunaIndexDeleteReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "name",
                    RpbYokozunaIndexDeleteReq::get_name_for_reflect,
                    RpbYokozunaIndexDeleteReq::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbYokozunaIndexDeleteReq>(
                    "RpbYokozunaIndexDeleteReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbYokozunaIndexDeleteReq {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbYokozunaIndexDeleteReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbYokozunaIndexDeleteReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbYokozunaSchema {
    // message fields
    name: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    content: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbYokozunaSchema {}

impl RpbYokozunaSchema {
    pub fn new() -> RpbYokozunaSchema {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbYokozunaSchema {
        static mut instance: ::protobuf::lazy::Lazy<RpbYokozunaSchema> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbYokozunaSchema,
        };
        unsafe {
            instance.get(RpbYokozunaSchema::new)
        }
    }

    // required bytes name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::vec::Vec<u8>) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::vec::Vec<u8> {
        self.name.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_name(&self) -> &[u8] {
        match self.name.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.name
    }

    // optional bytes content = 2;

    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    pub fn has_content(&self) -> bool {
        self.content.is_some()
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ::std::vec::Vec<u8>) {
        self.content = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.content.is_none() {
            self.content.set_default();
        };
        self.content.as_mut().unwrap()
    }

    // Take field
    pub fn take_content(&mut self) -> ::std::vec::Vec<u8> {
        self.content.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_content(&self) -> &[u8] {
        match self.content.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_content_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.content
    }

    fn mut_content_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.content
    }
}

impl ::protobuf::Message for RpbYokozunaSchema {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.content)?;
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
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.content.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.content.as_ref() {
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

impl ::protobuf::MessageStatic for RpbYokozunaSchema {
    fn new() -> RpbYokozunaSchema {
        RpbYokozunaSchema::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbYokozunaSchema>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "name",
                    RpbYokozunaSchema::get_name_for_reflect,
                    RpbYokozunaSchema::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "content",
                    RpbYokozunaSchema::get_content_for_reflect,
                    RpbYokozunaSchema::mut_content_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbYokozunaSchema>(
                    "RpbYokozunaSchema",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbYokozunaSchema {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_content();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbYokozunaSchema {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbYokozunaSchema {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbYokozunaSchemaPutReq {
    // message fields
    schema: ::protobuf::SingularPtrField<RpbYokozunaSchema>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbYokozunaSchemaPutReq {}

impl RpbYokozunaSchemaPutReq {
    pub fn new() -> RpbYokozunaSchemaPutReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbYokozunaSchemaPutReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbYokozunaSchemaPutReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbYokozunaSchemaPutReq,
        };
        unsafe {
            instance.get(RpbYokozunaSchemaPutReq::new)
        }
    }

    // required .RpbYokozunaSchema schema = 1;

    pub fn clear_schema(&mut self) {
        self.schema.clear();
    }

    pub fn has_schema(&self) -> bool {
        self.schema.is_some()
    }

    // Param is passed by value, moved
    pub fn set_schema(&mut self, v: RpbYokozunaSchema) {
        self.schema = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_schema(&mut self) -> &mut RpbYokozunaSchema {
        if self.schema.is_none() {
            self.schema.set_default();
        };
        self.schema.as_mut().unwrap()
    }

    // Take field
    pub fn take_schema(&mut self) -> RpbYokozunaSchema {
        self.schema.take().unwrap_or_else(|| RpbYokozunaSchema::new())
    }

    pub fn get_schema(&self) -> &RpbYokozunaSchema {
        self.schema.as_ref().unwrap_or_else(|| RpbYokozunaSchema::default_instance())
    }

    fn get_schema_for_reflect(&self) -> &::protobuf::SingularPtrField<RpbYokozunaSchema> {
        &self.schema
    }

    fn mut_schema_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RpbYokozunaSchema> {
        &mut self.schema
    }
}

impl ::protobuf::Message for RpbYokozunaSchemaPutReq {
    fn is_initialized(&self) -> bool {
        if self.schema.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.schema)?;
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
        if let Some(v) = self.schema.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.schema.as_ref() {
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

impl ::protobuf::MessageStatic for RpbYokozunaSchemaPutReq {
    fn new() -> RpbYokozunaSchemaPutReq {
        RpbYokozunaSchemaPutReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbYokozunaSchemaPutReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RpbYokozunaSchema>>(
                    "schema",
                    RpbYokozunaSchemaPutReq::get_schema_for_reflect,
                    RpbYokozunaSchemaPutReq::mut_schema_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbYokozunaSchemaPutReq>(
                    "RpbYokozunaSchemaPutReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbYokozunaSchemaPutReq {
    fn clear(&mut self) {
        self.clear_schema();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbYokozunaSchemaPutReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbYokozunaSchemaPutReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbYokozunaSchemaGetReq {
    // message fields
    name: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbYokozunaSchemaGetReq {}

impl RpbYokozunaSchemaGetReq {
    pub fn new() -> RpbYokozunaSchemaGetReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbYokozunaSchemaGetReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbYokozunaSchemaGetReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbYokozunaSchemaGetReq,
        };
        unsafe {
            instance.get(RpbYokozunaSchemaGetReq::new)
        }
    }

    // required bytes name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::vec::Vec<u8>) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::vec::Vec<u8> {
        self.name.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_name(&self) -> &[u8] {
        match self.name.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.name
    }
}

impl ::protobuf::Message for RpbYokozunaSchemaGetReq {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.name)?;
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
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
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

impl ::protobuf::MessageStatic for RpbYokozunaSchemaGetReq {
    fn new() -> RpbYokozunaSchemaGetReq {
        RpbYokozunaSchemaGetReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbYokozunaSchemaGetReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "name",
                    RpbYokozunaSchemaGetReq::get_name_for_reflect,
                    RpbYokozunaSchemaGetReq::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbYokozunaSchemaGetReq>(
                    "RpbYokozunaSchemaGetReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbYokozunaSchemaGetReq {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbYokozunaSchemaGetReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbYokozunaSchemaGetReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpbYokozunaSchemaGetResp {
    // message fields
    schema: ::protobuf::SingularPtrField<RpbYokozunaSchema>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbYokozunaSchemaGetResp {}

impl RpbYokozunaSchemaGetResp {
    pub fn new() -> RpbYokozunaSchemaGetResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbYokozunaSchemaGetResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbYokozunaSchemaGetResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbYokozunaSchemaGetResp,
        };
        unsafe {
            instance.get(RpbYokozunaSchemaGetResp::new)
        }
    }

    // required .RpbYokozunaSchema schema = 1;

    pub fn clear_schema(&mut self) {
        self.schema.clear();
    }

    pub fn has_schema(&self) -> bool {
        self.schema.is_some()
    }

    // Param is passed by value, moved
    pub fn set_schema(&mut self, v: RpbYokozunaSchema) {
        self.schema = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_schema(&mut self) -> &mut RpbYokozunaSchema {
        if self.schema.is_none() {
            self.schema.set_default();
        };
        self.schema.as_mut().unwrap()
    }

    // Take field
    pub fn take_schema(&mut self) -> RpbYokozunaSchema {
        self.schema.take().unwrap_or_else(|| RpbYokozunaSchema::new())
    }

    pub fn get_schema(&self) -> &RpbYokozunaSchema {
        self.schema.as_ref().unwrap_or_else(|| RpbYokozunaSchema::default_instance())
    }

    fn get_schema_for_reflect(&self) -> &::protobuf::SingularPtrField<RpbYokozunaSchema> {
        &self.schema
    }

    fn mut_schema_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RpbYokozunaSchema> {
        &mut self.schema
    }
}

impl ::protobuf::Message for RpbYokozunaSchemaGetResp {
    fn is_initialized(&self) -> bool {
        if self.schema.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.schema)?;
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
        if let Some(v) = self.schema.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.schema.as_ref() {
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

impl ::protobuf::MessageStatic for RpbYokozunaSchemaGetResp {
    fn new() -> RpbYokozunaSchemaGetResp {
        RpbYokozunaSchemaGetResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbYokozunaSchemaGetResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RpbYokozunaSchema>>(
                    "schema",
                    RpbYokozunaSchemaGetResp::get_schema_for_reflect,
                    RpbYokozunaSchemaGetResp::mut_schema_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbYokozunaSchemaGetResp>(
                    "RpbYokozunaSchemaGetResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbYokozunaSchemaGetResp {
    fn clear(&mut self) {
        self.clear_schema();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpbYokozunaSchemaGetResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpbYokozunaSchemaGetResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x13, 0x72, 0x69, 0x61, 0x6b, 0x5f, 0x79, 0x6f, 0x6b, 0x6f, 0x7a, 0x75, 0x6e, 0x61, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x3f, 0x0a, 0x10, 0x52, 0x70, 0x62, 0x59, 0x6f, 0x6b, 0x6f,
    0x7a, 0x75, 0x6e, 0x61, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0e, 0x0a, 0x06, 0x73, 0x63, 0x68, 0x65, 0x6d,
    0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0d, 0x0a, 0x05, 0x6e, 0x5f, 0x76, 0x61, 0x6c,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x22, 0x26, 0x0a, 0x16, 0x52, 0x70, 0x62, 0x59, 0x6f, 0x6b,
    0x6f, 0x7a, 0x75, 0x6e, 0x61, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x47, 0x65, 0x74, 0x52, 0x65, 0x71,
    0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x3b,
    0x0a, 0x17, 0x52, 0x70, 0x62, 0x59, 0x6f, 0x6b, 0x6f, 0x7a, 0x75, 0x6e, 0x61, 0x49, 0x6e, 0x64,
    0x65, 0x78, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x12, 0x20, 0x0a, 0x05, 0x69, 0x6e, 0x64,
    0x65, 0x78, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x52, 0x70, 0x62, 0x59, 0x6f,
    0x6b, 0x6f, 0x7a, 0x75, 0x6e, 0x61, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x22, 0x4b, 0x0a, 0x16, 0x52,
    0x70, 0x62, 0x59, 0x6f, 0x6b, 0x6f, 0x7a, 0x75, 0x6e, 0x61, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x50,
    0x75, 0x74, 0x52, 0x65, 0x71, 0x12, 0x20, 0x0a, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x52, 0x70, 0x62, 0x59, 0x6f, 0x6b, 0x6f, 0x7a, 0x75,
    0x6e, 0x61, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x0f, 0x0a, 0x07, 0x74, 0x69, 0x6d, 0x65, 0x6f,
    0x75, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x22, 0x29, 0x0a, 0x19, 0x52, 0x70, 0x62, 0x59,
    0x6f, 0x6b, 0x6f, 0x7a, 0x75, 0x6e, 0x61, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x44, 0x65, 0x6c, 0x65,
    0x74, 0x65, 0x52, 0x65, 0x71, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x0c, 0x22, 0x32, 0x0a, 0x11, 0x52, 0x70, 0x62, 0x59, 0x6f, 0x6b, 0x6f, 0x7a, 0x75,
    0x6e, 0x61, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0f, 0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e,
    0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x3d, 0x0a, 0x17, 0x52, 0x70, 0x62, 0x59, 0x6f,
    0x6b, 0x6f, 0x7a, 0x75, 0x6e, 0x61, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x50, 0x75, 0x74, 0x52,
    0x65, 0x71, 0x12, 0x22, 0x0a, 0x06, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x0b, 0x32, 0x12, 0x2e, 0x52, 0x70, 0x62, 0x59, 0x6f, 0x6b, 0x6f, 0x7a, 0x75, 0x6e, 0x61,
    0x53, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x22, 0x27, 0x0a, 0x17, 0x52, 0x70, 0x62, 0x59, 0x6f, 0x6b,
    0x6f, 0x7a, 0x75, 0x6e, 0x61, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x47, 0x65, 0x74, 0x52, 0x65,
    0x71, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x22,
    0x3e, 0x0a, 0x18, 0x52, 0x70, 0x62, 0x59, 0x6f, 0x6b, 0x6f, 0x7a, 0x75, 0x6e, 0x61, 0x53, 0x63,
    0x68, 0x65, 0x6d, 0x61, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x12, 0x22, 0x0a, 0x06, 0x73,
    0x63, 0x68, 0x65, 0x6d, 0x61, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x52, 0x70,
    0x62, 0x59, 0x6f, 0x6b, 0x6f, 0x7a, 0x75, 0x6e, 0x61, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x42,
    0x29, 0x0a, 0x17, 0x63, 0x6f, 0x6d, 0x2e, 0x62, 0x61, 0x73, 0x68, 0x6f, 0x2e, 0x72, 0x69, 0x61,
    0x6b, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x42, 0x0e, 0x52, 0x69, 0x61, 0x6b,
    0x59, 0x6f, 0x6b, 0x6f, 0x7a, 0x75, 0x6e, 0x61, 0x50, 0x42, 0x4a, 0xa0, 0x0d, 0x0a, 0x06, 0x12,
    0x04, 0x1c, 0x00, 0x4e, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x1c, 0x00, 0x30, 0x0a,
    0x26, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x1c, 0x00, 0x30, 0x1a, 0x19, 0x20, 0x6a,
    0x61, 0x76, 0x61, 0x20, 0x70, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x20, 0x73, 0x70, 0x65, 0x63,
    0x69, 0x66, 0x69, 0x65, 0x72, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x12, 0x03, 0x1c, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x1c, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x1c, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x07, 0x12, 0x03, 0x1c,
    0x16, 0x2f, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x1d, 0x00, 0x2f, 0x0a, 0x0b, 0x0a, 0x04,
    0x08, 0xe7, 0x07, 0x01, 0x12, 0x03, 0x1d, 0x00, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07,
    0x01, 0x02, 0x12, 0x03, 0x1d, 0x07, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02,
    0x00, 0x12, 0x03, 0x1d, 0x07, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x1d, 0x07, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x07, 0x12,
    0x03, 0x1d, 0x1e, 0x2e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x21, 0x00, 0x25, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x21, 0x08, 0x18, 0x0a, 0x19, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x22, 0x04, 0x1f, 0x22, 0x0c, 0x20, 0x49, 0x6e, 0x64, 0x65,
    0x78, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x22, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x22, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x22, 0x13,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x22, 0x1d, 0x1e, 0x0a,
    0x1a, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x23, 0x04, 0x1f, 0x22, 0x0d, 0x20, 0x53,
    0x63, 0x68, 0x65, 0x6d, 0x61, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x23, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x23, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x23, 0x13, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x23, 0x1d, 0x1e, 0x0a, 0x16, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x24, 0x04, 0x1f,
    0x22, 0x09, 0x20, 0x4e, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x24, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x24, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x24, 0x14, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x24, 0x1d, 0x1e, 0x0a, 0x56, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x28, 0x00, 0x2a, 0x01, 0x1a,
    0x4a, 0x20, 0x47, 0x45, 0x54, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x2d, 0x20,
    0x49, 0x66, 0x20, 0x61, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x69, 0x73, 0x20, 0x67, 0x69, 0x76,
    0x65, 0x6e, 0x2c, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68,
    0x69, 0x6e, 0x67, 0x20, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x2c, 0x20, 0x65, 0x6c, 0x73, 0x65, 0x20,
    0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x61, 0x6c, 0x6c, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x01, 0x01, 0x12, 0x03, 0x28, 0x08, 0x1e, 0x0a, 0x19, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12,
    0x03, 0x29, 0x04, 0x1e, 0x22, 0x0c, 0x20, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x20, 0x6e, 0x61, 0x6d,
    0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x29, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x29, 0x0d, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x29, 0x13, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x29, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02,
    0x12, 0x04, 0x2c, 0x00, 0x2e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x2c,
    0x08, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x2d, 0x04, 0x2a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x2d, 0x0d, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2d, 0x1e, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x2d, 0x28, 0x29, 0x0a, 0x2e, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x31,
    0x00, 0x34, 0x01, 0x1a, 0x22, 0x20, 0x50, 0x55, 0x54, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x20, 0x2d, 0x20, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x20, 0x61, 0x20, 0x6e, 0x65, 0x77,
    0x20, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03,
    0x31, 0x08, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x32, 0x04, 0x2a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x32, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x32, 0x0d, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x32, 0x1e, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x32, 0x28, 0x29, 0x0a, 0x1c, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01,
    0x12, 0x03, 0x33, 0x04, 0x2a, 0x22, 0x0f, 0x20, 0x54, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x20,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x33, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x33,
    0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x33, 0x14, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x33, 0x28, 0x29, 0x0a, 0x2e,
    0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x37, 0x00, 0x39, 0x01, 0x1a, 0x22, 0x20, 0x44, 0x45, 0x4c,
    0x45, 0x54, 0x45, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x2d, 0x20, 0x52, 0x65,
    0x6d, 0x6f, 0x76, 0x65, 0x20, 0x61, 0x6e, 0x20, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x37, 0x08, 0x21, 0x0a, 0x19, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x00, 0x12, 0x03, 0x38, 0x04, 0x1e, 0x22, 0x0c, 0x20, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x20,
    0x6e, 0x61, 0x6d, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x38, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x38, 0x0d,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x38, 0x13, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x38, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x05, 0x12, 0x04, 0x3d, 0x00, 0x40, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01,
    0x12, 0x03, 0x3d, 0x08, 0x19, 0x0a, 0x19, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x3e,
    0x04, 0x20, 0x22, 0x0c, 0x20, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3e, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x3e, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3e, 0x13, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x3e, 0x1e, 0x1f, 0x0a, 0x1a, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01,
    0x12, 0x03, 0x3f, 0x04, 0x20, 0x22, 0x0d, 0x20, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x20, 0x64,
    0x61, 0x74, 0x61, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x3f,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x05, 0x12, 0x03, 0x3f, 0x0d, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3f, 0x13, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3f, 0x1e, 0x1f, 0x0a, 0x45, 0x0a, 0x02,
    0x04, 0x06, 0x12, 0x04, 0x43, 0x00, 0x45, 0x01, 0x1a, 0x39, 0x20, 0x50, 0x55, 0x54, 0x20, 0x72,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x2d, 0x20, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x20,
    0x6f, 0x72, 0x20, 0x70, 0x6f, 0x74, 0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x75,
    0x70, 0x64, 0x61, 0x74, 0x65, 0x20, 0x61, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x73, 0x63, 0x68, 0x65,
    0x6d, 0x61, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x43, 0x08, 0x1f, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x44, 0x04, 0x2b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x44, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x44, 0x0d, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x44, 0x1f, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x44, 0x29, 0x2a, 0x0a, 0x3a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x48, 0x00, 0x4a, 0x01,
    0x1a, 0x2e, 0x20, 0x47, 0x45, 0x54, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x2d,
    0x20, 0x52, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67,
    0x20, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x20, 0x62, 0x79, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x0a,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x48, 0x08, 0x1f, 0x0a, 0x1a, 0x0a, 0x04,
    0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x49, 0x04, 0x1e, 0x22, 0x0d, 0x20, 0x53, 0x63, 0x68, 0x65,
    0x6d, 0x61, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x49, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x49, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x49,
    0x13, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x49, 0x1c, 0x1d,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x4c, 0x00, 0x4e, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x08, 0x01, 0x12, 0x03, 0x4c, 0x08, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00,
    0x12, 0x03, 0x4d, 0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x4d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x06, 0x12, 0x03, 0x4d, 0x0b,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4d, 0x1d, 0x23, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4d, 0x27, 0x28,
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
