use std::str::FromStr;

use bstr::BString;
use pretty_assertions::assert_eq;

use super::*;

const COOKIE: &str = "./test-resource/BinaryCookies.cookie";
const PAGE: &str = "./test-resource/BinaryCookies.page";
const BINARY_COOKIE: &str = "./test-resource/BinaryCookies.binarycookies";

#[ignore = "Need real env"]
#[test]
fn test_safari_cookies() {
    let mut path = dirs::home_dir().unwrap();
    path.push("Library/Containers/com.apple.Safari/Data/Library/Cookies/Cookies.binarycookies");

    let a = std::fs::read(path).unwrap();

    let mut input = StreamIn::new(&a);
    let bc = CookieDecoder::binary_cookies(&mut input).unwrap();

    // FIXME: my impl not correct
    // assert_eq!(bc.checksum, bc.checksum());
    for (idx, page) in bc.pages.iter().enumerate() {
        assert_eq!(bc.page_sizes[idx], page.size());
    }
    for page in bc.pages() {
        assert_eq!(page.cookie_offsets, page.cookie_offsets());

        for ele in page.iter_cookies() {
            assert_eq!(ele.domain_offset, ele.domain_offset());
            assert_eq!(ele.name_offset, ele.name_offset());
            assert_eq!(ele.path_offset, ele.path_offset());
            assert_eq!(ele.value_offset, ele.value_offset());
            assert_eq!(ele.comment_offset, ele.comment_offset());
            assert_eq!(ele.raw_expires, Cookie::time_to_f64(ele.expires.unwrap()));
            assert_eq!(ele.raw_creation, Cookie::time_to_f64(ele.creation.unwrap()));

            if let Some(c) = &ele.comment {
                assert_eq!(ele.domain_offset - ele.comment_offset, (c.len() + 1) as u32);
            }
            assert_eq!(
                (ele.domain.len() + 1) as u32,
                ele.name_offset - ele.domain_offset
            );
            assert_eq!(
                (ele.name.len() + 1) as u32,
                ele.path_offset - ele.name_offset
            );
            assert_eq!(
                (ele.path.len() + 1) as u32,
                ele.value_offset - ele.path_offset
            );
            assert_eq!((ele.value.len() + 1) as u32, ele.size() - ele.value_offset);
        }
    }
}

#[test]
fn test_cookie() {
    let a = std::fs::read(COOKIE).unwrap();
    let mut input = StreamIn::new(&a);
    let a = CookieDecoder::cookie(&mut input).unwrap();
    let var = Cookie {
        version: 0,
        flags: 1090602376,
        domain_offset: 64,
        name_offset: 71,
        path_offset: 76,
        value_offset: 81,
        comment_offset: 56,
        raw_expires: 243043200.0,
        raw_creation: 98064000.0,
        port: None,
        comment: BString::from_str("comment").ok(),
        domain: BString::from_str("domain").unwrap(),
        name: BString::from_str("name").unwrap(),
        path: BString::from_str("path").unwrap(),
        value: BString::from_str("value").unwrap(),
        expires: 243043200.0.to_utc(),
        creation: 98064000.0.to_utc(),
        same_site: SameSite::None,
        is_secure: false,
        is_http_only: false,
    };
    assert_eq!(a, var);
}

#[test]
fn test_page() {
    let a = std::fs::read(PAGE).unwrap();
    let mut input = StreamIn::new(&a);
    let a = CookieDecoder::page(&mut input).unwrap();
    let page = Page {
        cookie_offsets: vec![20, 107],
        cookies: vec![
            Cookie {
                version: 0,
                flags: 3628807294,
                domain_offset: 64,
                name_offset: 71,
                path_offset: 76,
                value_offset: 81,
                comment_offset: 56,
                raw_expires: 279676800.0,
                raw_creation: 288662400.0,
                port: None,
                comment: BString::from_str("comment").ok(),
                domain: BString::from_str("domain").unwrap(),
                name: BString::from_str("name").unwrap(),
                path: BString::from_str("path").unwrap(),
                value: BString::from_str("value").unwrap(),
                expires: 279676800.0.to_utc(),
                creation: 288662400.0.to_utc(),
                same_site: SameSite::Strict,
                is_secure: false,
                is_http_only: true,
            },
            Cookie {
                version: 0,
                flags: 3004896092,
                domain_offset: 64,
                name_offset: 71,
                path_offset: 76,
                value_offset: 81,
                comment_offset: 56,
                raw_expires: 89856000.0,
                raw_creation: 227145600.0,
                port: None,
                comment: BString::from_str("comment").ok(),
                domain: BString::from_str("domain").unwrap(),
                name: BString::from_str("name").unwrap(),
                path: BString::from_str("path").unwrap(),
                value: BString::from_str("value").unwrap(),
                expires: 89856000.0.to_utc(),
                creation: 227145600.0.to_utc(),
                same_site: SameSite::None,
                is_secure: false,
                is_http_only: true,
            },
        ],
    };
    assert_eq!(a, page);
}

