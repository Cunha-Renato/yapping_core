use l3gion_rust::UUID;
use serde::{Deserialize, Serialize};

use crate::{chat::Chat, message::Message, user::{User, UserCreationInfo}};

pub mod coms_manager;
pub use coms_manager::ComsManager;

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerMessage {
    pub uuid: UUID,
    pub content: ServerMessageContent,
}
impl ServerMessage {
    pub fn new(uuid: UUID, content: ServerMessageContent) -> Self {
        Self {
            uuid,
            content,
        }
    }
    
    pub fn from(content: ServerMessageContent) -> Self {
        Self {
            uuid: UUID::generate(),
            content,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]

pub enum ServerMessageContent {
    RESPONSE(Response),
    SESSION(Session),
    NOTIFICATION(Notification),
    MODIFICATION(Modification),
    QUERY(Query),
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Response {
    OK_SESSION(Session),
    OK_NOTIFICATION(Notification),
    OK_MODIFICATION(Modification),
    OK_QUERY(Query),
    Err(String),
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]

pub enum Notification {
    /// Chat uuid, Message
    MESSAGE(UUID, Message),
    NEW_CHAT(Chat),
    /// Sender Receiver
    FRIEND_REQUEST(UUID, UUID),
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Session {
    LOGIN(UserCreationInfo),
    SIGN_UP(UserCreationInfo),
    TOKEN(User),
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]

pub enum Modification {
    /// User uuid, User tag
    USER_TAG(UUID, String),
    /// User uuid, User email
    USER_EMAIL(UUID, String),
    /// User uuid, User pic
    USER_PIC(UUID, Vec<u8>),
    /// User uuid, User password
    USER_PASSWORD(UUID, UUID),
    DELETE_USER(UUID),
}
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]

pub enum Query {
    USERS_CONTAINS_TAG(String),
    USERS_BY_TAG(Vec<String>),
    USERS_BY_UUID(Vec<UUID>),
    RESULT(Vec<User>),
}