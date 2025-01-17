use crate::player::Player;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::net::SocketAddr;
use tokio::sync::Mutex;  // Use Tokio's async Mutex
use std::sync::Arc;

use crate::gamesession::GameSession;


#[derive(Clone, Debug)]
pub struct Game {
    pool: Vec<Arc<Mutex<Player>>>,
    sessions: Vec<GameSession>,
    sessionCounter: u32
}

impl Game {
    pub fn new() -> Game {
        Game {
            pool: Vec::new(),
            sessions: Vec::new(),
            sessionCounter: 0
        }
    }

    pub async fn send_all(&mut self, message:String) {
    // Clone the message to avoid lifetime issues
        let message = message.clone();

        // Use `tokio::spawn` to send the message to each player
        for player in &self.pool {
            let player_clone = player.clone();
            let message_clone = message.clone(); // Clone the message for each async block
            tokio::spawn(async move {
                // Lock the player inside the async block
                let mut p = player_clone.lock().await;  // Use async lock
                p.send_message(message_clone).await; // Pass the cloned message
            });
        }
    }

    pub fn create_sessions(&mut self) {

        if self.pool.len() < 2 {
            return;
        }

        while self.pool.len() >= 2 {
            let player1 = &self.pool.remove(0);
            let player2 = &self.pool.remove(0);
            self.sessionCounter += 1;
            let gs = GameSession::new(Arc::clone(player1), Arc::clone(player2), self.sessionCounter);

            self.sessions.push(gs);
        } 
        let len = self.sessions.len()-1;
        let mut gs = self.sessions[len].clone();
        println!("{:#?}", self);

        tokio::spawn(async move{
            gs.start_game().await;
        });
    }

    pub async fn add_player(&mut self, player: Arc<Mutex<Player>>) {
        println!("Player joined!");
        
        // Clone the Arc before passing it into the async block
        let player_clone = player.clone();
        
        let message = format!("Player {} has Joined!1", player_clone.lock().await.name());
        let mut game = self.clone();        
        /*tokio::spawn(async move{ 
            game.send_all(message).await;
        });*/

        tokio::spawn(async move {
            // Lock the player inside the async block
            let mut p = player_clone.lock().await;  // Use async lock
            p.send_message(String::from("Hello, Welcome to Rock Paper Scissors!")).await;
        });

        // Push the player into the pool (no need to clone again here)
        self.pool.push(player);
        println!("{:#?}", self.pool);
        println!("Player Count: {}", self.pool.len());
        
        if self.pool.len() >= 2 {
            self.create_sessions();
        }
    }
}

