use std::{io::Read, marker::PhantomData};

use bytecheck::CheckBytes;
use rdkafka::message::{FromBytes, ToBytes};
use rkyv::{
    de::deserializers::SharedDeserializeMap, ser::serializers::AllocSerializer, Archive,
    Deserialize, Fallible, Serialize,
};

pub struct KafkaByteMessage {
    bytes: Vec<u8>,
}

impl From<KafkaMessage> for KafkaByteMessage {
    fn from(message: KafkaMessage) -> Self {
        KafkaByteMessage {
            bytes: rkyv::to_bytes::<KafkaMessage, 256>(&message)
                .unwrap()
                .to_vec(),
        }
    }
}

impl From<KafkaByteMessage> for Vec<u8> {
    fn from(byte_message: KafkaByteMessage) -> Self {
        byte_message.bytes
    }
}

impl From<KafkaMessage> for Vec<u8> {
    fn from(message: KafkaMessage) -> Self {
        let bytes = KafkaByteMessage::from(message);
        bytes.bytes
    }
}

impl Into<KafkaMessage> for KafkaByteMessage {
    fn into(self) -> KafkaMessage {
        unsafe { rkyv::from_bytes_unchecked::<KafkaMessage>(&self.bytes[..]) }.unwrap()
    }
}

impl Into<KafkaByteMessage> for &[u8] {
    fn into(self) -> KafkaByteMessage {
        KafkaByteMessage {
            bytes: self.to_vec(),
        }
    }
}

impl Into<KafkaMessage> for &[u8] {
    fn into(self) -> KafkaMessage {
        let byte_message: KafkaByteMessage = self.into();
        let what: KafkaMessage = byte_message.into();
        what
    }
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug))]
pub enum KafkaMessage {
    Extract(ExtractMessage),
    Other(OtherMessage),
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug))]
pub struct ExtractMessage {
    pub name: String,
    pub number: i32,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug))]
pub struct OtherMessage {
    pub name: String,
    pub number: i32,
}

pub trait KafkaBytes<T> {
    fn to_bytes<const N: usize>(msg: &T) -> Vec<u8>
    where
        T: Serialize<AllocSerializer<N>>,
    {
        rkyv::to_bytes::<T, N>(&msg).unwrap().to_vec()
    }

    fn from_bytes(bytes: &Vec<u8>) -> T
    where
        T: Archive,
        T::Archived: Deserialize<T, SharedDeserializeMap>,
    {
        unsafe { rkyv::from_bytes_unchecked::<T>(&bytes[..]) }.unwrap()
    }
}

impl KafkaBytes<ExtractMessage> for ExtractMessage {}
impl KafkaBytes<OtherMessage> for OtherMessage {}
impl KafkaBytes<KafkaMessage> for KafkaMessage {}
