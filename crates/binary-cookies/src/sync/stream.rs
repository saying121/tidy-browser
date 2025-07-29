use std::{io::Read, mem};

use oval::Buffer;
use snafu::ResultExt;

use crate::{
    decode::{
        binary_cookies::BinaryCookieFsm,
        cookies::CookieFsm,
        meta::MetaFsm,
        pages::PageFsm,
        stream::{State, Values},
        DecodeResult,
    },
    error::{self, Result},
};

#[derive(Clone)]
#[derive(Debug)]
pub struct StreamDecoder<R: Read> {
    state: State,
    rd: R,
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
                let readed = self
                    .rd
                    .read(fsm.buffer.space())
                    .context(error::ReadSnafu)?;
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
                let readed = self
                    .rd
                    .read(fsm.buffer.space())
                    .context(error::ReadSnafu)?;
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
                let readed = self
                    .rd
                    .read(fsm.buffer.space())
                    .context(error::ReadSnafu)?;
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
                let readed = self
                    .rd
                    .read(fsm.buffer.space())
                    .context(error::ReadSnafu)?;
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
            State::Finished => Err(error::ParsingCompletedSnafu.build()),
            State::Transition => unreachable!(),
        }
    }
}
