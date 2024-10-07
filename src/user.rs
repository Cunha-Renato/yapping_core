use std::fmt::Debug;

use l3gion_rust::UUID;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserCreationInfo {
    tag: String,
    email: String,
    password: UUID,
}
impl Debug for UserCreationInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UserCreationInfo").field("tag", &self.tag).field("email", &self.email).field("password", &"Hidden").finish()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum UserState {
    ONLINE,
    OFFLINE,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    uuid: UUID,
    tag: String,
    profile_pic: UUID,
    friends: Vec<Self>,
    state: UserState,
}