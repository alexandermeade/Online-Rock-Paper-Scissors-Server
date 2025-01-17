use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct Player {
    stream: Arc<Mutex<TcpStream>>,  // Wrap TcpStream in Arc<Mutex>
    addr: SocketAddr,
    name: String
}

impl Player {
    pub fn new(stream: TcpStream, addr: SocketAddr, name: String) -> Player {
        Player {
            stream: Arc::new(Mutex::new(stream)),
            addr,
            name
        }
    }

    pub async fn send_message(&self, message: String) {
        // Lock the stream for mutable access
        let mut stream = self.stream.lock().await;  // Lock the Mutex to get mutable access to TcpStream

        // Now you can mutate the TcpStream (e.g., write to it)
        if let Err(e) = stream.write_all(message.as_bytes()).await {
            eprintln!("Failed to send message to {}: {}", self.addr, e);
            return;
        }
        println!("Message sent successfully to {}!", self.addr);
    }

    pub async fn get_input(&self, message:String) -> String {
        self.send_message(message).await;
        let mut buffer = [0; 512];
        let mut stream = self.stream.lock().await;
        match stream.read(&mut buffer).await {
            Ok(bytes_read) if bytes_read > 0 => {
                let input = String::from_utf8_lossy(&buffer[..bytes_read]).trim().to_string();
                input
            }
            Ok(_) => {
                println!("Client disconnected without sending data.");
                String::from("X")
            }
            Err(e) => {
                eprintln!("Failed to read from client: {}", e);
                String::from("X")
            }
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

