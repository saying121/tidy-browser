use oval::Buffer;
use winnow::{
    error::{ErrMode, Needed},
    Parser,
};

use super::DecodeResult;
use crate::{
    cookie::{BinaryCookies, Checksum, Metadata},
    decode::StreamIn,
    error::{ParseError, Result},
};

/// Decoder page size and the end `plist` metadata
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct BinaryCookieDecoder {
    state: State,
    buffer: Buffer,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct PagesDecoder {
    offsets: Offsets,
}

type ProcessResult =
    Result<DecodeResult<BinaryCookieDecoder, (Checksum, Option<Metadata>, PagesDecoder)>>;

impl BinaryCookieDecoder {
    /// 4(`signature`) + 4(`num_pages`) + 4 * `num_pages`
    /// assume `num_pages` = 6 round up to a power of 2
    const BUF_SIZE: usize = 4 + 4 + 4 * 6;

    pub fn new() -> Self {
        Self {
            state: State::Head,
            buffer: Buffer::with_capacity(Self::BUF_SIZE),
        }
    }

    pub fn wants_read(&self) -> usize {
        match &self.state {
            State::Head => self.buffer.available_data(),
            State::Tail { offsets } => self.buffer.available_data() + offsets.tail_offset,
        }
    }

    pub fn process(mut self) -> ProcessResult {
        let mut input: StreamIn = StreamIn::new(self.buffer.data());
        // let start = input.checkpoint();
        match self.state {
            State::Head => {
                let e = match BinaryCookies::parser_head.parse_next(&mut input) {
                    Ok(offsets) => {
                        self.state = State::Tail { offsets };
                        self.buffer.reset();
                        return Ok(DecodeResult::Continue(self));
                    },
                    Err(e) => e,
                };
                match e {
                    ErrMode::Backtrack(e) | ErrMode::Cut(e) => Err(ParseError::WinnowCtx(e)),
                    ErrMode::Incomplete(Needed::Unknown) => {
                        // The branch is unreachable?
                        let new_cap = self.buffer.capacity() * 2;
                        self.buffer.grow(new_cap);
                        Ok(DecodeResult::Continue(self))
                    },
                    ErrMode::Incomplete(Needed::Size(size)) => {
                        let need_size = size.get();
                        self.buffer
                            .grow(self.buffer.available_data() + need_size);

                        if self.buffer.available_space() < need_size {
                            self.buffer.shift();
                        }
                        Ok(DecodeResult::Continue(self))
                    },
                }
            },
            State::Tail { .. } => {
                let e = match BinaryCookies::parser_tail.parse_next(&mut input) {
                    Ok((checksum, meta)) => {
                        let State::Tail { offsets } = self.state
                        else {
                            unreachable!()
                        };
                        return Ok(DecodeResult::Done((
                            checksum,
                            meta,
                            PagesDecoder { offsets },
                        )));
                    },
                    Err(e) => e,
                };
                match e {
                    ErrMode::Backtrack(e) | ErrMode::Cut(e) => Err(ParseError::WinnowCtx(e)),
                    ErrMode::Incomplete(Needed::Unknown) => {
                        // The branch is unreachable?
                        let new_cap = self.buffer.capacity() * 2;
                        self.buffer.grow(new_cap);
                        Ok(DecodeResult::Continue(self))
                    },
                    ErrMode::Incomplete(Needed::Size(size)) => {
                        let need_size = size.get();
                        self.buffer
                            .grow(self.buffer.available_data() + need_size);
                        if self.buffer.available_space() < need_size {
                            self.buffer.shift();
                        }
                        Ok(DecodeResult::Continue(self))
                    },
                }
            },
        }
    }
}

impl Default for BinaryCookieDecoder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum State {
    Head,
    Tail { offsets: Offsets },
}

// binary cookies metadata
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Offsets {
    pub(crate) page_sizes: Vec<u32>, // be
    pub(crate) tail_offset: usize,
}

#[cfg(test)]
mod tests {
    use std::{fs::File, os::unix::prelude::FileExt};

    use super::*;

    #[test]
    fn test_name() {
        let mut decoder = BinaryCookieDecoder::new();
        let f = File::open("./test-resource/BinaryCookies.binarycookies").unwrap();
        let res = loop {
            let offset = decoder.wants_read();
            dbg!(offset);
            f.read_exact_at(decoder.buffer.space(), offset as u64)
                .unwrap();
            let count = decoder.buffer.space().len();
            decoder.buffer.fill(count);
            let a = decoder.process().unwrap();
            match a {
                DecodeResult::Continue(c) => decoder = c,
                DecodeResult::Done(done) => break done,
            }
        };
        dbg!(res);
    }
}
