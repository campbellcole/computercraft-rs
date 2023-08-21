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

    let server = Server::listen_on("0.0.0.0:3389");

    info!("Server listening, waiting for connection...");

    let computer = server.wait_for_connection().await?;

    info!("Connected. Sending echo requests...");

    let chars_100kb = [b'x'; 100 * 1024].to_vec();
    // let chars_128kb = [b'x'; 128 * 1024].to_vec();
    // let chars_200kb = [b'x'; 200 * 1024].to_vec();

    computer
        .echo(String::from_utf8(chars_100kb).unwrap())
        .await?;
    // computer
    //     .echo(String::from_utf8(chars_128kb).unwrap())
    //     .await?;
    // computer
    //     .echo(String::from_utf8(chars_200kb).unwrap())
    //     .await?;

    Ok(())
}
