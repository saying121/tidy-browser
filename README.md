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

tidy-browser -a
cd results

# Parse BinaryCookies
tidy-browser binary-cookies -i ~/Library/Containers/com.apple.Safari/Data/Library/Cookies/Cookies.binarycookies
cat ./binary_cookies.csv
```

## Core crate

[decrypt-cookies](./crates/decrypt-cookies)

## Status

[status](./crates/decrypt-cookies/README.md#test-status)
