# Decrypt Cookies

<!--toc:start-->

- [Decrypt Cookies](#decrypt-cookies)
  - [Example](#example)
  - [TODO](#todo)
  - [TEST STATUS](#test-status)
  - [Thanks To](#thanks-to)
  <!--toc:end-->

## Example

```rust
use decrypt_cookies::{Browser, ChromiumBuilder};

#[tokio::main]
async fn main() -> miette::Result<()> {
    let chromium = ChromiumBuilder::new(Browser::Chromium)
        .build()
        .await?;
    let all_cookies = chromium.get_cookies_all().await?;

    dbg!(&all_cookies[0]);

    Ok(())
}
```

## To add new

### ChromiumBase

- ./src/browser/info.rs `ChromiumInfo` trait,
  `safe_name`, `storage` method, modify and impl.

### FirefoxBase

- ./src/browser/info.rs `FfInfo` trait, modify and impl.

## TODO

- Decrypt passwd etc.

## TEST STATUS

- Linux:

|  Browser  | Cookies | passwd | Test Date  |
| :-------: | :-----: | :----: | :--------: |
|  Firefox  |   🔑    |   🚫   | 2024-03-29 |
| Librewolf |   🔑    |   🚫   | 2024-03-29 |
|  Chrome   |   🔑    |   🚫   | 2024-03-29 |
|   Edge    |   🔑    |   🚫   | 2024-03-29 |
| Chromium  |   🔑    |   🚫   | 2024-03-29 |
|   Brave   |   🔑    |   🚫   | 2024-03-29 |
|  Yandex   |   🔑    |   🚫   | 2024-03-29 |
|  Vivaldi  |   🔑    |   🚫   | 2024-03-29 |
|   Opera   |   🔑    |   🚫   | 2024-03-29 |

- Windows:

|  Browser  |     Cookies     | passwd | Test Date  |
| :-------: | :-------------: | :----: | :--------: |
|  Firefox  |       🔑        |   🚫   | 2024-03-29 |
| Librewolf |       🔑        |   🚫   | 2024-03-29 |
|  Chrome   |       🔑        |   🚫   | 2024-03-29 |
|   Edge    |       🔑        |   🚫   | 2024-03-29 |
| Chromium  |       🔑        |   🚫   | 2024-03-29 |
|   Brave   |       🔑        |   🚫   | 2024-03-29 |
|  Yandex   |       🔑        |   🚫   | 2024-03-29 |
|  Vivaldi  |       🔑        |   🚫   | 2024-03-29 |
|   Opera   |       🔑        |   🚫   | 2024-03-29 |
|  OperaGX  |       🔑        |   🚫   | 2024-03-29 |
|  CocCoc   |       🔑        |   🚫   | 2024-03-29 |
|    Arc    | 🚫(not support) |   🚫   |            |

- Macos:

|  Browser  |   Cookies    | passwd | Test Date  |
| :-------: | :----------: | :----: | :--------: |
|  Firefox  |      🔑      |   🚫   | 2024-03-29 |
| Librewolf |      🔑      |   🚫   | 2024-03-29 |
|  Chrome   |      🔑      |   🚫   | 2024-03-29 |
|   Edge    |      🔑      |   🚫   | 2024-03-29 |
| Chromium  |      🔑      |   🚫   | 2024-03-29 |
|   Brave   |      🔑      |   🚫   | 2024-03-29 |
|  Yandex   |      🔑      |   🚫   | 2024-03-29 |
|  Vivaldi  |      🔑      |   🚫   | 2024-03-29 |
|   Opera   |      🔑      |   🚫   | 2024-03-29 |
|  OperaGX  |      🔑      |   🚫   | 2024-03-29 |
|  CocCoc   |      🔑      |   🚫   | 2024-03-29 |
|    Arc    | 🚫(not test) |   🚫   |            |
|  safari   |      🔑      |   🚫   | 2024-03-29 |

## Thanks To

[HackBrowserData](https://github.com/moonD4rk/HackBrowserData)
