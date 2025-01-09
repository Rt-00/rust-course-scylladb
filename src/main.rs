use scylla::{DeserializeRow, SerializeRow, SessionBuilder};

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

static INSERT_MESSAGE_QUERY: &str = r#"
    INSERT INTO messaging.messages (channel_id, message_id, author, content) VALUES (?, ?, ?, ?);
"#;

static SELECT_MESSAGE_QUERY: &str =
    "SELECT channel_id, message_id, author, content FROM messaging.messages;";

static CURRENT_KEYSPACE: &str = "messaging";

// Preciso implementar o SerializeRow para conseguir fazer o bind da minha struct para a minha query
// Preciso implementar o DeserializeRow para conseguir fazer o bind da minha query para minha struct

// SerializeRow: Struct -> Query -> Insert no Banco
// DeserializeRow: Select no Banco -> Query -> Struct

// SerializeRow: Serializa a Struct para a query e faz o insert
// DeserializeRow: Deserializa a select query para a minha struct
#[derive(SerializeRow, DeserializeRow)]
struct Message {
    channel_id: i32,
    message_id: i32,
    author: String,
    content: String,
}

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
    session.use_keyspace(CURRENT_KEYSPACE, true).await?;

    // Insert date in messages table
    let message = Message {
        channel_id: 1,
        message_id: 1,
        author: "rtoledo".to_string(),
        content: "salves!".to_string(),
    };
    session.query_unpaged(INSERT_MESSAGE_QUERY, message).await?;

    let rows_result = session
        .query_unpaged(SELECT_MESSAGE_QUERY, ())
        .await?
        .into_rows_result()?;

    for row in rows_result.rows::<Message>()? {
        let message: Message = row?;

        println!("{}: {}", message.author, message.content);
    }

    Ok(())
}
