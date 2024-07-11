use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Default, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct LeetCodeCookies {
    pub csrf:    String,
    pub session: String,
    #[serde(skip)]
    pub expiry:  bool,
}

impl LeetCodeCookies {
    pub fn is_completion(&self) -> bool {
        !(self.expiry || self.csrf.is_empty() || self.session.is_empty())
    }
}

impl std::fmt::Display for LeetCodeCookies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("LEETCODE_SESSION={};csrftoken={};", self.session, self.csrf).fmt(f)
    }
}

pub trait CookiesInfo {
    /// <https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie>
    fn get_set_cookie_header(&self) -> String {
        let mut properties = vec![
            format!("{}={}", self.name(), self.value()),
            format!("Path={}", self.path()),
        ];
        if !self.name().starts_with("__Host-") {
            properties.push(format!("Domain={}", self.domain()));
        }
        if let Some(expiry) = self.expiry() {
            properties.push(format!("Expires={}", expiry));
        }
        if self.is_secure() {
            properties.push("Secure".to_owned());
        }
        if self.is_http_only() {
            properties.push("HttpOnly".to_owned());
        }
        properties.push(format!("SameSite={}", self.same_site()));

        properties.join("; ")
    }

    fn get_url(&self) -> String {
        format!("https://{}{}", self.domain().trim_matches('.'), self.path())
    }

    fn name(&self) -> &str;
    fn value(&self) -> &str;
    fn path(&self) -> &str;
    fn domain(&self) -> &str;
    fn expiry(&self) -> Option<String>;
    fn is_secure(&self) -> bool;
    fn is_http_only(&self) -> bool;
    fn same_site(&self) -> SameSite;
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum SameSite {
    #[default]
    None,
    Lax,
    Strict,
}

impl Display for SameSite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => "None",
            Self::Lax => "Lax",
            Self::Strict => "Strict",
        }
        .fmt(f)
    }
}
