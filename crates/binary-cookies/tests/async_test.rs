use binary_cookies::{cookie::Metadata, decode::stream::Values, tokio::stream::StreamDecoder};
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
