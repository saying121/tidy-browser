#!/usr/bin/env python3

import asyncio
from http.cookiejar import CookieJar
from decrypt_cookies import ChromeGetter, to_cookiejar
import requests


async def test_c() -> CookieJar:
    getter = await ChromeGetter()
    cookies = await getter.cookies_all()
    return to_cookiejar(cookies)


jar = asyncio.run(test_c())
a = requests.get("https://pypi.org/", cookies=jar)
print(a.text)
