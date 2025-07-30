use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use prost::Message;

pub fn encode_proto_message<T: Message>(msg: T) -> String {
    let mut buf = vec![];
    T::encode(&msg, &mut buf).unwrap();
    BASE64_STANDARD.encode(&buf)
}

pub(crate) fn encode_bytes_message<T: Message>(msg: &T) -> Result<Vec<u8>, prost::EncodeError> {
    let mut buffer = Vec::new();
    msg.encode(&mut buffer)?; // Encode the message using prost
    Ok(buffer)
}