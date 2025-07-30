use std::{fmt::Display, fs::File, io::IoSlice, path::Path};

use binary_cookies::{
    cookie::Cookie,
    sync::{self, DecodeBinaryCookie},
};
use rayon::{iter::ParallelIterator, prelude::ParallelBridge};
use snafu::ResultExt;

use crate::{
    args::Format,
    error::{BinaryCookiesSnafu, IoSnafu, JsonSnafu, Result},
    utils,
};

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct BinaryCookiesWriter;

impl BinaryCookiesWriter {
    pub fn write_data<A, B, S>(path: A, out: B, sep: S, format: Format) -> Result<()>
    where
        A: AsRef<Path>,
        B: AsRef<Path>,
        S: Display + Send + Clone + 'static + Sync,
    {
        let path = path.as_ref();
        let out_path = out.as_ref();

        let file = File::open(path).with_context(|_| IoSnafu { path: path.to_owned() })?;
        let mut out = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(out_path)
            .with_context(|_| IoSnafu { path: out_path.to_owned() })?;

        let a = file
            .decode()
            .context(BinaryCookiesSnafu)?;
        let (pages_handle, _meta_decoder) = a.into_handles();

        match format {
            Format::Csv => {
                let csvs: Vec<_> = pages_handle
                    .decoders()
                    .par_bridge()
                    .filter_map(|mut v| v.decode().ok())
                    .map(sync::CookieHandle::into_decoders)
                    .flat_map(|v| {
                        v.par_bridge().filter_map(|mut v| {
                            v.decode()
                                .map(|v| v.to_csv(sep.clone()))
                                .ok()
                        })
                    })
                    .collect();

                let mut slices = Vec::with_capacity(2 + csvs.len() * 2);

                let header = Cookie::csv_header(sep.clone());
                slices.push(IoSlice::new(header.as_bytes()));
                slices.push(IoSlice::new(b"\n"));

                for cookie in &csvs {
                    slices.push(IoSlice::new(cookie.as_bytes()));
                    slices.push(IoSlice::new(b"\n"));
                }
                utils::write_all_vectored(&mut out, &mut slices)
                    .with_context(|_| IoSnafu { path: out_path.to_owned() })
            },
            Format::Json => {
                let cookies = pages_handle
                    .decoders()
                    .par_bridge()
                    .filter_map(|mut v| v.decode().ok())
                    .map(sync::CookieHandle::into_decoders)
                    .map(|v| {
                        v.par_bridge()
                            .filter_map(|mut v| v.decode().ok())
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();
                serde_json::to_writer(out, &cookies).context(JsonSnafu)
            },
        }
    }
}
