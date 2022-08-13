use std::io::Error;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut stream = TcpStream::connect("localhost:14014").await?;

    stream.write(&[2]).await?; // send action upload.

    let mut a = vec![0 as u8; 1];
    stream.read(&mut a).await?; // read if action is valid.
    println!("ACTION UPLOAD: {:?}", a);

    let mut file = File::open("example.txt").await?;
    let metadata = file.metadata().await?;

    let size = metadata.len();
    let size_buffer = size.to_be_bytes();
    println!("SIZE_BUFFER: {:?}", size_buffer);
    println!("SIZE: {:?}", size);

    stream
        .write(&[
            0,
            1,
            0,
            size_buffer[0],
            size_buffer[1],
            size_buffer[2],
            size_buffer[3],
            size_buffer[4],
            size_buffer[5],
            size_buffer[6],
            size_buffer[7],
        ])
        .await?; // send client version and file size.
    let mut b = vec![0 as u8; 1];
    stream.read(&mut b).await?; // read server response from version and file size.
    println!("VERSION AND FILE SIZE RECEIVE BY SERVER: {:?}", b);

    let mut file_buffer = vec![0 as u8; size as usize];
    file.read(&mut file_buffer).await?;  // read file content locallly.
    stream.write(&file_buffer).await?; // send file to server.

    let mut uuid_buffer = [0 as u8; 16];
    stream.read(&mut uuid_buffer).await?; // read uuid from server.

    println!("UUID FROM THE SERVER: {}", Uuid::from_bytes(uuid_buffer));

    Ok(())
}
