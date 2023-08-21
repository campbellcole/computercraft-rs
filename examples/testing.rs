//! This example does not demonstrate anything, it is only used for testing new features
//! as they are added.

use computercraft::{
    peripheral::IntoWrappedPeripheral,
    wrappers::{
        ap::{colony_integrator::ColonyIntegrator, rs_bridge::RsBridge},
        monitor::Monitor,
        shared::color::Color,
    },
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

    info!("Connecting to peripheral...");

    let peripheral = computer.find_peripheral("left").await?;

    let rs: RsBridge = peripheral.into_wrapped().await?;

    let items = rs.list_items().await?;

    println!("{items:?}");

    // let colony: ColonyIntegrator = peripheral.into_wrapped().await?;

    // let res = colony.get_citizens().await?;

    // println!("{res:?}");

    Ok(())
}
