use decrypt_cookies::prelude::*;

#[ignore = "need realy environment"]
#[tokio::test]
async fn passwd_browsers() {
    macro_rules! test_chromium_pwd {
        ($($browser:ident), *) => {
            $(
                let getter = match ChromiumBuilder::<$browser>::new()
                    .build()
                    .await
                {
                    Ok(it) => it,
                    Err(e) => {
                        eprintln!("{e}");
                        return;
                    },
                };
                let res = match getter.all_logins().await {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("{e}");
                        vec![]
                    },
                };
                match res.first() {
                    Some(first) => {
                        println!(
                            "{} {} {} {} ",
                            $browser,
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
                    None => println!("None ============= {}",$browser),
                };
                println!("=============");
            )*
        }
    }

    test_chromium_pwd!(Chrome, Edge, Chromium, Brave, Yandex, Vivaldi, Opera);
    #[cfg(not(target_os = "linux"))]
    test_chromium_pwd!(OperaGX, CocCoc, Arc);
}
