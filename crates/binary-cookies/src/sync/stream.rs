use std::{io::Read, mem};

use oval::Buffer;

use crate::{
    cookie::{Checksum, Cookie, Metadata},
    decode::{
        binary_cookies::BinaryCookieFsm,
        cookies::{CookieFsm, CookiesOffsetInPage},
        meta::{MetaFsm, MetaOffset},
        pages::{PageFsm, PagesOffset},
        DecodeResult,
    },
    error::{ParseError, Result},
};

#[derive(Clone)]
#[derive(Debug)]
pub struct StreamDecoder<R: Read> {
    state: State,
    rd: R,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
enum State {
    Bc {
        fsm: BinaryCookieFsm,
    },
    Page {
        fsm: PageFsm,
        remaining_page: u32,
    },
    Cookie {
        fsm: CookieFsm,
        remaining_cookie: u32,
        remaining_page: u32,
    },
    Meta {
        fsm: MetaFsm,
    },
    Finished,
    #[default]
    Transition,
}

#[derive(Clone)]
#[derive(Debug)]
pub enum Values {
    /// Some metadata
    Bc {
        meta_offset: MetaOffset,
        pages_offset: PagesOffset,
    },
    /// Some metadata
    Page(CookiesOffsetInPage),
    /// A cookie
    Cookie(Cookie),
    /// Binarycookies metadata
    Meta {
        checksum: Checksum,
        meta: Option<Metadata>,
    },
}

impl<R: Read> StreamDecoder<R> {
    const BUF_SIZE: usize = 64;

    pub fn new(rd: R) -> Self {
        Self {
            state: State::Bc {
                fsm: BinaryCookieFsm {
                    buffer: Buffer::with_capacity(Self::BUF_SIZE),
                },
            },
            rd,
        }
    }

    pub fn decode(&mut self) -> Result<Values> {
        match mem::take(&mut self.state) {
            State::Bc { mut fsm } => loop {
                let readed = self.rd.read(fsm.buffer.space())?;
                fsm.buffer.fill(readed);

                match fsm.process()? {
                    DecodeResult::Continue(fsm_) => {
                        fsm = fsm_;
                        continue;
                    },
                    DecodeResult::Done((meta_offset, pages_offset, buffer)) => {
                        self.state = State::Page {
                            fsm: PageFsm { buffer },
                            remaining_page: pages_offset.offset_sizes.len() as u32,
                        };
                        return Ok(Values::Bc { meta_offset, pages_offset });
                    },
                }
            },
            State::Page { mut fsm, remaining_page } => loop {
                let readed = self.rd.read(fsm.buffer.space())?;
                fsm.buffer.fill(readed);
                match fsm.process()? {
                    DecodeResult::Continue(fsm_) => {
                        fsm = fsm_;
                        continue;
                    },
                    DecodeResult::Done((c, buffer)) => {
                        self.state = State::Cookie {
                            fsm: CookieFsm { buffer },
                            remaining_cookie: c.len() as u32,
                            remaining_page: remaining_page - 1,
                        };
                        return Ok(Values::Page(c));
                    },
                }
            },
            State::Cookie {
                remaining_cookie,
                remaining_page,
                mut fsm,
            } => loop {
                let readed = self.rd.read(fsm.buffer.space())?;
                fsm.buffer.fill(readed);

                match fsm.process()? {
                    DecodeResult::Continue(fsm_) => {
                        fsm = fsm_;
                        continue;
                    },
                    DecodeResult::Done((cookie, buffer)) => {
                        let remaining_cookie = remaining_cookie - 1;
                        match (remaining_cookie, remaining_page) {
                            (0, 0) => self.state = State::Meta { fsm: MetaFsm { buffer } },
                            (0, _) => {
                                self.state = State::Page {
                                    fsm: PageFsm { buffer },
                                    remaining_page,
                                };
                            },
                            _ => {
                                self.state = State::Cookie {
                                    fsm: CookieFsm { buffer },
                                    remaining_cookie,
                                    remaining_page,
                                };
                            },
                        }
                        return Ok(Values::Cookie(cookie));
                    },
                }
            },
            State::Meta { mut fsm } => loop {
                let readed = self.rd.read(fsm.buffer.space())?;
                fsm.buffer.fill(readed);

                match fsm.process()? {
                    DecodeResult::Continue(fsm_) => {
                        fsm = fsm_;
                        continue;
                    },
                    DecodeResult::Done((checksum, meta)) => {
                        self.state = State::Finished;
                        return Ok(Values::Meta { checksum, meta });
                    },
                }
            },
            State::Finished => Err(ParseError::ParsingCompleted),
            State::Transition => unreachable!(),
        }
    }
}
