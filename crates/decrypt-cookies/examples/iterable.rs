use std::sync::Arc;

use decrypt_cookies::prelude::*;
use snafu::{ResultExt, Whatever};

#[snafu::report]
#[tokio::main]
async fn main() -> Result<(), Whatever> {
    let chromiums = Box::pin(chromium_cookies_getter()).await;
    let mut chromium_cookies = vec![];
    for ele in chromiums {
        let Ok(getter) = ele
        else {
            continue;
        };
        let Ok(ck) = getter.cookies_all().await
        else {
            continue;
        };
        chromium_cookies.extend(ck);
    }
    let jar: reqwest::cookie::Jar = chromium_cookies.iter().collect();

    let firefoxes = firefox_cookies_getter().await;
    let mut firefox_cookies = vec![];
    for ele in firefoxes {
        let getter = match ele {
            Ok(getter) => getter,
            Err(e) => {
                tracing::error!("The computer not install the browser: {}", e);
                continue;
            },
        };
        let Ok(ck) = getter.cookies_all().await
        else {
            continue;
        };

        firefox_cookies.extend(ck);
    }

    // Add firefox cookies
    decrypt_cookies::firefox::jar_extend_firefox(&jar, &firefox_cookies);

    let _client = reqwest::Client::builder()
        .cookie_provider(Arc::new(jar))
        .build()
        .whatever_context("reqwest Client build failed")?;

    Ok(())
}
