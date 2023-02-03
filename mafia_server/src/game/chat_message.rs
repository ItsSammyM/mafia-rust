use super::{phase::Phase, grave::Grave, role::Role, player::PlayerID};

pub enum ChatMessage {
    System {
        title: Option<String>,
        msg: Option<String>,
    },
    Player {
        player_id: String,
        msg: String,
    },
}