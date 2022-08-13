use log::{error, info, trace};
use std::io::Error;
use std::rc::Rc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use server::action::Action;
use server::download;
use server::network;
use server::upload;

pub async fn serve() -> Result<(), Error> {
    let address = "localhost:14014";
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
        trace!("action got");

        match action.unwrap() {
            Action::DOWNLOAD => {
                info!("download action");
                let bootstraped = download::bootstrap(connection.clone()).await;
                if bootstraped.is_err() {
                    error!("error bootstraping download");
                    network::shutdown(connection.clone()).await?;

                    continue;
                }
                let version = bootstraped.unwrap();
                info!("client version: {version}");

                let downloaded = download::download(connection.clone()).await.unwrap();
                if downloaded {
                    info!("download successful");
                } else {
                    info!("download failed");
                }

                network::shutdown(connection.clone()).await?;
                continue;
            }
            Action::UPLOAD => {
                info!("upload action");
                let bootstraped = upload::bootstrap(connection.clone()).await;
                if bootstraped.is_err() {
                    error!("error bootstraping upload");
                    network::shutdown(connection.clone()).await?;

                    continue;
                }

                let (version, size) = bootstraped.unwrap();
                info!("client version: {version}; file size: {size}");

                let uploaded = upload::upload(connection.clone(), size).await.unwrap();
                if uploaded {
                    info!("upload successful");
                } else {
                    info!("upload failed");
                }

                network::shutdown(connection.clone()).await?;
                continue;
            }
            Action::UNKNOWN => {
                info!("unknown action");

                network::shutdown(connection.clone()).await?;
                continue;
            }
        }
    }
}
