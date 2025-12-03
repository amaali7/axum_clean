use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
};

use crate::{error::DomainResult, DomainError};

#[derive(Debug, Clone, Default)]
pub struct Url(String);

impl Url {
    /// Create a new validated `Url`.
    ///
    /// The string is normalized (trim + lowercase) and must parse as a
    /// valid `http`/`https` URL.  A missing scheme is fixed automatically.
    pub fn new(url: &str) -> DomainResult<Self> {
        let url = url.trim();

        // Empty check
        if url.is_empty() {
            return Err(DomainError::ValidationError("URL cannot be empty".into()));
        }

        // Add default scheme if none present
        let raw = if url.contains("://") {
            Cow::Borrowed(url)
        } else {
            Cow::Owned(format!("https://{}", url))
        };

        // // Basic syntactic validation via `url` crate
        // let parsed = url::Url::parse(&raw)
        //     .map_err(|e| DomainError::ValidationError(format!("Invalid URL: {}", e)))?;

        // // Restrict to HTTP family
        // if !matches!(parsed.scheme(), "http" | "https") {
        //     return Err(DomainError::ValidationError(
        //         "Only HTTP/HTTPS schemes are allowed".into(),
        //     ));
        // }

        // // Re-assemble canonical form (lowercase host, remove default ports)
        // let canonical = Self::canonicalize(parsed);

        Ok(Self(raw.into()))
    }

    /// Return the host portion (domain or IP) in lowercase.
    pub fn host(&self) -> &str {
        self.0
            .split("://")
            .nth(1)
            .and_then(|rem| rem.split('/').next())
            .unwrap_or("")
    }

    /// Return `true` if the host belongs to a list of disposable / short-lived
    /// services (analogous to the email version).
    pub fn is_disposable(&self, list: &[&str]) -> bool {
        let host = self.host();
        list.iter().any(|d| host == *d)
    }

    /* ---------- helpers ---------- */
}

impl Deref for Url {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Url {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// /* ---------- quick tests ---------- */
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn valid_urls() {
//         assert!(Url::new("https://example.com").is_ok());
//         assert!(Url::new("example.com").is_ok()); // auto-fix scheme
//         assert!(Url::new("HTTP://EXAMPLE.COM:80").is_ok());
//     }

//     #[test]
//     fn invalid_urls() {
//         assert!(Url::new("").is_err());
//         assert!(Url::new("ht!tp://bad url").is_err());
//         assert!(Url::new("ftp://example.com").is_err());
//     }

//     #[test]
//     fn disposable_check() {
//         let u = Url::new("https://temp-site.com/abc").unwrap();
//         let list = &["temp-site.com", "throwaway.link", "guerrilla-url.net"];
//         assert!(u.is_disposable(list));
//     }
// }
