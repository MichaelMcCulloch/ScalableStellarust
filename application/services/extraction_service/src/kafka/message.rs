use bytecheck::CheckBytes;
use rkyv::{Archive, Deserialize, Serialize};

impl From<KafkaMessage> for Vec<u8> {
    fn from(message: KafkaMessage) -> Self {
        rkyv::to_bytes::<KafkaMessage, 256>(&message)
            .unwrap()
            .to_vec()
    }
}

impl Into<KafkaMessage> for Vec<u8> {
    fn into(self) -> KafkaMessage {
        self.as_slice().into()
    }
}
impl Into<KafkaMessage> for &[u8] {
    fn into(self) -> KafkaMessage {
        unsafe { rkyv::from_bytes_unchecked::<KafkaMessage>(&self[..]) }
            .unwrap()
            .into()
    }
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug))]
pub enum KafkaMessage {
    Extract(ExtractMessage),
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug))]
pub struct ExtractMessage {
    pub name: String,
    pub number: i32,
}
