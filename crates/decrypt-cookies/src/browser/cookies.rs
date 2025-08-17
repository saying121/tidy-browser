use std::fmt::Display;

use chrono::{DateTime, Utc};

#[derive(Default, Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LeetCodeCookies {
    pub csrf: String,
    pub session: String,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub expiry: bool,
}

impl LeetCodeCookies {
    pub fn is_completion(&self) -> bool {
        !(self.expiry || self.csrf.is_empty() || self.session.is_empty())
    }
}

impl Display for LeetCodeCookies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "LEETCODE_SESSION={};csrftoken={};",
            self.session, self.csrf
        ))
    }
}

pub trait CookiesInfo {
    fn csv_header<D: Display>(sep: D) -> String {
        format!("domain{sep}name{sep}path{sep}value{sep}creation{sep}expires{sep}is_secure{sep}is_http_only")
    }

    fn to_csv<D: Display>(&self, sep: D) -> String {
        format!(
            "{}{sep}{}{sep}{}{sep}{}{sep}{}{sep}{}{sep}{}{sep}{}",
            self.domain(),
            self.name(),
            self.path(),
            self.value(),
            self.creation().unwrap_or_default(),
            self.expires().unwrap_or_default(),
            self.is_secure(),
            self.is_http_only(),
        )
    }

    /// <https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie>
    fn set_cookie_header(&self) -> String {
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

    // TODO: reanme to `url`
    fn url(&self) -> String {
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
    fn creation(&self) -> Option<DateTime<Utc>>;
    fn expires(&self) -> Option<DateTime<Utc>>;
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SameSite {
    #[default]
    None = 0,
    Lax = 1,
    Strict = 2,
}

impl From<i32> for SameSite {
    fn from(value: i32) -> Self {
        #[expect(clippy::wildcard_in_or_patterns, reason = "this is more clear")]
        match value {
            1 => Self::Lax,
            2 => Self::Strict,
            0 | _ => Self::None,
        }
    }
}

#[cfg(feature = "Safari")]
impl From<binary_cookies::cookie::SameSite> for SameSite {
    fn from(value: binary_cookies::cookie::SameSite) -> Self {
        match value {
            binary_cookies::cookie::SameSite::None => Self::None,
            binary_cookies::cookie::SameSite::Lax => Self::Lax,
            binary_cookies::cookie::SameSite::Strict => Self::Strict,
        }
    }
}

impl From<Option<i32>> for SameSite {
    fn from(value: Option<i32>) -> Self {
        value.unwrap_or_default().into()
    }
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
