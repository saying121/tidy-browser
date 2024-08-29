pub mod browser;
pub mod chromium;
pub mod firefox;
#[cfg(target_os = "macos")]
pub mod safari;

pub use browser::{cookies::LeetCodeCookies, Browser};
pub use chromium::{ChromiumBuilder, ChromiumGetter};
pub use firefox::{FirefoxBuilder, FirefoxGetter};
use miette::Result;
pub use sea_orm::prelude::ColumnTrait;

cfg_if::cfg_if!(
    if #[cfg(target_os = "macos")] {
        pub use safari::{SafariBuilder, SafariGetter};
    }
);

/// get csrf and session
///
/// * `borwser`: Firefox, Librewolf, edge, chrome
pub async fn get_cookie<T>(browser: T, host: &str) -> Result<LeetCodeCookies>
where
    T: Into<Browser> + Clone + Send,
{
    let res = match browser.clone().into() {
        Browser::Firefox | Browser::Librewolf => {
            let getter = FirefoxBuilder::new(browser.into())
                .build()
                .await?;
            getter
                .get_session_csrf(host)
                .await?
        },

        #[cfg(target_os = "macos")]
        Browser::Safari => {
            let getter = safari::items::cookie::CookiesGetter::build::<&str>(None).await?;
            getter.get_session_csrf(host)
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
