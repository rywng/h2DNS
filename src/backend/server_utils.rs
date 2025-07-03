use std::env;
pub use rusqlite::params;

thread_local! {
    pub static DB: rusqlite::Connection = {
        let db_name = if cfg!(test) {
            "test_database.db"
        } else {
            "domains.db"
        };

        let conn = rusqlite::Connection::open(db_name)
            .expect("Failed to open database");

        // Create the "dogs" table if it doesn't already exist
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS domains (
                id INTEGER PRIMARY KEY,
                domain TEXT NOT NULL,
                ip TEXT NOT NULL
            );",
        ).unwrap();

        // Return the connection
        conn
    };
    pub static PW_HASH: String = {
        // Password Hash, example:
        // Password: changemeho4r04lu
        // Hash: $argon2id$v=19$m=19456,t=2,p=1$sBaUosHhTi+3W5Bin5K+jQ$c+Tbk0pFp0Wt8TjJTLzP8ulHKg7Yyeoe6E82+2IkKOI
        let pass: String = env::var("PASSWORD").unwrap_or("changemehor404lu".to_string());
        password_auth::generate_hash(&pass)
    };
}

