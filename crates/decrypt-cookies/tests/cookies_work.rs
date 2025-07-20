use decrypt_cookies::{
    browser::{Chrome, Firefox},
    prelude::*,
};

#[ignore = "need realy environment"]
#[tokio::test]
async fn chromium_cookies_test() {
    macro_rules! test_browser {
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
                let res = match getter.all_cookies().await {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("{e}");
                        vec![]
                    },
                };
                match res.first() {
                    Some(i) => {
                        println!(
                            "{}, {}, {}, {}, value: {}",
                            $browser,
                            i.name,
                            i.expires_utc.unwrap(),
                            i.creation_utc.unwrap(),
                            i.decrypted_value.as_ref().unwrap(),
                        );
                    },
                    None => println!("None ============= {}",$browser),
                };
                println!("=============");
            )*
        }
    }

    test_browser!(Chrome, Edge, Chromium, Brave, Yandex, Vivaldi, Opera);
    #[cfg(not(target_os = "linux"))]
    test_browser!(OperaGX, CocCoc, Arc);
}

#[ignore = "need realy environment"]
#[tokio::test]
async fn ff_cookies_test() {
    macro_rules! test_browser {
        ($($browser:ident), *) => {
            $(
                let getter = match FirefoxBuilder::<$browser>::new()
                    .build()
                    .await
                {
                    Ok(it) => it,
                    Err(e) => {
                        eprintln!("{e}");
                        return;
                    },
                };
                let res = match getter.all_cookies().await {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("{e}");
                        vec![]
                    },
                };
                match res.first() {
                    Some(i) => {
                        println!(
                            "{}, {}, {}, {}, value: {}",
                            $browser,
                            i.name,
                            i.expiry.unwrap(),
                            i.creation_time.unwrap(),
                            i.value,
                        );
                    },
                    None => println!("None ============= {}",$browser),
                };
                println!("=============");
            )*
        }
    }

    test_browser!(Firefox, Librewolf, Floorp);
}
