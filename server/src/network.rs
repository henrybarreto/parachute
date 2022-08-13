use log::{debug, error};
use std::io::Error;
use std::rc::Rc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

use crate::action::Action;

pub const POSITIVE: u8 = 1;
pub const NEGATIVE: u8 = 2;

pub async fn send(stream: Rc<Mutex<TcpStream>>, data: &[u8]) -> Result<usize, Error> {
    let mut local = stream.lock().await;

    let wrote = local.write(&data).await?;

    Ok(wrote)
}

pub async fn receive(stream: Rc<Mutex<TcpStream>>, data: &mut [u8]) -> Result<usize, Error> {
    let mut local = stream.lock().await;

    let read = local.read(data).await?;

    Ok(read)
}

/// Landing point to select the action to be performed on Parachute server: download or upload.
/// It receives the action what the client wants to perferm a action on the server to return a boolean that informs if the action
/// was successful or not.
///
/// The first byte represents the action; one means download and two, upload.
pub async fn landing(stream: Rc<Mutex<TcpStream>>) -> Result<Action, Error> {
    let mut buffer = vec![0; 1]; // action buffer.

    receive(stream.clone(), &mut buffer).await?;
    let action = Action::from_buffer(buffer);
    if let Action::UNKNOWN = action {
        error!("unknown action");
        send(stream.clone(), &[NEGATIVE; 1]).await?;
    } else {
        debug!("valid action");
        send(stream.clone(), &[POSITIVE; 1]).await?;
    }

    Ok(action)
}

/// Shutdown a client connection to the server.
pub async fn shutdown(stream: Rc<Mutex<TcpStream>>) -> Result<(), Error> {
    let mut local = stream.lock().await;
    local.shutdown().await?;

    Ok(())
}
