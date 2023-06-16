use crate::user::User;

#[derive(Clone, Debug)]
pub struct Channel {
    pub name: String,
    pub users: Vec<User>,
    pub creator: ChannelCreator,
}

#[derive(Clone, Debug)]
pub struct ChannelCreator(pub String);
