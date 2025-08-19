#!/usr/bin/env python3

import asyncio
from http.cookiejar import CookieJar
from decrypt_cookies import ChromeGetter, to_cookiejar, SafariGetter
from decrypt_cookies.decrypt_cookies import FirefoxGetter
import requests


async def test_c() -> CookieJar:
    safari_getter = await SafariGetter()
    cookies = safari_getter.cookies_all()

    ff_getter = await FirefoxGetter()
    cookies = await ff_getter.cookies_all()

    getter = await ChromeGetter()
    cookies = await getter.cookies_all()
    return to_cookiejar(cookies)


jar = asyncio.run(test_c())
a = requests.get("https://pypi.org/", cookies=jar)
print(a.text)
