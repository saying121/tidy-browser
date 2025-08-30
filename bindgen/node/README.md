# Decrypt Cookies

Rust crate node bindgen.

Easily make a request using the authorization data from your browser.

```bash
npm add decrypt-cookies
```

```js
import { ChromiumGetter } from "decrypt-cookies";

var chromium_getter = await ChromiumGetter.new();
var all = await chromium_getter.cookiesAll();

for (let index = 0; index < 5; index++) {
    const element = all[index];
    console.log(element.decryptedValue);
}
```
