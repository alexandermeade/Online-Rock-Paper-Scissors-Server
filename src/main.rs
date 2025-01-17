use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

mod game;
mod player;
mod gamesession;

async fn get_name(stream: &mut TcpStream) -> String {
    let mut buffer = [0; 512];

    match stream.read(&mut buffer).await {
        Ok(bytes_read) if bytes_read > 0 => {
            let name = String::from_utf8_lossy(&buffer[..bytes_read]).trim().to_string();

            let response = format!("Welcome to Rock Paper Scissors, {}!\n", name);
            if let Err(e) = stream.write_all(response.as_bytes()).await {
                eprintln!("Failed to send greeting to {}: {}", name, e);
            }
            return name;
        }
        Ok(_) => {
            println!("Client disconnected without sending data.");
            String::from("\0")
        }
        Err(e) => {
            eprintln!("Failed to read from client: {}", e);
            String::from("\0")
        }
    }
}


#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    
    let mut game = Arc::new(Mutex::new(game::Game::new()));

    loop {
        match listener.accept().await {
            Ok((mut stream, addr)) => {

                let mut game_clone = Arc::clone(&game);
                tokio::spawn(async move { 
                    let mut game_arc = game_clone.lock().await;
                    println!("New connection from {}", addr);
                    
                    let name = get_name(&mut stream).await;
                    if name != "\0" {
                        game_arc.add_player(Arc::new(Mutex::new(player::Player::new(stream, addr, name)))).await;
                    }
                });
//                tokio::spawn(async move {
//                    handle_client(stream).await;
//                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}

