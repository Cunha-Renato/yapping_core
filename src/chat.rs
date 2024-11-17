use l3gion_rust::{StdError, UUID};
use serde::{Deserialize, Serialize};

use crate::message::{DbMessage, Message};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Chat {
    uuid: UUID,
    tag: String,
    users: Vec<UUID>,
    // TODO: Max size of 50.
    messages: Vec<Message>,
}
impl Chat {
    pub fn new(tag: &str, users: Vec<UUID>) -> Self {
        Self {
            uuid: UUID::generate(),
            tag: tag.to_string(),
            users,
            messages: Vec::default(),
        }
    }

    pub fn from(db_chat: DbChat) -> Result<Self, StdError> {
        Ok(Self {
            uuid: UUID::from_u128(db_chat._id.parse()?),
            tag: db_chat.tag,
            users: db_chat.users
                .iter()
                .map(|u| UUID::from_u128(u.parse().unwrap()))
                .collect(),
            messages: db_chat.messages
                .into_iter()
                .map(|m| Message::from(m).unwrap())
                .collect(),
        })
    }

    pub fn uuid(&self) -> UUID {
        self.uuid
    }
    
    pub fn tag(&self) -> &str {
        &self.tag
    }
    
    pub fn users(&self) -> &[UUID] {
        &self.users
    }
    
    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    pub fn clear_messages(&mut self) {
        self.messages.clear();
    }

    pub fn append_messages(&mut self, messages: &mut Vec<Message>) {
        self.messages.append(messages);
    }
    
    pub fn push_message(&mut self, message: Message) {
        self.messages.push(message)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DbChat {
    _id: String,
    tag: String,
    users: Vec<String>,
    messages: Vec<DbMessage>,
}
impl DbChat {
    pub fn new(uuid: UUID, tag: &str, members: &[UUID]) -> Self {
        Self {
            _id: uuid.to_string(),
            tag: tag.to_string(),
            users: members.iter().map(|m| m.to_string()).collect(),
            messages: Vec::default(),
        }
    }
        
    pub fn users(&self) -> &[String] {
        &self.users
    }
}
impl From<Chat> for DbChat {
    fn from(value: Chat) -> Self {
        Self {
            _id: value.uuid.to_string(),
            tag: value.tag,
            users: value.users.iter().map(|u| u.to_string()).collect(),
            messages: value.messages.into_iter().map(|m| DbMessage::from(m)).collect(),
        }
    }
}