use decrypt_cookies::prelude::*;

#[tokio::test]
async fn can_compile() {
    macro_rules! test_chromium_compile {
        ($($browser:ident), *) => {
            $(
                let Ok(getter) = ChromiumBuilder::<$browser>::new()
                    .build()
                    .await
                else {
                    return;
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
                    None => return,
                };
            )*
        };
    }

    test_chromium_compile!(Chrome, Edge, Chromium, Brave, Yandex, Vivaldi, Opera);
    #[cfg(not(target_os = "linux"))]
    test_chromium_compile!(OperaGX, CocCoc, Arc);

    macro_rules! test_ff_compile {
        ($($browser:ident), *) => {
            $(
                let Ok(getter) = FirefoxBuilder::<$browser>::new()
                    .unwrap()
                    .build()
                    .await
                else {
                    return;
                };
                let res = match getter.get_cookies_all().await {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("{e}");
                        vec![]
                    },
                };
                match res.first() {
                    Some(first) => {
                        println!("{} {} {} ", first.host, first.name, first.value);
                    },
                    None => return,
                };
            )*
        };
    }
    test_ff_compile!(Firefox, Librewolf);
}
