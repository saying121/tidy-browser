use decrypt_cookies::{Browser, ChromiumBuilder};
use strum::IntoEnumIterator;

#[ignore = "need realy environment"]
#[tokio::test]
async fn passwd() {
    let edge_getter = match ChromiumBuilder::new(Browser::Yandex)
        .build()
        .await
    {
        Ok(it) => it,
        Err(e) => {
            eprintln!("{e}");
            return;
        },
    };
    let res = edge_getter
        .get_logins_all()
        .await
        .unwrap();
    dbg!(&res[0]);
    for i in res.into_iter().take(6) {
        println!(
            "{}, {}, {}, {}",
            i.origin_url,
            i.username_value
                .unwrap_or_default(),
            i.username_element
                .unwrap_or_default(),
            i.password_value
                .unwrap_or_default()
        );
    }
}

#[ignore = "need realy environment"]
#[tokio::test]
async fn passwd_browsers() {
    for browser in Browser::iter().skip_while(|v| !v.is_chromium_base()) {
        dbg!(browser);
        let getter = match ChromiumBuilder::new(browser)
            .build()
            .await
        {
            Ok(it) => it,
            Err(e) => {
                eprintln!("{e}");
                continue;
            },
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
            None => {
                println!("=============");
                continue;
            },
        };
        println!("=============");
    }
}
