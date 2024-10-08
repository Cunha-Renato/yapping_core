use l3gion_rust::UUID;
use serde::{Deserialize, Serialize};

use crate::message::Message;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chat {
    uuid: UUID,
    users: Vec<UUID>,
    messages: Vec<Message>,
}