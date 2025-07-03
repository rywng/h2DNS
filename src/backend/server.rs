use dioxus::prelude::*;
#[cfg(feature = "server")]
use password_auth::{self, verify_password};
#[cfg(feature = "server")]
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
pub static PW_HASH: String = {
        // Password Hash, example:
        // Password: ho4r04lu
        // Hash: $argon2id$v=19$m=19456,t=2,p=1$sBaUosHhTi+3W5Bin5K+jQ$c+Tbk0pFp0Wt8TjJTLzP8ulHKg7Yyeoe6E82+2IkKOI
    let hash = env::var("PW_HASH");
    match hash {
        Ok(val) => {
            val
        },
        Err(_) => {
            let passgen = passwords::PasswordGenerator::new();
            let pass: String = passgen.generate_one().unwrap();
            password_auth::generate_hash(&pass)
        }
    }
};
}

#[server(endpoint = "resolve")]
pub async fn resolve_domain(domain: String) -> Result<IpAddr, ServerFnError> {
    let domains: Vec<String> = DB.with(|f| {
        f.prepare("SELECT ip FROM domains WHERE domain = (?1)")
            .unwrap()
            .query_map([domain], |row| row.get(0))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });
    dbg!(&domains);
    let res: IpAddr = match domains.first() {
        Some(addr) => addr.parse()?,
        None => {
            return Err(ServerFnError::ServerError("Domain not exist".to_string()));
        }
    };

    Ok(res)
}

#[server(endpoint = "register")]
pub async fn register_domain(
    domain: String,
    ip: String,
    pass: String,
) -> Result<(), ServerFnError> {
    PW_HASH.with(|hash| verify_password(pass, hash))?;
    DB.with(|conn| {
        conn.execute(
            "INSERT INTO domains (domain, ip) VALUES (?1, ?2)",
            (&domain, &ip),
        )
    })?;

    Ok(())
}
