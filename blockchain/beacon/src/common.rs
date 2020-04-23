// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

// This file is generated by rust-protobuf 2.14.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `common.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_14_0;

#[derive(PartialEq,Clone,Default)]
pub struct Empty {
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Empty {
    fn default() -> &'a Empty {
        <Empty as ::protobuf::Message>::default_instance()
    }
}

impl Empty {
    pub fn new() -> Empty {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for Empty {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Empty {
        Empty::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<Empty>(
                    "Empty",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Empty {
        static mut instance: ::protobuf::lazy::Lazy<Empty> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(Empty::new)
        }
    }
}

impl ::protobuf::Clear for Empty {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Empty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Empty {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Identity {
    // message fields
    pub address: ::std::string::String,
    pub key: ::std::vec::Vec<u8>,
    pub tls: bool,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Identity {
    fn default() -> &'a Identity {
        <Identity as ::protobuf::Message>::default_instance()
    }
}

impl Identity {
    pub fn new() -> Identity {
        ::std::default::Default::default()
    }

    // string address = 1;


    pub fn get_address(&self) -> &str {
        &self.address
    }
    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::string::String) {
        self.address = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address(&mut self) -> &mut ::std::string::String {
        &mut self.address
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.address, ::std::string::String::new())
    }

    // bytes key = 2;


    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }

    // bool tls = 3;


    pub fn get_tls(&self) -> bool {
        self.tls
    }
    pub fn clear_tls(&mut self) {
        self.tls = false;
    }

    // Param is passed by value, moved
    pub fn set_tls(&mut self, v: bool) {
        self.tls = v;
    }
}

impl ::protobuf::Message for Identity {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.address)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.tls = tmp;
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
        if !self.address.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.address);
        }
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.key);
        }
        if self.tls != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.address.is_empty() {
            os.write_string(1, &self.address)?;
        }
        if !self.key.is_empty() {
            os.write_bytes(2, &self.key)?;
        }
        if self.tls != false {
            os.write_bool(3, self.tls)?;
        }
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Identity {
        Identity::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "address",
                    |m: &Identity| { &m.address },
                    |m: &mut Identity| { &mut m.address },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    |m: &Identity| { &m.key },
                    |m: &mut Identity| { &mut m.key },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "tls",
                    |m: &Identity| { &m.tls },
                    |m: &mut Identity| { &mut m.tls },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<Identity>(
                    "Identity",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Identity {
        static mut instance: ::protobuf::lazy::Lazy<Identity> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(Identity::new)
        }
    }
}

