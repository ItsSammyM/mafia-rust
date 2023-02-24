pub(crate) mod grave;
pub(crate) mod phase;
pub mod player;
pub(crate) mod phase_resetting;
pub(crate) mod chat;
pub(crate) mod role;
pub(crate) mod visit;
pub(crate) mod vote;
pub(crate) mod role_list;

#[allow(clippy::module_inception)]
mod game;
mod settings;
pub use game::*;