#[test]
fn test_binary_cookie() {
    let a = std::fs::read(BINARY_COOKIE).unwrap();
    let mut input = StreamIn::new(&a);
    let a = CookieDecoder::binary_cookies(&mut input).unwrap();
    let val = BinaryCookies {
        page_sizes: vec![196, 196],
        pages: vec![
            Page {
                cookie_offsets: vec![20, 107],
                cookies: vec![
                    Cookie {
                        version: 0,
                        flags: 1418700252,
                        domain_offset: 64,
                        name_offset: 71,
                        path_offset: 76,
                        value_offset: 81,
                        comment_offset: 56,
                        raw_expires: 68688000.0,
                        raw_creation: 195609600.0,
                        port: None,
                        comment: BString::from_str("comment").ok(),
                        domain: BString::from_str("domain").unwrap(),
                        name: BString::from_str("name").unwrap(),
                        path: BString::from_str("path").unwrap(),
                        value: BString::from_str("value").unwrap(),
                        expires: 68688000.0.to_utc(),
                        creation: 195609600.0.to_utc(),
                        same_site: SameSite::None,
                        is_secure: false,
                        is_http_only: true,
                    },
                    Cookie {
                        version: 1,
                        flags: 2399795353,
                        domain_offset: 66,
                        name_offset: 73,
                        path_offset: 78,
                        value_offset: 83,
                        comment_offset: 58,
                        raw_expires: 194659200.0,
                        raw_creation: 270000000.0,
                        port: Some(34046),
                        comment: BString::from_str("comment").ok(),
                        domain: BString::from_str("domain").unwrap(),
                        name: BString::from_str("name").unwrap(),
                        path: BString::from_str("path").unwrap(),
                        value: BString::from_str("value").unwrap(),
                        expires: 194659200.0.to_utc(),
                        creation: 270000000.0.to_utc(),
                        same_site: SameSite::None,
                        is_secure: true,
                        is_http_only: false,
                    },
                ],
            },
            Page {
                cookie_offsets: vec![20, 107],
                cookies: vec![
                    Cookie {
                        version: 0,
                        flags: 1921380239,
                        domain_offset: 64,
                        name_offset: 71,
                        path_offset: 76,
                        value_offset: 81,
                        comment_offset: 56,
                        raw_expires: 76204800.0,
                        raw_creation: 183686400.0,
                        port: None,
                        comment: BString::from_str("comment").ok(),
                        domain: BString::from_str("domain").unwrap(),
                        name: BString::from_str("name").unwrap(),
                        path: BString::from_str("path").unwrap(),
                        value: BString::from_str("value").unwrap(),
                        expires: 76204800.0.to_utc(),
                        creation: 183686400.0.to_utc(),
                        same_site: SameSite::None,
                        is_secure: true,
                        is_http_only: true,
                    },
                    Cookie {
                        version: 1,
                        flags: 3640326490,
                        domain_offset: 66,
                        name_offset: 73,
                        path_offset: 78,
                        value_offset: 83,
                        comment_offset: 58,
                        raw_expires: 131846400.0,
                        raw_creation: 191289600.0,
                        port: Some(50037),
                        comment: BString::from_str("comment").ok(),
                        domain: BString::from_str("domain").unwrap(),
                        name: BString::from_str("name").unwrap(),
                        path: BString::from_str("path").unwrap(),
                        value: BString::from_str("value").unwrap(),
                        expires: 131846400.0.to_utc(),
                        creation: 191289600.0.to_utc(),
                        same_site: SameSite::None,
                        is_secure: false,
                        is_http_only: false,
                    },
                ],
            },
        ],
        checksum: 5672,
        metadata: Some(Metadata { nshttp_cookie_accept_policy: 2 }),
    };
    assert!(input.is_empty());
    assert_eq!(a, val);
}
