use std::collections::HashMap;

use crate::autoodo::UserPresence;

#[derive(Clone)]
pub struct State {
    pub is_connected: bool,
    pub users_presence: HashMap<String, UserPresence>, //< Key: User's name
}

impl State {
    pub fn new() -> Self {
        Self {
            is_connected: false,
            users_presence: HashMap::new(),
        }
    }
}

impl From<Vec<UserPresence>> for State {
    fn from(users_presence: Vec<UserPresence>) -> Self {
        let mut state = State::new();
        state.users_presence = users_presence.into_iter().map(|user| user.into()).collect();
        state
    }
}
