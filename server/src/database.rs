use redis::Commands;

pub const DATABASE_ADDRESS: &str = "redis://localhost:6379";

pub async fn connect() -> Result<redis::Connection, redis::RedisError> {
    let client = redis::Client::open(DATABASE_ADDRESS)?;

    Ok(client.get_connection()?)
}

pub async fn save(
    connection: &mut redis::Connection,
    uuid: &str,
    data: Vec<u8>,
) -> Result<(), redis::RedisError> {
    connection.set(uuid, data)?;

    Ok(())
}

pub async fn restore(
    connection: &mut redis::Connection,
    uuid: &str,
) -> Result<Vec<u8>, redis::RedisError> {
    let data = connection.get(uuid)?;

    Ok(data)
}
