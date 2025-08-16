use snafu::{ResultExt, Whatever};

fn main() -> Result<(), Whatever> {
    let stub = decrypt_cookies_py::stub_info().whatever_context("Stub info")?;
    stub.generate()
        .whatever_context("generte stub")?;
    Ok(())
}
