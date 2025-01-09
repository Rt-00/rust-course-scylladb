use scylla::SessionBuilder;

static CREATE_KEYSPACE: &str = r#"
CREATE KEYSPACE IF NOT EXISTS messaging
    WITH replication = {
        'class': 'NetworkTopologyStrategy',
        'replication_factor': 3
    }
    AND durable_writes = true
    AND tablets = {'enabled': true};
"#;

static CREATE_MESSAGES_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS messaging.messages (
   channel_id int,
   message_id int,
   author text,
   content text,

   PRIMARY KEY (channel_id, message_id)
);
"#;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let session = SessionBuilder::new()
        .known_node("127.0.0.1:9042")
        .build()
        .await
        .expect("Connection error");

    // Create "messaging" KeySpace.
    session.query_unpaged(CREATE_KEYSPACE, ()).await?;
    // Create "messages" Table.
    session.query_unpaged(CREATE_MESSAGES_TABLE, ()).await?;

    // Use "messaging" as default Keyspace.
    session.use_keyspace("messaging", true).await?;

    // Insert date in messages table
    let insert_query = "INSERT INTO messages (channel_id, message_id, author, content) VALUES (1, 1, 'rtoledo', 'hello');";
    session.query_unpaged(insert_query, ()).await?;

    Ok(())
}
