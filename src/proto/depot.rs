// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct TypeSignifier {
    // message fields
    field_type: ::std::option::Option<ServerMessageType>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TypeSignifier {}

impl TypeSignifier {
    pub fn new() -> TypeSignifier {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TypeSignifier {
        static mut instance: ::protobuf::lazy::Lazy<TypeSignifier> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TypeSignifier,
        };
        unsafe {
            instance.get(TypeSignifier::new)
        }
    }

    // required .depot.ServerMessageType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ServerMessageType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> ServerMessageType {
        self.field_type.unwrap_or(ServerMessageType::INIT)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<ServerMessageType> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<ServerMessageType> {
        &mut self.field_type
    }
}

impl ::protobuf::Message for TypeSignifier {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
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
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_enum(1, v.value())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TypeSignifier {
    fn new() -> TypeSignifier {
        TypeSignifier::new()
    }

    fn descriptor_static(_: ::std::option::Option<TypeSignifier>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ServerMessageType>>(
                    "type",
                    TypeSignifier::get_field_type_for_reflect,
                    TypeSignifier::mut_field_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TypeSignifier>(
                    "TypeSignifier",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TypeSignifier {
    fn clear(&mut self) {
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TypeSignifier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TypeSignifier {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ServerInit {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    ip: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ServerInit {}

impl ServerInit {
    pub fn new() -> ServerInit {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ServerInit {
        static mut instance: ::protobuf::lazy::Lazy<ServerInit> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ServerInit,
        };
        unsafe {
            instance.get(ServerInit::new)
        }
    }

    // required string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // required string ip = 2;

    pub fn clear_ip(&mut self) {
        self.ip.clear();
    }

    pub fn has_ip(&self) -> bool {
        self.ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ip(&mut self, v: ::std::string::String) {
        self.ip = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ip(&mut self) -> &mut ::std::string::String {
        if self.ip.is_none() {
            self.ip.set_default();
        }
        self.ip.as_mut().unwrap()
    }

    // Take field
    pub fn take_ip(&mut self) -> ::std::string::String {
        self.ip.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_ip(&self) -> &str {
        match self.ip.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_ip_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.ip
    }

    fn mut_ip_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.ip
    }
}

impl ::protobuf::Message for ServerInit {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        }
        if self.ip.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.ip)?;
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.ip.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.ip.as_ref() {
            os.write_string(2, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ServerInit {
    fn new() -> ServerInit {
        ServerInit::new()
    }

    fn descriptor_static(_: ::std::option::Option<ServerInit>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    ServerInit::get_name_for_reflect,
                    ServerInit::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ip",
                    ServerInit::get_ip_for_reflect,
                    ServerInit::mut_ip_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ServerInit>(
                    "ServerInit",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ServerInit {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_ip();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ServerInit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ServerInit {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ServerConfig {
    // message fields
    uuid: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    body: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ServerConfig {}

impl ServerConfig {
    pub fn new() -> ServerConfig {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ServerConfig {
        static mut instance: ::protobuf::lazy::Lazy<ServerConfig> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ServerConfig,
        };
        unsafe {
            instance.get(ServerConfig::new)
        }
    }

    // required string uuid = 1;

    pub fn clear_uuid(&mut self) {
        self.uuid.clear();
    }

    pub fn has_uuid(&self) -> bool {
        self.uuid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uuid(&mut self, v: ::std::string::String) {
        self.uuid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uuid(&mut self) -> &mut ::std::string::String {
        if self.uuid.is_none() {
            self.uuid.set_default();
        }
        self.uuid.as_mut().unwrap()
    }

    // Take field
    pub fn take_uuid(&mut self) -> ::std::string::String {
        self.uuid.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_uuid(&self) -> &str {
        match self.uuid.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_uuid_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.uuid
    }

    fn mut_uuid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.uuid
    }

    // required string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // required string body = 3;

    pub fn clear_body(&mut self) {
        self.body.clear();
    }

    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }

    // Param is passed by value, moved
    pub fn set_body(&mut self, v: ::std::string::String) {
        self.body = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_body(&mut self) -> &mut ::std::string::String {
        if self.body.is_none() {
            self.body.set_default();
        }
        self.body.as_mut().unwrap()
    }

    // Take field
    pub fn take_body(&mut self) -> ::std::string::String {
        self.body.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_body(&self) -> &str {
        match self.body.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_body_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.body
    }

    fn mut_body_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.body
    }
}

impl ::protobuf::Message for ServerConfig {
    fn is_initialized(&self) -> bool {
        if self.uuid.is_none() {
            return false;
        }
        if self.name.is_none() {
            return false;
        }
        if self.body.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.uuid)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.body)?;
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
        if let Some(ref v) = self.uuid.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.body.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.uuid.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.body.as_ref() {
            os.write_string(3, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ServerConfig {
    fn new() -> ServerConfig {
        ServerConfig::new()
    }

    fn descriptor_static(_: ::std::option::Option<ServerConfig>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "uuid",
                    ServerConfig::get_uuid_for_reflect,
                    ServerConfig::mut_uuid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    ServerConfig::get_name_for_reflect,
                    ServerConfig::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "body",
                    ServerConfig::get_body_for_reflect,
                    ServerConfig::mut_body_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ServerConfig>(
                    "ServerConfig",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ServerConfig {
    fn clear(&mut self) {
        self.clear_uuid();
        self.clear_name();
        self.clear_body();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ServerConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ServerConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ServerReport {
    // message fields
    server_uuid: ::protobuf::SingularField<::std::string::String>,
    ep_num: ::std::option::Option<u32>,
    has_config: ::std::option::Option<bool>,
    done: ::std::option::Option<bool>,
    config_uuid: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ServerReport {}

impl ServerReport {
    pub fn new() -> ServerReport {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ServerReport {
        static mut instance: ::protobuf::lazy::Lazy<ServerReport> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ServerReport,
        };
        unsafe {
            instance.get(ServerReport::new)
        }
    }

    // required string server_uuid = 1;

    pub fn clear_server_uuid(&mut self) {
        self.server_uuid.clear();
    }

    pub fn has_server_uuid(&self) -> bool {
        self.server_uuid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_uuid(&mut self, v: ::std::string::String) {
        self.server_uuid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_server_uuid(&mut self) -> &mut ::std::string::String {
        if self.server_uuid.is_none() {
            self.server_uuid.set_default();
        }
        self.server_uuid.as_mut().unwrap()
    }

    // Take field
    pub fn take_server_uuid(&mut self) -> ::std::string::String {
        self.server_uuid.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_server_uuid(&self) -> &str {
        match self.server_uuid.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_server_uuid_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.server_uuid
    }

    fn mut_server_uuid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.server_uuid
    }

    // required uint32 ep_num = 3;

    pub fn clear_ep_num(&mut self) {
        self.ep_num = ::std::option::Option::None;
    }

    pub fn has_ep_num(&self) -> bool {
        self.ep_num.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ep_num(&mut self, v: u32) {
        self.ep_num = ::std::option::Option::Some(v);
    }

    pub fn get_ep_num(&self) -> u32 {
        self.ep_num.unwrap_or(0)
    }

    fn get_ep_num_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ep_num
    }

    fn mut_ep_num_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ep_num
    }

    // required bool has_config = 4;

    pub fn clear_has_config(&mut self) {
        self.has_config = ::std::option::Option::None;
    }

    pub fn has_has_config(&self) -> bool {
        self.has_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_has_config(&mut self, v: bool) {
        self.has_config = ::std::option::Option::Some(v);
    }

    pub fn get_has_config(&self) -> bool {
        self.has_config.unwrap_or(false)
    }

    fn get_has_config_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.has_config
    }

    fn mut_has_config_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.has_config
    }

    // optional bool done = 5;

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

    // optional string config_uuid = 6;

    pub fn clear_config_uuid(&mut self) {
        self.config_uuid.clear();
    }

    pub fn has_config_uuid(&self) -> bool {
        self.config_uuid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_config_uuid(&mut self, v: ::std::string::String) {
        self.config_uuid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_config_uuid(&mut self) -> &mut ::std::string::String {
        if self.config_uuid.is_none() {
            self.config_uuid.set_default();
        }
        self.config_uuid.as_mut().unwrap()
    }

    // Take field
    pub fn take_config_uuid(&mut self) -> ::std::string::String {
        self.config_uuid.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_config_uuid(&self) -> &str {
        match self.config_uuid.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_config_uuid_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.config_uuid
    }

    fn mut_config_uuid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.config_uuid
    }
}

impl ::protobuf::Message for ServerReport {
    fn is_initialized(&self) -> bool {
        if self.server_uuid.is_none() {
            return false;
        }
        if self.ep_num.is_none() {
            return false;
        }
        if self.has_config.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.server_uuid)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ep_num = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.has_config = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.done = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.config_uuid)?;
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
        if let Some(ref v) = self.server_uuid.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.ep_num {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.has_config {
            my_size += 2;
        }
        if let Some(v) = self.done {
            my_size += 2;
        }
        if let Some(ref v) = self.config_uuid.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.server_uuid.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.ep_num {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.has_config {
            os.write_bool(4, v)?;
        }
        if let Some(v) = self.done {
            os.write_bool(5, v)?;
        }
        if let Some(ref v) = self.config_uuid.as_ref() {
            os.write_string(6, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ServerReport {
    fn new() -> ServerReport {
        ServerReport::new()
    }

    fn descriptor_static(_: ::std::option::Option<ServerReport>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "server_uuid",
                    ServerReport::get_server_uuid_for_reflect,
                    ServerReport::mut_server_uuid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ep_num",
                    ServerReport::get_ep_num_for_reflect,
                    ServerReport::mut_ep_num_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "has_config",
                    ServerReport::get_has_config_for_reflect,
                    ServerReport::mut_has_config_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "done",
                    ServerReport::get_done_for_reflect,
                    ServerReport::mut_done_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "config_uuid",
                    ServerReport::get_config_uuid_for_reflect,
                    ServerReport::mut_config_uuid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ServerReport>(
                    "ServerReport",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ServerReport {
    fn clear(&mut self) {
        self.clear_server_uuid();
        self.clear_ep_num();
        self.clear_has_config();
        self.clear_done();
        self.clear_config_uuid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ServerReport {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ServerReport {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ServerMessageType {
    INIT = 0,
    CONFIG = 1,
    REPORT = 2,
}

impl ::protobuf::ProtobufEnum for ServerMessageType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ServerMessageType> {
        match value {
            0 => ::std::option::Option::Some(ServerMessageType::INIT),
            1 => ::std::option::Option::Some(ServerMessageType::CONFIG),
            2 => ::std::option::Option::Some(ServerMessageType::REPORT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ServerMessageType] = &[
            ServerMessageType::INIT,
            ServerMessageType::CONFIG,
            ServerMessageType::REPORT,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ServerMessageType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ServerMessageType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ServerMessageType {
}

impl ::protobuf::reflect::ProtobufValue for ServerMessageType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0bdepot.proto\x12\x05depot\"7\n\rTypeSignifier\x12&\n\x04type\x18\
    \x01\x20\x02(\x0e2\x18.depot.ServerMessageType\"&\n\nServerInit\x12\x0c\
    \n\x04name\x18\x01\x20\x02(\t\x12\n\n\x02ip\x18\x02\x20\x02(\t\"8\n\x0cS\
    erverConfig\x12\x0c\n\x04uuid\x18\x01\x20\x02(\t\x12\x0c\n\x04name\x18\
    \x02\x20\x02(\t\x12\x0c\n\x04body\x18\x03\x20\x02(\t\"j\n\x0cServerRepor\
    t\x12\x13\n\x0bserver_uuid\x18\x01\x20\x02(\t\x12\x0e\n\x06ep_num\x18\
    \x03\x20\x02(\r\x12\x12\n\nhas_config\x18\x04\x20\x02(\x08\x12\x0c\n\x04\
    done\x18\x05\x20\x01(\x08\x12\x13\n\x0bconfig_uuid\x18\x06\x20\x01(\t*5\
    \n\x11ServerMessageType\x12\x08\n\x04INIT\x10\0\x12\n\n\x06CONFIG\x10\
    \x01\x12\n\n\x06REPORT\x10\x02J\xc9\n\n\x06\x12\x04\0\0%\x01\n\x08\n\x01\
    \x02\x12\x03\x02\x08\r\n?\n\x02\x05\0\x12\x04\x05\0\t\x01\x1a3\x20Enum\
    \x20representing\x20the\x20type\x20of\x20message\x20being\x20sent.\n\n\n\
    \n\x03\x05\0\x01\x12\x03\x05\x05\x16\n\x0b\n\x04\x05\0\x02\0\x12\x03\x06\
    \x04\r\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x06\x04\x08\n\x0c\n\x05\x05\0\
    \x02\0\x02\x12\x03\x06\x0b\x0c\n\x0b\n\x04\x05\0\x02\x01\x12\x03\x07\x04\
    \x0f\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\x07\x04\n\n\x0c\n\x05\x05\0\
    \x02\x01\x02\x12\x03\x07\r\x0e\n\x0b\n\x04\x05\0\x02\x02\x12\x03\x08\x04\
    \x0f\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x03\x08\x04\n\n\x0c\n\x05\x05\0\
    \x02\x02\x02\x12\x03\x08\r\x0e\nD\n\x02\x04\0\x12\x04\x0c\0\x0e\x01\x1a8\
    \x20Message\x20representing\x20the\x20type\x20of\x20the\x20top-level\x20\
    message\n\n\n\n\x03\x04\0\x01\x12\x03\x0c\x08\x15\n\x0b\n\x04\x04\0\x02\
    \0\x12\x03\r\x04(\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\r\x04\x0c\n\x0c\n\
    \x05\x04\0\x02\0\x06\x12\x03\r\r\x1e\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\
    \r\x1f#\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\r&'\nT\n\x02\x04\x01\x12\x04\
    \x11\0\x14\x01\x1aH\x20Message\x20sent\x20by\x20worker\x20sever\x20to\
    \x20register\x20with\x20depot\x20config\x20dispatcher.\n\n\n\n\x03\x04\
    \x01\x01\x12\x03\x11\x08\x12\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x12\x04\
    \x1d\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\x12\x04\x0c\n\x0c\n\x05\x04\
    \x01\x02\0\x05\x12\x03\x12\r\x13\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\
    \x12\x14\x18\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x12\x1b\x1c\n\x0b\n\
    \x04\x04\x01\x02\x01\x12\x03\x13\x04\x1b\n\x0c\n\x05\x04\x01\x02\x01\x04\
    \x12\x03\x13\x04\x0c\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x13\r\x13\n\
    \x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x13\x14\x16\n\x0c\n\x05\x04\x01\
    \x02\x01\x03\x12\x03\x13\x19\x1a\n[\n\x02\x04\x02\x12\x04\x17\0\x1b\x01\
    \x1aO\x20Message\x20sent\x20by\x20depot\x20config\x20dispatcher\x20to\
    \x20server\x20with\x20new\x20config\x20to\x20process.\n\n\n\n\x03\x04\
    \x02\x01\x12\x03\x17\x08\x14\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x18\x04\
    \x1d\n\x0c\n\x05\x04\x02\x02\0\x04\x12\x03\x18\x04\x0c\n\x0c\n\x05\x04\
    \x02\x02\0\x05\x12\x03\x18\r\x13\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\
    \x18\x14\x18\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x18\x1b\x1c\n\x0b\n\
    \x04\x04\x02\x02\x01\x12\x03\x19\x04\x1d\n\x0c\n\x05\x04\x02\x02\x01\x04\
    \x12\x03\x19\x04\x0c\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\x19\r\x13\n\
    \x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x19\x14\x18\n\x0c\n\x05\x04\x02\
    \x02\x01\x03\x12\x03\x19\x1b\x1c\n\x0b\n\x04\x04\x02\x02\x02\x12\x03\x1a\
    \x04\x1d\n\x0c\n\x05\x04\x02\x02\x02\x04\x12\x03\x1a\x04\x0c\n\x0c\n\x05\
    \x04\x02\x02\x02\x05\x12\x03\x1a\r\x13\n\x0c\n\x05\x04\x02\x02\x02\x01\
    \x12\x03\x1a\x14\x18\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03\x1a\x1b\x1c\
    \nM\n\x02\x04\x03\x12\x04\x1e\0%\x01\x1aA\x20Message\x20sent\x20by\x20wo\
    rker\x20server\x20to\x20depot\x20dispatcher\x20as\x20heartbeat.\n\n\n\n\
    \x03\x04\x03\x01\x12\x03\x1e\x08\x14\n\x0b\n\x04\x04\x03\x02\0\x12\x03\
    \x1f\x04$\n\x0c\n\x05\x04\x03\x02\0\x04\x12\x03\x1f\x04\x0c\n\x0c\n\x05\
    \x04\x03\x02\0\x05\x12\x03\x1f\r\x13\n\x0c\n\x05\x04\x03\x02\0\x01\x12\
    \x03\x1f\x14\x1f\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03\x1f\"#\n\x0b\n\
    \x04\x04\x03\x02\x01\x12\x03\x20\x04\x1f\n\x0c\n\x05\x04\x03\x02\x01\x04\
    \x12\x03\x20\x04\x0c\n\x0c\n\x05\x04\x03\x02\x01\x05\x12\x03\x20\r\x13\n\
    \x0c\n\x05\x04\x03\x02\x01\x01\x12\x03\x20\x14\x1a\n\x0c\n\x05\x04\x03\
    \x02\x01\x03\x12\x03\x20\x1d\x1e\n\x0b\n\x04\x04\x03\x02\x02\x12\x03\"\
    \x04!\n\x0c\n\x05\x04\x03\x02\x02\x04\x12\x03\"\x04\x0c\n\x0c\n\x05\x04\
    \x03\x02\x02\x05\x12\x03\"\r\x11\n\x0c\n\x05\x04\x03\x02\x02\x01\x12\x03\
    \"\x12\x1c\n\x0c\n\x05\x04\x03\x02\x02\x03\x12\x03\"\x1f\x20\n\x0b\n\x04\
    \x04\x03\x02\x03\x12\x03#\x04\x1b\n\x0c\n\x05\x04\x03\x02\x03\x04\x12\
    \x03#\x04\x0c\n\x0c\n\x05\x04\x03\x02\x03\x05\x12\x03#\r\x11\n\x0c\n\x05\
    \x04\x03\x02\x03\x01\x12\x03#\x12\x16\n\x0c\n\x05\x04\x03\x02\x03\x03\
    \x12\x03#\x19\x1a\n\x0b\n\x04\x04\x03\x02\x04\x12\x03$\x04$\n\x0c\n\x05\
    \x04\x03\x02\x04\x04\x12\x03$\x04\x0c\n\x0c\n\x05\x04\x03\x02\x04\x05\
    \x12\x03$\r\x13\n\x0c\n\x05\x04\x03\x02\x04\x01\x12\x03$\x14\x1f\n\x0c\n\
    \x05\x04\x03\x02\x04\x03\x12\x03$\"#\
";

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
