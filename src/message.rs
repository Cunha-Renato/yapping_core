use l3gion_rust::UUID;
use serde::{Deserialize, Serialize};

use crate::date_time::DateTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageType {
    TEXT(String),
    FILE(Vec<u8>) // TODO: Find a way of making this work
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    sender: UUID,
    content: MessageType,
    date_time: DateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DbMessage {
    sender: UUID,
    content: MessageType,
    date_time: DateTime,
}