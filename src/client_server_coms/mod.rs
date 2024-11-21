use l3gion_rust::{StdError, UUID};
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
    OK,
    Err(String),
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Notification {
    uuid: UUID,
    pub notification_type: NotificationType,
}
impl Notification {
    pub fn new_with_uuid(uuid: UUID, n_type: NotificationType) -> Self {
        let mut result = Self::new(n_type);
        result.uuid = uuid;
        
        result
    }

    pub fn new(n_type: NotificationType) -> Self {
        Self {
            uuid: UUID::generate(),
            notification_type: n_type,
        }
    }

    pub fn from(value: DbNotification) -> Result<Self, StdError> {
        Ok(Self {
            uuid: UUID::from_u128(value._id.parse::<u128>()?),
            notification_type: NotificationType::from(&value.notification_type)?,
        })
    }
    
    pub fn uuid(&self) -> UUID {
        self.uuid
    }
    
    pub fn notification_type(&self) -> &NotificationType {
        &self.notification_type
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DbNotification {
    _id: String,
    user: String,
    notification_type: DbNotificationType,
}
impl DbNotification {
    pub fn new(user: UUID, notification: &Notification) -> Self {
        Self {
            _id: notification.uuid.to_string(),
            user: user.to_string(),
            notification_type: DbNotificationType::from(&notification.notification_type),
        }
    }
    
    pub fn notification_type(&self) -> &DbNotificationType {
        &self.notification_type
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub enum NotificationType {
    /// Chat uuid, Message
    NEW_MESSAGE(UUID, Message),
    MESSAGE(UUID),
    MESSAGE_READ(UUID),
    NEW_CHAT(Chat),
    /// Sender Receiver
    FRIEND_REQUEST(UUID, UUID),
    /// Sender Receiver
    FRIEND_ACCEPTED(UUID, UUID),
    RESEND_USER(UUID),
}
impl NotificationType {
    pub fn from(value: &DbNotificationType) -> Result<Self, StdError> {
        Ok(match value {
            DbNotificationType::MESSAGE(chat_uuid) => Self::MESSAGE(UUID::from_u128(chat_uuid.parse()?)),
            DbNotificationType::FRIEND_REQUEST(sender, receiver) => Self::FRIEND_REQUEST(UUID::from_u128(sender.parse()?), UUID::from_u128(receiver.parse()?)),
            DbNotificationType::FRIEND_ACCEPTED(sender, receiver) => Self::FRIEND_ACCEPTED(UUID::from_u128(sender.parse()?), UUID::from_u128(receiver.parse()?)),
        })
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]

pub enum DbNotificationType {
    MESSAGE(String),
    // MESSAGE_READ(UUID),
    // NEW_CHAT(Chat),
    /// Sender Receiver
    FRIEND_REQUEST(String, String),
    /// Sender Receiver
    FRIEND_ACCEPTED(String, String),
}
impl DbNotificationType {
    pub fn from(value: &NotificationType) -> Self {
        match value {
            NotificationType::MESSAGE(chat_uuid) => Self::MESSAGE(chat_uuid.to_string()),
            NotificationType::FRIEND_REQUEST(sender, receiver) => Self::FRIEND_REQUEST(sender.to_string(), receiver.to_string()),
            NotificationType::FRIEND_ACCEPTED(sender, receiver) => Self::FRIEND_ACCEPTED(sender.to_string(), receiver.to_string()),
            _ => todo!()
        }
    }
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
    REMOVE_FRIEND(UUID),
}
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]

pub enum Query {
    USERS_CONTAINS_TAG(String),
    USERS_BY_TAG(Vec<String>),
    USERS_BY_UUID(Vec<UUID>),
    FRIEND_REQUESTS,
    USER_CHATS,
    CHAT_MESSAGES(UUID),
    RESULT_CHAT_MESSAGES(Vec<Message>),
    RESULT_USER(Vec<User>),
    RESULT_FRIEND_REQUESTS(Vec<Notification>),
    RESULT_CHATS(Vec<Chat>),// TODO: I don't know how Im going to do this.
}