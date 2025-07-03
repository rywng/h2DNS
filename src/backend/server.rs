use dioxus::prelude::*;
use password_auth;
use passwords;
use std::env;
use std::net::IpAddr;

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        let conn = rusqlite::Connection::open("domains.db").expect("Failed to open database");

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
}

#[cfg(feature = "server")]
thread_local! {
pub static PW_HASH: String = {
    let hash = env::var("PW_HASH");
    let hash = match hash {
        Ok(val) => {
            val
        },
        Err(_) => {
            let passgen = passwords::PasswordGenerator::new();
            let pass: String = passgen.generate_one().unwrap();
            let pass_hash = password_auth::generate_hash(&pass);
            pass_hash
        }
    };

    hash
};
}

#[server]
pub async fn resolve_domain(domain: String) -> Result<IpAddr, ServerFnError> {
    let domains: Vec<String> = DB.with(|f| {
        f.prepare("SELECT ip FROM domains WHERE domain = (?1)")
            .unwrap()
            .query_map([domain], |row| Ok(row.get(0)?))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });
    dbg!(&domains);
    let res: IpAddr = match domains.first() {
        Some(addr) => addr.parse()?,
        None => {
            return Err(ServerFnError::Args("Domain not exist".to_string()));
        }
    };

    Ok(res)
}

#[server]
pub async fn register_domain(
    domain: String,
    ip: String,
    pass: String,
) -> Result<(), ServerFnError> {
    todo!()
}
