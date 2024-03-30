# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.3] - 2024-03-30

- Feat: Linux decrypt passwd
- Fix: win, mac: use correct arg for `from_utf8_lossy`

## [0.4.2] - 2024-03-30

- Perf: Linux decrypter retrieves all passwds at once, trading space for time.
