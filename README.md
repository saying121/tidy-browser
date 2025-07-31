# Tidy Browser

<!--toc:start-->

- [Tidy Browser](#tidy-browser)
  - [Install And Usage](#install-and-usage)
  - [Core crate](#core-crate)
  - [Status](#status)
  <!--toc:end-->

## Install And Usage

```bash
cargo install tidy-browser

# Get data for all available browsers
tidy-browser -a
cd results

# Filter by host/domain
tidy-browser -a --host github.com
cd results

# Available data formats: csv, json, jsonl(jsonlines)
tidy-browser --out-format json -a

# Parse BinaryCookies
tidy-browser binary-cookies -i ~/Library/Containers/com.apple.Safari/Data/Library/Cookies/Cookies.binarycookies
cat ./binary_cookies.csv
```

## Core crate

[decrypt-cookies](./crates/decrypt-cookies)

## Status

[status](./crates/decrypt-cookies/README.md#test-status)
