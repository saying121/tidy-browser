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

## TODO

- The database/file can sometimes be locked.
- Decrypt passwd etc.

## TEST STATUS

- Linux:

|  Browser  | Cookies | passwd | Test Date  |
| :-------: | :-----: | :----: | :--------: |
|  Firefox  |   🔑    |   🚫   | 2024-03-25 |
| Librewolf |   🔑    |   🚫   | 2024-03-25 |
|  Chrome   |   🔑    |   🚫   | 2024-03-25 |
|   Edge    |   🔑    |   🚫   | 2024-03-25 |
| Chromium  |   🔑    |   🚫   | 2024-03-25 |
|   Brave   |   🔑    |   🚫   | 2024-03-25 |
|  Yandex   |   🔑    |   🚫   | 2024-03-25 |
|  Vivaldi  |   🔑    |   🚫   | 2024-03-25 |
|   Opera   |   🔑    |   🚫   | 2024-03-25 |

- Windows:

|  Browser  |     Cookies     | passwd | Test Date  |
| :-------: | :-------------: | :----: | :--------: |
|  Firefox  |       🔑        |   🚫   | 2024-03-25 |
| Librewolf |       🔑        |   🚫   | 2024-03-25 |
|  Chrome   |       🔑        |   🚫   | 2024-03-25 |
|   Edge    |       🔑        |   🚫   | 2024-03-25 |
| Chromium  |       🔑        |   🚫   | 2024-03-25 |
|   Brave   |       🔑        |   🚫   | 2024-03-25 |
|  Yandex   |       🔑        |   🚫   | 2024-03-25 |
|  Vivaldi  |       🔑        |   🚫   | 2024-03-25 |
|   Opera   |       🔑        |   🚫   | 2024-03-25 |
|  OperaGX  |       🔑        |   🚫   | 2024-03-25 |
|  CocCoc   |       🔑        |   🚫   | 2024-03-25 |
|    Arc    | 🚫(not support) |   🚫   |            |

- Macos:

|  Browser  |   Cookies    | passwd | Test Date  |
| :-------: | :----------: | :----: | :--------: |
|  Firefox  |      🔑      |   🚫   | 2024-03-25 |
| Librewolf |      🔑      |   🚫   | 2024-03-25 |
|  Chrome   |      🔑      |   🚫   | 2024-03-25 |
|   Edge    |      🔑      |   🚫   | 2024-03-25 |
| Chromium  |      🔑      |   🚫   | 2024-03-25 |
|   Brave   |      🔑      |   🚫   | 2024-03-25 |
|  Yandex   |      🔑      |   🚫   | 2024-03-25 |
|  Vivaldi  |      🔑      |   🚫   | 2024-03-25 |
|   Opera   |      🔑      |   🚫   | 2024-03-25 |
|  OperaGX  |      🔑      |   🚫   | 2024-03-25 |
|  CocCoc   |      🔑      |   🚫   | 2024-03-25 |
|    Arc    | 🚫(not test) |   🚫   |            |
|  safari   |      🔑      |   🚫   | 2024-03-25 |

## Thanks To

[HackBrowserData](https://github.com/moonD4rk/HackBrowserData)