impl ::protobuf::Clear for Identity {
    fn clear(&mut self) {
        self.address.clear();
        self.key.clear();
        self.tls = false;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Identity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Identity {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GroupPacket {
    // message fields
    pub nodes: ::protobuf::RepeatedField<Identity>,
    pub threshold: u32,
    pub period: u32,
    pub genesis_time: u64,
    pub transition_time: u64,
    pub genesis_seed: ::std::vec::Vec<u8>,
    pub dist_key: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a GroupPacket {
    fn default() -> &'a GroupPacket {
        <GroupPacket as ::protobuf::Message>::default_instance()
    }
}

impl GroupPacket {
    pub fn new() -> GroupPacket {
        ::std::default::Default::default()
    }

    // repeated .drand.Identity nodes = 1;


    pub fn get_nodes(&self) -> &[Identity] {
        &self.nodes
    }
    pub fn clear_nodes(&mut self) {
        self.nodes.clear();
    }

    // Param is passed by value, moved
    pub fn set_nodes(&mut self, v: ::protobuf::RepeatedField<Identity>) {
        self.nodes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_nodes(&mut self) -> &mut ::protobuf::RepeatedField<Identity> {
        &mut self.nodes
    }

    // Take field
    pub fn take_nodes(&mut self) -> ::protobuf::RepeatedField<Identity> {
        ::std::mem::replace(&mut self.nodes, ::protobuf::RepeatedField::new())
    }

    // uint32 threshold = 2;


    pub fn get_threshold(&self) -> u32 {
        self.threshold
    }
    pub fn clear_threshold(&mut self) {
        self.threshold = 0;
    }

    // Param is passed by value, moved
    pub fn set_threshold(&mut self, v: u32) {
        self.threshold = v;
    }

    // uint32 period = 3;


    pub fn get_period(&self) -> u32 {
        self.period
    }
    pub fn clear_period(&mut self) {
        self.period = 0;
    }

    // Param is passed by value, moved
    pub fn set_period(&mut self, v: u32) {
        self.period = v;
    }

    // uint64 genesis_time = 4;


    pub fn get_genesis_time(&self) -> u64 {
        self.genesis_time
    }
    pub fn clear_genesis_time(&mut self) {
        self.genesis_time = 0;
    }

    // Param is passed by value, moved
    pub fn set_genesis_time(&mut self, v: u64) {
        self.genesis_time = v;
    }

    // uint64 transition_time = 5;


    pub fn get_transition_time(&self) -> u64 {
        self.transition_time
    }
    pub fn clear_transition_time(&mut self) {
        self.transition_time = 0;
    }

    // Param is passed by value, moved
    pub fn set_transition_time(&mut self, v: u64) {
        self.transition_time = v;
    }

    // bytes genesis_seed = 6;


    pub fn get_genesis_seed(&self) -> &[u8] {
        &self.genesis_seed
    }
    pub fn clear_genesis_seed(&mut self) {
        self.genesis_seed.clear();
    }

    // Param is passed by value, moved
    pub fn set_genesis_seed(&mut self, v: ::std::vec::Vec<u8>) {
        self.genesis_seed = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_genesis_seed(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.genesis_seed
    }

    // Take field
    pub fn take_genesis_seed(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.genesis_seed, ::std::vec::Vec::new())
    }

    // repeated bytes dist_key = 7;


    pub fn get_dist_key(&self) -> &[::std::vec::Vec<u8>] {
        &self.dist_key
    }
    pub fn clear_dist_key(&mut self) {
        self.dist_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_dist_key(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.dist_key = v;
    }

    // Mutable pointer to the field.
    pub fn mut_dist_key(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.dist_key
    }

    // Take field
    pub fn take_dist_key(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.dist_key, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for GroupPacket {
    fn is_initialized(&self) -> bool {
        for v in &self.nodes {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.nodes)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.threshold = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.period = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.genesis_time = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.transition_time = tmp;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.genesis_seed)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.dist_key)?;
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
        for value in &self.nodes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.threshold != 0 {
            my_size += ::protobuf::rt::value_size(2, self.threshold, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.period != 0 {
            my_size += ::protobuf::rt::value_size(3, self.period, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.genesis_time != 0 {
            my_size += ::protobuf::rt::value_size(4, self.genesis_time, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.transition_time != 0 {
            my_size += ::protobuf::rt::value_size(5, self.transition_time, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.genesis_seed.is_empty() {
            my_size += ::protobuf::rt::bytes_size(6, &self.genesis_seed);
        }
        for value in &self.dist_key {
            my_size += ::protobuf::rt::bytes_size(7, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.nodes {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.threshold != 0 {
            os.write_uint32(2, self.threshold)?;
        }
        if self.period != 0 {
            os.write_uint32(3, self.period)?;
        }
        if self.genesis_time != 0 {
            os.write_uint64(4, self.genesis_time)?;
        }
        if self.transition_time != 0 {
            os.write_uint64(5, self.transition_time)?;
        }
        if !self.genesis_seed.is_empty() {
            os.write_bytes(6, &self.genesis_seed)?;
        }
        for v in &self.dist_key {
            os.write_bytes(7, &v)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> GroupPacket {
        GroupPacket::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Identity>>(
                    "nodes",
                    |m: &GroupPacket| { &m.nodes },
                    |m: &mut GroupPacket| { &mut m.nodes },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "threshold",
                    |m: &GroupPacket| { &m.threshold },
                    |m: &mut GroupPacket| { &mut m.threshold },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "period",
                    |m: &GroupPacket| { &m.period },
                    |m: &mut GroupPacket| { &mut m.period },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "genesis_time",
                    |m: &GroupPacket| { &m.genesis_time },
                    |m: &mut GroupPacket| { &mut m.genesis_time },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "transition_time",
                    |m: &GroupPacket| { &m.transition_time },
                    |m: &mut GroupPacket| { &mut m.transition_time },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "genesis_seed",
                    |m: &GroupPacket| { &m.genesis_seed },
                    |m: &mut GroupPacket| { &mut m.genesis_seed },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "dist_key",
                    |m: &GroupPacket| { &m.dist_key },
                    |m: &mut GroupPacket| { &mut m.dist_key },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<GroupPacket>(
                    "GroupPacket",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static GroupPacket {
        static mut instance: ::protobuf::lazy::Lazy<GroupPacket> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(GroupPacket::new)
        }
    }
}

impl ::protobuf::Clear for GroupPacket {
    fn clear(&mut self) {
        self.nodes.clear();
        self.threshold = 0;
        self.period = 0;
        self.genesis_time = 0;
        self.transition_time = 0;
        self.genesis_seed.clear();
        self.dist_key.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GroupPacket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GroupPacket {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GroupRequest {
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a GroupRequest {
    fn default() -> &'a GroupRequest {
        <GroupRequest as ::protobuf::Message>::default_instance()
    }
}

impl GroupRequest {
    pub fn new() -> GroupRequest {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for GroupRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> GroupRequest {
        GroupRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<GroupRequest>(
                    "GroupRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static GroupRequest {
        static mut instance: ::protobuf::lazy::Lazy<GroupRequest> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(GroupRequest::new)
        }
    }
}

impl ::protobuf::Clear for GroupRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GroupRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GroupRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0ccommon.proto\x12\x05drand\"\x07\n\x05Empty\"H\n\x08Identity\x12\
    \x18\n\x07address\x18\x01\x20\x01(\tR\x07address\x12\x10\n\x03key\x18\
    \x02\x20\x01(\x0cR\x03key\x12\x10\n\x03tls\x18\x03\x20\x01(\x08R\x03tls\
    \"\xf4\x01\n\x0bGroupPacket\x12%\n\x05nodes\x18\x01\x20\x03(\x0b2\x0f.dr\
    and.IdentityR\x05nodes\x12\x1c\n\tthreshold\x18\x02\x20\x01(\rR\tthresho\
    ld\x12\x16\n\x06period\x18\x03\x20\x01(\rR\x06period\x12!\n\x0cgenesis_t\
    ime\x18\x04\x20\x01(\x04R\x0bgenesisTime\x12'\n\x0ftransition_time\x18\
    \x05\x20\x01(\x04R\x0etransitionTime\x12!\n\x0cgenesis_seed\x18\x06\x20\
    \x01(\x0cR\x0bgenesisSeed\x12\x19\n\x08dist_key\x18\x07\x20\x03(\x0cR\
    \x07distKey\"\x0e\n\x0cGroupRequestB\x07Z\x05drandb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy::INIT;

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
