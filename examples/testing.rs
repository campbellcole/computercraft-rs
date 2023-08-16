use computercraft::Server;

#[macro_use]
extern crate tracing;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

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

    let Some(peripheral) = computer.find_peripheral("monitor_0".into()).await else {
        error!("Computer disconnected or monitor not found!");
        return;
    };

    info!("Connected. Sending monitor requests...");

    // TODO: implement monitor requests
}
