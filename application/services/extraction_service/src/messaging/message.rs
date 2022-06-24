use std::fmt::Debug;

use bytecheck::CheckBytes;
use derive_builder::Builder;
use rkyv::{ser::serializers::AllocSerializer, Archive, Deserialize, Serialize};

impl From<&KafkaMessageKind> for Vec<u8> {
    fn from(message: &KafkaMessageKind) -> Self {
        rkyv::to_bytes::<KafkaMessageKind, 256>(&message)
            .unwrap()
            .to_vec()
    }
}

impl Into<KafkaMessageKind> for &[u8] {
    fn into(self) -> KafkaMessageKind {
        unsafe { rkyv::from_bytes_unchecked::<KafkaMessageKind>(&self[..]) }
            .unwrap()
            .into()
    }
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug))]
pub enum KafkaMessageKind {
    Extract(ExtractMessage),
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Eq, Default, Builder)]
#[builder(build_fn(private, name = "super_build"), setter(into))]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug))]
pub struct ExtractMessage {
    pub name: String,
    pub number: i32,
}

pub trait KafkaMessage: Serialize<AllocSerializer<256>> + Archive {
    fn encode(&self) -> Vec<u8>;
    fn decode(message: &[u8]) -> KafkaMessageKind;
}

impl KafkaMessage for KafkaMessageKind {
    fn encode(&self) -> Vec<u8> {
        self.into()
    }

    fn decode(message: &[u8]) -> Self {
        message.into()
    }
}

impl ExtractMessageBuilder {
    pub fn build(&self) -> Vec<u8> {
        let message = KafkaMessageKind::Extract(self.super_build().unwrap());
        message.encode()
    }
}
