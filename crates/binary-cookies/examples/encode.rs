use binary_cookies::{
    bstr::BString,
    cookie::{BinaryCookies, Cookie, Metadata, Page, SameSite},
};
use chrono::DateTime;

fn main() {
    let bc = BinaryCookies {
        pages: vec![Page {
            cookies: vec![Cookie {
                version: 0,
                flags: 1,
                port: None,
                comment: None,
                domain: BString::new(b"domain".to_vec()),
                name: BString::new(b"name".to_vec()),
                path: BString::new(b"path".to_vec()),
                value: BString::new(b"value".to_vec()),
                expires: Some(DateTime::UNIX_EPOCH),
                creation: Some(DateTime::UNIX_EPOCH),
                same_site: SameSite::Lax,
                is_secure: true,
                is_http_only: true,
            }],
        }],
        metadata: Metadata { nshttp_cookie_accept_policy: 1 }.into(),
    };
    let _val = bc.encode();
}
