use crate::autoodo::UserPresence;

#[derive(Clone)]
pub struct State {
    pub is_connected: bool,
    pub users_presence: Vec<UserPresence>,
}

impl State {
    pub fn new() -> Self {
        Self {
            is_connected: false,
            users_presence: Vec::new(),
        }
    }
}
