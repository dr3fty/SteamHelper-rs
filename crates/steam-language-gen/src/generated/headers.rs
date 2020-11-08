use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::generated::enums::EMsg;
use crate::{DeserializableBytes, MessageHeader, MessageHeaderExt, SerializableBytes};

// use steam_protobuf::steam::steammessages_base::CMsgProtoBufHeader;

// add protobuf
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MessageHeaders {
    Standard,
    Extended,
}

// add protobuf matching
impl MessageHeaders {
    pub fn header_from_emsg(emsg: EMsg) -> Option<MessageHeaders> {
        match emsg {
            EMsg::ChannelEncryptRequest | EMsg::ChannelEncryptResponse | EMsg::ChannelEncryptResult => {
                Some(MessageHeaders::Standard)
            }
            _ => Some(MessageHeaders::Extended),
        }
    }
}

#[derive(new, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, MsgHeader)]
pub struct StandardMessageHeader {
    #[new(value = "std::u64::MAX")]
    pub target_job_id: u64,
    #[new(value = "std::u64::MAX")]
    pub source_job_id: u64,
}


#[derive(new, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, MsgHeader)]
pub struct ExtendedMessageHeader {
    #[new(value = "32")]
    header_size: u8,
    #[new(value = "2")]
    header_version: u16,
    #[new(value = "std::u64::MAX")]
    pub target_job_id: u64,
    #[new(value = "std::u64::MAX")]
    pub source_job_id: u64,
    #[new(value = "239")]
    header_canary: u8,
    #[new(value = "0")]
    steam_id: u64,
    #[new(value = "0")]
    session_id: i32,
}

struct GCProtobufMessageHeader {
    header_length: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct GCMessageHeader {
    header_version: u16,
    target_job_id: u64,
    source_job_id: u64,
}

#[derive(Debug)]
pub struct ProtobufMessageHeader {
    header_length: i32,
    proto_header: CMsgProtoBufHeader,
}
