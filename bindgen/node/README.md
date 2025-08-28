# Decrypt Cookies

Rust crate python bindgen.

Easily make a request using the authorization data from your browser.

```bash
npm add decrypt-cookies
```

```js
import { ChromeGetter, EdgeGetter } from "decrypt-cookies";

var chromium_getter = await EdgeGetter.new();
var all = await chromium_getter.cookiesAll();
console.log("===" + all.length);
for (let index = 0; index < 5; index++) {
	const element = all[index];
	console.log(element.decryptedValue);
}
```
