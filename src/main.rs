pub mod server;

use clap::{App, SubCommand};
use log::trace;
use simple_logger::SimpleLogger;
use std::io::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().init().unwrap();

    let matches = App::new("parachute")
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand_required(true)
        .subcommand(SubCommand::with_name("server").about("Starts a Parachute server"))
        .subcommand(SubCommand::with_name("client").about("Starts a Parachute client"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("server") {
        trace!("init a Parachute server was selected");

        server::serve().await?
    } else if let Some(_) = matches.subcommand_matches("client") {
        // TODO: implement a Parachute client.
        // There are a code's example created in the client module.
        todo!()
    }

    Ok(())
}
