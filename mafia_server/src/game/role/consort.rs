use super::*;

create_role! { Consort

    defense: 0;
    roleblockable: false;
    witchable: true;
    suspicious: true;

    fn do_night_action(actor: &mut Player, game: &mut Game) {
        todo!()
    }

    fn can_night_target(actor: &Player, target: &Player, game: &Game) -> bool {
        todo!()
    }

    fn do_day_action(actor: &mut Player, target: &mut Player, game: &mut Game) {
        todo!()
    }

    fn can_day_target(actor: &Player, target: &Player, game: &Game) -> bool {
        todo!()
    }

    fn convert_targets_to_visits(targets: &[PlayerID], game: &Game) -> Vec<Visit> {
        todo!()
    }
}