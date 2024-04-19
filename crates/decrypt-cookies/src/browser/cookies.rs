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
    fn is_expiry(&self) -> bool;
}
