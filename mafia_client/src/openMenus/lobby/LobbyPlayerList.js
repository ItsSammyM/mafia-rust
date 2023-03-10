import React from "react";
import gameManager from "../../index.js";
import "./lobbyMenu.css";

export class LobbyPlayerList extends React.Component {
    constructor(props) {
        super(props);

        this.state = {            
            name: "",

            // Player list
            gameState: gameManager.gameState
        };
        this.listener = ()=>{
            this.setState({
                gameState: gameManager.gameState
            });
        }
    }
    componentDidMount() {
        gameManager.addStateListener(this.listener);
    }
    componentWillUnmount() {
        gameManager.removeStateListener(this.listener);
    }
    
    render(){return(<div className="lm-player-list-pane">
        {this.renderName()}
        {this.renderPlayers()}
    </div>)}
 
    renderName(){return(<div className="input-box lm-name-box">
        <input className="input-field" type="text" value={this.state.name}
            onChange={(e)=>{this.setState({name: e.target.value})}}
            onKeyUp={(e)=>{
                if(e.key === 'Enter')
                    gameManager.setName_button(this.state.name);
            }}
        />
        <button className="button" onClick={()=>{
            gameManager.setName_button(this.state.name)
        }}>Set Name</button>
    </div>)}

    renderPlayers(){return(<div>
        {this.state.gameState.players.map((player, i)=>{
            return(<div key={i}>{i+1}:{player.name}<br/></div>)
        })}
    </div>)}
}