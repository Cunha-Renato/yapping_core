use std::fmt::Debug;
use l3gion_rust::{StdError, UUID};
use serde::{Deserialize, Serialize};

/// When printing the password shows, it's not a problem because this is a prototype.
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct UserCreationInfo {
    pub tag: String,
    pub email: String,
    pub password: UUID,
}
impl Debug for UserCreationInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UserCreationInfo").field("tag", &self.tag).field("email", &self.email).field("password", &self.password).finish()
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, Copy)]
pub enum UserState {
    ONLINE,
    #[default]
    OFFLINE,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct User {
    uuid: UUID,
    tag: String,
    profile_pic: Option<UUID>,
    friends: Vec<User>,
    chats: Vec<UUID>,
    state: UserState,
}
impl User {
    pub fn uuid(&self) -> UUID {
        self.uuid
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }
    
    pub fn profile_pic(&self) -> Option<UUID> {
        self.profile_pic.clone()
    }

    pub fn state(&self) -> UserState {
        self.state
    }
    
    pub fn friends(&self) -> &[User] {
        &self.friends
    }
    
    pub fn chats(&self) -> &[UUID] {
        &self.chats
    }

    pub fn from(db_user: DbUser) -> Result<Self, StdError> {
        Ok(Self {
            uuid: UUID::from_u128(db_user._id.parse::<u128>()?),
            tag: db_user.tag,
            profile_pic: db_user.profile_pic.and_then(|uuid| Some(UUID::from_u128(uuid.parse::<u128>().ok()?))),
            friends: Vec::default(),
            chats: db_user.chats
                .iter()
                .map(|id| UUID::from_u128(id.parse::<u128>().unwrap())) // Unwrap is bad, but Im lazy.
                .collect(),
            state: db_user.state,
        })
    }

    pub fn strip_info(&mut self) {
        self.clear_friends();
        self.chats.clear();
    }

    pub fn clear_friends(&mut self) {
        self.friends.clear();
    }
    
    pub fn add_friend(&mut self, friend: User) {
        self.friends.push(friend);
    }
    
    pub fn set_friends(&mut self, friends: Vec<User>) {
        self.friends = friends;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DbUser {
    _id: String,
    tag: String,
    email: String,
    password: String,
    profile_pic: Option<String>,
    friends: Vec<String>, // UUID
    chats: Vec<String>, // UUID
    state: UserState,
}
impl DbUser {
    pub fn new(info: UserCreationInfo) -> Self {
        Self {
            _id: UUID::generate().to_string(),
            tag: info.tag,
            email: info.email,
            password: info.password.to_string(),
            profile_pic: None,
            friends: Vec::default(),
            chats: Vec::default(),
            state: UserState::ONLINE,
        }
    }
    
    pub fn friends(&self) -> &[String] {
        &self.friends
    }
    
    pub fn chats(&self) -> &[String] {
        &self.chats
    }
}