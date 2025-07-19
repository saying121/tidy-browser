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
    let all_cookies = chromium.all_cookies().await?;

    dbg!(&all_cookies[0]);

    let jar: reqwest::cookie::Jar = all_cookies
        .into_iter()
        .collect();

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

|  Browser  | Cookies | passwd | Test Date  |    Version     |
| :-------: | :-----: | :----: | :--------: | :------------: |
|  Firefox  |   🔑    |   🚫   | 2025-07-19 |    140.0.4     |
| Librewolf |   🔑    |   🚫   | 2025-07-19 |   140.0.2-1    |
|  Floorp   |   🔑    |   🚫   | 2025-07-19 |     141.0      |
|  Chrome   |   🔑    |   🔑   | 2025-07-19 | 138.0.7204.157 |
|   Edge    |   🔑    |   🔑   | 2025-07-19 | 138.0.3351.95  |
| Chromium  |   🔑    |   🔑   | 2025-07-19 | 138.0.7204.157 |
|   Brave   |   🔑    |   🔑   | 2025-07-19 |  138.1.80.122  |
|  Yandex   |   🔑    |   🚫   | 2025-07-19 |  25.4.1.1213   |
|  Vivaldi  |   🔑    |   🔑   | 2025-07-19 |  7.5.3735.54   |
|   Opera   |   🔑    |   🔑   | 2025-07-19 | 120.0.5543.93  |

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

|  Browser  | Cookies | passwd | Test Date  |      version (`brew info *` or `* --version`)      |
| :-------: | :-----: | :----: | :--------: | :------------------------------------------------: |
|  Firefox  |   🔑    |   🚫   | 2025-07-19 |                      140.0.4                       |
| Librewolf |   🔑    |   🚫   | 2025-07-19 |                     140.0.4,1                      |
|  Chrome   |   🔑    |   🔑   | 2025-07-19 |                   138.0.7204.158                   |
|   Edge    |   🔑    |   🔑   | 2025-07-19 | 138.0.3351.95,70a9712a-3712-420f-a3f0-8f2032f1c838 |
| Chromium  |   🔑    |   🔑   | 2025-07-19 |                    140.0.7306.0                    |
|   Brave   |   🔑    |   🔑   | 2025-07-19 |                     1.80.122.0                     |
|  Yandex   |   🔑    |   🚫   | 2025-07-19 |                 25.6.0.2391,84025                  |
|  Vivaldi  |   🔑    |   🔑   | 2025-07-19 |                    7.5.3735.54                     |
|   Opera   |   🔑    |   🔑   | 2025-07-19 |                   120.0.5543.93                    |
|  OperaGX  |   🔑    |   🔑   | 2025-07-19 |                   120.0.5543.85                    |
|  CocCoc   |   🔑    |   🔑   | 2025-07-19 |                   136.0.7103.154                   |
|    Arc    |   🔑    |   🔑   | 2025-07-19 |                   1.104.0,65533                    |
|  Safari   |   🔑    |   🚫   | 2025-07-19 |                                                    |

## Credits

- [HackBrowserData](https://github.com/moonD4rk/HackBrowserData)
