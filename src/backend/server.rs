use dioxus::prelude::*;
use std::net::{IpAddr, Ipv4Addr};

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        let conn = rusqlite::Connection::open("domains.db").expect("Failed to open database");

        // Create the "dogs" table if it doesn't already exist
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS domains (
                id INTEGER PRIMARY KEY,
                domain TEXT NOT NULL,
                url TEXT NOT NULL
            );",
        ).unwrap();

        // Return the connection
        conn
    };
}

#[server]
pub async fn resolve_domain(domain: String) -> Result<IpAddr, ServerFnError> {
    Ok(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)))
}
