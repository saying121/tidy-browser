pub mod browser;
pub mod chromium;
pub mod firefox;

use browser::{Browser, Cookies};
use miette::Result;

/// get csrf and session
///
/// * `borwser`: firefox, librewolf, edge, chrome
pub async fn get_cookie<T>(borwser: T, host: &str) -> Result<Cookies>
where
    T: Into<Browser>,
{
    let res = match borwser.into() {
        Browser::Firefox => firefox::get_session_csrf(Browser::Firefox, host).await?,
        Browser::Librewolf => firefox::get_session_csrf(Browser::Librewolf, host).await?,

        Browser::Edge => chromium::items::cookie::get_session_csrf(Browser::Edge, host).await?,
        Browser::Chrome => chromium::items::cookie::get_session_csrf(Browser::Chrome, host).await?,
        Browser::Chromium => {
            chromium::items::cookie::get_session_csrf(Browser::Chromium, host).await?
        },
        Browser::Brave => chromium::items::cookie::get_session_csrf(Browser::Brave, host).await?,
        Browser::Yandex => chromium::items::cookie::get_session_csrf(Browser::Yandex, host).await?,
        Browser::Vivaldi => {
            chromium::items::cookie::get_session_csrf(Browser::Vivaldi, host).await?
        },
        Browser::Opera => chromium::items::cookie::get_session_csrf(Browser::Opera, host).await?,

        #[cfg(not(target_os = "linux"))]
        Browser::OperaGX => {
            chromium::items::cookie::get_session_csrf(Browser::OperaGX, host).await?
        },
        #[cfg(not(target_os = "linux"))]
        Browser::CocCoc => chromium::items::cookie::get_session_csrf(Browser::CocCoc, host).await?,
        #[cfg(not(target_os = "linux"))]
        Browser::Arc => chromium::items::cookie::get_session_csrf(Browser::Arc, host).await?,
        #[cfg(target_os = "macos")]
        Browser::Safari => unimplemented!(),
    };

    Ok(res)
}
