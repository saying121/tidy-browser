use decrypt_cookies::{Browser, ChromiumBuilder};
use strum::IntoEnumIterator;

#[tokio::test]
async fn can_compile() {
    for ele in Browser::iter().skip_while(|v| !v.is_chromium_base()) {
        dbg!(ele);
        let Ok(getter) = ChromiumBuilder::new(ele)
            .build()
            .await
        else {
            continue;
        };
        let res = match getter.get_logins_all().await {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{e}");
                vec![]
            },
        };
        match res.first() {
            Some(first) => {
                println!(
                    "{} {} {} ",
                    first.origin_url,
                    first
                        .username_value
                        .as_deref()
                        .unwrap_or_default(),
                    first
                        .password_value
                        .as_deref()
                        .unwrap_or_default()
                );
            },
            None => continue,
        };
        println!("=============");
    }
}
