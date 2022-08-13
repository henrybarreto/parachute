use log::{debug, error};
use std::io::{Error, Read, Write};
use std::rc::Rc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

use crate::action::Action;
use crate::constant::{PARACHUTE_TRUE, PARACHUTE_FALSE};

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
    let mut buffer = vec![0; 1];

    receive(stream.clone(), &mut buffer).await?;
    let action = Action::from_buffer(buffer);
    if let Action::UNKNOWN = action {
        error!("unknown action");
        send(stream.clone(), &[PARACHUTE_FALSE; 1]).await?;
    } else {
        debug!("valid action");
        send(stream.clone(), &[PARACHUTE_TRUE; 1]).await?;
    }

    Ok(action)
}
