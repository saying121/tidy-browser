use std::fs;

use snafu::{OptionExt, ResultExt, Whatever};

/// At python project root run: `cargo r && maturin develop`
#[snafu::report]
fn main() -> Result<(), Whatever> {
    let stub = decrypt_cookies_py::stub_info().whatever_context("Stub info")?;
    stub.generate()
        .whatever_context("Generte stub")?;
    let pyi_path = "./python/decrypt_cookies.pyi";
    let pyi = fs::read_to_string(pyi_path).whatever_context("Not found pyi file")?;

    let mut res = String::new();

    let mut last_class_name = "";

    for ele in pyi.lines() {
        if ele.starts_with("class") {
            let mut sp = ele.split(' ');
            sp.next();
            last_class_name = sp
                .next()
                .whatever_context("No class name")?
                .trim_end_matches(':');
        }

        if ele.contains("__new__") && !last_class_name.is_empty() {
            let new = ele.replace("ChromiumGetter", last_class_name);
            let new = new.replace("FirefoxGetter", last_class_name);

            let new = new.replace("FirefoxCookieGetter", last_class_name);
            let new = new.replace("ChromiumCookieGetter", last_class_name);

            res.push_str(&new);
            res.push('\n');
            continue;
        }

        res.push_str(ele);
        res.push('\n');
    }

    fs::write(pyi_path, res).whatever_context("Write failed")?;
    fs::rename(pyi_path, "./python/decrypt_cookies/decrypt_cookies.pyi")
        .whatever_context("Move file")?;

    Ok(())
}
