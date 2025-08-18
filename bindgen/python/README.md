# Decrypt Cookies

Rust crate python bindgen.

Easily make a request using the authorization data from your browser.

[example](./python/example.py)

```python
#!/usr/bin/env python3

import asyncio

from decrypt_cookies import ChromeGetter

async def get_cookies():
    chrome_getter = await ChromeGetter()
    c = await chrome_getter.cookies_all()
    print(c[0])


asyncio.run(test_c())
```

## Status

[status](https://github.com/saying121/tidy-browser/tree/master/crates/decrypt-cookies/README.md#test-status)
