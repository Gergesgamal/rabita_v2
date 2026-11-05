use std::{env, net::SocketAddr};

const DEFAULT_PORT: u16 = 3000;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub valkey_url: String,
    pub bind_addr: SocketAddr,
}

impl Config {
    /// Reads and validates all settings. Every problem is reported in one error,
    /// so a misconfigured deployment shows its full list at once.
    pub fn from_env() -> Result<Self, String> {
        let mut errors = Vec::new();

        let database_url = required(&mut errors, "DATABASE_URL");
        if let Some(url) = &database_url {
            if !(url.starts_with("postgres://") || url.starts_with("postgresql://")) {
                errors.push("DATABASE_URL must start with postgres:// or postgresql://".to_string());
            }
        }

        let valkey_url = required(&mut errors, "VALKEY_URL");
        if let Some(url) = &valkey_url {
            if url.starts_with("valkey://") {
                errors.push(
                    "VALKEY_URL must use redis:// or rediss://: Valkey speaks the Redis protocol, \
                     and the client does not accept the valkey:// scheme"
                        .to_string(),
                );
            } else if !(url.starts_with("redis://") || url.starts_with("rediss://")) {
                errors.push("VALKEY_URL must start with redis:// or rediss://".to_string());
            }
        }

        let port = match env::var("PORT") {
            Ok(raw) => match raw.trim().parse::<u16>() {
                Ok(p) if p != 0 => p,
                _ => {
                    errors.push(format!("PORT must be an integer from 1 to 65535, got {raw:?}"));
                    DEFAULT_PORT
                }
            },
            Err(_) => DEFAULT_PORT,
        };

        let bind_addr = match env::var("BIND_ADDR") {
            Ok(raw) => match raw.trim().parse::<SocketAddr>() {
                Ok(addr) => Some(addr),
                Err(_) => {
                    errors.push(format!("BIND_ADDR must be host:port, got {raw:?}"));
                    None
                }
            },
            Err(_) => Some(SocketAddr::from(([0, 0, 0, 0], port))),
        };

        match (database_url, valkey_url, bind_addr) {
            (Some(database_url), Some(valkey_url), Some(bind_addr)) if errors.is_empty() => {
                Ok(Config { database_url, valkey_url, bind_addr })
            }
            _ => Err(errors.join("\n")),
        }
    }

    /// One-line summary for startup logs. Credentials are masked.
    pub fn summary(&self) -> String {
        format!(
            "bind={} database={} valkey={}",
            self.bind_addr,
            redact(&self.database_url),
            redact(&self.valkey_url)
        )
    }
}

fn required(errors: &mut Vec<String>, key: &str) -> Option<String> {
    match env::var(key) {
        Ok(v) if !v.trim().is_empty() => Some(v.trim().to_string()),
        _ => {
            errors.push(format!("{key} is required"));
            None
        }
    }
}

/// Replaces the user:password part of a URL with ***.
fn redact(url: &str) -> String {
    match (url.find("://"), url.rfind('@')) {
        (Some(scheme_end), Some(at)) if at > scheme_end => {
            format!("{}://***@{}", &url[..scheme_end], &url[at + 1..])
        }
        _ => url.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::redact;

    #[test]
    fn masks_credentials() {
        assert_eq!(
            redact("postgres://rabita:secret@db:5432/rabita"),
            "postgres://***@db:5432/rabita"
        );
    }

    #[test]
    fn leaves_urls_without_credentials_alone() {
        assert_eq!(redact("redis://127.0.0.1:6379"), "redis://127.0.0.1:6379");
    }
}
