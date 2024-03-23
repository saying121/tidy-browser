use serde::{Deserialize, Serialize};

#[derive(Default, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub struct LeetCodeCookies {
    pub csrf: String,
    pub session: String,
}

impl LeetCodeCookies {
    pub fn is_completion(&self) -> bool {
        !(self.csrf.is_empty() || self.session.is_empty())
    }
}

impl std::fmt::Display for LeetCodeCookies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("LEETCODE_SESSION={};csrftoken={};", self.session, self.csrf).fmt(f)
    }
}
