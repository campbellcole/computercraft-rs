use computercraft::{
    wrappers::{printer::Printer, IntoWrappedPeripheral},
    Server,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::listen();

    let computer = server.wait_for_connection().await?;

    let peripheral = computer.find_peripheral("printer_0").await?;

    let printer: Printer = peripheral.into_wrapped().await?;

    if !printer.new_page().await? {
        eprintln!("failed to create a new page!");
    }

    printer.set_page_title("Rust printer testing!").await?;
    printer.set_cursor_pos(1, 1).await?;
    printer.write("Hello from Rust!").await?;

    if !printer.end_page().await? {
        eprintln!("failed to end page!");
    }

    Ok(())
}
