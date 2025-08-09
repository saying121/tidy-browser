# Decrypt Cookies

<!--toc:start-->

- [Decrypt Cookies](#decrypt-cookies)
  - [Example](#example)
  - [To add a new browser](#to-add-a-new-browser)
  - [Test Status](#test-status)
  - [TODO](#todo)
  - [Credits](#credits)
  <!--toc:end-->

## Example

Easily make a request using the authorization data from your browser.

See: [reqwest](./examples/reqwest.rs)

```rust
use std::sync::Arc;

use decrypt_cookies::{chromium::GetCookies, prelude::*};
use reqwest::cookie::Jar;
use snafu::{ResultExt, Whatever};

#[snafu::report]
#[tokio::main]
async fn main() -> Result<(), Whatever> {
    let chromium = ChromiumBuilder::<Chrome>::new()
        .build()
        .await
        .whatever_context("Chromium build failed")?;
    let all_cookies: Jar = chromium
        .cookies_all()
        .await
        .whatever_context("Get cookies failed")?
        .into_iter()
        .collect();

    let client = reqwest::Client::builder()
        .cookie_provider(Arc::new(all_cookies))
        .build()
        .whatever_context("reqwest Client build failed")?;

    let resp = client
        .get("https://www.rust-lang.org")
        .send()
        .await
        .whatever_context("Get send failed")?
        .text()
        .await
        .whatever_context("get text failed")?;
    println!("{resp}");

    Ok(())
}
```

## To add a new browser

- `BASE`: A browser all data location relative to home dir.
- `COOKIES`, `LOGIN_DATA`, `KEY`: Relative to `BASE` path.
- `NAME`: browser name

Implement [`ChromiumInfo`, `FirefoxPath`](./src/browser/mod.rs) trait.

## Test Status

> [!NOTE]
>
> These are the latest status updates, not the released status.
> Please check out the newest tag for the released status.

- Linux:

|    Browser    | Cookies | Passwd | Test Date  |    Version     |
| :-----------: | :-----: | :----: | :--------: | :------------: |
|  [`Firefox`]  |   🔑    |   🚫   | 2025-07-19 |    140.0.4     |
| [`Librewolf`] |   🔑    |   🚫   | 2025-07-19 |   140.0.2-1    |
|  [`Floorp`]   |   🔑    |   🚫   | 2025-07-19 |     141.0      |
|    [`Zen`]    |   🔑    |   🚫   | 2025-08-02 |    1.14.5b     |
|  [`Chrome`]   |   🔑    |   🔑   | 2025-07-19 | 138.0.7204.157 |
|   [`Edge`]    |   🔑    |   🔑   | 2025-07-19 | 138.0.3351.95  |
| [`Chromium`]  |   🔑    |   🔑   | 2025-07-19 | 138.0.7204.157 |
|   [`Brave`]   |   🔑    |   🔑   | 2025-07-19 |  138.1.80.122  |
|  [`Yandex`]   |   🔑    |   🚫   | 2025-07-19 |  25.4.1.1213   |
|  [`Vivaldi`]  |   🔑    |   🔑   | 2025-07-19 |  7.5.3735.54   |
|   [`Opera`]   |   🔑    |   🔑   | 2025-07-19 | 120.0.5543.93  |

- Windows:

|    Browser    | Cookies | Passwd | Test Date  |    Version     |
| :-----------: | :-----: | :----: | :--------: | :------------: |
|  [`Firefox`]  |   🔑    |   🚫   | 2025-07-20 |    140.0.4     |
| [`Librewolf`] |   🔑    |   🚫   | 2025-07-20 |   140.0.4-1    |
|  [`Floorp`]   |   🔑    |   🚫   | 2025-07-20 |    11.28.0     |
|    [`Zen`]    |   🔑    |   🚫   | 2025-08-02 |    1.14.9b     |
|  [`Chrome`]   |   🔑    |   🔑   | 2025-07-20 | 138.0.7204.158 |
|   [`Edge`]    |   🔑    |   🔑   | 2025-07-20 | 138.0.3351.95  |
| [`Chromium`]  |   🔑    |   🔑   | 2025-07-20 | 138.0.7204.158 |
|   [`Brave`]   |   🔑    |   🔑   | 2025-07-20 |  138.1.80.122  |
|  [`Yandex`]   |   🔑    |   🚫   | 2025-07-20 |   25.6.2.425   |
|  [`Vivaldi`]  |   🔑    |   🔑   | 2025-07-20 |  7.5.3735.54   |
|   [`Opera`]   |   🔑    |   🔑   | 2025-07-20 | 120.0.5543.93  |
|  [`OperaGX`]  |   🔑    |   🔑   | 2025-07-20 | 119.0.5497.186 |
|  [`CocCoc`]   |   🔑    |   🔑   | 2025-07-20 | 137.0.7151.124 |
|    [`Arc`]    |   🔑    |   🔑   | 2025-07-20 |   1.62.0.172   |

- Macos:

|    Browser    | Cookies | Passwd | Test Date  |                      Version                       |
| :-----------: | :-----: | :----: | :--------: | :------------------------------------------------: |
|  [`Firefox`]  |   🔑    |   🚫   | 2025-07-19 |                      140.0.4                       |
| [`Librewolf`] |   🔑    |   🚫   | 2025-07-19 |                     140.0.4,1                      |
|  [`Floorp`]   |   🔑    |   🚫   | 2025-07-19 |                      12.0.15                       |
|    [`Zen`]    |   🔑    |   🚫   | 2025-08-02 |                      1.14.9b                       |
|  [`Chrome`]   |   🔑    |   🔑   | 2025-07-19 |                   138.0.7204.158                   |
|   [`Edge`]    |   🔑    |   🔑   | 2025-07-19 | 138.0.3351.95,70a9712a-3712-420f-a3f0-8f2032f1c838 |
| [`Chromium`]  |   🔑    |   🔑   | 2025-07-19 |                    140.0.7306.0                    |
|   [`Brave`]   |   🔑    |   🔑   | 2025-07-19 |                     1.80.122.0                     |
|  [`Yandex`]   |   🔑    |   🚫   | 2025-07-19 |                 25.6.0.2391,84025                  |
|  [`Vivaldi`]  |   🔑    |   🔑   | 2025-07-19 |                    7.5.3735.54                     |
|   [`Opera`]   |   🔑    |   🔑   | 2025-07-19 |                   120.0.5543.93                    |
|  [`OperaGX`]  |   🔑    |   🔑   | 2025-07-19 |                   120.0.5543.85                    |
|  [`CocCoc`]   |   🔑    |   🔑   | 2025-07-19 |                   136.0.7103.154                   |
|    [`Arc`]    |   🔑    |   🔑   | 2025-07-19 |                   1.104.0,65533                    |
|   `Safari`    |   🔑    |   🚫   | 2025-07-19 |                                                    |

## TODO

- Decrypt firefox passwd.

## Credits

- [HackBrowserData](https://github.com/moonD4rk/HackBrowserData)

<!-- links -->

[`Firefox`]: crate::browser::firefox::Firefox
[`Librewolf`]: crate::browser::firefox::Librewolf
[`Floorp`]: crate::browser::firefox::Floorp
[`Zen`]: crate::browser::firefox::Zen
[`Chrome`]: crate::browser::chromium::Chrome
[`Edge`]: crate::browser::chromium::Edge
[`Chromium`]: crate::browser::chromium::Chromium
[`Brave`]: crate::browser::chromium::Brave
[`Yandex`]: crate::browser::chromium::Yandex
[`Vivaldi`]: crate::browser::chromium::Vivaldi
[`Opera`]: crate::browser::chromium::Opera
[`OperaGX`]: crate::browser::chromium::OperaGX
[`CocCoc`]: crate::browser::chromium::CocCoc
[`Arc`]: crate::browser::chromium::Arc
