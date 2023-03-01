

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;

use crate::network::packet::{ToServerPacket, ToClientPacket, self, PlayerButtons};
use crate::prelude::*;
use super::{phase::{PhaseStateMachine, PhaseType}, player::{Player, PlayerIndex}, role_list::RoleList, settings::Settings, grave::Grave};

pub struct Game {
    pub settings : Settings,

    pub players: Vec<Player>,   // PlayerIndex is the index into this vec, should be unchanging as game goes on
    pub graves: Vec<Grave>,

    pub phase_machine : PhaseStateMachine,

    pub player_on_trial: Option<PlayerIndex>,   //Morning
    pub trials_left: u8,                //Morning
}

impl Game {
    pub fn new(settings: Settings, players_sender_and_name: Vec<(UnboundedSender<ToClientPacket>, String)>)->Self{

        let mut players = Vec::new();

        //create players
        for player_index in 0..players_sender_and_name.len(){
            let (sender, name) = &players_sender_and_name[player_index];
            players.push(Player::new(player_index, name.clone(), sender.clone(), super::role::Role::Consort));  //TODO sheriff!
        }

        let game = Self{
            players,
            graves: Vec::new(),
            phase_machine: PhaseStateMachine::new(settings.phase_times.clone()),
            settings,

            player_on_trial: None,
            trials_left: 0,
        };

        //send to players all game information stuff
        let player_names: Vec<String> = game.players.iter().map(|p|{return p.name.clone()}).collect();
        game.send_to_all(ToClientPacket::Players { names: player_names });
        for player in game.players.iter(){
            player.send(ToClientPacket::PlayerButtons { buttons: 
                PlayerButtons::from(&game, player.index)
            });
        }

        //start clock TODO
        //call phase tick stuff
        // tokio::spawn(||{
        //     game.phase_machine
        // })
        
        game
    }

    pub fn get_current_phase(&self) -> PhaseType {
        self.phase_machine.current_state
    }

    pub fn reset(&mut self, phase: PhaseType){
        match phase {
            PhaseType::Morning => {
                self.player_on_trial = None;
                self.trials_left = 3;
            },
            PhaseType::Discussion => {},
            PhaseType::Voting => {},
            PhaseType::Testimony => {},
            PhaseType::Judgement => {},
            PhaseType::Evening => {},
            PhaseType::Night => {},
        }
    }

    pub fn on_client_message(&mut self, player_index: PlayerIndex, incoming_packet : ToServerPacket){

    }
    pub fn send_to_all(&self, packet: ToClientPacket){
        for player in self.players.iter(){
            player.send(packet.clone());
        }
    }
    
}