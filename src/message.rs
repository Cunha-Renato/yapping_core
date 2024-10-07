use l3gion_rust::UUID;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageType {
    TEXT(String),
    FILE(Vec<u8>) // TODO: Find a way of making this work
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    content: MessageType,
    sender: UUID,
}