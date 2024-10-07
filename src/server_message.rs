use std::fmt::Debug;

use l3gion_rust::UUID;
use serde::{Deserialize, Serialize};
use crate::{chat::Chat, message::Message, user::{User, UserCreationInfo}};

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    NEW_USER(UserCreationInfo),
    NEW_CHAT(Chat),
    // Chat uuid, Message
    MESSAGE_SEND(UUID, Message),
    // User uuid, User tag
    UPDATE_USER_TAG(UUID, String),
    // User uuid, User email
    UPDATE_USER_EMAIL(UUID, String),
    // User uuid, User pic
    UPDATE_USER_PIC(UUID, Vec<u8>),
    // User uuid, User password
    UPDATE_USER_PASSWORD(User, UUID),
    DELETE_USER(UUID),
    // Sender, Receiver
    FRIEND_REQUEST(UUID, UUID),
}
impl Debug for ClientMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NEW_USER(arg0) => f.debug_tuple("NEW_USER").field(arg0).finish(),
            Self::NEW_CHAT(arg0) => f.debug_tuple("NEW_CHAT").field(arg0).finish(),
            Self::MESSAGE_SEND(arg0, arg1) => f.debug_tuple("MESSAGE_SEND").field(arg0).field(arg1).finish(),
            Self::UPDATE_USER_TAG(arg0, arg1) => f.debug_tuple("UPDATE_USER_TAG").field(arg0).field(arg1).finish(),
            Self::UPDATE_USER_EMAIL(arg0, arg1) => f.debug_tuple("UPDATE_USER_EMAIL").field(arg0).field(arg1).finish(),
            Self::UPDATE_USER_PIC(arg0, arg1) => f.debug_tuple("UPDATE_USER_PIC").field(arg0).field(arg1).finish(),
            Self::UPDATE_USER_PASSWORD(arg0, _) => f.debug_tuple("UPDATE_USER_PASSWORD").field(arg0).finish(),
            Self::DELETE_USER(arg0) => f.debug_tuple("DELETE_USER").field(arg0).finish(),
            Self::FRIEND_REQUEST(arg0, arg1) => f.debug_tuple("FRIEND_REQUEST").field(arg0).field(arg1).finish(),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    SUCCESS,
    FAIL(String),
    // Chat uuid, Message
    MESSAGE_RECEIVED(UUID, Message),
    // Sender
    FRIEND_REQUEST(UUID),
}