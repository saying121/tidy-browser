use std::sync::Arc;

use binary_cookies::{
    cookie::Metadata,
    decode::stream::Values,
    tokio::{DecodeBinaryCookie, StreamDecoder},
};
use positioned_io::RandomAccessFile;
use tokio::fs::File;

const BINARY_COOKIE: &str = "./test-resource/BinaryCookies.binarycookies";

#[tokio::test]
async fn test_binary_cookie_stream() {
    let f = File::open(BINARY_COOKIE)
        .await
        .unwrap();
    let mut sd = StreamDecoder::new(f);

    let mut page_cookies = vec![];

    let mut cookies = vec![];
    loop {
        let a = sd.decode().await.unwrap();
        match a {
            Values::Bc { meta_offset, .. } => {
                assert_eq!(meta_offset, 408);
            },
            Values::Page(_) => {
                page_cookies.push(std::mem::take(&mut cookies));
            },
            Values::Cookie(c) => {
                cookies.push(c);
            },
            Values::Meta { checksum, meta } => {
                assert_eq!(
                    (checksum, meta),
                    (5672, Some(Metadata { nshttp_cookie_accept_policy: 2 }))
                );
                break;
            },
        }
    }
    assert_eq!(page_cookies.len(), 2);
}

#[tokio::test]
async fn test_binary_cookie_decoder() {
    let f = Arc::new(RandomAccessFile::open(BINARY_COOKIE).unwrap());
    let decoder = f.decode().await.unwrap();
    let (ph, mut meta_d) = decoder.into_handles();
    let meta = meta_d.decode().await.unwrap();
    assert_eq!(
        meta,
        (5672, Some(Metadata { nshttp_cookie_accept_policy: 2 }))
    );

    let mut res = vec![];
    for mut pd in ph.decoders() {
        let ch = pd.decode().await.unwrap();
        for mut cd in ch.decoders() {
            let a = cd.decode().await.unwrap();
            res.push(a);
        }
    }
    assert_eq!(res.len(), 4);
}
