use decrypt_cookies::prelude::*;

#[ignore = "need realy environment"]
#[tokio::test]
async fn passwd() {
    let edge_getter = match ChromiumBuilder::<Chrome>::new()
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
        .all_logins()
        .await
        .unwrap();
    // dbg!(&res[0]);
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
                        return;
                    },
                };
                println!("=============");
            )*
        }
    }

    test_chromium_pwd!(Chrome, Edge, Chromium, Brave, Yandex, Vivaldi, Opera);
    #[cfg(not(target_os = "linux"))]
    test_chromium_pwd!(OperaGX, CocCoc, Arc);
}
