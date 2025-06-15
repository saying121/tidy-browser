use bstr::BString;
use chrono::{Days, TimeZone as _, Utc};
use pretty_assertions::assert_eq;
use rand::{random, random_range, Rng, SeedableRng};

use crate::{
    cookie::{BinaryCookies, Cookie, Metadata, Page},
    decode::{CookieDecoder, Stream},
};

#[test]
fn encode_cookie_flags() {
    for _ in 0..10 {
        let cookie = Cookie::random();
        assert_eq!(cookie.flags, cookie.flags());
    }
}

#[test]
fn encode_cookie() {
    let cookie = Cookie::random();
    assert_eq!(cookie.flags, cookie.flags());
    let v = cookie.encode();

    let mut var = Stream::new(&v);
    let a = CookieDecoder::cookie(&mut var).unwrap();
    assert_eq!(cookie, a);
}

#[test]
fn encode_page() {
    let page = Page::random();
    let v = page.encode().0;

    let mut var = Stream::new(&v);
    let a = CookieDecoder::page(&mut var).unwrap();
    assert_eq!(a, page);
}

#[test]
fn encode_binary_cookies() {
    let cookie = BinaryCookies::random();
    let v = cookie.encode();

    let mut var = Stream::new(&v);
    let a = CookieDecoder::binary_cookies(&mut var).unwrap();
    assert_eq!(a, cookie);
}

// //////

impl BinaryCookies {
    fn random() -> Self {
        let pages = (0..rand::random_range(2..5))
            .map(|_| Page::random())
            .collect();
        let mut s = Self::new(pages);
        s.metadata = Some(Metadata { nshttp_cookie_accept_policy: 2 });
        s
    }
}

impl Page {
    fn random() -> Self {
        let cookies = (0..rand::random_range(2..5))
            .map(|_| Cookie::random())
            .collect();
        let mut self_ = Self { cookie_offsets: vec![], cookies };
        self_.cookie_offsets = self_.cookie_offsets();
        self_
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

        let is_secure = CookieDecoder::is_secure(flags);
        let is_http_only = CookieDecoder::is_http_only(flags);
        let same_site = CookieDecoder::same_site(flags);

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
