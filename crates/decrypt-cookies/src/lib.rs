pub mod browser;
pub mod chromium;
pub mod firefox;
#[cfg(target_os = "macos")]
pub mod safari;

pub use browser::{cookies::LeetCodeCookies, Browser};
pub use chromium::{ChromiumBuilder, ChromiumGetter};
pub use firefox::{FirefoxBuilder, FirefoxGetter};
use miette::Result;
#[cfg(target_os = "macos")]
pub use safari::SafariGetter;
#[cfg(target_os = "macos")]
pub use safari::SafariBuilder;

/// get csrf and session
///
/// * `borwser`: firefox, librewolf, edge, chrome
pub async fn get_cookie<T>(borwser: T, host: &str) -> Result<LeetCodeCookies>
where
    T: Into<Browser>,
{
    let res = match borwser.into() {
        Browser::Firefox => {
            let getter = FirefoxBuilder::new(Browser::Firefox)
                .build()
                .await?;
            getter
                .get_session_csrf(host)
                .await?
        },
        Browser::Librewolf => {
            let getter = FirefoxBuilder::new(Browser::Librewolf)
                .build()
                .await?;
            getter
                .get_session_csrf(host)
                .await?
        },

        #[cfg(target_os = "macos")]
        Browser::Safari => {
            let getter = safari::items::cookie::CookiesGetter::build::<&str>(None).await?;
            getter
                .get_session_csrf(host)
                .ok_or_else(|| miette::miette!("empty cookies"))
        },

        chromium => {
            let getter = ChromiumBuilder::new(chromium)
                .build()
                .await?;
            getter
                .get_cookies_session_csrf(host)
                .await?
        },
    };

    Ok(res)
}
