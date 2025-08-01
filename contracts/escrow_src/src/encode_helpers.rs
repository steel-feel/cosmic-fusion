use prost::Message;

pub(crate) fn encode_bytes_message<T: Message>(msg: &T) -> Result<Vec<u8>, prost::EncodeError> {
    let mut buffer = Vec::new();
    msg.encode(&mut buffer)?; // Encode the message using prost
    Ok(buffer)
}