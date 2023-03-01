use crate::game::chat::ChatGroup;
use crate::game::phase::PhaseType;
use crate::game::player::{Player, PlayerIndex};
use crate::game::role_list::FactionAlignment;
use crate::game::visit::Visit;
use crate::game::Game;

pub(super) const DEFENSE: u8 = 0;
pub(super) const ROLEBLOCKABLE: bool = true;
pub(super) const WITCHABLE: bool = true;
pub(super) const SUSPICIOUS: bool = false;
pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::TownInvestigative;


pub(super) fn do_night_action(actor: PlayerIndex, game: &mut Game) {
    todo!();
}
pub(super) fn can_night_target(actor: PlayerIndex, target: PlayerIndex, game: &Game) -> bool {
    actor != target && game.players[actor].alive && game.players[target].alive
}
pub(super) fn do_day_action(actor: PlayerIndex, game: &mut Game) {
    
}
pub(super) fn can_day_target(actor: PlayerIndex, target: PlayerIndex, game: &Game) -> bool {
    false
}
pub(super) fn convert_targets_to_visits(actor: PlayerIndex, targets: Vec<PlayerIndex>, game: &Game) -> Vec<Visit> {
    if targets.len() > 0{
        vec![Visit{ target: targets[0], astral: true, attack: false }]
    }else{
        Vec::new()
    }
}
pub(super) fn get_current_chat_groups(actor: PlayerIndex, game: &Game) -> Vec<ChatGroup> {
    if !game.players[actor].alive{
        return vec![ChatGroup::Dead];
    }

    match game.phase_machine.current_state {
        crate::game::phase::PhaseType::Morning => vec![],
        crate::game::phase::PhaseType::Discussion => vec![ChatGroup::All],
        crate::game::phase::PhaseType::Voting => vec![ChatGroup::All],
        crate::game::phase::PhaseType::Testimony => {if game.player_on_trial == Some(actor) {vec![ChatGroup::All]} else {vec![]}},
        crate::game::phase::PhaseType::Judgement => vec![ChatGroup::All],
        crate::game::phase::PhaseType::Evening => vec![ChatGroup::All],
        crate::game::phase::PhaseType::Night => vec![],
    }
}