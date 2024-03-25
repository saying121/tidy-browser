pub mod browser;
pub mod chromium;
pub mod firefox;
#[cfg(target_os = "macos")]
pub mod safari;

pub use browser::{cookies::LeetCodeCookies, Browser};
pub use firefox::items::FirefoxGetter;
pub use chromium::ChromiumGetter;
#[cfg(target_os = "macos")]
pub use safari::items::SafariGetter;
use miette::Result;

/// get csrf and session
///
/// * `borwser`: firefox, librewolf, edge, chrome
pub async fn get_cookie<T>(borwser: T, host: &str) -> Result<LeetCodeCookies>
where
    T: Into<Browser>,
{
    let res = match borwser.into() {
        Browser::Firefox => firefox::get_session_csrf(Browser::Firefox, host).await?,
        Browser::Librewolf => firefox::get_session_csrf(Browser::Librewolf, host).await?,

        // #[cfg(target_os = "macos")]
        Browser::Safari => {
            let getter = safari::items::cookie::CookiesGetter::build::<&str>(None).await?;
            getter.get_session_csrf(host)
        },

        chromium => {
            let getter = ChromiumGetter::build(chromium).await?;
            getter
                .get_session_csrf(host)
                .await?
        },
    };

    Ok(res)
}
