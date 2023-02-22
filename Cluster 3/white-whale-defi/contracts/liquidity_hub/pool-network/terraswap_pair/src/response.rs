// This file is generated by rust-protobuf 3.1.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
// @generated
#![cfg(not(tarpaulin_include))]

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `src/response.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

///  MsgInstantiateContractResponse return instantiation result data
#[derive(PartialEq, Clone, Default, Debug)]
// @@protoc_insertion_point(message:MsgInstantiateContractResponse)
pub struct MsgInstantiateContractResponse {
    // message fields
    ///  Address is the bech32 address of the new contract instance.
    // @@protoc_insertion_point(field:MsgInstantiateContractResponse.address)
    pub address: ::std::string::String,
    ///  Data contains base64-encoded bytes to returned from the contract
    // @@protoc_insertion_point(field:MsgInstantiateContractResponse.data)
    pub data: ::std::vec::Vec<u8>,
    // special fields
    // @@protoc_insertion_point(special_field:MsgInstantiateContractResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MsgInstantiateContractResponse {
    fn default() -> &'a MsgInstantiateContractResponse {
        <MsgInstantiateContractResponse as ::protobuf::Message>::default_instance()
    }
}

impl MsgInstantiateContractResponse {
    pub fn new() -> MsgInstantiateContractResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "address",
            |m: &MsgInstantiateContractResponse| { &m.address },
            |m: &mut MsgInstantiateContractResponse| { &mut m.address },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "data",
            |m: &MsgInstantiateContractResponse| { &m.data },
            |m: &mut MsgInstantiateContractResponse| { &mut m.data },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MsgInstantiateContractResponse>(
            "MsgInstantiateContractResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MsgInstantiateContractResponse {
    const NAME: &'static str = "MsgInstantiateContractResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.address = is.read_string()?;
                },
                18 => {
                    self.data = is.read_bytes()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.address.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.address);
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.address.is_empty() {
            os.write_string(1, &self.address)?;
        }
        if !self.data.is_empty() {
            os.write_bytes(2, &self.data)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> MsgInstantiateContractResponse {
        MsgInstantiateContractResponse::new()
    }

    fn clear(&mut self) {
        self.address.clear();
        self.data.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MsgInstantiateContractResponse {
        static instance: MsgInstantiateContractResponse = MsgInstantiateContractResponse {
            address: ::std::string::String::new(),
            data: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MsgInstantiateContractResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MsgInstantiateContractResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MsgInstantiateContractResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MsgInstantiateContractResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12src/response.proto\"N\n\x1eMsgInstantiateContractResponse\x12\x18\
    \n\x07address\x18\x01\x20\x01(\tR\x07address\x12\x12\n\x04data\x18\x02\
    \x20\x01(\x0cR\x04dataJ\xde\x02\n\x06\x12\x04\0\0\x08\x01\n\x08\n\x01\
    \x0c\x12\x03\0\0\x12\nM\n\x02\x04\0\x12\x04\x03\0\x08\x01\x1aA\x20MsgIns\
    tantiateContractResponse\x20return\x20instantiation\x20result\x20data\n\
    \n\n\n\x03\x04\0\x01\x12\x03\x03\x08&\nJ\n\x04\x04\0\x02\0\x12\x03\x05\
    \x02\x15\x1a=\x20Address\x20is\x20the\x20bech32\x20address\x20of\x20the\
    \x20new\x20contract\x20instance.\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\
    \x05\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x05\t\x10\n\x0c\n\x05\
    \x04\0\x02\0\x03\x12\x03\x05\x13\x14\nO\n\x04\x04\0\x02\x01\x12\x03\x07\
    \x02\x11\x1aB\x20Data\x20contains\x20base64-encoded\x20bytes\x20to\x20re\
    turned\x20from\x20the\x20contract\n\n\x0c\n\x05\x04\0\x02\x01\x05\x12\
    \x03\x07\x02\x07\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x07\x08\x0c\n\x0c\
    \n\x05\x04\0\x02\x01\x03\x12\x03\x07\x0f\x10b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MsgInstantiateContractResponse::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
