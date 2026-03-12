use crate::autoodo::UserPresence;

pub trait State {
    fn is_connected(&self) -> bool;
    fn users_presence(&self) -> Vec<UserPresence>;
}
