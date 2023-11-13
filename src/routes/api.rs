use lexi_rs::{client::Client, lexi_type::LexiType};

use crate::db::get_entries;

pub async fn entries_get() -> axum::response::Json<Vec<LexiType>> {
    let mut client = match Client::new("127.0.0.1:6969") {
        Ok(c) => c,
        Err(_) => return axum::response::Json(Vec::new()),
    };
    match client.connect().await {
        Ok(()) => {},
        Err(_) => return axum::response::Json(Vec::new()),
    };
    let entries = match get_entries(&mut client).await {
        Ok(e) => e,
        Err(_) => Vec::new(),
    };
    axum::response::Json(entries)
}
