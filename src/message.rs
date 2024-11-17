use l3gion_rust::{StdError, UUID};
use serde::{Deserialize, Serialize};

use crate::date_time::{self, DateTime};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MessageType {
    TEXT(String),
    FILE(Vec<u8>) // TODO: Find a way of making this work
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Message {
    sender: UUID,
    content: MessageType,
    date_time: DateTime,
}
impl Message {
    pub fn from(db_message: DbMessage) -> Result<Self, StdError> {
        Ok(Self {
            sender: UUID::from_u128(db_message.sender.parse()?),
            content: db_message.content,
            date_time: db_message.date_time,
        })
    }
    
    pub fn new(sender: UUID, content: MessageType, date_time: DateTime) -> Self {
        Self {
            sender,
            content,
            date_time
        }
    }
    
    pub fn sender(&self) -> UUID {
        self.sender
    }

    pub fn content(&self) -> &MessageType {
        &self.content
    }
    
    pub fn date_time(&self) -> DateTime {
        self.date_time
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DbMessage {
    sender: String,
    content: MessageType,
    date_time: DateTime,
}
impl DbMessage {
    pub fn from(message: Message) -> Self {
        Self {
            sender: message.sender.to_string(),
            content: message.content,
            date_time: message.date_time,
        }
    }
}