#[cfg(unix)]
use std::os::unix::prelude::MetadataExt;
#[cfg(not(unix))]
use std::os::windows::prelude::MetadataExt;
use std::{fs::File, str::FromStr};

use bstr::BString;
use pretty_assertions::assert_eq;

use crate::{
    cookie::{BinaryCookies, Cookie, Metadata, Page, SameSite},
    decode::{pages::PagesOffset, stream::Values, F64ToSafariTime as _, OffsetSize},
    sync::{
        bc::DecodeBinaryCookie, cookie::CookieDecoder, cursor::CookieCursor, page::PageDecoder,
        stream::StreamDecoder,
    },
};

const COOKIE: &str = "./test-resource/BinaryCookies.cookie";
const PAGE: &str = "./test-resource/BinaryCookies.page";
const BINARY_COOKIE: &str = "./test-resource/BinaryCookies.binarycookies";

#[test]
fn test_binary_cookie_stream() {
    let f = File::open(BINARY_COOKIE).unwrap();
    let mut sd = StreamDecoder::new(f);

    loop {
        let a = sd.decode().unwrap();
        match a {
            Values::Bc { meta_offset, pages_offset } => {
                assert_eq!(meta_offset, 408);
                assert_eq!(
                    pages_offset,
                    PagesOffset {
                        offset_sizes: vec![
                            OffsetSize { offset: 16, size: 196 },
                            OffsetSize { offset: 212, size: 196 },
                        ],
                    }
                );
            },
            Values::Page(_) | Values::Cookie(_) => {},
            Values::Meta { checksum, meta } => {
                assert_eq!(
                    (checksum, meta),
                    (5672, Some(Metadata { nshttp_cookie_accept_policy: 2 }))
                );
                break;
            },
        }
    }
}

#[test]
fn test_binary_cookie() {
    let f = File::open(BINARY_COOKIE).unwrap();
    let bch = f.decode().unwrap();
    assert_eq!(bch.meta_offset, 408);
    assert_eq!(
        bch.pages_offset,
        PagesOffset {
            offset_sizes: vec![
                OffsetSize { offset: 16, size: 196 },
                OffsetSize { offset: 212, size: 196 },
            ],
        }
    );
    let (pages_handle, mut meta_h) = bch.into_handles();
    let meta = meta_h.decode().unwrap();
    assert_eq!(
        meta,
        (5672, Some(Metadata { nshttp_cookie_accept_policy: 2 }))
    );
    let page_cookies: Vec<Vec<Cookie>> = pages_handle
        .decoders()
        .map(|mut p| {
            p.decode()
                .unwrap()
                .into_decoders()
                .map(|mut c| c.decode().unwrap())
                .collect()
        })
        .collect();

    let val = vec![
        vec![
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
        vec![
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
    ];

    assert_eq!(val, page_cookies);
}

#[test]
fn test_cookie() {
    let mut decoder = CookieDecoder {
        rd: File::open(COOKIE).unwrap(),
        size: 1,
    };
    let a = decoder.decode().unwrap();
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
    let file = File::open(PAGE).unwrap();
    let mut pd = PageDecoder {
        file: &file,
        rd: file.cursor_at(0),
        offset: 0,
        size: {
            #[cfg(any(target_os = "macos", target_os = "linux"))]
            {
                file.metadata().unwrap().size()
            }
            #[cfg(target_os = "windows")]
            {
                file.metadata()
                    .unwrap()
                    .file_size()
            }
        } as u32,
    };
    let a = pd.decode().unwrap();
    let cookies: Vec<_> = a
        .decoders()
        .map(|mut v| v.decode().unwrap())
        .collect();

    let page_cks = vec![
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
    ];
    assert_eq!(cookies, page_cks);
}

#[ignore = "Need real env"]
#[test]
fn test_safari_cookies() {
    let mut path = dirs::home_dir().unwrap();
    path.push("Library/Containers/com.apple.Safari/Data/Library/Cookies/Cookies.binarycookies");

    let f = File::open(path).unwrap();
    let bch = f.decode().unwrap();
    let (page_h, mut meta_d) = bch.into_handles();

    let (_checksum, meta) = meta_d.decode().unwrap();
    assert_eq!(meta, Some(Metadata { nshttp_cookie_accept_policy: 2 }));

    let mut pages = vec![];

    for mut page_d in page_h.into_decoders() {
        let mut cookies = vec![];

        let cookie_h = page_d.decode().unwrap();
        for mut cookie_d in cookie_h.into_decoders() {
            let ele = cookie_d.decode().unwrap();

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
            cookies.push(ele);
        }
        let page = Page { cookies };
        pages.push(page);
    }

    let _bc = BinaryCookies { pages, metadata: None };

    // FIXME: my checksum impl not correct
    // assert_eq!(checksum, bc.checksum());
}
