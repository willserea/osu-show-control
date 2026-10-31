use anyhow::Result;
use futures_util::SinkExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio_tungstenite::accept_async;

pub struct WebOverlay {
    sender: broadcast::Sender<String>,
    initial_messages: Vec<String>,
}

impl WebOverlay {
    pub fn new(initial_messages: Vec<String>) -> Self {
        let (sender, _) = broadcast::channel(100);

        Self {
            sender,
            initial_messages,
        }
    }

    pub fn sender(&self) -> broadcast::Sender<String> {
        self.sender.clone()
    }

    pub async fn run(&self, address: &str) -> Result<()> {
        let listener = TcpListener::bind(address).await?;
        println!("Web-Overlay WebSocket läuft auf ws://{}", address);

        loop {
            let (stream, _) = listener.accept().await?;
            let receiver = self.sender.subscribe();
            let initial_messages = self.initial_messages.clone();

            tokio::spawn(async move {
                if let Err(error) = handle_connection(stream, receiver, initial_messages).await {
                    eprintln!("WebSocket-Client Fehler: {}", error);
                }
            });
        }
    }
}

async fn handle_connection(
    stream: TcpStream,
    mut receiver: broadcast::Receiver<String>,
    initial_messages: Vec<String>,
) -> Result<()> {
    let mut websocket = accept_async(stream).await?;

    for message in initial_messages {
        websocket.send(message.into()).await?;
    }

    while let Ok(message) = receiver.recv().await {
        websocket.send(message.into()).await?;
    }

    Ok(())
}
