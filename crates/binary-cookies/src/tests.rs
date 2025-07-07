use bstr::BString;
use chrono::{Days, TimeZone as _, Utc};
use rand::{random, random_range, Rng, SeedableRng};

use crate::{
    cookie::{BinaryCookies, Cookie, Metadata, Page},
    decode::stream,
    sync::stream::StreamDecoder,
};

#[test]
fn encode_binary_cookies() {
    let cookie = BinaryCookies::random();
    let v = cookie.encode();

    let mut a = StreamDecoder::new(std::io::Cursor::new(v));
    loop {
        let a = a.decode().unwrap();
        match a {
            stream::Values::Bc { .. } | stream::Values::Page(_) | stream::Values::Cookie(_) => {},
            stream::Values::Meta { .. } => break,
        }
    }
}

// //////

impl BinaryCookies {
    fn random() -> Self {
        let pages = (0..rand::random_range(2..5))
            .map(|_| Page::random())
            .collect();
        Self {
            pages,
            metadata: Some(Metadata { nshttp_cookie_accept_policy: 2 }),
        }
    }
}

impl Page {
    fn random() -> Self {
        let cookies = (0..rand::random_range(2..5))
            .map(|_| Cookie::random())
            .collect();
        Self { cookies }
    }
}

impl Cookie {
    fn random() -> Self {
        let port = rand::random_bool(0.5).then(random);
        let base_datetime = Utc
            .with_ymd_and_hms(2001, 1, 1, 0, 0, 0)
            .unwrap();

        let expires = base_datetime + Days::new(random_range(365..3650));
        let creation = base_datetime + Days::new(random_range(365..3650));

        let flags = random();

        let is_secure = Self::is_secure(flags);
        let is_http_only = Self::is_http_only(flags);
        let same_site = Self::same_site(flags);

        let mut self_ = Self {
            port,
            version: random_range(0..2),
            flags,
            same_site,
            is_secure,
            is_http_only,

            domain_offset: 0,
            name_offset: 0,
            path_offset: 0,
            value_offset: 0,
            comment_offset: 0,

            raw_expires: Self::time_to_f64(expires),
            expires: expires.into(),
            raw_creation: Self::time_to_f64(creation),
            creation: creation.into(),
            comment: BString::new(rand_vec()).into(),
            domain: BString::new(rand_vec()),
            name: BString::new(rand_vec()),
            path: BString::new(rand_vec()),
            value: BString::new(rand_vec()),
        };

        self_.domain_offset = self_.domain_offset();
        self_.name_offset = self_.name_offset();
        self_.path_offset = self_.path_offset();
        self_.value_offset = self_.value_offset();
        self_.comment_offset = self_.comment_offset();

        self_
    }
}

fn rand_vec() -> Vec<u8> {
    let rng = rand::rngs::StdRng::from_os_rng();
    let size = random_range(30..40);
    rng.random_iter()
        .take(size)
        .collect()
}
