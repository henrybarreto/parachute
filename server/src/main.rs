pub mod action;
pub mod network;
pub mod upload;
pub mod constant;

use log::{debug, error, info};
use simple_logger::SimpleLogger;
use std::io::Error;
use std::rc::Rc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use action::Action;
use constant::{PARACHUTE_SERVER_ADDRESS};

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().init().unwrap();

    let address = PARACHUTE_SERVER_ADDRESS;
    info!("starting Parachute server");
    info!("version: {}", env!("CARGO_PKG_VERSION"));

    let listener = TcpListener::bind(address).await.unwrap();
    info!("listening on {address}");
    loop {
        let (stream, address) = listener.accept().await.unwrap();

        info!("new connection from {address}");
        let connection = Rc::new(Mutex::new(stream));
        let action = network::landing(connection.clone()).await;
        if action.is_err() {
            error!("error landing");

            continue;
        }
        debug!("action got");

        match action.unwrap() {
            Action::DOWNLOAD => {
                info!("download action");

                continue;
            }
            Action::UPLOAD => {
                info!("upload action");
                let bootstraped = upload::bootstrap(connection.clone()).await;
                if bootstraped.is_err() {
                    error!("error bootstraping");

                    continue;
                }

                let (version, size) = bootstraped.unwrap();
                debug!("{version} and {size}");

                let uploaded = upload::upload(connection.clone(), size).await.unwrap();
                if uploaded {
                    info!("upload successful");
                } else {
                    info!("upload failed");
                }

                continue;
            }
            Action::UNKNOWN => {
                info!("unknown action");

                continue;
            }
        }
    }
}
