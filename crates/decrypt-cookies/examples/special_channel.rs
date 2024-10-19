use decrypt_cookies::prelude::*;

#[cfg(target_os = "linux")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut p = dirs::config_dir().expect("Get config dir failed");
    p.push("google-chrome-beta");

    // p: `"$HOME/.config/google-chrome-beta"`
    let chromium = ChromiumBuilder::<Chrome>::with_user_data_dir(p)
        .build()
        .await?;
    let all_cookies = chromium.get_cookies_all().await?;

    dbg!(&all_cookies.first());

    let filtered_cookies = chromium
        .get_cookies_filter(ChromiumCookieCol::HostKey.contains("google.com"))
        .await?;

    dbg!(&filtered_cookies.first());

    let mut p = dirs::home_dir().expect("Get home dir failed");
    p.push(".mozilla/firefox-esr");

    // p: `"$HOME/.mozilla/firefox-esr"`
    // let firefox = FirefoxBuilder::<Firefox>::with_path_profile(Some(p), Some("default-release"))?
    // or
    let firefox = FirefoxBuilder::<Firefox>::with_path_profile(p, "default")?
        .build()
        .await?;
    // TODO: make it show FirefoxEsr?
    dbg!(firefox.to_string());
    let all_cookies = firefox.get_cookies_all().await?;

    dbg!(&all_cookies.first());

    let filtered_cookies = firefox
        .get_cookies_filter(MozCookiesCol::Host.contains("google.com"))
        .await?;

    dbg!(&filtered_cookies.first());

    Ok(())
}
