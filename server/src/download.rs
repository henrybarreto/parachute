use log::{debug, error, info};
use std::io::Error;
use std::rc::Rc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::database::{connect, restore};
use crate::network;

pub async fn bootstrap(stream: Rc<Mutex<TcpStream>>) -> Result<u64, Error> {
    // version and uuid buffer.
    // 3 bytes for version, 16 bytes for uuid.
    let mut buffer = [0 as u8; 19];

    let read = network::receive(stream.clone(), &mut buffer).await;
    if read.is_err() {
        let err = read.err().unwrap();
        error!("error reading version: {}", err);
        network::send(stream.clone(), &[network::NEGATIVE; 1]).await?;

        Err(err)
    } else {
        let version_major = buffer[0];
        let version_minor = buffer[1];
        let version_patch = buffer[2];
        debug!("client version: {version_major}.{version_minor}.{version_patch}");

        let version = (version_major * 100 + version_minor * 10 + version_patch) as u64;
        debug!("client version stamp: {version}");

        network::send(stream.clone(), &[network::POSITIVE; 1]).await?;

        Ok(version)
    }
}

pub async fn download(stream: Rc<Mutex<TcpStream>>) -> Result<bool, Error> {
    // version and uuid buffer.
    // 3 bytes for version, 16 bytes for uuid.
    let mut buffer = [0 as u8; 16];

    let read = network::receive(stream.clone(), &mut buffer).await;
    if read.is_err() {
        let err = read.err().unwrap();
        error!("error reading uuid: {}", err);
        network::send(stream.clone(), &[0; 8]).await?; // send a empty u64.

        Err(err)
    } else {
        let uuid = Uuid::from_bytes(buffer);
        debug!("client uuid: {uuid}");

        let connection = connect().await;
        if connection.is_err() {
            panic!("error connecting to database");
        }

        let data = restore(&mut connection.unwrap(), &uuid.to_string()).await;
        if data.is_err() {
            network::send(stream.clone(), &[0; 8]).await?; // send a empty u64.

            return Ok(false);
        } else {
            let file = data.unwrap();
            let size = file.len() as u64;
            debug!("file size: {size}");

            let wrote = network::send(stream.clone(), &size.to_be_bytes()).await;
            if wrote.is_err() {
                let err = wrote.err().unwrap();
                error!("error sending file size to client: {}", err);
                network::send(stream.clone(), &[0; 8]).await?; // send a empty u64.

                Ok(false)
            } else {
                let mut buffer = [0 as u8, 1];
                let read = network::receive(stream.clone(), &mut buffer).await;
                if read.is_err() {
                    let err = read.err().unwrap();
                    error!("error reading file size response: {}", err);

                    Ok(false)
                } else {
                    let accepted = buffer[0] == 1;
                    if accepted {
                        debug!("client accepted: {accepted}");
                        let wrote = network::send(stream.clone(), &file).await;
                        if wrote.is_err() {
                            let err = wrote.err().unwrap();
                            error!("error sending file: {}", err);

                            Ok(false)
                        } else {
                            info!("file sent: {uuid}");

                            Ok(true)
                        }
                    } else {
                        debug!("client rejected file");

                        Ok(false)
                    }
                }
            }
        }
    }
}
