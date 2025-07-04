#[cfg(feature = "server")]
pub mod server_utils;

use dioxus::logger::tracing::debug;
use dioxus::prelude::*;
#[cfg(feature = "server")]
use password_auth::{self, verify_password};
use std::env;
use std::net::IpAddr;

#[server(endpoint = "resolve")]
pub async fn resolve_domain(domain: String) -> Result<IpAddr, ServerFnError> {
    let domains: Vec<String> = server_utils::DB.with(|f| {
        f.prepare("SELECT ip FROM domains WHERE domain=?1;")
            .unwrap()
            .query_map(server_utils::params![domain], |row| row.get(0))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });
    dbg!(&domains);
    dbg!(&domain);
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
    let _: IpAddr = ip.parse()?;
    server_utils::PW_HASH.with(|hash| verify_password(&pass, hash))?;

    server_utils::DB.with(|conn| {
        conn.execute(
            "INSERT OR REPLACE INTO domains (id, domain, ip) VALUES ((select id from domains where domain=?1) ,?1, ?2);",
            (&domain, &ip),
        )
    })?;

    debug!(
        "Successfully registered: {}, {} with password {}",
        &domain, &ip, &pass
    );

    Ok(())
}

#[server(endpoint = "forwarder")]
pub async fn forward_ddns(
    ip: String,
    domains: String,
    token: String,
    password: String,
) -> Result<String, ServerFnError> {
    let ip: IpAddr = ip.parse()?;
    if ip.is_loopback() || ip.is_unspecified() {
        return Err(ServerFnError::ServerError("Invalid IP Address".to_string()));
    }
    server_utils::PW_HASH.with(|hash| verify_password(&password, hash))?;

    let client = server_utils::reqwest::Client::new();
    let ip_param = if ip.is_ipv6() { "ipv6" } else { "ip" };
    let res = client
        .get("https://www.duckdns.org/update")
        .query(&[
            ("domains", domains),
            ("token", token),
            (ip_param, ip.to_string()),
        ])
        .send()
        .await?;

    let res_status = res.status();
    let res_content = res.text().await?;
    if res_status.is_success() && res_content == "OK" {
        Ok(res_content)
    } else {
        Err(ServerFnError::ServerError(format!(
            "Remote returned {} with status code {}",
            res_status, res_content
        )))
    }
}

#[cfg(test)]
fn clean_database() {
    server_utils::DB.with(|conn| {
        conn.execute_batch("DELETE FROM domains").unwrap();
    });
}

#[cfg(test)]
mod test {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    use super::{clean_database, register_domain, resolve_domain};

    #[tokio::test]
    async fn test_database_operation() {
        clean_database();
        assert!(resolve_domain("test-domain".to_string()).await.is_err());
        let target_ipv4 = IpAddr::V4(Ipv4Addr::new(192, 168, 2, 3));

        register_domain(
            "test-domain".to_string(),
            target_ipv4.to_string(),
            "changemehor404lu".to_string(),
        )
        .await
        .unwrap();

        assert_eq!(
            target_ipv4,
            resolve_domain("test-domain".to_string()).await.unwrap()
        );

        assert!(resolve_domain("test-v6-domain".to_string()).await.is_err());
        let target_ipv6 = IpAddr::V6(Ipv6Addr::new(1, 2, 3, 2, 1, 2, 3, 3));

        register_domain(
            "test-v6-domain".to_string(),
            target_ipv6.to_string(),
            "changemehor404lu".to_string(),
        )
        .await
        .unwrap();

        assert_eq!(
            target_ipv6,
            resolve_domain("test-v6-domain".to_string()).await.unwrap()
        );
    }
}
