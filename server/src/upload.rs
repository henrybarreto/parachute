use log::{debug, error, info};
use std::io::Error;
use std::rc::Rc;
use std::vec;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::database::{connect, save};
use crate::network;

pub const FILE_LIMIT_SIZE: u64 = 10485760;

/// Bootstaps a single Parachute upload.
/// It receives the client's version, the file's size and flags to return the number of chucks that will be sent.
///
/// The first three bytes compose the client's version. The first one is the major version, the next byte is the minor
/// and the last byte is the patch version. After the version, it receives the file's size inside the next eithe bytes.
/// The flags occupies the three bytes which are password, times and address, respectively.
///
/// - The `password` flag is one if the file is password is required to download the file, zero otherwise.
/// - The `times` flag is one if the file has a limited times what can be downloaded, zero otherwise.
/// - The `address` flag is one if the file only can be downloaded from the address specified, zero otherwise.
///
/// When any flags are one, the space next from three bytes occupied by flag is used to store password's value, times's
/// value and address's value, respectively. The password's value is a string with a maximum of 12 characters, the
/// time's value is a 4 bytes integer and the address's value is a string in IP format.
// TODO: add a hash of the file to verify the integrity of the file.
// TODO: add flags feature.
pub async fn bootstrap(stream: Rc<Mutex<TcpStream>>) -> Result<(u64, u64), Error> {
    let mut buffer = vec![0; 11]; // 3 bytes for version, 8 bytes for size.

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

        let size_buffer = [
            buffer[3], buffer[4], buffer[5], buffer[6], buffer[7], buffer[8], buffer[9], buffer[10],
        ];

        let size = u64::from_be_bytes(size_buffer);
        debug!("file size size: {size}");
        /*
        NOTICE: The flags are not implemented yet.
        NOTICE: code generated by GitHub copilot.

        let flags = u64::from_be_bytes(buffer);
        let password = flags & 0x01 == 0x01;
        let times = flags & 0x02 == 0x02;
        let address = flags & 0x04 == 0x04;

        let mut password_buffer = [0; 12];
        let mut times_buffer = [0; 4];
        let mut address_buffer = [0; 4];
        if password {
            stream.read_exact(&mut password_buffer)?;
        }
        if times {
            stream.read_exact(&mut times_buffer)?;
        }
        if address {
            stream.read_exact(&mut address_buffer)?;
        }

        let password = String::from_utf8(password_buffer.to_vec()).unwrap();
        let times = u32::from_be_bytes(times_buffer);
        let address = String::from_utf8(address_buffer.to_vec()).unwrap();
        */

        let version = (version_major * 100 + version_minor * 10 + version_patch) as u64;
        debug!("client version stamp: {version}");

        //stream.try_write(&[TRUE; 1])?;
        network::send(stream.clone(), &[network::POSITIVE; 1]).await?;

        Ok((version, size))
    }
}

/// Uploads a single file to Parachute.
/// Uploads the file within FILE_LIMIT_SIZE from a client and saves it into the database to return a boolean that informs if the upload was
/// successful or not.
// TODO: handle upload by chunks.
pub async fn upload(stream: Rc<Mutex<TcpStream>>, size: u64) -> Result<bool, Error> {
    if size > FILE_LIMIT_SIZE {
        error!("file size is too big");

        Ok(false)
    } else {
        let mut buffer = vec![0; size as usize]; // file buffer.

        let read = network::receive(stream.clone(), &mut buffer).await;
        if read.is_err() {
            error!("error reading file");
            let wrote = network::send(stream.clone(), &[0; 16]).await;
            if wrote.is_err() {
                error!("error writing response from error reading file");
            }

            debug!("error reading file");
            Ok(false)
        } else {
            let uuid = &Uuid::new_v4().to_string();
            debug!("file uuid: {uuid}");

            info!("saving file to database");
            let connection = connect().await; // TODO: unwrap is not safe.
            if connection.is_err() {
                panic!("error connecting to database");
            }
            let wrote = save(&mut connection.unwrap(), uuid, buffer).await;
            if wrote.is_err() {
                error!("error saving file into redist database");
                Ok(false)
            } else {
                let wrote = network::send(stream.clone(), uuid.as_bytes()).await;
                if wrote.is_err() {
                    error!("error writing response from reading file");
                    Ok(false)
                } else {
                    info!("file saved successfully");
                    Ok(true)
                }
            }
        }
    }
}
