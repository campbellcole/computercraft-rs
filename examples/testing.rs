//! This example does not demonstrate anything, it is only used for testing new features
//! as they are added.

use computercraft::{
    peripheral::IntoWrappedPeripheral,
    wrappers::{monitor::Monitor, shared::color::Color},
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

    let server = Server::listen();

    info!("Server listening, waiting for connection...");

    let computer = server.wait_for_connection().await?;

    info!("Connected. Sending echo requests...");

    for i in 0..5 {
        let received = computer.echo(format!("echo: {i}")).await?;

        info!("Received: {}", received);
    }

    info!("Connecting to monitor...");

    let peripheral = computer.find_peripheral("monitor_0").await?;

    info!("Connected. Sending monitor requests...");

    let monitor: Monitor = peripheral.into_wrapped().await?;

    monitor.set_background_color(Color::Black).await?;
    monitor.set_text_color(Color::White).await?;
    monitor.clear().await?;
    monitor.set_cursor_pos(1, 1).await?;

    for (idx, color) in Color::colors().into_iter().enumerate() {
        monitor.set_background_color(color).await?;
        let x = idx * 2 + 1;

        monitor.set_cursor_pos(x, 1).await?;
        monitor.write("  ").await?;
        monitor.set_cursor_pos(x, 2).await?;
        monitor.write("  ").await?;
    }

    Ok(())
}
