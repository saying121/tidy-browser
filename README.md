# Tidy Browser

<!--toc:start-->

- [Tidy Browser](#tidy-browser)
  - [Install And Usage](#install-and-usage)
  - [Core crate](#core-crate)
  - [Status](#status)
  <!--toc:end-->

## Install And Usage

Using [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)

```bash
cargo binstall tidy-browser

# Get data for all available browsers
tidy-browser -a
cd results

# Get Chrome cookie and login info
tidy-browser chromium -n Chrome -v cookie,login
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

## Shell completion

```bash
eval $(tidy-browser completions zsh)
eval $(tidy-browser completions <your shell>)
```

## Core crate

[decrypt-cookies](https://github.com/saying121/tidy-browser/tree/master/crates/decrypt-cookies)

## Status

[status](https://github.com/saying121/tidy-browser/tree/master/crates/decrypt-cookies/README.md#test-status)
