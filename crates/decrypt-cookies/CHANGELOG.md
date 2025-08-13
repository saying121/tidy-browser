<!-- markdownlint-disable MD024 -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.10.2] - 2025-08-13

### Added

- Export macros for easy browser registration

## [0.10.1] - 2025-08-09

### Changed

- Update chromium-crypto

## [0.10.0] - 2025-08-06

### Changed

- Split cookies and logins

## [0.9.2] - 2025-08-03

### Added

- Add feature control

## [0.9.1] - 2025-08-02

### Added

- Support Zen

### Changed

## [0.9.0] - 2025-08-01

### Changed

- Better error handle

## [0.8.0] - 2025-07-27

### Changed

- Rename apis: `*Getter`: `all_cookies` -> `cookies_all`, `CookiesInfo`: `get_set_cookie_header` -> `set_cookie_header`, `get_url` -> `url`.
- Rename Error: `HOME` -> `Home`

### Added

- SafariGetter: add `cookies_by_host`.

## [0.7.0] - 2025-07-20

### Changed

- Use `binary-cookies` crate for Safari cookies.
- Rename apis

## [0.6.1] - 2024-08-31

### Changed

- Update dependencies

## [0.6.0] - 2024-07-13

### Added

- Filter safe storage in Linux for cache.
- Parse SameSite.
- Add `get_set_cookie_header` and other method take from [bench_scraper](https://github.com/goakley/bench_scraper/blob/main/src/cookie.rs#L43)

### Changed

- pub use `ColumnTrait`.
- `Browser::chromiums` and `Browser::firefoxs` return iterator.
- Cookie times return `Option`.
- Rename `DecryptedCookies` -> `ChromiumCookie`

## [0.5.3] - 2024-04-19

### Added

- Check cookies expiry.

### Fixed

- Firefox: `secs_to_moz_utc` method.

## [0.5.2] - 2024-04-02

- Change: pub `SafariCookie` fields

## [0.5.1] - 2024-03-30

### Fixed

- Double import

## [0.5.0] - 2024-03-30

### Added

- decrypt chromium based passwd

## [0.4.3] - 2024-03-30

### Added

- Linux decrypt chromium based passwd

### Fixed

- Win, Mac: use correct arg for `from_utf8_lossy`

## [0.4.2] - 2024-03-30

### Added

- Perf: Linux decrypter retrieves all passwds at once, trading space for time.
