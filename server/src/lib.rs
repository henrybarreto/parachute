use std::{net::{TcpListener, TcpStream, Shutdown}, io::{Error, Read, Write}};

async fn send(mut stream: TcpStream, data: Vec<u8>) -> Result<i32, Error> {
    Ok(0)
}

async fn receive(mut stream: TcpStream) -> Result<Vec<u8>, Error> {
    Ok(vec![])
}

enum Action {
    DOWNLOAD = 1,
    UPLOAD = 2
}

impl Action {
    fn from_u8(value: u8) -> Action {
        match value {
            1 => Action::DOWNLOAD,
            2 => Action::UPLOAD,
            _ => Action::DOWNLOAD // TODO: panic
        }
    }
}

async fn landing(mut stream: TcpStream) -> Result<Action, Error> {
    let mut data = [0; 1];
    stream.read_exact(&mut data)?; 

    Ok(Action::from_u8(data[0]))
}

// Size of each chuck of data to be receives from the client to a file.
const CHUNK_SIZE: u64 = 1024;

/// Bootstaps a single Parachute upload. 
/// It receives the client's version, the file's size and flags to return the number of chucks that will be sent.
///
/// It receives the client's version and the file's size to the client. The first three bytes compose the client's 
/// version. The first one is the major version, the next byte is the minor and the last byte is the patch version.
/// After the version, it receives the file's size inside the next eithe bytes. The flags occupies the three bytes 
/// which are password, times and address, respectively. 
///
/// - The `password` flag is one if the file is password is required to download the file, zero otherwise.
/// - The `times` flag is one if the file has a limited times what can be downloaded, zero otherwise.
/// - The `address` flag is one if the file only can be downloaded from the address specified, zero otherwise.
///
/// When any flags are one, the space next from three bytes occupied by flag is used to store password's value, times's
/// value and address's value, respectively. The password's value is a string with a maximum of 12 characters, the 
/// time's value is a 4 bytes integer and the address's value is a string in IP format.
// TODO: add a hash of the file to verify the integrity of the file.
async fn bootstrap(mut stream: TcpStream) -> Result<(u64, u64), Error> {
    /*let mut buffer = [0; 12];
    stream.read_exact(&mut buffer)?;

    let _version = [buffer[0], buffer[1], buffer[2]];

    let size = u64::from_be_bytes(buffer[3..11].try_into().unwrap());
    let mut chucks: u64;
    if size > CHUNK_SIZE {
        chucks = size / CHUNK_SIZE;
        if size % CHUNK_SIZE != 0 {
            chucks += 1;
        }
    } else {
        chucks = 1;
    }

    stream.write_all(&chucks.to_be_bytes())?;

    Ok((size,chucks))*/

    Ok((0, 0))
}

async fn handle(mut stream: TcpStream) -> Result<(), Error> {

    Ok(())
}

async fn serve(address: &str, port : u16) {
    let listener = TcpListener::bind(format!("{address}:{port}")).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                //bootstrap(stream);
                //handle(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        // TODO: close connection when the action is concluded.
    }

}
