use scylla::{DeserializeRow, SessionBuilder};
use std::net::IpAddr;

#[derive(DeserializeRow)]
struct MyRow {
    address: IpAddr,
    username: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let session = SessionBuilder::new()
        .known_node("127.0.0.1:9042")
        .build()
        .await
        .expect("Connection error");

    // mesmo que eu não tenha parametro devo passar como segundo argumento uma tupla vazia
    let result = session
        .query_unpaged("SELECT address, username FROM system.clients", ())
        .await?
        .into_rows_result()?;

    // Tipando a volta do resultado
    for row in result.rows::<MyRow>()? {
        let my_row = row?;

        println!("IP: {} for {}!", my_row.address, my_row.username);
    }

    Ok(())
}
