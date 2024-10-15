use std::fmt::Debug;
use l3gion_rust::{StdError, UUID};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct UserCreationInfo {
    pub tag: String,
    pub email: String,
    pub password: UUID,
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
    profile_pic: Option<UUID>,
    friends: Vec<Friend>,
    state: UserState,
}
impl User {
    pub fn as_friend(self) -> Friend {
        Friend {
            uuid: self.uuid,
            tag: self.tag,
            profile_pic: self.profile_pic,
            state: self.state,
        }
    }

    pub fn from(value: DbUser) -> Result<Self, StdError> {
        Ok(Self {
            uuid: UUID::from_u128(value._id.parse::<u128>()?),
            tag: value.tag,
            profile_pic: match value.profile_pic {
                Some(uuid) => Some(UUID::from_u128(uuid.parse::<u128>()?)),
                None => None,
            },
            friends: Vec::default(),
            state: value.state,
        })
    }
    
    pub fn add_friend(&mut self, friend: Friend) {
        self.friends.push(friend);
    }
    
    pub fn set_friends(&mut self, friends: Vec<Friend>) {
        self.friends = friends;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DbUser {
    _id: String,
    tag: String,
    email: String,
    profile_pic: Option<String>,
    friends: Vec<String>, // UUID
    state: UserState,
}
impl DbUser {
    pub fn new(info: UserCreationInfo) -> Self {
        Self {
            _id: UUID::generate().to_string(),
            tag: info.tag,
            email: info.email,
            profile_pic: None,
            friends: Vec::default(),
            state: UserState::ONLINE,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Friend {
    uuid: UUID,
    tag: String,
    profile_pic: Option<UUID>,
    state: UserState,
}
