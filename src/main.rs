mod show_interface;
mod show_logic;
mod tosu_client;
mod web_overlay;

use anyhow::Result;
use serde_json::json;
use show_interface::OscSender;
use web_overlay::WebOverlay;

#[tokio::main]
async fn main() -> Result<()> {
    let osc = OscSender::new("127.0.0.1:9000")?;

    let event_definitions_message = json!({
        "type": "event_definitions",
        "events": show_logic::osu_events(),
    })
    .to_string();

    let overlay = WebOverlay::new(vec![event_definitions_message]);
    let web_sender = overlay.sender();

    tokio::spawn(async move {
        if let Err(error) = overlay.run("127.0.0.1:3001").await {
            eprintln!("Web-Overlay Fehler: {}", error);
        }
    });

    tosu_client::watch_tosu(&osc, web_sender).await?;

    Ok(())
}
