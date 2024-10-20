use l3gion_rust::UUID;

pub struct ProfilePic {
    uuid: UUID,
    bytes: Vec<u8>,
}

pub struct DbProfilePic {
    _id: String,
    bytes: Vec<u8>,
}