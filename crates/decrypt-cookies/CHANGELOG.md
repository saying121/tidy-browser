<!-- markdownlint-disable MD024 -->
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Filter safe storage in Linux for cache.

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
