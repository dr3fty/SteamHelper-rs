// This file is generated by rust-protobuf 2.20.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `steammessages_unified_base.steamclient.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_20_0;

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct NoResponse {
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a NoResponse {
    fn default() -> &'a NoResponse {
        <NoResponse as ::protobuf::Message>::default_instance()
    }
}

impl NoResponse {
    pub fn new() -> NoResponse {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for NoResponse {
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
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> NoResponse {
        NoResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let fields = ::std::vec::Vec::new();
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<NoResponse>(
                "NoResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static NoResponse {
        static instance: ::protobuf::rt::LazyV2<NoResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(NoResponse::new)
    }
}

impl ::protobuf::Clear for NoResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NoResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NoResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub enum EProtoExecutionSite {
    k_EProtoExecutionSiteUnknown = 0,
    k_EProtoExecutionSiteSteamClient = 2,
}

impl ::protobuf::ProtobufEnum for EProtoExecutionSite {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EProtoExecutionSite> {
        match value {
            0 => ::std::option::Option::Some(EProtoExecutionSite::k_EProtoExecutionSiteUnknown),
            2 => ::std::option::Option::Some(EProtoExecutionSite::k_EProtoExecutionSiteSteamClient),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EProtoExecutionSite] = &[
            EProtoExecutionSite::k_EProtoExecutionSiteUnknown,
            EProtoExecutionSite::k_EProtoExecutionSiteSteamClient,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<EProtoExecutionSite>("EProtoExecutionSite", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for EProtoExecutionSite {
}

impl ::std::default::Default for EProtoExecutionSite {
    fn default() -> Self {
        EProtoExecutionSite::k_EProtoExecutionSiteUnknown
    }
}

impl ::protobuf::reflect::ProtobufValue for EProtoExecutionSite {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

/// Extension fields
pub mod exts {

    pub const description: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::protobuf::types::ProtobufTypeString> = ::protobuf::ext::ExtFieldOptional { field_number: 50000, phantom: ::std::marker::PhantomData };

    pub const service_description: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::ServiceOptions, ::protobuf::types::ProtobufTypeString> = ::protobuf::ext::ExtFieldOptional { field_number: 50000, phantom: ::std::marker::PhantomData };

    pub const service_execution_site: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::ServiceOptions, ::protobuf::types::ProtobufTypeEnum<super::EProtoExecutionSite>> = ::protobuf::ext::ExtFieldOptional { field_number: 50008, phantom: ::std::marker::PhantomData };

    pub const method_description: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MethodOptions, ::protobuf::types::ProtobufTypeString> = ::protobuf::ext::ExtFieldOptional { field_number: 50000, phantom: ::std::marker::PhantomData };

    pub const enum_description: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::EnumOptions, ::protobuf::types::ProtobufTypeString> = ::protobuf::ext::ExtFieldOptional { field_number: 50000, phantom: ::std::marker::PhantomData };

    pub const enum_value_description: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::EnumValueOptions, ::protobuf::types::ProtobufTypeString> = ::protobuf::ext::ExtFieldOptional { field_number: 50000, phantom: ::std::marker::PhantomData };
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n,steammessages_unified_base.steamclient.proto\x1a\x20google/protobuf/d\
    escriptor.proto\"\x0c\n\nNoResponse*]\n\x13EProtoExecutionSite\x12\x20\n\
    \x1ck_EProtoExecutionSiteUnknown\x10\0\x12$\n\x20k_EProtoExecutionSiteSt\
    eamClient\x10\x02:A\n\x0bdescription\x18\xd0\x86\x03\x20\x01(\t\x12\x1d.\
    google.protobuf.FieldOptionsR\x0bdescription:R\n\x13service_description\
    \x18\xd0\x86\x03\x20\x01(\t\x12\x1f.google.protobuf.ServiceOptionsR\x12s\
    erviceDescription:\x8b\x01\n\x16service_execution_site\x18\xd8\x86\x03\
    \x20\x01(\x0e2\x14.EProtoExecutionSite\x12\x1f.google.protobuf.ServiceOp\
    tions:\x1ck_EProtoExecutionSiteUnknownR\x14serviceExecutionSite:O\n\x12m\
    ethod_description\x18\xd0\x86\x03\x20\x01(\t\x12\x1e.google.protobuf.Met\
    hodOptionsR\x11methodDescription:I\n\x10enum_description\x18\xd0\x86\x03\
    \x20\x01(\t\x12\x1c.google.protobuf.EnumOptionsR\x0fenumDescription:Y\n\
    \x16enum_value_description\x18\xd0\x86\x03\x20\x01(\t\x12!.google.protob\
    uf.EnumValueOptionsR\x14enumValueDescriptionB\x05H\x01\x80\x01\0\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
