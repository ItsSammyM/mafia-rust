use std::collections::HashMap;

use tokio_tungstenite::tungstenite::Message;
use serde::{Deserialize, Serialize};

use crate::game::player::PlayerID;

#[derive(Serialize, Debug)]
pub enum ToClientPacket{

    //#region h
    AcceptJoin,
    AcceptHost,
    //#endregion
    
    //#region Lobby
    GameStarted,
    Players{
        names: HashMap<PlayerID, String>
    },
    Kicked,

    PhaseTimesSetting,
    RoleList,
    InvestigatorResults,
    //#endregion


    ////////All of these are just for syncronizing variables between the 2 so client can see what their doing
    Phase,   //how much time is left with this
    PlayerOnTrial,

    NewChatMessage,

    YourTarget,
    YourVoting,
    YourJudgement,
    YourWhispering,
    YourRole,
    YourWill,

    ChatGroups,

    PlayerButtons,

    //a way to syncronise the entire game for someone who joined late
    //#endregion
}
impl ToClientPacket {
    pub fn to_json_string(&self)->String{
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Deserialize, Debug)]
pub enum ToServerPacket{

    Join,
    Host,

    //
    SetName{
        name: String
    },
    StartGame,
    Kick,
    SetRoleList,
    SetPhaseTimes,
    SetInvestigatorResults,

    //
    Vote,   //Accusation
    Target,
    DayTarget,
    Judgement,  //Vote
    Whisper,
    SendMessage,
    SaveWill,
}
