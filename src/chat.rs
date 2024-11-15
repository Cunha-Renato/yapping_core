use l3gion_rust::UUID;
use serde::{Deserialize, Serialize};

use crate::message::Message;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chat {
    uuid: UUID,
    tag: String,
    users: Vec<UUID>,
    messages: Vec<Message>,
}
impl Chat {
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
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DbChat {
    _id: String,
    tag: String,
    users: Vec<String>,
    messages: Vec<Message>,
}