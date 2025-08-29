use napi_derive::napi;

mod chromium;
mod firefox;
mod safari;

#[napi]
#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum SameSite {
    #[default]
    None = 0,
    Lax = 1,
    Strict = 2,
}
