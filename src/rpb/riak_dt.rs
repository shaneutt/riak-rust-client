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
pub struct MapField {
    // message fields
    name: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    field_type: ::std::option::Option<MapField_MapFieldType>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MapField {}

impl MapField {
    pub fn new() -> MapField {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MapField {
        static mut instance: ::protobuf::lazy::Lazy<MapField> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MapField,
        };
        unsafe {
            instance.get(MapField::new)
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

    // required .MapField.MapFieldType type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: MapField_MapFieldType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> MapField_MapFieldType {
        self.field_type.unwrap_or(MapField_MapFieldType::COUNTER)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<MapField_MapFieldType> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<MapField_MapFieldType> {
        &mut self.field_type
    }
}

impl ::protobuf::Message for MapField {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        if self.field_type.is_none() {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.field_type = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(2, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.field_type {
            os.write_enum(2, v.value())?;
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

impl ::protobuf::MessageStatic for MapField {
    fn new() -> MapField {
        MapField::new()
    }

    fn descriptor_static(_: ::std::option::Option<MapField>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "name",
                    MapField::get_name_for_reflect,
                    MapField::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<MapField_MapFieldType>>(
                    "type",
                    MapField::get_field_type_for_reflect,
                    MapField::mut_field_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MapField>(
                    "MapField",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MapField {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MapField {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MapField {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MapField_MapFieldType {
    COUNTER = 1,
    SET = 2,
    REGISTER = 3,
    FLAG = 4,
    MAP = 5,
}

impl ::protobuf::ProtobufEnum for MapField_MapFieldType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MapField_MapFieldType> {
        match value {
            1 => ::std::option::Option::Some(MapField_MapFieldType::COUNTER),
            2 => ::std::option::Option::Some(MapField_MapFieldType::SET),
            3 => ::std::option::Option::Some(MapField_MapFieldType::REGISTER),
            4 => ::std::option::Option::Some(MapField_MapFieldType::FLAG),
            5 => ::std::option::Option::Some(MapField_MapFieldType::MAP),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [MapField_MapFieldType] = &[
            MapField_MapFieldType::COUNTER,
            MapField_MapFieldType::SET,
            MapField_MapFieldType::REGISTER,
            MapField_MapFieldType::FLAG,
            MapField_MapFieldType::MAP,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<MapField_MapFieldType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("MapField_MapFieldType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for MapField_MapFieldType {
}

impl ::protobuf::reflect::ProtobufValue for MapField_MapFieldType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MapEntry {
    // message fields
    field: ::protobuf::SingularPtrField<MapField>,
    counter_value: ::std::option::Option<i64>,
    set_value: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    register_value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    flag_value: ::std::option::Option<bool>,
    map_value: ::protobuf::RepeatedField<MapEntry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MapEntry {}

impl MapEntry {
    pub fn new() -> MapEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MapEntry {
        static mut instance: ::protobuf::lazy::Lazy<MapEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MapEntry,
        };
        unsafe {
            instance.get(MapEntry::new)
        }
    }

    // required .MapField field = 1;

    pub fn clear_field(&mut self) {
        self.field.clear();
    }

    pub fn has_field(&self) -> bool {
        self.field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field(&mut self, v: MapField) {
        self.field = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field(&mut self) -> &mut MapField {
        if self.field.is_none() {
            self.field.set_default();
        };
        self.field.as_mut().unwrap()
    }

    // Take field
    pub fn take_field(&mut self) -> MapField {
        self.field.take().unwrap_or_else(|| MapField::new())
    }

    pub fn get_field(&self) -> &MapField {
        self.field.as_ref().unwrap_or_else(|| MapField::default_instance())
    }

    fn get_field_for_reflect(&self) -> &::protobuf::SingularPtrField<MapField> {
        &self.field
    }

    fn mut_field_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<MapField> {
        &mut self.field
    }

    // optional sint64 counter_value = 2;

    pub fn clear_counter_value(&mut self) {
        self.counter_value = ::std::option::Option::None;
    }

    pub fn has_counter_value(&self) -> bool {
        self.counter_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_counter_value(&mut self, v: i64) {
        self.counter_value = ::std::option::Option::Some(v);
    }

    pub fn get_counter_value(&self) -> i64 {
        self.counter_value.unwrap_or(0)
    }

    fn get_counter_value_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.counter_value
    }

    fn mut_counter_value_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.counter_value
    }

    // repeated bytes set_value = 3;

    pub fn clear_set_value(&mut self) {
        self.set_value.clear();
    }

    // Param is passed by value, moved
    pub fn set_set_value(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.set_value = v;
    }

    // Mutable pointer to the field.
    pub fn mut_set_value(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.set_value
    }

    // Take field
    pub fn take_set_value(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.set_value, ::protobuf::RepeatedField::new())
    }

    pub fn get_set_value(&self) -> &[::std::vec::Vec<u8>] {
        &self.set_value
    }

    fn get_set_value_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.set_value
    }

    fn mut_set_value_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.set_value
    }

    // optional bytes register_value = 4;

    pub fn clear_register_value(&mut self) {
        self.register_value.clear();
    }

    pub fn has_register_value(&self) -> bool {
        self.register_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_register_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.register_value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_register_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.register_value.is_none() {
            self.register_value.set_default();
        };
        self.register_value.as_mut().unwrap()
    }

    // Take field
    pub fn take_register_value(&mut self) -> ::std::vec::Vec<u8> {
        self.register_value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_register_value(&self) -> &[u8] {
        match self.register_value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_register_value_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.register_value
    }

    fn mut_register_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.register_value
    }

    // optional bool flag_value = 5;

    pub fn clear_flag_value(&mut self) {
        self.flag_value = ::std::option::Option::None;
    }

    pub fn has_flag_value(&self) -> bool {
        self.flag_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flag_value(&mut self, v: bool) {
        self.flag_value = ::std::option::Option::Some(v);
    }

    pub fn get_flag_value(&self) -> bool {
        self.flag_value.unwrap_or(false)
    }

    fn get_flag_value_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.flag_value
    }

    fn mut_flag_value_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.flag_value
    }

    // repeated .MapEntry map_value = 6;

    pub fn clear_map_value(&mut self) {
        self.map_value.clear();
    }

    // Param is passed by value, moved
    pub fn set_map_value(&mut self, v: ::protobuf::RepeatedField<MapEntry>) {
        self.map_value = v;
    }

    // Mutable pointer to the field.
    pub fn mut_map_value(&mut self) -> &mut ::protobuf::RepeatedField<MapEntry> {
        &mut self.map_value
    }

    // Take field
    pub fn take_map_value(&mut self) -> ::protobuf::RepeatedField<MapEntry> {
        ::std::mem::replace(&mut self.map_value, ::protobuf::RepeatedField::new())
    }

    pub fn get_map_value(&self) -> &[MapEntry] {
        &self.map_value
    }

    fn get_map_value_for_reflect(&self) -> &::protobuf::RepeatedField<MapEntry> {
        &self.map_value
    }

    fn mut_map_value_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<MapEntry> {
        &mut self.map_value
    }
}

impl ::protobuf::Message for MapEntry {
    fn is_initialized(&self) -> bool {
        if self.field.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.field)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_sint64()?;
                    self.counter_value = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.set_value)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.register_value)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.flag_value = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.map_value)?;
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
        if let Some(v) = self.field.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.counter_value {
            my_size += ::protobuf::rt::value_varint_zigzag_size(2, v);
        };
        for value in &self.set_value {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        if let Some(v) = self.register_value.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        };
        if let Some(v) = self.flag_value {
            my_size += 2;
        };
        for value in &self.map_value {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.counter_value {
            os.write_sint64(2, v)?;
        };
        for v in &self.set_value {
            os.write_bytes(3, &v)?;
        };
        if let Some(v) = self.register_value.as_ref() {
            os.write_bytes(4, &v)?;
        };
        if let Some(v) = self.flag_value {
            os.write_bool(5, v)?;
        };
        for v in &self.map_value {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for MapEntry {
    fn new() -> MapEntry {
        MapEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<MapEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MapField>>(
                    "field",
                    MapEntry::get_field_for_reflect,
                    MapEntry::mut_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint64>(
                    "counter_value",
                    MapEntry::get_counter_value_for_reflect,
                    MapEntry::mut_counter_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "set_value",
                    MapEntry::get_set_value_for_reflect,
                    MapEntry::mut_set_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "register_value",
                    MapEntry::get_register_value_for_reflect,
                    MapEntry::mut_register_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "flag_value",
                    MapEntry::get_flag_value_for_reflect,
                    MapEntry::mut_flag_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MapEntry>>(
                    "map_value",
                    MapEntry::get_map_value_for_reflect,
                    MapEntry::mut_map_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MapEntry>(
                    "MapEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MapEntry {
    fn clear(&mut self) {
        self.clear_field();
        self.clear_counter_value();
        self.clear_set_value();
        self.clear_register_value();
        self.clear_flag_value();
        self.clear_map_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MapEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MapEntry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DtFetchReq {
    // message fields
    bucket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    field_type: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    r: ::std::option::Option<u32>,
    pr: ::std::option::Option<u32>,
    basic_quorum: ::std::option::Option<bool>,
    notfound_ok: ::std::option::Option<bool>,
    timeout: ::std::option::Option<u32>,
    sloppy_quorum: ::std::option::Option<bool>,
    n_val: ::std::option::Option<u32>,
    include_context: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DtFetchReq {}

impl DtFetchReq {
    pub fn new() -> DtFetchReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DtFetchReq {
        static mut instance: ::protobuf::lazy::Lazy<DtFetchReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DtFetchReq,
        };
        unsafe {
            instance.get(DtFetchReq::new)
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

    // required bytes type = 3;

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

    // optional uint32 r = 4;

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

    // optional uint32 pr = 5;

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

    // optional bool basic_quorum = 6;

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

    // optional bool notfound_ok = 7;

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

    // optional bool sloppy_quorum = 9;

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

    // optional uint32 n_val = 10;

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

    // optional bool include_context = 11;

    pub fn clear_include_context(&mut self) {
        self.include_context = ::std::option::Option::None;
    }

    pub fn has_include_context(&self) -> bool {
        self.include_context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_include_context(&mut self, v: bool) {
        self.include_context = ::std::option::Option::Some(v);
    }

    pub fn get_include_context(&self) -> bool {
        self.include_context.unwrap_or(true)
    }

    fn get_include_context_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.include_context
    }

    fn mut_include_context_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.include_context
    }
}

impl ::protobuf::Message for DtFetchReq {
    fn is_initialized(&self) -> bool {
        if self.bucket.is_none() {
            return false;
        };
        if self.key.is_none() {
            return false;
        };
        if self.field_type.is_none() {
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
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.r = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.pr = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.basic_quorum = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.notfound_ok = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.timeout = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.sloppy_quorum = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.n_val = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.include_context = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.r {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.pr {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.basic_quorum {
            my_size += 2;
        };
        if let Some(v) = self.notfound_ok {
            my_size += 2;
        };
        if let Some(v) = self.timeout {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.sloppy_quorum {
            my_size += 2;
        };
        if let Some(v) = self.n_val {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.include_context {
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
        if let Some(v) = self.field_type.as_ref() {
            os.write_bytes(3, &v)?;
        };
        if let Some(v) = self.r {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.pr {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.basic_quorum {
            os.write_bool(6, v)?;
        };
        if let Some(v) = self.notfound_ok {
            os.write_bool(7, v)?;
        };
        if let Some(v) = self.timeout {
            os.write_uint32(8, v)?;
        };
        if let Some(v) = self.sloppy_quorum {
            os.write_bool(9, v)?;
        };
        if let Some(v) = self.n_val {
            os.write_uint32(10, v)?;
        };
        if let Some(v) = self.include_context {
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

impl ::protobuf::MessageStatic for DtFetchReq {
    fn new() -> DtFetchReq {
        DtFetchReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<DtFetchReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "bucket",
                    DtFetchReq::get_bucket_for_reflect,
                    DtFetchReq::mut_bucket_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    DtFetchReq::get_key_for_reflect,
                    DtFetchReq::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "type",
                    DtFetchReq::get_field_type_for_reflect,
                    DtFetchReq::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "r",
                    DtFetchReq::get_r_for_reflect,
                    DtFetchReq::mut_r_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "pr",
                    DtFetchReq::get_pr_for_reflect,
                    DtFetchReq::mut_pr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "basic_quorum",
                    DtFetchReq::get_basic_quorum_for_reflect,
                    DtFetchReq::mut_basic_quorum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "notfound_ok",
                    DtFetchReq::get_notfound_ok_for_reflect,
                    DtFetchReq::mut_notfound_ok_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "timeout",
                    DtFetchReq::get_timeout_for_reflect,
                    DtFetchReq::mut_timeout_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "sloppy_quorum",
                    DtFetchReq::get_sloppy_quorum_for_reflect,
                    DtFetchReq::mut_sloppy_quorum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "n_val",
                    DtFetchReq::get_n_val_for_reflect,
                    DtFetchReq::mut_n_val_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "include_context",
                    DtFetchReq::get_include_context_for_reflect,
                    DtFetchReq::mut_include_context_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DtFetchReq>(
                    "DtFetchReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DtFetchReq {
    fn clear(&mut self) {
        self.clear_bucket();
        self.clear_key();
        self.clear_field_type();
        self.clear_r();
        self.clear_pr();
        self.clear_basic_quorum();
        self.clear_notfound_ok();
        self.clear_timeout();
        self.clear_sloppy_quorum();
        self.clear_n_val();
        self.clear_include_context();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DtFetchReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DtFetchReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DtValue {
    // message fields
    counter_value: ::std::option::Option<i64>,
    set_value: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    map_value: ::protobuf::RepeatedField<MapEntry>,
    hll_value: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DtValue {}

impl DtValue {
    pub fn new() -> DtValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DtValue {
        static mut instance: ::protobuf::lazy::Lazy<DtValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DtValue,
        };
        unsafe {
            instance.get(DtValue::new)
        }
    }

    // optional sint64 counter_value = 1;

    pub fn clear_counter_value(&mut self) {
        self.counter_value = ::std::option::Option::None;
    }

    pub fn has_counter_value(&self) -> bool {
        self.counter_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_counter_value(&mut self, v: i64) {
        self.counter_value = ::std::option::Option::Some(v);
    }

    pub fn get_counter_value(&self) -> i64 {
        self.counter_value.unwrap_or(0)
    }

    fn get_counter_value_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.counter_value
    }

    fn mut_counter_value_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.counter_value
    }

    // repeated bytes set_value = 2;

    pub fn clear_set_value(&mut self) {
        self.set_value.clear();
    }

    // Param is passed by value, moved
    pub fn set_set_value(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.set_value = v;
    }

    // Mutable pointer to the field.
    pub fn mut_set_value(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.set_value
    }

    // Take field
    pub fn take_set_value(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.set_value, ::protobuf::RepeatedField::new())
    }

    pub fn get_set_value(&self) -> &[::std::vec::Vec<u8>] {
        &self.set_value
    }

    fn get_set_value_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.set_value
    }

    fn mut_set_value_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.set_value
    }

    // repeated .MapEntry map_value = 3;

    pub fn clear_map_value(&mut self) {
        self.map_value.clear();
    }

    // Param is passed by value, moved
    pub fn set_map_value(&mut self, v: ::protobuf::RepeatedField<MapEntry>) {
        self.map_value = v;
    }

    // Mutable pointer to the field.
    pub fn mut_map_value(&mut self) -> &mut ::protobuf::RepeatedField<MapEntry> {
        &mut self.map_value
    }

    // Take field
    pub fn take_map_value(&mut self) -> ::protobuf::RepeatedField<MapEntry> {
        ::std::mem::replace(&mut self.map_value, ::protobuf::RepeatedField::new())
    }

    pub fn get_map_value(&self) -> &[MapEntry] {
        &self.map_value
    }

    fn get_map_value_for_reflect(&self) -> &::protobuf::RepeatedField<MapEntry> {
        &self.map_value
    }

    fn mut_map_value_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<MapEntry> {
        &mut self.map_value
    }

    // optional uint64 hll_value = 4;

    pub fn clear_hll_value(&mut self) {
        self.hll_value = ::std::option::Option::None;
    }

    pub fn has_hll_value(&self) -> bool {
        self.hll_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hll_value(&mut self, v: u64) {
        self.hll_value = ::std::option::Option::Some(v);
    }

    pub fn get_hll_value(&self) -> u64 {
        self.hll_value.unwrap_or(0)
    }

    fn get_hll_value_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.hll_value
    }

    fn mut_hll_value_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.hll_value
    }
}

impl ::protobuf::Message for DtValue {
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
                    self.counter_value = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.set_value)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.map_value)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.hll_value = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.counter_value {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, v);
        };
        for value in &self.set_value {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in &self.map_value {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.hll_value {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.counter_value {
            os.write_sint64(1, v)?;
        };
        for v in &self.set_value {
            os.write_bytes(2, &v)?;
        };
        for v in &self.map_value {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.hll_value {
            os.write_uint64(4, v)?;
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

impl ::protobuf::MessageStatic for DtValue {
    fn new() -> DtValue {
        DtValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<DtValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint64>(
                    "counter_value",
                    DtValue::get_counter_value_for_reflect,
                    DtValue::mut_counter_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "set_value",
                    DtValue::get_set_value_for_reflect,
                    DtValue::mut_set_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MapEntry>>(
                    "map_value",
                    DtValue::get_map_value_for_reflect,
                    DtValue::mut_map_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "hll_value",
                    DtValue::get_hll_value_for_reflect,
                    DtValue::mut_hll_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DtValue>(
                    "DtValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DtValue {
    fn clear(&mut self) {
        self.clear_counter_value();
        self.clear_set_value();
        self.clear_map_value();
        self.clear_hll_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DtValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DtValue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DtFetchResp {
    // message fields
    context: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    field_type: ::std::option::Option<DtFetchResp_DataType>,
    value: ::protobuf::SingularPtrField<DtValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DtFetchResp {}

impl DtFetchResp {
    pub fn new() -> DtFetchResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DtFetchResp {
        static mut instance: ::protobuf::lazy::Lazy<DtFetchResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DtFetchResp,
        };
        unsafe {
            instance.get(DtFetchResp::new)
        }
    }

    // optional bytes context = 1;

    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context(&mut self, v: ::std::vec::Vec<u8>) {
        self.context = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.context.is_none() {
            self.context.set_default();
        };
        self.context.as_mut().unwrap()
    }

    // Take field
    pub fn take_context(&mut self) -> ::std::vec::Vec<u8> {
        self.context.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_context(&self) -> &[u8] {
        match self.context.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_context_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.context
    }

    fn mut_context_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.context
    }

    // required .DtFetchResp.DataType type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: DtFetchResp_DataType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> DtFetchResp_DataType {
        self.field_type.unwrap_or(DtFetchResp_DataType::COUNTER)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<DtFetchResp_DataType> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<DtFetchResp_DataType> {
        &mut self.field_type
    }

    // optional .DtValue value = 3;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: DtValue) {
        self.value = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut DtValue {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> DtValue {
        self.value.take().unwrap_or_else(|| DtValue::new())
    }

    pub fn get_value(&self) -> &DtValue {
        self.value.as_ref().unwrap_or_else(|| DtValue::default_instance())
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularPtrField<DtValue> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DtValue> {
        &mut self.value
    }
}

impl ::protobuf::Message for DtFetchResp {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.context)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.value)?;
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
        if let Some(v) = self.context.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(2, v);
        };
        if let Some(v) = self.value.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.context.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.field_type {
            os.write_enum(2, v.value())?;
        };
        if let Some(v) = self.value.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for DtFetchResp {
    fn new() -> DtFetchResp {
        DtFetchResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<DtFetchResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "context",
                    DtFetchResp::get_context_for_reflect,
                    DtFetchResp::mut_context_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<DtFetchResp_DataType>>(
                    "type",
                    DtFetchResp::get_field_type_for_reflect,
                    DtFetchResp::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DtValue>>(
                    "value",
                    DtFetchResp::get_value_for_reflect,
                    DtFetchResp::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DtFetchResp>(
                    "DtFetchResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DtFetchResp {
    fn clear(&mut self) {
        self.clear_context();
        self.clear_field_type();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DtFetchResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DtFetchResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DtFetchResp_DataType {
    COUNTER = 1,
    SET = 2,
    MAP = 3,
    HLL = 4,
}

impl ::protobuf::ProtobufEnum for DtFetchResp_DataType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DtFetchResp_DataType> {
        match value {
            1 => ::std::option::Option::Some(DtFetchResp_DataType::COUNTER),
            2 => ::std::option::Option::Some(DtFetchResp_DataType::SET),
            3 => ::std::option::Option::Some(DtFetchResp_DataType::MAP),
            4 => ::std::option::Option::Some(DtFetchResp_DataType::HLL),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DtFetchResp_DataType] = &[
            DtFetchResp_DataType::COUNTER,
            DtFetchResp_DataType::SET,
            DtFetchResp_DataType::MAP,
            DtFetchResp_DataType::HLL,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<DtFetchResp_DataType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DtFetchResp_DataType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DtFetchResp_DataType {
}

impl ::protobuf::reflect::ProtobufValue for DtFetchResp_DataType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CounterOp {
    // message fields
    increment: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CounterOp {}

impl CounterOp {
    pub fn new() -> CounterOp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CounterOp {
        static mut instance: ::protobuf::lazy::Lazy<CounterOp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CounterOp,
        };
        unsafe {
            instance.get(CounterOp::new)
        }
    }

    // optional sint64 increment = 1;

    pub fn clear_increment(&mut self) {
        self.increment = ::std::option::Option::None;
    }

    pub fn has_increment(&self) -> bool {
        self.increment.is_some()
    }

    // Param is passed by value, moved
    pub fn set_increment(&mut self, v: i64) {
        self.increment = ::std::option::Option::Some(v);
    }

    pub fn get_increment(&self) -> i64 {
        self.increment.unwrap_or(0)
    }

    fn get_increment_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.increment
    }

    fn mut_increment_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.increment
    }
}

impl ::protobuf::Message for CounterOp {
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
                    self.increment = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.increment {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.increment {
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

impl ::protobuf::MessageStatic for CounterOp {
    fn new() -> CounterOp {
        CounterOp::new()
    }

    fn descriptor_static(_: ::std::option::Option<CounterOp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint64>(
                    "increment",
                    CounterOp::get_increment_for_reflect,
                    CounterOp::mut_increment_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CounterOp>(
                    "CounterOp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CounterOp {
    fn clear(&mut self) {
        self.clear_increment();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CounterOp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CounterOp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetOp {
    // message fields
    adds: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    removes: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetOp {}

impl SetOp {
    pub fn new() -> SetOp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetOp {
        static mut instance: ::protobuf::lazy::Lazy<SetOp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetOp,
        };
        unsafe {
            instance.get(SetOp::new)
        }
    }

    // repeated bytes adds = 1;

    pub fn clear_adds(&mut self) {
        self.adds.clear();
    }

    // Param is passed by value, moved
    pub fn set_adds(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.adds = v;
    }

    // Mutable pointer to the field.
    pub fn mut_adds(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.adds
    }

    // Take field
    pub fn take_adds(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.adds, ::protobuf::RepeatedField::new())
    }

    pub fn get_adds(&self) -> &[::std::vec::Vec<u8>] {
        &self.adds
    }

    fn get_adds_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.adds
    }

    fn mut_adds_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.adds
    }

    // repeated bytes removes = 2;

    pub fn clear_removes(&mut self) {
        self.removes.clear();
    }

    // Param is passed by value, moved
    pub fn set_removes(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.removes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_removes(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.removes
    }

    // Take field
    pub fn take_removes(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.removes, ::protobuf::RepeatedField::new())
    }

    pub fn get_removes(&self) -> &[::std::vec::Vec<u8>] {
        &self.removes
    }

    fn get_removes_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.removes
    }

    fn mut_removes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.removes
    }
}

impl ::protobuf::Message for SetOp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.adds)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.removes)?;
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
        for value in &self.adds {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.removes {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.adds {
            os.write_bytes(1, &v)?;
        };
        for v in &self.removes {
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

impl ::protobuf::MessageStatic for SetOp {
    fn new() -> SetOp {
        SetOp::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetOp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "adds",
                    SetOp::get_adds_for_reflect,
                    SetOp::mut_adds_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "removes",
                    SetOp::get_removes_for_reflect,
                    SetOp::mut_removes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetOp>(
                    "SetOp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetOp {
    fn clear(&mut self) {
        self.clear_adds();
        self.clear_removes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SetOp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetOp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HllOp {
    // message fields
    adds: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HllOp {}

impl HllOp {
    pub fn new() -> HllOp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HllOp {
        static mut instance: ::protobuf::lazy::Lazy<HllOp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HllOp,
        };
        unsafe {
            instance.get(HllOp::new)
        }
    }

    // repeated bytes adds = 1;

    pub fn clear_adds(&mut self) {
        self.adds.clear();
    }

    // Param is passed by value, moved
    pub fn set_adds(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.adds = v;
    }

    // Mutable pointer to the field.
    pub fn mut_adds(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.adds
    }

    // Take field
    pub fn take_adds(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.adds, ::protobuf::RepeatedField::new())
    }

    pub fn get_adds(&self) -> &[::std::vec::Vec<u8>] {
        &self.adds
    }

    fn get_adds_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.adds
    }

    fn mut_adds_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.adds
    }
}

impl ::protobuf::Message for HllOp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.adds)?;
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
        for value in &self.adds {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.adds {
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

impl ::protobuf::MessageStatic for HllOp {
    fn new() -> HllOp {
        HllOp::new()
    }

    fn descriptor_static(_: ::std::option::Option<HllOp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "adds",
                    HllOp::get_adds_for_reflect,
                    HllOp::mut_adds_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HllOp>(
                    "HllOp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HllOp {
    fn clear(&mut self) {
        self.clear_adds();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HllOp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HllOp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MapUpdate {
    // message fields
    field: ::protobuf::SingularPtrField<MapField>,
    counter_op: ::protobuf::SingularPtrField<CounterOp>,
    set_op: ::protobuf::SingularPtrField<SetOp>,
    register_op: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    flag_op: ::std::option::Option<MapUpdate_FlagOp>,
    map_op: ::protobuf::SingularPtrField<MapOp>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MapUpdate {}

impl MapUpdate {
    pub fn new() -> MapUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MapUpdate {
        static mut instance: ::protobuf::lazy::Lazy<MapUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MapUpdate,
        };
        unsafe {
            instance.get(MapUpdate::new)
        }
    }

    // required .MapField field = 1;

    pub fn clear_field(&mut self) {
        self.field.clear();
    }

    pub fn has_field(&self) -> bool {
        self.field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field(&mut self, v: MapField) {
        self.field = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field(&mut self) -> &mut MapField {
        if self.field.is_none() {
            self.field.set_default();
        };
        self.field.as_mut().unwrap()
    }

    // Take field
    pub fn take_field(&mut self) -> MapField {
        self.field.take().unwrap_or_else(|| MapField::new())
    }

    pub fn get_field(&self) -> &MapField {
        self.field.as_ref().unwrap_or_else(|| MapField::default_instance())
    }

    fn get_field_for_reflect(&self) -> &::protobuf::SingularPtrField<MapField> {
        &self.field
    }

    fn mut_field_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<MapField> {
        &mut self.field
    }

    // optional .CounterOp counter_op = 2;

    pub fn clear_counter_op(&mut self) {
        self.counter_op.clear();
    }

    pub fn has_counter_op(&self) -> bool {
        self.counter_op.is_some()
    }

    // Param is passed by value, moved
    pub fn set_counter_op(&mut self, v: CounterOp) {
        self.counter_op = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_counter_op(&mut self) -> &mut CounterOp {
        if self.counter_op.is_none() {
            self.counter_op.set_default();
        };
        self.counter_op.as_mut().unwrap()
    }

    // Take field
    pub fn take_counter_op(&mut self) -> CounterOp {
        self.counter_op.take().unwrap_or_else(|| CounterOp::new())
    }

    pub fn get_counter_op(&self) -> &CounterOp {
        self.counter_op.as_ref().unwrap_or_else(|| CounterOp::default_instance())
    }

    fn get_counter_op_for_reflect(&self) -> &::protobuf::SingularPtrField<CounterOp> {
        &self.counter_op
    }

    fn mut_counter_op_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CounterOp> {
        &mut self.counter_op
    }

    // optional .SetOp set_op = 3;

    pub fn clear_set_op(&mut self) {
        self.set_op.clear();
    }

    pub fn has_set_op(&self) -> bool {
        self.set_op.is_some()
    }

    // Param is passed by value, moved
    pub fn set_set_op(&mut self, v: SetOp) {
        self.set_op = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_set_op(&mut self) -> &mut SetOp {
        if self.set_op.is_none() {
            self.set_op.set_default();
        };
        self.set_op.as_mut().unwrap()
    }

    // Take field
    pub fn take_set_op(&mut self) -> SetOp {
        self.set_op.take().unwrap_or_else(|| SetOp::new())
    }

    pub fn get_set_op(&self) -> &SetOp {
        self.set_op.as_ref().unwrap_or_else(|| SetOp::default_instance())
    }

    fn get_set_op_for_reflect(&self) -> &::protobuf::SingularPtrField<SetOp> {
        &self.set_op
    }

    fn mut_set_op_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SetOp> {
        &mut self.set_op
    }

    // optional bytes register_op = 4;

    pub fn clear_register_op(&mut self) {
        self.register_op.clear();
    }

    pub fn has_register_op(&self) -> bool {
        self.register_op.is_some()
    }

    // Param is passed by value, moved
    pub fn set_register_op(&mut self, v: ::std::vec::Vec<u8>) {
        self.register_op = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_register_op(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.register_op.is_none() {
            self.register_op.set_default();
        };
        self.register_op.as_mut().unwrap()
    }

    // Take field
    pub fn take_register_op(&mut self) -> ::std::vec::Vec<u8> {
        self.register_op.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_register_op(&self) -> &[u8] {
        match self.register_op.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_register_op_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.register_op
    }

    fn mut_register_op_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.register_op
    }

    // optional .MapUpdate.FlagOp flag_op = 5;

    pub fn clear_flag_op(&mut self) {
        self.flag_op = ::std::option::Option::None;
    }

    pub fn has_flag_op(&self) -> bool {
        self.flag_op.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flag_op(&mut self, v: MapUpdate_FlagOp) {
        self.flag_op = ::std::option::Option::Some(v);
    }

    pub fn get_flag_op(&self) -> MapUpdate_FlagOp {
        self.flag_op.unwrap_or(MapUpdate_FlagOp::ENABLE)
    }

    fn get_flag_op_for_reflect(&self) -> &::std::option::Option<MapUpdate_FlagOp> {
        &self.flag_op
    }

    fn mut_flag_op_for_reflect(&mut self) -> &mut ::std::option::Option<MapUpdate_FlagOp> {
        &mut self.flag_op
    }

    // optional .MapOp map_op = 6;

    pub fn clear_map_op(&mut self) {
        self.map_op.clear();
    }

    pub fn has_map_op(&self) -> bool {
        self.map_op.is_some()
    }

    // Param is passed by value, moved
    pub fn set_map_op(&mut self, v: MapOp) {
        self.map_op = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_map_op(&mut self) -> &mut MapOp {
        if self.map_op.is_none() {
            self.map_op.set_default();
        };
        self.map_op.as_mut().unwrap()
    }

    // Take field
    pub fn take_map_op(&mut self) -> MapOp {
        self.map_op.take().unwrap_or_else(|| MapOp::new())
    }

    pub fn get_map_op(&self) -> &MapOp {
        self.map_op.as_ref().unwrap_or_else(|| MapOp::default_instance())
    }

    fn get_map_op_for_reflect(&self) -> &::protobuf::SingularPtrField<MapOp> {
        &self.map_op
    }

    fn mut_map_op_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<MapOp> {
        &mut self.map_op
    }
}

impl ::protobuf::Message for MapUpdate {
    fn is_initialized(&self) -> bool {
        if self.field.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.field)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.counter_op)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.set_op)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.register_op)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.flag_op = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.map_op)?;
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
        if let Some(v) = self.field.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.counter_op.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.set_op.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.register_op.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        };
        if let Some(v) = self.flag_op {
            my_size += ::protobuf::rt::enum_size(5, v);
        };
        if let Some(v) = self.map_op.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.counter_op.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.set_op.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.register_op.as_ref() {
            os.write_bytes(4, &v)?;
        };
        if let Some(v) = self.flag_op {
            os.write_enum(5, v.value())?;
        };
        if let Some(v) = self.map_op.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for MapUpdate {
    fn new() -> MapUpdate {
        MapUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<MapUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MapField>>(
                    "field",
                    MapUpdate::get_field_for_reflect,
                    MapUpdate::mut_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CounterOp>>(
                    "counter_op",
                    MapUpdate::get_counter_op_for_reflect,
                    MapUpdate::mut_counter_op_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SetOp>>(
                    "set_op",
                    MapUpdate::get_set_op_for_reflect,
                    MapUpdate::mut_set_op_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "register_op",
                    MapUpdate::get_register_op_for_reflect,
                    MapUpdate::mut_register_op_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<MapUpdate_FlagOp>>(
                    "flag_op",
                    MapUpdate::get_flag_op_for_reflect,
                    MapUpdate::mut_flag_op_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MapOp>>(
                    "map_op",
                    MapUpdate::get_map_op_for_reflect,
                    MapUpdate::mut_map_op_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MapUpdate>(
                    "MapUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MapUpdate {
    fn clear(&mut self) {
        self.clear_field();
        self.clear_counter_op();
        self.clear_set_op();
        self.clear_register_op();
        self.clear_flag_op();
        self.clear_map_op();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MapUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MapUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MapUpdate_FlagOp {
    ENABLE = 1,
    DISABLE = 2,
}

impl ::protobuf::ProtobufEnum for MapUpdate_FlagOp {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MapUpdate_FlagOp> {
        match value {
            1 => ::std::option::Option::Some(MapUpdate_FlagOp::ENABLE),
            2 => ::std::option::Option::Some(MapUpdate_FlagOp::DISABLE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [MapUpdate_FlagOp] = &[
            MapUpdate_FlagOp::ENABLE,
            MapUpdate_FlagOp::DISABLE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<MapUpdate_FlagOp>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("MapUpdate_FlagOp", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for MapUpdate_FlagOp {
}

impl ::protobuf::reflect::ProtobufValue for MapUpdate_FlagOp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MapOp {
    // message fields
    removes: ::protobuf::RepeatedField<MapField>,
    updates: ::protobuf::RepeatedField<MapUpdate>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MapOp {}

impl MapOp {
    pub fn new() -> MapOp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MapOp {
        static mut instance: ::protobuf::lazy::Lazy<MapOp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MapOp,
        };
        unsafe {
            instance.get(MapOp::new)
        }
    }

    // repeated .MapField removes = 1;

    pub fn clear_removes(&mut self) {
        self.removes.clear();
    }

    // Param is passed by value, moved
    pub fn set_removes(&mut self, v: ::protobuf::RepeatedField<MapField>) {
        self.removes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_removes(&mut self) -> &mut ::protobuf::RepeatedField<MapField> {
        &mut self.removes
    }

    // Take field
    pub fn take_removes(&mut self) -> ::protobuf::RepeatedField<MapField> {
        ::std::mem::replace(&mut self.removes, ::protobuf::RepeatedField::new())
    }

    pub fn get_removes(&self) -> &[MapField] {
        &self.removes
    }

    fn get_removes_for_reflect(&self) -> &::protobuf::RepeatedField<MapField> {
        &self.removes
    }

    fn mut_removes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<MapField> {
        &mut self.removes
    }

    // repeated .MapUpdate updates = 2;

    pub fn clear_updates(&mut self) {
        self.updates.clear();
    }

    // Param is passed by value, moved
    pub fn set_updates(&mut self, v: ::protobuf::RepeatedField<MapUpdate>) {
        self.updates = v;
    }

    // Mutable pointer to the field.
    pub fn mut_updates(&mut self) -> &mut ::protobuf::RepeatedField<MapUpdate> {
        &mut self.updates
    }

    // Take field
    pub fn take_updates(&mut self) -> ::protobuf::RepeatedField<MapUpdate> {
        ::std::mem::replace(&mut self.updates, ::protobuf::RepeatedField::new())
    }

    pub fn get_updates(&self) -> &[MapUpdate] {
        &self.updates
    }

    fn get_updates_for_reflect(&self) -> &::protobuf::RepeatedField<MapUpdate> {
        &self.updates
    }

    fn mut_updates_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<MapUpdate> {
        &mut self.updates
    }
}

impl ::protobuf::Message for MapOp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.removes)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.updates)?;
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
        for value in &self.removes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.updates {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.removes {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.updates {
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

impl ::protobuf::MessageStatic for MapOp {
    fn new() -> MapOp {
        MapOp::new()
    }

    fn descriptor_static(_: ::std::option::Option<MapOp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MapField>>(
                    "removes",
                    MapOp::get_removes_for_reflect,
                    MapOp::mut_removes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MapUpdate>>(
                    "updates",
                    MapOp::get_updates_for_reflect,
                    MapOp::mut_updates_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MapOp>(
                    "MapOp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MapOp {
    fn clear(&mut self) {
        self.clear_removes();
        self.clear_updates();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MapOp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MapOp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DtOp {
    // message fields
    counter_op: ::protobuf::SingularPtrField<CounterOp>,
    set_op: ::protobuf::SingularPtrField<SetOp>,
    map_op: ::protobuf::SingularPtrField<MapOp>,
    hll_op: ::protobuf::SingularPtrField<HllOp>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DtOp {}

impl DtOp {
    pub fn new() -> DtOp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DtOp {
        static mut instance: ::protobuf::lazy::Lazy<DtOp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DtOp,
        };
        unsafe {
            instance.get(DtOp::new)
        }
    }

    // optional .CounterOp counter_op = 1;

    pub fn clear_counter_op(&mut self) {
        self.counter_op.clear();
    }

    pub fn has_counter_op(&self) -> bool {
        self.counter_op.is_some()
    }

    // Param is passed by value, moved
    pub fn set_counter_op(&mut self, v: CounterOp) {
        self.counter_op = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_counter_op(&mut self) -> &mut CounterOp {
        if self.counter_op.is_none() {
            self.counter_op.set_default();
        };
        self.counter_op.as_mut().unwrap()
    }

    // Take field
    pub fn take_counter_op(&mut self) -> CounterOp {
        self.counter_op.take().unwrap_or_else(|| CounterOp::new())
    }

    pub fn get_counter_op(&self) -> &CounterOp {
        self.counter_op.as_ref().unwrap_or_else(|| CounterOp::default_instance())
    }

    fn get_counter_op_for_reflect(&self) -> &::protobuf::SingularPtrField<CounterOp> {
        &self.counter_op
    }

    fn mut_counter_op_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CounterOp> {
        &mut self.counter_op
    }

    // optional .SetOp set_op = 2;

    pub fn clear_set_op(&mut self) {
        self.set_op.clear();
    }

    pub fn has_set_op(&self) -> bool {
        self.set_op.is_some()
    }

    // Param is passed by value, moved
    pub fn set_set_op(&mut self, v: SetOp) {
        self.set_op = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_set_op(&mut self) -> &mut SetOp {
        if self.set_op.is_none() {
            self.set_op.set_default();
        };
        self.set_op.as_mut().unwrap()
    }

    // Take field
    pub fn take_set_op(&mut self) -> SetOp {
        self.set_op.take().unwrap_or_else(|| SetOp::new())
    }

    pub fn get_set_op(&self) -> &SetOp {
        self.set_op.as_ref().unwrap_or_else(|| SetOp::default_instance())
    }

    fn get_set_op_for_reflect(&self) -> &::protobuf::SingularPtrField<SetOp> {
        &self.set_op
    }

    fn mut_set_op_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SetOp> {
        &mut self.set_op
    }

    // optional .MapOp map_op = 3;

    pub fn clear_map_op(&mut self) {
        self.map_op.clear();
    }

    pub fn has_map_op(&self) -> bool {
        self.map_op.is_some()
    }

    // Param is passed by value, moved
    pub fn set_map_op(&mut self, v: MapOp) {
        self.map_op = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_map_op(&mut self) -> &mut MapOp {
        if self.map_op.is_none() {
            self.map_op.set_default();
        };
        self.map_op.as_mut().unwrap()
    }

    // Take field
    pub fn take_map_op(&mut self) -> MapOp {
        self.map_op.take().unwrap_or_else(|| MapOp::new())
    }

    pub fn get_map_op(&self) -> &MapOp {
        self.map_op.as_ref().unwrap_or_else(|| MapOp::default_instance())
    }

    fn get_map_op_for_reflect(&self) -> &::protobuf::SingularPtrField<MapOp> {
        &self.map_op
    }

    fn mut_map_op_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<MapOp> {
        &mut self.map_op
    }

    // optional .HllOp hll_op = 4;

    pub fn clear_hll_op(&mut self) {
        self.hll_op.clear();
    }

    pub fn has_hll_op(&self) -> bool {
        self.hll_op.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hll_op(&mut self, v: HllOp) {
        self.hll_op = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hll_op(&mut self) -> &mut HllOp {
        if self.hll_op.is_none() {
            self.hll_op.set_default();
        };
        self.hll_op.as_mut().unwrap()
    }

    // Take field
    pub fn take_hll_op(&mut self) -> HllOp {
        self.hll_op.take().unwrap_or_else(|| HllOp::new())
    }

    pub fn get_hll_op(&self) -> &HllOp {
        self.hll_op.as_ref().unwrap_or_else(|| HllOp::default_instance())
    }

    fn get_hll_op_for_reflect(&self) -> &::protobuf::SingularPtrField<HllOp> {
        &self.hll_op
    }

    fn mut_hll_op_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<HllOp> {
        &mut self.hll_op
    }
}

impl ::protobuf::Message for DtOp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.counter_op)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.set_op)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.map_op)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.hll_op)?;
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
        if let Some(v) = self.counter_op.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.set_op.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.map_op.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.hll_op.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.counter_op.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.set_op.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.map_op.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.hll_op.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for DtOp {
    fn new() -> DtOp {
        DtOp::new()
    }

    fn descriptor_static(_: ::std::option::Option<DtOp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CounterOp>>(
                    "counter_op",
                    DtOp::get_counter_op_for_reflect,
                    DtOp::mut_counter_op_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SetOp>>(
                    "set_op",
                    DtOp::get_set_op_for_reflect,
                    DtOp::mut_set_op_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MapOp>>(
                    "map_op",
                    DtOp::get_map_op_for_reflect,
                    DtOp::mut_map_op_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<HllOp>>(
                    "hll_op",
                    DtOp::get_hll_op_for_reflect,
                    DtOp::mut_hll_op_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DtOp>(
                    "DtOp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DtOp {
    fn clear(&mut self) {
        self.clear_counter_op();
        self.clear_set_op();
        self.clear_map_op();
        self.clear_hll_op();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DtOp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DtOp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DtUpdateReq {
    // message fields
    bucket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    field_type: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    context: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    op: ::protobuf::SingularPtrField<DtOp>,
    w: ::std::option::Option<u32>,
    dw: ::std::option::Option<u32>,
    pw: ::std::option::Option<u32>,
    return_body: ::std::option::Option<bool>,
    timeout: ::std::option::Option<u32>,
    sloppy_quorum: ::std::option::Option<bool>,
    n_val: ::std::option::Option<u32>,
    include_context: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DtUpdateReq {}

impl DtUpdateReq {
    pub fn new() -> DtUpdateReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DtUpdateReq {
        static mut instance: ::protobuf::lazy::Lazy<DtUpdateReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DtUpdateReq,
        };
        unsafe {
            instance.get(DtUpdateReq::new)
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

    // required bytes type = 3;

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

    // optional bytes context = 4;

    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context(&mut self, v: ::std::vec::Vec<u8>) {
        self.context = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.context.is_none() {
            self.context.set_default();
        };
        self.context.as_mut().unwrap()
    }

    // Take field
    pub fn take_context(&mut self) -> ::std::vec::Vec<u8> {
        self.context.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_context(&self) -> &[u8] {
        match self.context.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_context_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.context
    }

    fn mut_context_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.context
    }

    // required .DtOp op = 5;

    pub fn clear_op(&mut self) {
        self.op.clear();
    }

    pub fn has_op(&self) -> bool {
        self.op.is_some()
    }

    // Param is passed by value, moved
    pub fn set_op(&mut self, v: DtOp) {
        self.op = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_op(&mut self) -> &mut DtOp {
        if self.op.is_none() {
            self.op.set_default();
        };
        self.op.as_mut().unwrap()
    }

    // Take field
    pub fn take_op(&mut self) -> DtOp {
        self.op.take().unwrap_or_else(|| DtOp::new())
    }

    pub fn get_op(&self) -> &DtOp {
        self.op.as_ref().unwrap_or_else(|| DtOp::default_instance())
    }

    fn get_op_for_reflect(&self) -> &::protobuf::SingularPtrField<DtOp> {
        &self.op
    }

    fn mut_op_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DtOp> {
        &mut self.op
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

    // optional uint32 dw = 7;

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

    // optional bool return_body = 9;

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

    // optional bool include_context = 13;

    pub fn clear_include_context(&mut self) {
        self.include_context = ::std::option::Option::None;
    }

    pub fn has_include_context(&self) -> bool {
        self.include_context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_include_context(&mut self, v: bool) {
        self.include_context = ::std::option::Option::Some(v);
    }

    pub fn get_include_context(&self) -> bool {
        self.include_context.unwrap_or(true)
    }

    fn get_include_context_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.include_context
    }

    fn mut_include_context_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.include_context
    }
}

impl ::protobuf::Message for DtUpdateReq {
    fn is_initialized(&self) -> bool {
        if self.bucket.is_none() {
            return false;
        };
        if self.field_type.is_none() {
            return false;
        };
        if self.op.is_none() {
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
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.context)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.op)?;
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
                    self.dw = ::std::option::Option::Some(tmp);
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
                    self.return_body = ::std::option::Option::Some(tmp);
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.include_context = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.context.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        };
        if let Some(v) = self.op.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.w {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.dw {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.pw {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.return_body {
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
        if let Some(v) = self.include_context {
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
        if let Some(v) = self.field_type.as_ref() {
            os.write_bytes(3, &v)?;
        };
        if let Some(v) = self.context.as_ref() {
            os.write_bytes(4, &v)?;
        };
        if let Some(v) = self.op.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.w {
            os.write_uint32(6, v)?;
        };
        if let Some(v) = self.dw {
            os.write_uint32(7, v)?;
        };
        if let Some(v) = self.pw {
            os.write_uint32(8, v)?;
        };
        if let Some(v) = self.return_body {
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
        if let Some(v) = self.include_context {
            os.write_bool(13, v)?;
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

impl ::protobuf::MessageStatic for DtUpdateReq {
    fn new() -> DtUpdateReq {
        DtUpdateReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<DtUpdateReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "bucket",
                    DtUpdateReq::get_bucket_for_reflect,
                    DtUpdateReq::mut_bucket_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    DtUpdateReq::get_key_for_reflect,
                    DtUpdateReq::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "type",
                    DtUpdateReq::get_field_type_for_reflect,
                    DtUpdateReq::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "context",
                    DtUpdateReq::get_context_for_reflect,
                    DtUpdateReq::mut_context_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DtOp>>(
                    "op",
                    DtUpdateReq::get_op_for_reflect,
                    DtUpdateReq::mut_op_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "w",
                    DtUpdateReq::get_w_for_reflect,
                    DtUpdateReq::mut_w_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "dw",
                    DtUpdateReq::get_dw_for_reflect,
                    DtUpdateReq::mut_dw_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "pw",
                    DtUpdateReq::get_pw_for_reflect,
                    DtUpdateReq::mut_pw_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "return_body",
                    DtUpdateReq::get_return_body_for_reflect,
                    DtUpdateReq::mut_return_body_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "timeout",
                    DtUpdateReq::get_timeout_for_reflect,
                    DtUpdateReq::mut_timeout_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "sloppy_quorum",
                    DtUpdateReq::get_sloppy_quorum_for_reflect,
                    DtUpdateReq::mut_sloppy_quorum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "n_val",
                    DtUpdateReq::get_n_val_for_reflect,
                    DtUpdateReq::mut_n_val_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "include_context",
                    DtUpdateReq::get_include_context_for_reflect,
                    DtUpdateReq::mut_include_context_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DtUpdateReq>(
                    "DtUpdateReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DtUpdateReq {
    fn clear(&mut self) {
        self.clear_bucket();
        self.clear_key();
        self.clear_field_type();
        self.clear_context();
        self.clear_op();
        self.clear_w();
        self.clear_dw();
        self.clear_pw();
        self.clear_return_body();
        self.clear_timeout();
        self.clear_sloppy_quorum();
        self.clear_n_val();
        self.clear_include_context();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DtUpdateReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DtUpdateReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DtUpdateResp {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    context: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    counter_value: ::std::option::Option<i64>,
    set_value: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    map_value: ::protobuf::RepeatedField<MapEntry>,
    hll_value: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DtUpdateResp {}

impl DtUpdateResp {
    pub fn new() -> DtUpdateResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DtUpdateResp {
        static mut instance: ::protobuf::lazy::Lazy<DtUpdateResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DtUpdateResp,
        };
        unsafe {
            instance.get(DtUpdateResp::new)
        }
    }

    // optional bytes key = 1;

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

    // optional bytes context = 2;

    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context(&mut self, v: ::std::vec::Vec<u8>) {
        self.context = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.context.is_none() {
            self.context.set_default();
        };
        self.context.as_mut().unwrap()
    }

    // Take field
    pub fn take_context(&mut self) -> ::std::vec::Vec<u8> {
        self.context.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_context(&self) -> &[u8] {
        match self.context.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_context_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.context
    }

    fn mut_context_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.context
    }

    // optional sint64 counter_value = 3;

    pub fn clear_counter_value(&mut self) {
        self.counter_value = ::std::option::Option::None;
    }

    pub fn has_counter_value(&self) -> bool {
        self.counter_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_counter_value(&mut self, v: i64) {
        self.counter_value = ::std::option::Option::Some(v);
    }

    pub fn get_counter_value(&self) -> i64 {
        self.counter_value.unwrap_or(0)
    }

    fn get_counter_value_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.counter_value
    }

    fn mut_counter_value_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.counter_value
    }

    // repeated bytes set_value = 4;

    pub fn clear_set_value(&mut self) {
        self.set_value.clear();
    }

    // Param is passed by value, moved
    pub fn set_set_value(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.set_value = v;
    }

    // Mutable pointer to the field.
    pub fn mut_set_value(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.set_value
    }

    // Take field
    pub fn take_set_value(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.set_value, ::protobuf::RepeatedField::new())
    }

    pub fn get_set_value(&self) -> &[::std::vec::Vec<u8>] {
        &self.set_value
    }

    fn get_set_value_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.set_value
    }

    fn mut_set_value_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.set_value
    }

    // repeated .MapEntry map_value = 5;

    pub fn clear_map_value(&mut self) {
        self.map_value.clear();
    }

    // Param is passed by value, moved
    pub fn set_map_value(&mut self, v: ::protobuf::RepeatedField<MapEntry>) {
        self.map_value = v;
    }

    // Mutable pointer to the field.
    pub fn mut_map_value(&mut self) -> &mut ::protobuf::RepeatedField<MapEntry> {
        &mut self.map_value
    }

    // Take field
    pub fn take_map_value(&mut self) -> ::protobuf::RepeatedField<MapEntry> {
        ::std::mem::replace(&mut self.map_value, ::protobuf::RepeatedField::new())
    }

    pub fn get_map_value(&self) -> &[MapEntry] {
        &self.map_value
    }

    fn get_map_value_for_reflect(&self) -> &::protobuf::RepeatedField<MapEntry> {
        &self.map_value
    }

    fn mut_map_value_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<MapEntry> {
        &mut self.map_value
    }

    // optional uint64 hll_value = 6;

    pub fn clear_hll_value(&mut self) {
        self.hll_value = ::std::option::Option::None;
    }

    pub fn has_hll_value(&self) -> bool {
        self.hll_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hll_value(&mut self, v: u64) {
        self.hll_value = ::std::option::Option::Some(v);
    }

    pub fn get_hll_value(&self) -> u64 {
        self.hll_value.unwrap_or(0)
    }

    fn get_hll_value_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.hll_value
    }

    fn mut_hll_value_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.hll_value
    }
}

impl ::protobuf::Message for DtUpdateResp {
    fn is_initialized(&self) -> bool {
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
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.context)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_sint64()?;
                    self.counter_value = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.set_value)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.map_value)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.hll_value = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.context.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.counter_value {
            my_size += ::protobuf::rt::value_varint_zigzag_size(3, v);
        };
        for value in &self.set_value {
            my_size += ::protobuf::rt::bytes_size(4, &value);
        };
        for value in &self.map_value {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.hll_value {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.context.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.counter_value {
            os.write_sint64(3, v)?;
        };
        for v in &self.set_value {
            os.write_bytes(4, &v)?;
        };
        for v in &self.map_value {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.hll_value {
            os.write_uint64(6, v)?;
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

impl ::protobuf::MessageStatic for DtUpdateResp {
    fn new() -> DtUpdateResp {
        DtUpdateResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<DtUpdateResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    DtUpdateResp::get_key_for_reflect,
                    DtUpdateResp::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "context",
                    DtUpdateResp::get_context_for_reflect,
                    DtUpdateResp::mut_context_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint64>(
                    "counter_value",
                    DtUpdateResp::get_counter_value_for_reflect,
                    DtUpdateResp::mut_counter_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "set_value",
                    DtUpdateResp::get_set_value_for_reflect,
                    DtUpdateResp::mut_set_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MapEntry>>(
                    "map_value",
                    DtUpdateResp::get_map_value_for_reflect,
                    DtUpdateResp::mut_map_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "hll_value",
                    DtUpdateResp::get_hll_value_for_reflect,
                    DtUpdateResp::mut_hll_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DtUpdateResp>(
                    "DtUpdateResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DtUpdateResp {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_context();
        self.clear_counter_value();
        self.clear_set_value();
        self.clear_map_value();
        self.clear_hll_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DtUpdateResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DtUpdateResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0d, 0x72, 0x69, 0x61, 0x6b, 0x5f, 0x64, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22,
    0x85, 0x01, 0x0a, 0x08, 0x4d, 0x61, 0x70, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x0c, 0x0a, 0x04,
    0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x24, 0x0a, 0x04, 0x74, 0x79,
    0x70, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x16, 0x2e, 0x4d, 0x61, 0x70, 0x46, 0x69,
    0x65, 0x6c, 0x64, 0x2e, 0x4d, 0x61, 0x70, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x54, 0x79, 0x70, 0x65,
    0x22, 0x45, 0x0a, 0x0c, 0x4d, 0x61, 0x70, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x54, 0x79, 0x70, 0x65,
    0x12, 0x0b, 0x0a, 0x07, 0x43, 0x4f, 0x55, 0x4e, 0x54, 0x45, 0x52, 0x10, 0x01, 0x12, 0x07, 0x0a,
    0x03, 0x53, 0x45, 0x54, 0x10, 0x02, 0x12, 0x0c, 0x0a, 0x08, 0x52, 0x45, 0x47, 0x49, 0x53, 0x54,
    0x45, 0x52, 0x10, 0x03, 0x12, 0x08, 0x0a, 0x04, 0x46, 0x4c, 0x41, 0x47, 0x10, 0x04, 0x12, 0x07,
    0x0a, 0x03, 0x4d, 0x41, 0x50, 0x10, 0x05, 0x22, 0x98, 0x01, 0x0a, 0x08, 0x4d, 0x61, 0x70, 0x45,
    0x6e, 0x74, 0x72, 0x79, 0x12, 0x18, 0x0a, 0x05, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x4d, 0x61, 0x70, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x15,
    0x0a, 0x0d, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x12, 0x12, 0x11, 0x0a, 0x09, 0x73, 0x65, 0x74, 0x5f, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0c, 0x12, 0x16, 0x0a, 0x0e, 0x72, 0x65, 0x67, 0x69,
    0x73, 0x74, 0x65, 0x72, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c,
    0x12, 0x12, 0x0a, 0x0a, 0x66, 0x6c, 0x61, 0x67, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x08, 0x12, 0x1c, 0x0a, 0x09, 0x6d, 0x61, 0x70, 0x5f, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x18, 0x06, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x4d, 0x61, 0x70, 0x45, 0x6e, 0x74,
    0x72, 0x79, 0x22, 0xcf, 0x01, 0x0a, 0x0a, 0x44, 0x74, 0x46, 0x65, 0x74, 0x63, 0x68, 0x52, 0x65,
    0x71, 0x12, 0x0e, 0x0a, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x0c, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0c,
    0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x09, 0x0a, 0x01,
    0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0a, 0x0a, 0x02, 0x70, 0x72, 0x18, 0x05, 0x20,
    0x01, 0x28, 0x0d, 0x12, 0x14, 0x0a, 0x0c, 0x62, 0x61, 0x73, 0x69, 0x63, 0x5f, 0x71, 0x75, 0x6f,
    0x72, 0x75, 0x6d, 0x18, 0x06, 0x20, 0x01, 0x28, 0x08, 0x12, 0x13, 0x0a, 0x0b, 0x6e, 0x6f, 0x74,
    0x66, 0x6f, 0x75, 0x6e, 0x64, 0x5f, 0x6f, 0x6b, 0x18, 0x07, 0x20, 0x01, 0x28, 0x08, 0x12, 0x0f,
    0x0a, 0x07, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0d, 0x12,
    0x15, 0x0a, 0x0d, 0x73, 0x6c, 0x6f, 0x70, 0x70, 0x79, 0x5f, 0x71, 0x75, 0x6f, 0x72, 0x75, 0x6d,
    0x18, 0x09, 0x20, 0x01, 0x28, 0x08, 0x12, 0x0d, 0x0a, 0x05, 0x6e, 0x5f, 0x76, 0x61, 0x6c, 0x18,
    0x0a, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x1d, 0x0a, 0x0f, 0x69, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65,
    0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x08, 0x3a, 0x04,
    0x74, 0x72, 0x75, 0x65, 0x22, 0x64, 0x0a, 0x07, 0x44, 0x74, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12,
    0x15, 0x0a, 0x0d, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x12, 0x12, 0x11, 0x0a, 0x09, 0x73, 0x65, 0x74, 0x5f, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0c, 0x12, 0x1c, 0x0a, 0x09, 0x6d, 0x61, 0x70,
    0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x4d,
    0x61, 0x70, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x11, 0x0a, 0x09, 0x68, 0x6c, 0x6c, 0x5f, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x04, 0x22, 0x90, 0x01, 0x0a, 0x0b, 0x44,
    0x74, 0x46, 0x65, 0x74, 0x63, 0x68, 0x52, 0x65, 0x73, 0x70, 0x12, 0x0f, 0x0a, 0x07, 0x63, 0x6f,
    0x6e, 0x74, 0x65, 0x78, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x23, 0x0a, 0x04, 0x74,
    0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x15, 0x2e, 0x44, 0x74, 0x46, 0x65,
    0x74, 0x63, 0x68, 0x52, 0x65, 0x73, 0x70, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x54, 0x79, 0x70, 0x65,
    0x12, 0x17, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x08, 0x2e, 0x44, 0x74, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x32, 0x0a, 0x08, 0x44, 0x61, 0x74,
    0x61, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0b, 0x0a, 0x07, 0x43, 0x4f, 0x55, 0x4e, 0x54, 0x45, 0x52,
    0x10, 0x01, 0x12, 0x07, 0x0a, 0x03, 0x53, 0x45, 0x54, 0x10, 0x02, 0x12, 0x07, 0x0a, 0x03, 0x4d,
    0x41, 0x50, 0x10, 0x03, 0x12, 0x07, 0x0a, 0x03, 0x48, 0x4c, 0x4c, 0x10, 0x04, 0x22, 0x1e, 0x0a,
    0x09, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x4f, 0x70, 0x12, 0x11, 0x0a, 0x09, 0x69, 0x6e,
    0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x12, 0x22, 0x26, 0x0a,
    0x05, 0x53, 0x65, 0x74, 0x4f, 0x70, 0x12, 0x0c, 0x0a, 0x04, 0x61, 0x64, 0x64, 0x73, 0x18, 0x01,
    0x20, 0x03, 0x28, 0x0c, 0x12, 0x0f, 0x0a, 0x07, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x73, 0x18,
    0x02, 0x20, 0x03, 0x28, 0x0c, 0x22, 0x15, 0x0a, 0x05, 0x48, 0x6c, 0x6c, 0x4f, 0x70, 0x12, 0x0c,
    0x0a, 0x04, 0x61, 0x64, 0x64, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0c, 0x22, 0xd1, 0x01, 0x0a,
    0x09, 0x4d, 0x61, 0x70, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x12, 0x18, 0x0a, 0x05, 0x66, 0x69,
    0x65, 0x6c, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x4d, 0x61, 0x70, 0x46,
    0x69, 0x65, 0x6c, 0x64, 0x12, 0x1e, 0x0a, 0x0a, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x5f,
    0x6f, 0x70, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x43, 0x6f, 0x75, 0x6e, 0x74,
    0x65, 0x72, 0x4f, 0x70, 0x12, 0x16, 0x0a, 0x06, 0x73, 0x65, 0x74, 0x5f, 0x6f, 0x70, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x53, 0x65, 0x74, 0x4f, 0x70, 0x12, 0x13, 0x0a, 0x0b,
    0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x5f, 0x6f, 0x70, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0c, 0x12, 0x22, 0x0a, 0x07, 0x66, 0x6c, 0x61, 0x67, 0x5f, 0x6f, 0x70, 0x18, 0x05, 0x20, 0x01,
    0x28, 0x0e, 0x32, 0x11, 0x2e, 0x4d, 0x61, 0x70, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x46,
    0x6c, 0x61, 0x67, 0x4f, 0x70, 0x12, 0x16, 0x0a, 0x06, 0x6d, 0x61, 0x70, 0x5f, 0x6f, 0x70, 0x18,
    0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x4d, 0x61, 0x70, 0x4f, 0x70, 0x22, 0x21, 0x0a,
    0x06, 0x46, 0x6c, 0x61, 0x67, 0x4f, 0x70, 0x12, 0x0a, 0x0a, 0x06, 0x45, 0x4e, 0x41, 0x42, 0x4c,
    0x45, 0x10, 0x01, 0x12, 0x0b, 0x0a, 0x07, 0x44, 0x49, 0x53, 0x41, 0x42, 0x4c, 0x45, 0x10, 0x02,
    0x22, 0x40, 0x0a, 0x05, 0x4d, 0x61, 0x70, 0x4f, 0x70, 0x12, 0x1a, 0x0a, 0x07, 0x72, 0x65, 0x6d,
    0x6f, 0x76, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x4d, 0x61, 0x70,
    0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x1b, 0x0a, 0x07, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73,
    0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x4d, 0x61, 0x70, 0x55, 0x70, 0x64, 0x61,
    0x74, 0x65, 0x22, 0x6e, 0x0a, 0x04, 0x44, 0x74, 0x4f, 0x70, 0x12, 0x1e, 0x0a, 0x0a, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x65, 0x72, 0x5f, 0x6f, 0x70, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0a,
    0x2e, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x4f, 0x70, 0x12, 0x16, 0x0a, 0x06, 0x73, 0x65,
    0x74, 0x5f, 0x6f, 0x70, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x53, 0x65, 0x74,
    0x4f, 0x70, 0x12, 0x16, 0x0a, 0x06, 0x6d, 0x61, 0x70, 0x5f, 0x6f, 0x70, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x06, 0x2e, 0x4d, 0x61, 0x70, 0x4f, 0x70, 0x12, 0x16, 0x0a, 0x06, 0x68, 0x6c,
    0x6c, 0x5f, 0x6f, 0x70, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x48, 0x6c, 0x6c,
    0x4f, 0x70, 0x22, 0xf1, 0x01, 0x0a, 0x0b, 0x44, 0x74, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x52,
    0x65, 0x71, 0x12, 0x0e, 0x0a, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x0c, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x12,
    0x0c, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0f, 0x0a,
    0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x11,
    0x0a, 0x02, 0x6f, 0x70, 0x18, 0x05, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x05, 0x2e, 0x44, 0x74, 0x4f,
    0x70, 0x12, 0x09, 0x0a, 0x01, 0x77, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0a, 0x0a, 0x02,
    0x64, 0x77, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0a, 0x0a, 0x02, 0x70, 0x77, 0x18, 0x08,
    0x20, 0x01, 0x28, 0x0d, 0x12, 0x1a, 0x0a, 0x0b, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x5f, 0x62,
    0x6f, 0x64, 0x79, 0x18, 0x09, 0x20, 0x01, 0x28, 0x08, 0x3a, 0x05, 0x66, 0x61, 0x6c, 0x73, 0x65,
    0x12, 0x0f, 0x0a, 0x07, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x18, 0x0a, 0x20, 0x01, 0x28,
    0x0d, 0x12, 0x15, 0x0a, 0x0d, 0x73, 0x6c, 0x6f, 0x70, 0x70, 0x79, 0x5f, 0x71, 0x75, 0x6f, 0x72,
    0x75, 0x6d, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x08, 0x12, 0x0d, 0x0a, 0x05, 0x6e, 0x5f, 0x76, 0x61,
    0x6c, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x1d, 0x0a, 0x0f, 0x69, 0x6e, 0x63, 0x6c, 0x75,
    0x64, 0x65, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x08,
    0x3a, 0x04, 0x74, 0x72, 0x75, 0x65, 0x22, 0x87, 0x01, 0x0a, 0x0c, 0x44, 0x74, 0x55, 0x70, 0x64,
    0x61, 0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0c, 0x12, 0x0f, 0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x15, 0x0a, 0x0d, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72,
    0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x12, 0x12, 0x11, 0x0a, 0x09,
    0x73, 0x65, 0x74, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0c, 0x12,
    0x1c, 0x0a, 0x09, 0x6d, 0x61, 0x70, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x05, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x09, 0x2e, 0x4d, 0x61, 0x70, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x11, 0x0a,
    0x09, 0x68, 0x6c, 0x6c, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x04,
    0x42, 0x23, 0x0a, 0x17, 0x63, 0x6f, 0x6d, 0x2e, 0x62, 0x61, 0x73, 0x68, 0x6f, 0x2e, 0x72, 0x69,
    0x61, 0x6b, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x42, 0x08, 0x52, 0x69, 0x61,
    0x6b, 0x44, 0x74, 0x50, 0x42, 0x4a, 0xcf, 0x48, 0x0a, 0x07, 0x12, 0x05, 0x1c, 0x00, 0x91, 0x02,
    0x01, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x1c, 0x00, 0x30, 0x0a, 0x26, 0x0a, 0x04, 0x08,
    0xe7, 0x07, 0x00, 0x12, 0x03, 0x1c, 0x00, 0x30, 0x1a, 0x19, 0x20, 0x4a, 0x61, 0x76, 0x61, 0x20,
    0x70, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65,
    0x72, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x1c, 0x07,
    0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1c, 0x07, 0x13,
    0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x07, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x07, 0x12, 0x03, 0x1c, 0x16, 0x2f, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x1d, 0x00, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01,
    0x12, 0x03, 0x1d, 0x00, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03,
    0x1d, 0x07, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x1d,
    0x07, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d,
    0x07, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x07, 0x12, 0x03, 0x1d, 0x1e, 0x28,
    0x0a, 0xcc, 0x01, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x28, 0x00, 0x37, 0x01, 0x1a, 0xbf, 0x01,
    0x0a, 0x20, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x20, 0x69, 0x6e,
    0x20, 0x6d, 0x61, 0x70, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x73,
    0x65, 0x64, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x62, 0x69, 0x6e, 0x61, 0x72, 0x79, 0x20, 0x69,
    0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x61, 0x20,
    0x74, 0x79, 0x70, 0x65, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x73,
    0x6f, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x77, 0x6f, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x73, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x20, 0x66, 0x69,
    0x65, 0x6c, 0x64, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x61,
    0x6d, 0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x0a, 0x20, 0x62, 0x75, 0x74, 0x20, 0x64, 0x69, 0x66,
    0x66, 0x65, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2c, 0x20, 0x61, 0x6e,
    0x64, 0x20, 0x74, 0x68, 0x65, 0x79, 0x20, 0x63, 0x6f, 0x6e, 0x76, 0x65, 0x72, 0x67, 0x65, 0x20,
    0x69, 0x6e, 0x64, 0x65, 0x70, 0x65, 0x6e, 0x64, 0x65, 0x6e, 0x74, 0x6c, 0x79, 0x2e, 0x0a, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x28, 0x08, 0x10, 0x0a, 0x72, 0x0a, 0x04, 0x04,
    0x00, 0x04, 0x00, 0x12, 0x04, 0x2d, 0x04, 0x33, 0x05, 0x1a, 0x64, 0x0a, 0x20, 0x54, 0x68, 0x65,
    0x20, 0x74, 0x79, 0x70, 0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x63, 0x61, 0x6e, 0x20,
    0x62, 0x65, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x61, 0x20, 0x6d,
    0x61, 0x70, 0x20, 0x61, 0x72, 0x65, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x65, 0x64, 0x20, 0x74,
    0x6f, 0x20, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x73, 0x2c, 0x0a, 0x20, 0x73, 0x65, 0x74,
    0x73, 0x2c, 0x20, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x73, 0x2c, 0x20, 0x66, 0x6c,
    0x61, 0x67, 0x73, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x6d, 0x61, 0x70, 0x73, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x04, 0x00, 0x01, 0x12, 0x03, 0x2d, 0x09, 0x15, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x2e, 0x08, 0x15, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2e, 0x08, 0x0f, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x2e, 0x13, 0x14, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x2f, 0x08, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2f, 0x08, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x2f, 0x13, 0x14, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x00, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x30, 0x08, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x30, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x30, 0x13, 0x14, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00,
    0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x31, 0x08, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04,
    0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x31, 0x08, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04,
    0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x31, 0x13, 0x14, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04,
    0x00, 0x02, 0x04, 0x12, 0x03, 0x32, 0x08, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x32, 0x08, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00,
    0x02, 0x04, 0x02, 0x12, 0x03, 0x32, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x35, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x35, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x35, 0x0d,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x35, 0x1a, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x35, 0x21, 0x22, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x36, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x36, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x36, 0x0d, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x36, 0x1a, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x36,
    0x21, 0x22, 0x0a, 0x8e, 0x01, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x3e, 0x00, 0x45, 0x01, 0x1a,
    0x81, 0x01, 0x0a, 0x20, 0x41, 0x6e, 0x20, 0x65, 0x6e, 0x74, 0x72, 0x79, 0x20, 0x69, 0x6e, 0x20,
    0x61, 0x20, 0x6d, 0x61, 0x70, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x70, 0x61, 0x69, 0x72, 0x20,
    0x6f, 0x66, 0x20, 0x61, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x2d, 0x6e, 0x61, 0x6d, 0x65, 0x20,
    0x61, 0x6e, 0x64, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x74,
    0x79, 0x70, 0x65, 0x0a, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x64, 0x65, 0x74, 0x65, 0x72, 0x6d,
    0x69, 0x6e, 0x65, 0x73, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x69, 0x73, 0x20, 0x65, 0x78, 0x70, 0x65, 0x63, 0x74, 0x65,
    0x64, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x3e, 0x08, 0x10, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x3f, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x3f, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x3f, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x3f, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x40, 0x04,
    0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x40, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x40, 0x0d, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x40, 0x16, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x40, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x02, 0x12, 0x03, 0x41, 0x04, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x41, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x41,
    0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x41, 0x16, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x41, 0x27, 0x28, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x42, 0x04, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x42, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x03, 0x05, 0x12, 0x03, 0x42, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x42, 0x16, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x42, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x43, 0x04, 0x29,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x04, 0x12, 0x03, 0x43, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x05, 0x12, 0x03, 0x43, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x43, 0x16, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x43, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x05,
    0x12, 0x03, 0x44, 0x04, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x04, 0x12, 0x03,
    0x44, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x06, 0x12, 0x03, 0x44, 0x0d,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x44, 0x16, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x03, 0x12, 0x03, 0x44, 0x27, 0x28, 0x0a, 0xa8, 0x01,
    0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x50, 0x00, 0x62, 0x01, 0x1a, 0x9b, 0x01, 0x0a, 0x20, 0x54,
    0x68, 0x65, 0x20, 0x65, 0x71, 0x75, 0x69, 0x76, 0x61, 0x6c, 0x65, 0x6e, 0x74, 0x20, 0x6f, 0x66,
    0x20, 0x4b, 0x56, 0x27, 0x73, 0x20, 0x22, 0x52, 0x70, 0x62, 0x47, 0x65, 0x74, 0x52, 0x65, 0x71,
    0x22, 0x2c, 0x20, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x61, 0x20,
    0x44, 0x74, 0x46, 0x65, 0x74, 0x63, 0x68, 0x52, 0x65, 0x73, 0x70, 0x2e, 0x20, 0x54, 0x68, 0x65,
    0x0a, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2d, 0x74, 0x69, 0x6d, 0x65, 0x20, 0x6f,
    0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74,
    0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x6f, 0x6e, 0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20,
    0x61, 0x72, 0x65, 0x20, 0x72, 0x65, 0x6c, 0x65, 0x76, 0x61, 0x6e, 0x74, 0x20, 0x74, 0x6f, 0x0a,
    0x20, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x64, 0x20, 0x64, 0x61, 0x74, 0x61,
    0x2d, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12,
    0x03, 0x50, 0x08, 0x12, 0x0a, 0x3a, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x52, 0x04,
    0x1e, 0x1a, 0x2d, 0x20, 0x54, 0x68, 0x65, 0x20, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69,
    0x65, 0x72, 0x3a, 0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x2c, 0x20, 0x6b, 0x65, 0x79, 0x20,
    0x61, 0x6e, 0x64, 0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x2d, 0x74, 0x79, 0x70, 0x65, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x52, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x52, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x52, 0x13, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x52, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x53, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x53, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x53, 0x0d,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x53, 0x13, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x53, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x54, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x54, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x54, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x54, 0x13, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x54,
    0x1b, 0x1c, 0x0a, 0x1e, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x57, 0x04, 0x27, 0x1a,
    0x11, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e,
    0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x57, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x57, 0x0d, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x57, 0x14, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x57, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x58, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x04,
    0x12, 0x03, 0x58, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x05, 0x12, 0x03,
    0x58, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x58, 0x14,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x58, 0x25, 0x26, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x59, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x05, 0x04, 0x12, 0x03, 0x59, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x05, 0x05, 0x12, 0x03, 0x59, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05,
    0x01, 0x12, 0x03, 0x59, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x03, 0x12,
    0x03, 0x59, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x5a, 0x04,
    0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x04, 0x12, 0x03, 0x5a, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x05, 0x12, 0x03, 0x5a, 0x0d, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x06, 0x01, 0x12, 0x03, 0x5a, 0x14, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x06, 0x03, 0x12, 0x03, 0x5a, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x07, 0x12, 0x03, 0x5b, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x04, 0x12,
    0x03, 0x5b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x05, 0x12, 0x03, 0x5b,
    0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x01, 0x12, 0x03, 0x5b, 0x14, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x03, 0x12, 0x03, 0x5b, 0x25, 0x26, 0x0a, 0x31,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x08, 0x12, 0x03, 0x5c, 0x04, 0x27, 0x22, 0x24, 0x20, 0x45, 0x78,
    0x70, 0x65, 0x72, 0x69, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x6c, 0x2c, 0x20, 0x6d, 0x61, 0x79, 0x20,
    0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x2f, 0x64, 0x69, 0x73, 0x61, 0x70, 0x70, 0x65, 0x61, 0x72,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x04, 0x12, 0x03, 0x5c, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x05, 0x12, 0x03, 0x5c, 0x0d, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x08, 0x01, 0x12, 0x03, 0x5c, 0x14, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x08, 0x03, 0x12, 0x03, 0x5c, 0x25, 0x26, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x09, 0x12, 0x03, 0x5d, 0x04, 0x27, 0x22, 0x24, 0x20, 0x45, 0x78, 0x70, 0x65, 0x72, 0x69, 0x6d,
    0x65, 0x6e, 0x74, 0x61, 0x6c, 0x2c, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67,
    0x65, 0x2f, 0x64, 0x69, 0x73, 0x61, 0x70, 0x70, 0x65, 0x61, 0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x09, 0x04, 0x12, 0x03, 0x5d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x09, 0x05, 0x12, 0x03, 0x5d, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x09,
    0x01, 0x12, 0x03, 0x5d, 0x14, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x09, 0x03, 0x12,
    0x03, 0x5d, 0x24, 0x26, 0x0a, 0x88, 0x01, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x0a, 0x12, 0x03, 0x61,
    0x04, 0x36, 0x1a, 0x7b, 0x20, 0x46, 0x6f, 0x72, 0x20, 0x72, 0x65, 0x61, 0x64, 0x2d, 0x6f, 0x6e,
    0x6c, 0x79, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x20, 0x6f, 0x72, 0x20, 0x63,
    0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x2d, 0x66, 0x72, 0x65, 0x65, 0x20, 0x6f, 0x70, 0x65, 0x72,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2c, 0x20, 0x79, 0x6f, 0x75, 0x20, 0x63, 0x61, 0x6e, 0x20,
    0x73, 0x65, 0x74, 0x0a, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x66, 0x61, 0x6c,
    0x73, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x64, 0x75, 0x63, 0x65, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x73, 0x69, 0x7a, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x61, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x61, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x61, 0x12, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x0a, 0x03, 0x12, 0x03, 0x61, 0x24, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0a, 0x08,
    0x12, 0x03, 0x61, 0x27, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0a, 0x07, 0x12, 0x03,
    0x61, 0x30, 0x34, 0x0a, 0x89, 0x01, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x69, 0x00, 0x72, 0x01,
    0x1a, 0x7d, 0x0a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x65, 0x74, 0x63, 0x68, 0x65, 0x64, 0x20, 0x64, 0x61, 0x74,
    0x61, 0x20, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x70, 0x72, 0x65, 0x73, 0x65,
    0x6e, 0x74, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x2c, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x6e, 0x20, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x20,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x20, 0x28, 0x73, 0x65, 0x74, 0x73, 0x2c, 0x20, 0x6d, 0x61,
    0x70, 0x73, 0x29, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x74, 0x72,
    0x65, 0x61, 0x74, 0x65, 0x64, 0x20, 0x61, 0x73, 0x20, 0x73, 0x75, 0x63, 0x68, 0x2e, 0x0a, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x69, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x00, 0x12, 0x03, 0x6a, 0x04, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x6a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x6a, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6a,
    0x16, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6a, 0x26, 0x27,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x6b, 0x04, 0x28, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x6b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x6b, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x6b, 0x16, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x6b, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x6c,
    0x04, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x6c, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x06, 0x12, 0x03, 0x6c, 0x0d, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x6c, 0x16, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x6c, 0x26, 0x27, 0x0a, 0x53, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x03, 0x12, 0x03, 0x71, 0x04, 0x28, 0x1a, 0x46, 0x20, 0x57, 0x65, 0x20, 0x72, 0x65, 0x74,
    0x75, 0x72, 0x6e, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x73, 0x74, 0x69, 0x6d, 0x61, 0x74, 0x65, 0x64,
    0x20, 0x63, 0x61, 0x72, 0x64, 0x69, 0x6e, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x48, 0x79, 0x70, 0x65, 0x72, 0x6c, 0x6f, 0x67, 0x6c, 0x6f, 0x67, 0x20,
    0x73, 0x65, 0x74, 0x0a, 0x20, 0x6f, 0x6e, 0x20, 0x66, 0x65, 0x74, 0x63, 0x68, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x04, 0x12, 0x03, 0x71, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x03, 0x05, 0x12, 0x03, 0x71, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x03, 0x01, 0x12, 0x03, 0x71, 0x16, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x71, 0x26, 0x27, 0x0a, 0xd7, 0x03, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x05,
    0x7e, 0x00, 0x89, 0x01, 0x01, 0x1a, 0xc9, 0x03, 0x0a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x72, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x20, 0x22, 0x46, 0x65, 0x74,
    0x63, 0x68, 0x22, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x20, 0x49, 0x66, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x60, 0x69, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65, 0x5f, 0x63, 0x6f, 0x6e,
    0x74, 0x65, 0x78, 0x74, 0x60, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x20, 0x69, 0x73,
    0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x2c, 0x20, 0x61, 0x6e, 0x20, 0x6f,
    0x70, 0x61, 0x71, 0x75, 0x65, 0x20, 0x22, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x22, 0x20,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x72, 0x65,
    0x74, 0x75, 0x72, 0x6e, 0x65, 0x64, 0x20, 0x61, 0x6c, 0x6f, 0x6e, 0x67, 0x20, 0x77, 0x69, 0x74,
    0x68, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x75, 0x73, 0x65, 0x72, 0x2d, 0x66, 0x72, 0x69, 0x65,
    0x6e, 0x64, 0x6c, 0x79, 0x20, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x20, 0x57, 0x68, 0x65, 0x6e, 0x20,
    0x73, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x6e, 0x20, 0x22, 0x55, 0x70, 0x64, 0x61,
    0x74, 0x65, 0x22, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2c, 0x20, 0x74, 0x68, 0x65,
    0x0a, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20,
    0x73, 0x65, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78,
    0x74, 0x20, 0x61, 0x73, 0x20, 0x77, 0x65, 0x6c, 0x6c, 0x2c, 0x20, 0x73, 0x69, 0x6d, 0x69, 0x6c,
    0x61, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x68, 0x6f, 0x77, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x77, 0x6f,
    0x75, 0x6c, 0x64, 0x0a, 0x20, 0x73, 0x65, 0x6e, 0x64, 0x20, 0x61, 0x20, 0x76, 0x63, 0x6c, 0x6f,
    0x63, 0x6b, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x4b, 0x56, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65,
    0x73, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x60, 0x74, 0x79, 0x70, 0x65, 0x60, 0x20, 0x66, 0x69,
    0x65, 0x6c, 0x64, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73, 0x20, 0x77, 0x68,
    0x69, 0x63, 0x68, 0x0a, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20,
    0x74, 0x6f, 0x20, 0x65, 0x78, 0x70, 0x65, 0x63, 0x74, 0x2e, 0x20, 0x57, 0x68, 0x65, 0x6e, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x60, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x60, 0x20, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x20, 0x69, 0x73, 0x20, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x66, 0x72, 0x6f,
    0x6d, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2c, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c,
    0x64, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x70, 0x72, 0x65, 0x74, 0x20, 0x69, 0x74, 0x20, 0x61,
    0x73, 0x20, 0x61, 0x20, 0x22, 0x6e, 0x6f, 0x74, 0x20, 0x66, 0x6f, 0x75, 0x6e, 0x64, 0x22, 0x2e,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x7e, 0x08, 0x13, 0x0a, 0x0d, 0x0a,
    0x04, 0x04, 0x04, 0x04, 0x00, 0x12, 0x05, 0x7f, 0x04, 0x84, 0x01, 0x05, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x04, 0x00, 0x01, 0x12, 0x03, 0x7f, 0x09, 0x11, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x04,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0x80, 0x01, 0x08, 0x14, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x04,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0x80, 0x01, 0x08, 0x0f, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x04, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0x80, 0x01, 0x12, 0x13, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0x81, 0x01, 0x08, 0x14, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x04, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0x81, 0x01, 0x08, 0x0b, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0x81, 0x01, 0x12, 0x13, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0x82, 0x01, 0x08, 0x14, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0x82, 0x01, 0x08, 0x0b, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0x82, 0x01, 0x12, 0x13,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0x83, 0x01, 0x08, 0x14,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0x83, 0x01, 0x08,
    0x0b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0x83, 0x01,
    0x12, 0x13, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x04, 0x86, 0x01, 0x04, 0x22,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x04, 0x86, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x04, 0x86, 0x01, 0x0d, 0x12, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x04, 0x86, 0x01, 0x16, 0x1d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x04, 0x86, 0x01, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x01, 0x12, 0x04, 0x87, 0x01, 0x04, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x87, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x01, 0x06, 0x12, 0x04, 0x87, 0x01, 0x0d, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x01, 0x12, 0x04, 0x87, 0x01, 0x16, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03,
    0x12, 0x04, 0x87, 0x01, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x04,
    0x88, 0x01, 0x04, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x04, 0x88,
    0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x04, 0x88, 0x01,
    0x0d, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x04, 0x88, 0x01, 0x16,
    0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x04, 0x88, 0x01, 0x20, 0x21,
    0x0a, 0xba, 0x01, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x06, 0x94, 0x01, 0x00, 0x96, 0x01, 0x01, 0x1a,
    0xab, 0x01, 0x0a, 0x20, 0x41, 0x6e, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x20, 0x74, 0x6f, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x20, 0x61, 0x20, 0x43, 0x6f, 0x75,
    0x6e, 0x74, 0x65, 0x72, 0x2c, 0x20, 0x65, 0x69, 0x74, 0x68, 0x65, 0x72, 0x20, 0x6f, 0x6e, 0x20,
    0x69, 0x74, 0x73, 0x20, 0x6f, 0x77, 0x6e, 0x20, 0x6f, 0x72, 0x20, 0x69, 0x6e, 0x73, 0x69, 0x64,
    0x65, 0x20, 0x61, 0x0a, 0x20, 0x4d, 0x61, 0x70, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x60, 0x69,
    0x6e, 0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x60, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20,
    0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x76, 0x65, 0x20,
    0x6f, 0x72, 0x20, 0x6e, 0x65, 0x67, 0x61, 0x74, 0x69, 0x76, 0x65, 0x2e, 0x20, 0x57, 0x68, 0x65,
    0x6e, 0x20, 0x61, 0x62, 0x73, 0x65, 0x6e, 0x74, 0x2c, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d,
    0x65, 0x61, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x69, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x69, 0x6e, 0x63,
    0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x62, 0x79, 0x20, 0x31, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x05, 0x01, 0x12, 0x04, 0x94, 0x01, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05,
    0x02, 0x00, 0x12, 0x04, 0x95, 0x01, 0x04, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x95, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05,
    0x12, 0x04, 0x95, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12,
    0x04, 0x95, 0x01, 0x14, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x04,
    0x95, 0x01, 0x20, 0x21, 0x0a, 0xa6, 0x01, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x06, 0x9d, 0x01, 0x00,
    0xa0, 0x01, 0x01, 0x1a, 0x97, 0x01, 0x0a, 0x20, 0x41, 0x6e, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x6f, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x20, 0x61,
    0x20, 0x53, 0x65, 0x74, 0x2c, 0x20, 0x65, 0x69, 0x74, 0x68, 0x65, 0x72, 0x20, 0x6f, 0x6e, 0x20,
    0x69, 0x74, 0x73, 0x20, 0x6f, 0x77, 0x6e, 0x20, 0x6f, 0x72, 0x20, 0x69, 0x6e, 0x73, 0x69, 0x64,
    0x65, 0x20, 0x61, 0x20, 0x4d, 0x61, 0x70, 0x2e, 0x0a, 0x20, 0x53, 0x65, 0x74, 0x20, 0x6d, 0x65,
    0x6d, 0x62, 0x65, 0x72, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x6f, 0x70, 0x61, 0x71, 0x75, 0x65,
    0x20, 0x62, 0x69, 0x6e, 0x61, 0x72, 0x79, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x2c, 0x20,
    0x79, 0x6f, 0x75, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x61, 0x64, 0x64,
    0x20, 0x6f, 0x72, 0x20, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x6d,
    0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x61, 0x20, 0x53, 0x65, 0x74, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x06, 0x01, 0x12, 0x04, 0x9d, 0x01, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06,
    0x02, 0x00, 0x12, 0x04, 0x9e, 0x01, 0x04, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x9e, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05,
    0x12, 0x04, 0x9e, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12,
    0x04, 0x9e, 0x01, 0x13, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x04,
    0x9e, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x04, 0x9f, 0x01,
    0x04, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x04, 0x9f, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x04, 0x9f, 0x01, 0x0d, 0x12,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9f, 0x01, 0x13, 0x1a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x04, 0x9f, 0x01, 0x1d, 0x1e, 0x0a, 0x69,
    0x0a, 0x02, 0x04, 0x07, 0x12, 0x06, 0xa6, 0x01, 0x00, 0xa8, 0x01, 0x01, 0x1a, 0x5b, 0x0a, 0x20,
    0x41, 0x6e, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x6f, 0x20,
    0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x20, 0x61, 0x20, 0x48, 0x79, 0x70, 0x65, 0x72, 0x6c, 0x6f,
    0x67, 0x6c, 0x6f, 0x67, 0x20, 0x53, 0x65, 0x74, 0x2c, 0x20, 0x61, 0x20, 0x74, 0x6f, 0x70, 0x2d,
    0x6c, 0x65, 0x76, 0x65, 0x6c, 0x20, 0x44, 0x54, 0x2e, 0x0a, 0x20, 0x59, 0x6f, 0x75, 0x20, 0x63,
    0x61, 0x6e, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x61, 0x64, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x61,
    0x20, 0x48, 0x6c, 0x6c, 0x53, 0x65, 0x74, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x07, 0x01,
    0x12, 0x04, 0xa6, 0x01, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x04,
    0xa7, 0x01, 0x04, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa7,
    0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa7, 0x01,
    0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa7, 0x01, 0x13,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa7, 0x01, 0x1d, 0x1e,
    0x0a, 0xce, 0x01, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x06, 0xaf, 0x01, 0x00, 0xc5, 0x01, 0x01, 0x1a,
    0xbf, 0x01, 0x0a, 0x20, 0x41, 0x6e, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x20, 0x74,
    0x6f, 0x20, 0x61, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x64,
    0x20, 0x69, 0x6e, 0x20, 0x61, 0x20, 0x4d, 0x61, 0x70, 0x20, 0x2d, 0x2d, 0x20, 0x74, 0x68, 0x65,
    0x0a, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x6e,
    0x20, 0x55, 0x50, 0x44, 0x41, 0x54, 0x45, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x69, 0x73, 0x0a, 0x20,
    0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x64, 0x65, 0x70, 0x65, 0x6e, 0x64, 0x73, 0x20,
    0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x77, 0x68, 0x69, 0x63,
    0x68, 0x20, 0x69, 0x74, 0x20, 0x69, 0x73, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x2e,
    0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x04, 0xaf, 0x01, 0x08, 0x11, 0x0a, 0x83,
    0x01, 0x0a, 0x04, 0x04, 0x08, 0x04, 0x00, 0x12, 0x06, 0xb4, 0x01, 0x04, 0xb7, 0x01, 0x05, 0x1a,
    0x73, 0x0a, 0x20, 0x46, 0x6c, 0x61, 0x67, 0x73, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x65, 0x78,
    0x69, 0x73, 0x74, 0x20, 0x69, 0x6e, 0x73, 0x69, 0x64, 0x65, 0x20, 0x4d, 0x61, 0x70, 0x73, 0x20,
    0x61, 0x6e, 0x64, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x62, 0x65, 0x20,
    0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x20, 0x6f, 0x72, 0x0a, 0x20, 0x64, 0x69, 0x73, 0x61,
    0x62, 0x6c, 0x65, 0x64, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x72, 0x65, 0x20,
    0x61, 0x72, 0x65, 0x20, 0x6e, 0x6f, 0x20, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73,
    0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x04, 0x00, 0x01, 0x12, 0x04, 0xb4,
    0x01, 0x09, 0x0f, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x08, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0xb5,
    0x01, 0x08, 0x14, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xb5, 0x01, 0x08, 0x0e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12,
    0x04, 0xb5, 0x01, 0x12, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x08, 0x04, 0x00, 0x02, 0x01, 0x12,
    0x04, 0xb6, 0x01, 0x08, 0x14, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xb6, 0x01, 0x08, 0x0f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x01,
    0x02, 0x12, 0x04, 0xb6, 0x01, 0x12, 0x13, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12,
    0x04, 0xb9, 0x01, 0x04, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xb9, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x06, 0x12, 0x04, 0xb9,
    0x01, 0x0d, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb9, 0x01,
    0x17, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb9, 0x01, 0x25,
    0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x04, 0xbb, 0x01, 0x04, 0x27, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x04, 0xbb, 0x01, 0x04, 0x0c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x06, 0x12, 0x04, 0xbb, 0x01, 0x0d, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x04, 0xbb, 0x01, 0x17, 0x21, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x04, 0xbb, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x08, 0x02, 0x02, 0x12, 0x04, 0xbc, 0x01, 0x04, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x02, 0x04, 0x12, 0x04, 0xbc, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02,
    0x06, 0x12, 0x04, 0xbc, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x01,
    0x12, 0x04, 0xbc, 0x01, 0x17, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x03, 0x12,
    0x04, 0xbc, 0x01, 0x25, 0x26, 0x0a, 0x83, 0x01, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x03, 0x12, 0x04,
    0xc2, 0x01, 0x04, 0x27, 0x1a, 0x75, 0x0a, 0x20, 0x54, 0x68, 0x65, 0x72, 0x65, 0x20, 0x69, 0x73,
    0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x6e, 0x20, 0x61, 0x20, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65,
    0x72, 0x2c, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x69, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x73,
    0x65, 0x74, 0x20, 0x69, 0x74, 0x73, 0x0a, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x2c, 0x20, 0x74,
    0x68, 0x65, 0x72, 0x65, 0x66, 0x6f, 0x72, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x22, 0x6f, 0x70,
    0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x6e, 0x65, 0x77, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x03, 0x04, 0x12, 0x04, 0xc2, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x03, 0x05, 0x12, 0x04, 0xc2, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x03, 0x01, 0x12, 0x04, 0xc2, 0x01, 0x17, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03,
    0x03, 0x12, 0x04, 0xc2, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x04, 0x12,
    0x04, 0xc3, 0x01, 0x04, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x04, 0x12, 0x04,
    0xc3, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x06, 0x12, 0x04, 0xc3,
    0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x01, 0x12, 0x04, 0xc3, 0x01,
    0x17, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x03, 0x12, 0x04, 0xc3, 0x01, 0x25,
    0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x05, 0x12, 0x04, 0xc4, 0x01, 0x04, 0x27, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x04, 0x12, 0x04, 0xc4, 0x01, 0x04, 0x0c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x06, 0x12, 0x04, 0xc4, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x05, 0x01, 0x12, 0x04, 0xc4, 0x01, 0x17, 0x1d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x05, 0x03, 0x12, 0x04, 0xc4, 0x01, 0x25, 0x26, 0x0a, 0x65, 0x0a, 0x02, 0x04,
    0x09, 0x12, 0x06, 0xcb, 0x01, 0x00, 0xd3, 0x01, 0x01, 0x1a, 0x57, 0x0a, 0x20, 0x41, 0x6e, 0x20,
    0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x6f, 0x20, 0x75, 0x70, 0x64,
    0x61, 0x74, 0x65, 0x20, 0x61, 0x20, 0x4d, 0x61, 0x70, 0x2e, 0x20, 0x41, 0x6c, 0x6c, 0x20, 0x6f,
    0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x79, 0x20,
    0x74, 0x6f, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x76, 0x69, 0x64, 0x75, 0x61, 0x6c, 0x0a, 0x20, 0x66,
    0x69, 0x65, 0x6c, 0x64, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4d, 0x61, 0x70,
    0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x04, 0xcb, 0x01, 0x08, 0x0d, 0x0a,
    0x8b, 0x01, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x04, 0xd1, 0x01, 0x04, 0x23, 0x1a, 0x7d,
    0x0a, 0x20, 0x20, 0x52, 0x45, 0x4d, 0x4f, 0x56, 0x45, 0x20, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65,
    0x73, 0x20, 0x61, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4d, 0x61, 0x70,
    0x2e, 0x0a, 0x20, 0x55, 0x50, 0x44, 0x41, 0x54, 0x45, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x65,
    0x73, 0x20, 0x74, 0x79, 0x70, 0x65, 0x2d, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x63, 0x0a,
    0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x64,
    0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4d, 0x61, 0x70, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd1, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x00, 0x06, 0x12, 0x04, 0xd1, 0x01, 0x0d, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd1, 0x01, 0x17, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xd1, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x01, 0x12, 0x04, 0xd2, 0x01, 0x04, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xd2, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x06, 0x12,
    0x04, 0xd2, 0x01, 0x0d, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xd2, 0x01, 0x17, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd2,
    0x01, 0x21, 0x22, 0x0a, 0x75, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x06, 0xd9, 0x01, 0x00, 0xe2, 0x01,
    0x01, 0x1a, 0x67, 0x0a, 0x20, 0x41, 0x20, 0x22, 0x75, 0x6e, 0x69, 0x6f, 0x6e, 0x22, 0x20, 0x74,
    0x79, 0x70, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x20, 0x6f,
    0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x69,
    0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65, 0x64, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x0a, 0x20, 0x64, 0x65, 0x70, 0x65, 0x6e, 0x64, 0x73, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x64, 0x61, 0x74, 0x61, 0x74, 0x79, 0x70, 0x65, 0x20, 0x62, 0x65, 0x69, 0x6e, 0x67,
    0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0a,
    0x01, 0x12, 0x04, 0xd9, 0x01, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12,
    0x04, 0xda, 0x01, 0x04, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xda, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x06, 0x12, 0x04, 0xda,
    0x01, 0x0d, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x04, 0xda, 0x01,
    0x17, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x04, 0xda, 0x01, 0x24,
    0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x01, 0x12, 0x04, 0xdb, 0x01, 0x04, 0x26, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x04, 0x12, 0x04, 0xdb, 0x01, 0x04, 0x0c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x06, 0x12, 0x04, 0xdb, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x01, 0x01, 0x12, 0x04, 0xdb, 0x01, 0x17, 0x1d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x01, 0x03, 0x12, 0x04, 0xdb, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0a, 0x02, 0x02, 0x12, 0x04, 0xdc, 0x01, 0x04, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x02, 0x04, 0x12, 0x04, 0xdc, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02,
    0x06, 0x12, 0x04, 0xdc, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x01,
    0x12, 0x04, 0xdc, 0x01, 0x17, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x03, 0x12,
    0x04, 0xdc, 0x01, 0x24, 0x25, 0x0a, 0x5a, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x03, 0x12, 0x04, 0xe1,
    0x01, 0x04, 0x26, 0x1a, 0x4c, 0x20, 0x41, 0x64, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x20, 0x68, 0x79, 0x70, 0x65, 0x72, 0x6c, 0x6f,
    0x67, 0x6c, 0x6f, 0x67, 0x20, 0x28, 0x73, 0x65, 0x74, 0x29, 0x20, 0x69, 0x73, 0x20, 0x6a, 0x75,
    0x73, 0x74, 0x20, 0x6c, 0x69, 0x6b, 0x65, 0x20, 0x61, 0x64, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x73, 0x0a, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x20, 0x73, 0x65, 0x74, 0x2e,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x03, 0x04, 0x12, 0x04, 0xe1, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x03, 0x06, 0x12, 0x04, 0xe1, 0x01, 0x0d, 0x12, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x04, 0xe1, 0x01, 0x17, 0x1d, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x04, 0xe1, 0x01, 0x24, 0x25, 0x0a, 0x85, 0x02,
    0x0a, 0x02, 0x04, 0x0b, 0x12, 0x06, 0xea, 0x01, 0x00, 0xff, 0x01, 0x01, 0x1a, 0xf6, 0x01, 0x0a,
    0x20, 0x54, 0x68, 0x65, 0x20, 0x65, 0x71, 0x75, 0x69, 0x76, 0x61, 0x6c, 0x65, 0x6e, 0x74, 0x20,
    0x6f, 0x66, 0x20, 0x4b, 0x56, 0x27, 0x73, 0x20, 0x22, 0x52, 0x70, 0x62, 0x50, 0x75, 0x74, 0x52,
    0x65, 0x71, 0x22, 0x2c, 0x20, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x73, 0x20, 0x69, 0x6e, 0x20,
    0x61, 0x6e, 0x20, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x20, 0x6f, 0x72, 0x0a, 0x20, 0x22, 0x44, 0x74, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x52,
    0x65, 0x73, 0x70, 0x22, 0x20, 0x69, 0x66, 0x20, 0x60, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x5f,
    0x62, 0x6f, 0x64, 0x79, 0x60, 0x20, 0x69, 0x73, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69,
    0x65, 0x64, 0x2c, 0x20, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x69,
    0x73, 0x0a, 0x20, 0x61, 0x73, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x72,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2d, 0x74, 0x69, 0x6d, 0x65, 0x20, 0x6f, 0x70, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x65, 0x64, 0x20,
    0x74, 0x6f, 0x0a, 0x20, 0x6f, 0x6e, 0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x61, 0x72,
    0x65, 0x20, 0x72, 0x65, 0x6c, 0x65, 0x76, 0x61, 0x6e, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x74,
    0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x64, 0x20, 0x64, 0x61, 0x74, 0x61, 0x2d, 0x74, 0x79,
    0x70, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x04, 0xea, 0x01,
    0x08, 0x13, 0x0a, 0x1e, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x04, 0xec, 0x01, 0x04, 0x1e,
    0x1a, 0x10, 0x20, 0x54, 0x68, 0x65, 0x20, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65,
    0x72, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x04, 0xec, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x05, 0x12, 0x04, 0xec, 0x01, 0x0d, 0x12,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x04, 0xec, 0x01, 0x13, 0x19, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x04, 0xec, 0x01, 0x1c, 0x1d, 0x0a, 0x43,
    0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01, 0x12, 0x04, 0xed, 0x01, 0x04, 0x1e, 0x22, 0x35, 0x20, 0x6d,
    0x69, 0x73, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x72, 0x65, 0x73, 0x75, 0x6c,
    0x74, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2d, 0x61, 0x73, 0x73,
    0x69, 0x67, 0x6e, 0x65, 0x64, 0x20, 0x6b, 0x65, 0x79, 0x2c, 0x20, 0x6c, 0x69, 0x6b, 0x65, 0x20,
    0x4b, 0x56, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x04, 0x12, 0x04, 0xed, 0x01,
    0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x05, 0x12, 0x04, 0xed, 0x01, 0x0d,
    0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12, 0x04, 0xed, 0x01, 0x13, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x04, 0xed, 0x01, 0x1c, 0x1d, 0x0a,
    0x5d, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x02, 0x12, 0x04, 0xee, 0x01, 0x04, 0x1e, 0x22, 0x4f, 0x20,
    0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x74, 0x79, 0x70, 0x65, 0x2c, 0x20, 0x6e, 0x6f, 0x74,
    0x20, 0x64, 0x61, 0x74, 0x61, 0x2d, 0x74, 0x79, 0x70, 0x65, 0x20, 0x28, 0x62, 0x75, 0x74, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x64, 0x61, 0x74, 0x61, 0x2d, 0x74, 0x79, 0x70, 0x65, 0x20, 0x69, 0x73,
    0x20, 0x63, 0x6f, 0x6e, 0x73, 0x74, 0x72, 0x61, 0x69, 0x6e, 0x65, 0x64, 0x20, 0x70, 0x65, 0x72,
    0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x2d, 0x74, 0x79, 0x70, 0x65, 0x29, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x04, 0x12, 0x04, 0xee, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x02, 0x05, 0x12, 0x04, 0xee, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x02, 0x01, 0x12, 0x04, 0xee, 0x01, 0x13, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x02, 0x03, 0x12, 0x04, 0xee, 0x01, 0x1c, 0x1d, 0x0a, 0x25, 0x0a, 0x04, 0x04, 0x0b,
    0x02, 0x03, 0x12, 0x04, 0xf1, 0x01, 0x04, 0x1f, 0x1a, 0x17, 0x20, 0x4f, 0x70, 0x61, 0x71, 0x75,
    0x65, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x04, 0x12, 0x04, 0xf1, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x05, 0x12, 0x04, 0xf1, 0x01, 0x0d, 0x12, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x01, 0x12, 0x04, 0xf1, 0x01, 0x13, 0x1a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x03, 0x12, 0x04, 0xf1, 0x01, 0x1d, 0x1e, 0x0a, 0x1e, 0x0a,
    0x04, 0x04, 0x0b, 0x02, 0x04, 0x12, 0x04, 0xf4, 0x01, 0x04, 0x1a, 0x1a, 0x10, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x04, 0x04, 0x12, 0x04, 0xf4, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x04, 0x06, 0x12, 0x04, 0xf4, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x04, 0x01, 0x12, 0x04, 0xf4, 0x01, 0x13, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x04, 0x03, 0x12, 0x04, 0xf4, 0x01, 0x18, 0x19, 0x0a, 0x1f, 0x0a, 0x04, 0x04, 0x0b, 0x02,
    0x05, 0x12, 0x04, 0xf7, 0x01, 0x04, 0x29, 0x1a, 0x11, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x05, 0x04, 0x12, 0x04, 0xf7, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x05, 0x05, 0x12, 0x04, 0xf7, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05,
    0x01, 0x12, 0x04, 0xf7, 0x01, 0x14, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05, 0x03,
    0x12, 0x04, 0xf7, 0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x06, 0x12, 0x04,
    0xf8, 0x01, 0x04, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x06, 0x04, 0x12, 0x04, 0xf8,
    0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x06, 0x05, 0x12, 0x04, 0xf8, 0x01,
    0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x06, 0x01, 0x12, 0x04, 0xf8, 0x01, 0x14,
    0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x06, 0x03, 0x12, 0x04, 0xf8, 0x01, 0x27, 0x28,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x07, 0x12, 0x04, 0xf9, 0x01, 0x04, 0x29, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x07, 0x04, 0x12, 0x04, 0xf9, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x07, 0x05, 0x12, 0x04, 0xf9, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x07, 0x01, 0x12, 0x04, 0xf9, 0x01, 0x14, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x07, 0x03, 0x12, 0x04, 0xf9, 0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b,
    0x02, 0x08, 0x12, 0x04, 0xfa, 0x01, 0x04, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x08,
    0x04, 0x12, 0x04, 0xfa, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x08, 0x05,
    0x12, 0x04, 0xfa, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x08, 0x01, 0x12,
    0x04, 0xfa, 0x01, 0x14, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x08, 0x03, 0x12, 0x04,
    0xfa, 0x01, 0x27, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x08, 0x08, 0x12, 0x04, 0xfa,
    0x01, 0x29, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x08, 0x07, 0x12, 0x04, 0xfa, 0x01,
    0x32, 0x37, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x09, 0x12, 0x04, 0xfb, 0x01, 0x04, 0x29,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x09, 0x04, 0x12, 0x04, 0xfb, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x09, 0x05, 0x12, 0x04, 0xfb, 0x01, 0x0d, 0x13, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x09, 0x01, 0x12, 0x04, 0xfb, 0x01, 0x14, 0x1b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x09, 0x03, 0x12, 0x04, 0xfb, 0x01, 0x26, 0x28, 0x0a, 0x32, 0x0a, 0x04,
    0x04, 0x0b, 0x02, 0x0a, 0x12, 0x04, 0xfc, 0x01, 0x04, 0x29, 0x22, 0x24, 0x20, 0x45, 0x78, 0x70,
    0x65, 0x72, 0x69, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x6c, 0x2c, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x63,
    0x68, 0x61, 0x6e, 0x67, 0x65, 0x2f, 0x64, 0x69, 0x73, 0x61, 0x70, 0x70, 0x65, 0x61, 0x72, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0a, 0x04, 0x12, 0x04, 0xfc, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0a, 0x05, 0x12, 0x04, 0xfc, 0x01, 0x0d, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0a, 0x01, 0x12, 0x04, 0xfc, 0x01, 0x14, 0x21, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x0a, 0x03, 0x12, 0x04, 0xfc, 0x01, 0x26, 0x28, 0x0a, 0x32, 0x0a, 0x04,
    0x04, 0x0b, 0x02, 0x0b, 0x12, 0x04, 0xfd, 0x01, 0x04, 0x29, 0x22, 0x24, 0x20, 0x45, 0x78, 0x70,
    0x65, 0x72, 0x69, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x6c, 0x2c, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x63,
    0x68, 0x61, 0x6e, 0x67, 0x65, 0x2f, 0x64, 0x69, 0x73, 0x61, 0x70, 0x70, 0x65, 0x61, 0x72, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0b, 0x04, 0x12, 0x04, 0xfd, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0b, 0x05, 0x12, 0x04, 0xfd, 0x01, 0x0d, 0x13, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0b, 0x01, 0x12, 0x04, 0xfd, 0x01, 0x14, 0x19, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x0b, 0x03, 0x12, 0x04, 0xfd, 0x01, 0x26, 0x28, 0x0a, 0x4d, 0x0a, 0x04,
    0x04, 0x0b, 0x02, 0x0c, 0x12, 0x04, 0xfe, 0x01, 0x04, 0x38, 0x22, 0x3f, 0x20, 0x57, 0x68, 0x65,
    0x6e, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x5f, 0x62, 0x6f, 0x64, 0x79, 0x20, 0x69, 0x73,
    0x20, 0x74, 0x72, 0x75, 0x65, 0x2c, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x20, 0x62, 0x65, 0x20, 0x72, 0x65, 0x74,
    0x75, 0x72, 0x6e, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x6f, 0x3f, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x0c, 0x04, 0x12, 0x04, 0xfe, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x0c, 0x05, 0x12, 0x04, 0xfe, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x0c, 0x01, 0x12, 0x04, 0xfe, 0x01, 0x14, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0c,
    0x03, 0x12, 0x04, 0xfe, 0x01, 0x26, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0c, 0x08,
    0x12, 0x04, 0xfe, 0x01, 0x29, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0c, 0x07, 0x12,
    0x04, 0xfe, 0x01, 0x32, 0x36, 0x0a, 0xb0, 0x01, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x06, 0x87, 0x02,
    0x00, 0x91, 0x02, 0x01, 0x1a, 0xa1, 0x01, 0x0a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x65, 0x71, 0x75,
    0x69, 0x76, 0x61, 0x6c, 0x65, 0x6e, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x4b, 0x56, 0x27, 0x73, 0x20,
    0x22, 0x52, 0x70, 0x62, 0x50, 0x75, 0x74, 0x52, 0x65, 0x73, 0x70, 0x22, 0x2c, 0x20, 0x63, 0x6f,
    0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x73, 0x73, 0x69, 0x67,
    0x6e, 0x65, 0x64, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x69, 0x66, 0x0a, 0x20, 0x69, 0x74, 0x20, 0x77,
    0x61, 0x73, 0x20, 0x61, 0x73, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x0a, 0x20,
    0x69, 0x66, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x5f, 0x62, 0x6f, 0x64, 0x79, 0x20, 0x77,
    0x61, 0x73, 0x20, 0x73, 0x65, 0x74, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12,
    0x04, 0x87, 0x02, 0x08, 0x14, 0x0a, 0x32, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x04, 0x89,
    0x02, 0x04, 0x28, 0x1a, 0x24, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6b, 0x65, 0x79, 0x2c, 0x20, 0x69,
    0x66, 0x20, 0x61, 0x73, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x89, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00,
    0x05, 0x12, 0x04, 0x89, 0x02, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x89, 0x02, 0x16, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12,
    0x04, 0x89, 0x02, 0x26, 0x27, 0x0a, 0x4c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12, 0x04, 0x8c,
    0x02, 0x04, 0x28, 0x1a, 0x3e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6f, 0x70, 0x61, 0x71, 0x75, 0x65,
    0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x20,
    0x61, 0x6e, 0x64, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x72, 0x65,
    0x74, 0x75, 0x72, 0x6e, 0x5f, 0x62, 0x6f, 0x64, 0x79, 0x20, 0x77, 0x61, 0x73, 0x20, 0x73, 0x65,
    0x74, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x04, 0x12, 0x04, 0x8c, 0x02,
    0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x05, 0x12, 0x04, 0x8c, 0x02, 0x0d,
    0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8c, 0x02, 0x16, 0x1d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8c, 0x02, 0x26, 0x27, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x02, 0x12, 0x04, 0x8d, 0x02, 0x04, 0x28, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x02, 0x04, 0x12, 0x04, 0x8d, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x02, 0x05, 0x12, 0x04, 0x8d, 0x02, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x02, 0x01, 0x12, 0x04, 0x8d, 0x02, 0x16, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x02, 0x03, 0x12, 0x04, 0x8d, 0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02,
    0x03, 0x12, 0x04, 0x8e, 0x02, 0x04, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x04,
    0x12, 0x04, 0x8e, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x05, 0x12,
    0x04, 0x8e, 0x02, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x01, 0x12, 0x04,
    0x8e, 0x02, 0x16, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x03, 0x12, 0x04, 0x8e,
    0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x04, 0x12, 0x04, 0x8f, 0x02, 0x04,
    0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04, 0x04, 0x12, 0x04, 0x8f, 0x02, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04, 0x06, 0x12, 0x04, 0x8f, 0x02, 0x0d, 0x15, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04, 0x01, 0x12, 0x04, 0x8f, 0x02, 0x16, 0x1f, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04, 0x03, 0x12, 0x04, 0x8f, 0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0c, 0x02, 0x05, 0x12, 0x04, 0x90, 0x02, 0x04, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x05, 0x04, 0x12, 0x04, 0x90, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x05, 0x05, 0x12, 0x04, 0x90, 0x02, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x05, 0x01, 0x12, 0x04, 0x90, 0x02, 0x16, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x05,
    0x03, 0x12, 0x04, 0x90, 0x02, 0x26, 0x27,
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
