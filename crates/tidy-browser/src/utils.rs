use std::{
    fmt::Display,
    fs::File,
    io::{ErrorKind, IoSlice, Write},
    path::PathBuf,
};

use decrypt_cookies::{chromium::ChromiumCookie, prelude::cookies::CookiesInfo};
use snafu::ResultExt;
use tokio::task::{self, JoinHandle};

use crate::error;

/// Copy from nightly [`Write::write_all_vectored`]
/// TODO: Use std method
pub(crate) fn write_all_vectored(
    file: &mut File,
    mut bufs: &mut [IoSlice<'_>],
) -> std::io::Result<()> {
    IoSlice::advance_slices(&mut bufs, 0);
    while !bufs.is_empty() {
        match file.write_vectored(bufs) {
            Ok(0) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::WriteZero,
                    "failed to write whole buffer",
                ));
            },
            Ok(n) => IoSlice::advance_slices(&mut bufs, n),
            Err(ref e) if matches!(e.kind(), ErrorKind::Interrupted) => {},
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

pub(crate) fn write_cookies<S>(
    out_file: PathBuf,
    cookies: Vec<impl CookiesInfo + Send + 'static>,
    sep: S,
) -> JoinHandle<Result<(), error::Error>>
where
    S: Display + Send + Clone + 'static,
{
    task::spawn_blocking(move || {
        let mut file = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&out_file)
            .context(error::IoSnafu { path: out_file.clone() })?;

        let header = <ChromiumCookie as CookiesInfo>::csv_header(sep.clone());

        let mut slices = Vec::with_capacity(2 + cookies.len() * 2);
        slices.push(IoSlice::new(header.as_bytes()));
        slices.push(IoSlice::new(b"\n"));

        let csvs: Vec<_> = cookies
            .into_iter()
            .map(|v| v.to_csv(sep.clone()))
            .collect();

        for csv in &csvs {
            slices.push(IoSlice::new(csv.as_bytes()));
            slices.push(IoSlice::new(b"\n"));
        }

        write_all_vectored(&mut file, &mut slices)
            .context(error::IoSnafu { path: out_file.clone() })?;

        Ok::<(), error::Error>(())
    })
}
