use anyhow::Result;
use futures_util::StreamExt;
use serde_json::Value;
use serde_json::json;
use tokio::net::TcpStream;
use tokio::sync::broadcast;
use tokio::time::{Duration, sleep};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

use crate::show_interface::OscSender;
use crate::show_logic::ShowLogic;

pub async fn watch_tosu(osc: &OscSender, web_sender: broadcast::Sender<String>) -> Result<()> {
    let url = "ws://127.0.0.1:24050/websocket/v2";

    loop {
        println!("Verbinde mit tosu: {}", url);

        match connect_async(url).await {
            Ok((ws_stream, _)) => {
                println!("Mit tosu verbunden: {}", url);

                let result = read_tosu_stream(ws_stream, osc, web_sender.clone()).await;

                if let Err(error) = result {
                    eprintln!("tosu Verbindung verloren: {}", error);
                }
            }
            Err(error) => {
                eprintln!("tosu nicht erreichbar: {}", error);
            }
        }

        println!("Neuer Versuch in 3 Sekunden...");
        sleep(Duration::from_secs(3)).await;
    }
}

async fn read_tosu_stream(
    ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    osc: &OscSender,
    web_sender: broadcast::Sender<String>,
) -> Result<()> {
    let (_, mut read) = ws_stream.split();

    let mut show_logic = ShowLogic::new();

    while let Some(message) = read.next().await {
        let message = message?;

        if !message.is_text() {
            continue;
        }

        let text = message.to_text()?;
        let json: Value = serde_json::from_str(text)?;

        let events = show_logic.handle_tosu_update(&json);

        for event in events {
            println!("ShowEvent: {} {}", event.address, event.value);

            osc.send_string(&event.address, &event.value)?;

            let event_json = serde_json::to_string(&event)?;
            let _ = web_sender.send(event_json);
        }
    }

    Ok(())
}
