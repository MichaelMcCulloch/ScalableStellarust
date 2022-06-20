use bytecheck::CheckBytes;
use rdkafka::message::ToBytes;
use rkyv::{
    de::deserializers::SharedDeserializeMap, ser::serializers::AllocSerializer, Archive,
    Deserialize, Serialize,
};

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
