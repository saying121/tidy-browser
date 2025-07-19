use anyhow::Result;
use decrypt_cookies::{
    browser::{Chrome, Firefox},
    prelude::*,
};

#[ignore = "need realy environment"]
#[tokio::test]
async fn cookies_browsers() {
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

    test_chromium_pwd!(Chrome, Edge, Chromium, Brave, Yandex, Vivaldi, Opera);
    #[cfg(not(target_os = "linux"))]
    test_chromium_pwd!(OperaGX, CocCoc, Arc);
}

#[ignore = "need realy environment"]
#[tokio::test]
async fn ff_get_all_cookie_work() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .with_test_writer()
        .init();

    let ff = FirefoxBuilder::<Firefox>::new()
        .build()
        .await?;
    let a = ff.all_cookies().await?;
    for i in a.iter().take(6) {
        println!(
            "name: {}, last_accessed: {}, expiry: {}, creation_time: {}, value: {}",
            i.name,
            i.last_accessed.unwrap(),
            i.expiry.unwrap(),
            i.creation_time.unwrap(),
            i.value,
        );
    }

    Ok(())
}

#[ignore = "need realy environment"]
#[tokio::test]
async fn ff_cookies_browsers() {
    macro_rules! test_chromium_pwd {
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

    test_chromium_pwd!(Firefox, Librewolf);
}
