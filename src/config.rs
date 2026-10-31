use std::env;
use std::fmt;

use axum::http::{HeaderValue, Uri};

// Do not derive Debug: database_url can contain credentials.
pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub allowed_origins: Vec<HeaderValue>,
}

#[derive(Debug)]
pub struct ConfigError(String);

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ConfigError {}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let database_url = env::var("DATABASE_URL")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .ok_or_else(|| ConfigError("DATABASE_URL is required".into()))?;
        let port = match env::var("PORT") {
            Ok(value) => value
                .parse::<u16>()
                .map_err(|_| ConfigError("PORT must be a valid port number".into()))?,
            Err(env::VarError::NotPresent) => 8080,
            Err(_) => return Err(ConfigError("PORT must be valid Unicode".into())),
        };
        let allowed_origins = match env::var("ALLOWED_ORIGINS") {
            Ok(value) => parse_origins(&value)?,
            Err(env::VarError::NotPresent) => Vec::new(),
            Err(_) => return Err(ConfigError("ALLOWED_ORIGINS must be valid Unicode".into())),
        };
        Ok(Self {
            port,
            database_url,
            allowed_origins,
        })
    }
}

fn parse_origins(value: &str) -> Result<Vec<HeaderValue>, ConfigError> {
    value
        .split(',')
        .map(|origin| {
            let origin = origin.trim().trim_end_matches('/');
            let invalid =
                || ConfigError("ALLOWED_ORIGINS must contain HTTP(S) origins without paths".into());
            let uri: Uri = origin.parse().map_err(|_| invalid())?;
            if !matches!(uri.scheme_str(), Some("http" | "https"))
                || uri.host().is_none()
                || uri
                    .authority()
                    .is_some_and(|authority| authority.as_str().contains('@'))
                || uri
                    .path_and_query()
                    .is_some_and(|path| path.as_str() != "/")
            {
                return Err(invalid());
            }
            HeaderValue::from_str(origin).map_err(|_| invalid())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::parse_origins;

    #[test]
    fn origins_are_trimmed_and_validated() {
        let origins = parse_origins("https://example.com/, http://localhost:3000").unwrap();
        assert_eq!(origins[0], "https://example.com");
        assert_eq!(origins[1], "http://localhost:3000");
        for invalid in [
            "*",
            "",
            "https://example.com/path",
            "https://example.com?x=1",
            "https://user@example.com",
        ] {
            assert!(parse_origins(invalid).is_err(), "accepted {invalid}");
        }
    }
}
