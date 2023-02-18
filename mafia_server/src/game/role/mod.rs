macro_rules! make_role_enum {
    (
        $(
            $name:ident : $file:ident $({
                $($data_ident:ident: $data_type:ty = $data_def:expr),*
            })?
        ),*
    ) => {
        use crate::game::player::PlayerIndex;
        use crate::game::Game;
        use crate::game::chat::ChatGroup;
        use crate::game::role_list::FactionAlignment;
        $(mod $file;)*


        #[derive(Clone, Copy)]
        pub enum Role {
            $($name),*
        }

        #[derive(Clone, Copy)]
        pub enum RoleData {
            $($name $({
                $($data_ident: $data_type),*
            })?),*
        }

        impl Role {
            pub fn values() -> Vec<Role> {
                return vec![$(Role::$name),*];
            }

            /* Constants */

            // TODO: Make method, which takes Game as parameter
            pub fn default_data(&self) -> RoleData {
                match self {
                    $(Role::$name => RoleData::$name$({
                        $($data_ident: $data_def),*
                    })?),*
                }
            }

            pub fn is_suspicious(&self) -> bool {
                match self {
                    $(Role::$name => $file::SUSPICIOUS),*
                }
            }

            pub fn is_witchable(&self) -> bool {
                match self {
                    $(Role::$name => $file::WITCHABLE),*
                }
            }

            pub fn get_defense(&self) -> u8 {
                match self {
                    $(Role::$name => $file::DEFENSE),*
                }
            }

            pub fn is_roleblockable(&self) -> bool {
                match self {
                    $(Role::$name => $file::ROLEBLOCKABLE),*
                }
            }

            pub fn get_faction_alignment(&self) -> FactionAlignment {
                match self {
                    $(Role::$name => $file::FACTION_ALIGNMENT),*
                }
            }

            /* methods */

            pub fn do_night_action(&mut self, source: PlayerIndex, game: &mut Game) {
                match self {
                    $(Role::$name => $file::do_night_action(source, game)),*
                }
            }
            pub fn do_day_action(&mut self, source: PlayerIndex, game: &mut Game) {
                match self {
                    $(Role::$name => $file::do_day_action(source, game)),*
                }
            }
            pub fn can_night_target(&self, source: PlayerIndex, target: PlayerIndex, game: &Game) -> bool {
                match self {
                    $(Role::$name => $file::can_night_target(source, target, game)),*
                }
            }
            pub fn can_day_target(&self, source: PlayerIndex, target: PlayerIndex, game: &Game) -> bool {
                match self {
                    $(Role::$name => $file::can_day_target(source, target, game)),*
                }
            }
            pub fn get_current_chat_groups(&self, source: PlayerIndex, game: &Game) -> Vec<ChatGroup> {
                match self {
                    $(Role::$name => $file::get_current_chat_groups(source, game)),*
                }
            }
        }

        impl RoleData {
            pub fn role(&self) -> Role {
                match self {
                    $(RoleData::$name$({
                        $($data_ident: _),*
                    })? => Role::$name),*
                }
            }
        }
    }
}

// Creates the Role enum
make_role_enum! {
    Sheriff : sheriff,

    Doctor : doctor,

    Veteran : veteran {
        alerts_remaining: u8 = 3
    },

    Mafioso : mafioso,
    
    Consort : consort
}

/*
Proposed Priorities:

Visit objects created key:
nv = no visit
av = astral visit
v = visit

+1: Jester(Kill, av) Vigilante(Suicide, nv) Arsonist(Clear self, nv) Vampire(Choose Leader, nv) Witch(Activate sheild, nv) Veteran(Decide, av) Retributionist(Decide and witch, av, av) //non transportable or witchable
+2: Transporter(Swaps, v, v)
+3: Witch(Swap, v, av) 
+4: Escort/Consort(Roleblock, v)
+5: Godfather("witch mafioso if not rbd, clear targets on self", av)
+6: Doctor(Heal, v) Bodyguard("Witch attacker", v) //all attacks should happen after this
+7: Blackmailer, Arsonist(Douse&visitors, v&nv), Framer(Frame&visitors, v&nv), Disguiser("Swap", v, v) Werewolf("unframe", nv) Forger(Frame, v) Janitor(Frame, v)   //investigations happen after this
+8: Invest, Sheriff, Lookout, Tracker, Consig
+9: Mafioso/Godfather/Sk/Ww/Vet/Vig/Vamp/Arso/Bg/Vig("Kill")
+10: Doc(Notify both, nv) Bg(Notify both, nv) Janitor(notify, nv), Forger(notify, nv) Vamp(Inform Leader & new vamp, nv) Arsonist(Inform who is doused, nv)
+11: Exe/Amne/Vamp(Convert), spy(bug, v)
+12: Witch(bug)

*/

/*
Old Priorities:

-12: Veteran(Decides Alert) Vigilante(Suicide) Jester(Kill) Arsonist(Clean self) Vampire(Choose Leader)
-10: Transporter(Swaps)
-8: Witch(Swaps, Activate sheild)
-7: Retributionist(Choose to revive)
-6: Escort / Consort(Roleblock)
-4: Godfather(Swap mafioso target and clear self)
-2 bodyguard(swap)
0: visits happen here
+2: Doctor(Heal), Blackmailer(Decide), Crusader(Heal), Arsonist(Douse), Framer, Disguiser Werewolf(innos themself)
+4: Sheriff, Invest, Consig, Lookout, Tracker, Arsonist(Find who visited)
+6: Mafioso/Godfather, SerialKiller, Werewolf, Veteran, Vampire, Arsonist, Crusader, Bodyguard, Vigilante (All kill)
+8: Forger(Change info), Janitor(Clean & info), Doctor(Notify) Bodyguard(Notify) 
+10: Spy(Collect info) Vampire(Inform of leader) Arsonist(Inform who is doused)
+11: Amnesiac(Convert) Vampire(Convert) Executioner(convert)
+12: Witch(Steal info & Remove sheild)
*/