use computercraft::{
    peripheral::IntoWrappedPeripheral,
    wrappers::{monitor::Monitor, shared::color::Color},
    Server,
};

#[tokio::main]
async fn main() {
    let server = Server::listen();

    let computer = server.wait_for_connection().await;

    let peripheral = computer.find_peripheral("monitor_0").await.unwrap();

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
