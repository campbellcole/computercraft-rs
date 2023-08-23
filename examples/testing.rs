//! This example does not demonstrate anything, it is only used for testing new features
//! as they are added.

use computercraft::{
    wrappers::{monitor::Monitor, shared::color::Color, IntoWrappedPeripheral},
    Server,
};
use tracing_subscriber::prelude::*;

#[macro_use]
extern crate tracing;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_line_number(true)
                .with_file(true)
                .with_target(false),
        )
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let server = Server::listen_on("0.0.0.0:3389");

    info!("Server listening, waiting for connection...");

    let computer = server.wait_for_connection_from("testing").await?;

    let peripheral = computer.find_peripheral("monitor_0").await?;

    let monitor: Monitor = peripheral.into_wrapped().await?;

    monitor.set_background_color(Color::Black).await?;
    monitor.set_text_color(Color::White).await?;
    monitor.clear().await?;
    monitor.set_cursor_pos(1, 1).await?;

    for color in Color::colors() {
        monitor.set_background_color(color).await?;
        monitor.write(" ").await?;
    }

    Ok(())
}
