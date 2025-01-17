use crate::player::Player;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::net::SocketAddr;
use tokio::sync::Mutex;  // Use Tokio's async Mutex
use std::sync::Arc;




#[derive(Clone, Debug)]
pub struct GameSession {
    player1: Arc<Mutex<Player>>,
    player2: Arc<Mutex<Player>>,
    player1Turn: bool,
    victor: bool,
    player1Win: bool,
    id: u32
}

impl GameSession {
    pub fn new(player1: Arc<Mutex<Player>>, player2: Arc<Mutex<Player>>, id: u32) -> GameSession {
        GameSession {
           player1: player1,
           player2: player2,
           player1Turn: true,
           victor: false,
           player1Win: false,
           id
        }
    }

    pub async fn message_all(&self, message:String) {

        println!("message all players");
        let player1 = self.player1.lock().await;
        let player2 = self.player2.lock().await;
        
        let player1 = player1.clone();
        let player2 = player2.clone();

        let msg1 = message.clone(); 
        let msg2 = message.clone();
        tokio::join!(
            async {
                player1.send_message(msg1).await;
            },
            async {
                player2.send_message(msg2).await;
            }
        );
    }
    
    fn reverse_tuple<T, U>(t: (T, U)) -> (U, T) {
        (t.1, t.0)
    }

    fn winner_decal(input1:char, input2:char) -> String {

        if input1 == input2 {
            return format!("{} tie {}", input1, input2);
        }
        let result:String = String::new();
        match (input1, input2) {
            //unique winning matches            
            ('r', 's') =>  {return format!("player1 {} > {} player2", input1, input2)},
            ('s', 'p') =>  {return format!("player1 {} > {} player2", input1, input2)},
            ('p', 'r') => {return format!("player1 {} > {} player2", input1, input2)}, 
            ('r', 'p') => {return format!("player1 {} < {} player2", input1, input2)},
            ('p', 's') => {return format!("player1 {} < {} player2", input1, input2)},
            ('s', 'r') => {return format!("player1 {} < {} player2", input1, input2)},
            ('r', 'p') => {return format!("player1 {} < {} player2", input1, input2)},
            (_ , 'x') => {return format!("player1 {} > {} player2", input1, input2)},
            (_, _) => return format!(""),
            _ => {return format!("")}
        };
       
        return format!("");
    }

    pub async fn start_game(&mut self) {
        let player1 = self.player1.lock().await;
        let player2 = self.player2.lock().await;
        player1.send_message(format!("you have been connected to {}", player2.name())).await;
        player2.send_message(format!("you have been connected to {}", player1.name())).await;

        while !self.victor {
            player2.send_message(format!("player1 is making their turn")).await;
      
            // Use `tokio::join` to gather inputs concurrently
            let (player1_input, player2_input) = tokio::join!(
                player1.get_input("Input R,r for Rock, S,s for Scissors, P,p for Paper, or X to disconnect:".to_string()),
                player2.get_input("Input R,r for Rock, S,s for Scissors, P,p for Paper, or X to disconnect:".to_string())
            );
            self.message_all(format!("{}"), winner_decal(player1));
        }
    }
}
