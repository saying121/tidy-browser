#!/usr/bin/env python3

r"""
.. include:: ./README.md
"""

from collections.abc import Sequence
from .decrypt_cookies import *  # noqa: F403

from datetime import datetime
from http.cookiejar import Cookie, CookieJar
from decrypt_cookies import ChromiumCookie, MozCookie

__all__ = ["to_cookiejar"]


def to_cookiejar(cookies: Sequence[ChromiumCookie | MozCookie]) -> CookieJar:
    cookiejar = CookieJar()
    for cookie in cookies:
        if isinstance(cookie, ChromiumCookie):
            cookie.expires_utc
            cookiejar.set_cookie(
                cookie_new(
                    host=cookie.host_key,
                    expires_utc=cookie.expires_utc,
                    is_httponly=cookie.is_httponly,
                    is_secure=cookie.is_secure,
                    name=cookie.name,
                    path=cookie.path,
                    value=cookie.value,
                )
            )
        else:
            cookiejar.set_cookie(
                cookie_new(
                    host=cookie.host,
                    expires_utc=cookie.expiry,
                    is_httponly=cookie.is_http_only,
                    is_secure=cookie.is_secure,
                    name=cookie.name,
                    path=cookie.path,
                    value=cookie.value,
                )
            )
    return cookiejar


def cookie_new(
    host: str,
    value: str,
    name: str,
    path: str,
    is_secure: bool,
    is_httponly: bool,
    expires_utc: datetime | None,
) -> Cookie:
    return Cookie(
        port=None,
        comment=None,
        comment_url=None,
        discard=False,
        path_specified=True,
        port_specified=False,
        version=0,
        domain_initial_dot=host.startswith("."),
        domain_specified=True,
        domain=host,
        value=value,
        name=name,
        path=path,
        secure=is_secure,
        expires=None if expires_utc is None else int(expires_utc.timestamp()),
        rest={"HttpOnly": ""} if is_httponly else {},
    )
