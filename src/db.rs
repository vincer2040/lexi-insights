use lexi_rs::{client::Client, lexi_type::LexiType};

pub enum DbError {
    None,
    Error(anyhow::Error),
}

pub async fn get_entries(client: &mut Client) -> Result<Vec<LexiType>, DbError> {
    let entries = match client.entries().await {
        Ok(e) => e,
        Err(err) => return Err(DbError::Error(err)),
    };
    match entries {
        LexiType::BulkString(_) => Err(DbError::Error(anyhow::anyhow!(
            "invalid type returned from entries"
        ))),
        LexiType::Array(vals) => Ok(vals),
        LexiType::Int(_) => Err(DbError::Error(anyhow::anyhow!(
            "invalid type returned from entries"
        ))),
        LexiType::Simple(s) => {
            if s == "NONE" {
                return Err(DbError::None);
            } else {
                Err(DbError::Error(anyhow::anyhow!(
                    "invalid type returned from entries"
                )))
            }
        }
        LexiType::Error(err) => Err(DbError::Error(anyhow::anyhow!(err))),
    }
}
