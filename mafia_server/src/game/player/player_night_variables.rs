use crate::game::{visit::Visit, vote::Verdict};

use super::{PlayerIndex, Player};

pub struct PlayerNightVariables{
    pub alive_tonight:  bool,
    pub died:           bool,
    pub attacked:       bool,
    pub roleblocked:    bool,
    pub defense:        u8,    
    pub suspicious:     bool,

    pub janitor_cleaned:bool,
    //forger: Option<(Role, String)>, //this is new, maybe a bad idea? I dotn know, in old code this was ShownRole, ShownWill, ShownNote,
    pub disguised_as:   PlayerIndex,

    pub chosen_targets: Vec<PlayerIndex>,
    pub visits:         Vec<Visit>,
}
impl PlayerNightVariables{
    pub fn new()->Self{
        Self{
            alive_tonight:  true,
            died:           false,
            attacked:       false,
            roleblocked:    false,
            defense:        0,
            suspicious:     false,

            disguised_as:   0,
            janitor_cleaned:false,
            //forger: todo!(),

            chosen_targets: vec![],
            visits:         vec![],
        }
    }
    pub fn reset(&mut self, index: PlayerIndex, player: Player){
        self.alive_tonight=  true;
        self.died=           false;
        self.attacked=       false;
        self.roleblocked=    false;
        self.defense=        player.get_role().get_defense();
        self.suspicious=     player.get_role().is_suspicious();

        self.disguised_as=   index;
        self.janitor_cleaned=false;
        //forger= todo!();

        self.chosen_targets= vec![];
        self.visits=         vec![];
    }
}