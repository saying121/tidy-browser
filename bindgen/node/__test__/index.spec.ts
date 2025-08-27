import test from "ava";

import { ChromiumGetter, FirefoxGetter } from "../index.js";

test("node test", (t) => {
	t.is(ChromiumGetter.name, "ChromiumGetter");
	t.is(FirefoxGetter.name, "FirefoxGetter");
});
