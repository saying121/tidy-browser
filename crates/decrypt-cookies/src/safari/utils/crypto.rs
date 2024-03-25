use crate::Browser;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct Decrypter {
    browser: Browser,
    pass:    Vec<u8>,
}

impl Decrypter {
    pub fn new(browser: Browser) -> Self {
        unimplemented!()
    }
}
