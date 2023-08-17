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
async fn main() {
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

    let computer = server.wait_for_connection().await;

    info!("Connected. Sending echo requests...");

    for i in 0..5 {
        let Some(received) = computer.echo(format!("echo: {i}")).await else {
            error!("Computer disconnected before we received a response!");
            return;
        };

        info!("Received: {}", received);
    }

    info!("Connecting to monitor...");

    let Some(peripheral) = computer.find_peripheral("monitor_0").await else {
        error!("Computer disconnected or monitor not found!");
        return;
    };

    info!("Connected. Sending monitor requests...");

    let monitor: Monitor = peripheral.into_wrapped().await.unwrap();

    monitor.set_background_color(Color::Black).await;
    monitor.set_text_color(Color::White).await;
    monitor.clear().await;
    monitor.set_cursor_pos(1, 1).await;

    for color in Color::colors() {
        monitor.set_background_color(color).await;
        monitor.write(" ").await;
    }
}
