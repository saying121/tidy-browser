# Decrypt Cookies

<!--toc:start-->
- [Decrypt Cookies](#decrypt-cookies)
  - [Example](#example)
  - [To add new](#to-add-new)
    - [ChromiumBase](#chromiumbase)
    - [FirefoxBase](#firefoxbase)
  - [TODO](#todo)
  - [TEST STATUS](#test-status)
  - [Credits](#credits)
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

> [!NOTE]
>
> These are the latest status updates, not the released status.
> Please check out the newest tag for the released status.

- Linux:

|  Browser  | Cookies | passwd | Test Date |
| :-------: | :-----: | :----: | :-------: |
|  Firefox  |   🔑    |   🚫   | 2024-04-1 |
| Librewolf |   🔑    |   🚫   | 2024-04-1 |
|  Chrome   |   🔑    |   🔑   | 2024-04-1 |
|   Edge    |   🔑    |   🔑   | 2024-04-1 |
| Chromium  |   🔑    |   🔑   | 2024-04-1 |
|   Brave   |   🔑    |   🔑   | 2024-04-1 |
|  Yandex   |   🔑    |   🚫   | 2024-04-1 |
|  Vivaldi  |   🔑    |   🔑   | 2024-04-1 |
|   Opera   |   🔑    |   🔑   | 2024-04-1 |

- Windows:

|  Browser  |     Cookies     | passwd | Test Date |
| :-------: | :-------------: | :----: | :-------: |
|  Firefox  |       🔑        |   🚫   | 2024-04-1 |
| Librewolf |       🔑        |   🚫   | 2024-04-1 |
|  Chrome   |       🔑        |   🔑   | 2024-04-1 |
|   Edge    |       🔑        |   🔑   | 2024-04-1 |
| Chromium  |       🔑        |   🔑   | 2024-04-1 |
|   Brave   |       🔑        |   🔑   | 2024-04-1 |
|  Yandex   |       🔑        |   🚫   | 2024-04-1 |
|  Vivaldi  |       🔑        |   🔑   | 2024-04-1 |
|   Opera   |       🔑        |   🔑   | 2024-04-1 |
|  OperaGX  |       🔑        |   🔑   | 2024-04-1 |
|  CocCoc   |       🔑        |   🔑   | 2024-04-1 |
|    Arc    | 🚫(not support) |   🚫   |           |

- Macos:

|  Browser  |   Cookies    | passwd | Test Date |
| :-------: | :----------: | :----: | :-------: |
|  Firefox  |      🔑      |   🚫   | 2024-04-1 |
| Librewolf |      🔑      |   🚫   | 2024-04-1 |
|  Chrome   |      🔑      |   🔑   | 2024-04-1 |
|   Edge    |      🔑      |   🔑   | 2024-04-1 |
| Chromium  |      🔑      |   🔑   | 2024-04-1 |
|   Brave   |      🔑      |   🔑   | 2024-04-1 |
|  Yandex   |      🔑      |   🚫   | 2024-04-1 |
|  Vivaldi  |      🔑      |   🔑   | 2024-04-1 |
|   Opera   |      🔑      |   🔑   | 2024-04-1 |
|  OperaGX  |      🔑      |   🔑   | 2024-04-1 |
|  CocCoc   |      🔑      |   🔑   | 2024-04-1 |
|    Arc    | 🚫(not test) |   🚫   |           |
|  safari   |      🔑      |   🚫   | 2024-04-1 |

## Credits

- [HackBrowserData](https://github.com/moonD4rk/HackBrowserData)
