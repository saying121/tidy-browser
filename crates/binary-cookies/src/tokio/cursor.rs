use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{ready, Poll},
};

use positioned_io::{RandomAccessFile, ReadAt};
use tokio::{io::AsyncRead, task::JoinHandle};

pub trait CookieCursor {
    type Cursor<'a>: AsyncRead + Unpin + 'a
    where
        Self: 'a;

    fn cursor_at(&self, offset: u64) -> Self::Cursor<'_>;
}

impl CookieCursor for &[u8] {
    type Cursor<'a>
        = &'a [u8]
    where
        Self: 'a;

    fn cursor_at(&self, offset: u64) -> Self::Cursor<'_> {
        &self[offset as usize..]
    }
}

impl CookieCursor for Vec<u8> {
    type Cursor<'a>
        = &'a [u8]
    where
        Self: 'a;

    fn cursor_at(&self, offset: u64) -> Self::Cursor<'_> {
        &self[offset as usize..]
    }
}

impl CookieCursor for Arc<RandomAccessFile> {
    type Cursor<'a>
        = AsyncCursor
    where
        Self: 'a;

    fn cursor_at(&self, offset: u64) -> Self::Cursor<'_> {
        AsyncCursor {
            file: Self::clone(self),
            file_offset: offset,
            state: State::Idle(Some(Buf {
                buf: vec![0; 512],
                valid_len: 0,
                buf_offset: 0,
            })),
        }
    }
}

#[derive(Debug)]
pub struct AsyncCursor {
    file: Arc<RandomAccessFile>,
    file_offset: u64,
    state: State,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Buf {
    buf: Vec<u8>,
    valid_len: usize,
    buf_offset: usize,
}

#[derive(Debug)]
enum State {
    Idle(Option<Buf>),
    Busy(JoinHandle<Result<Buf, std::io::Error>>),
}

impl AsyncRead for AsyncCursor {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        loop {
            match &mut self.state {
                State::Idle(buf_cell) => {
                    #[expect(clippy::unwrap_used, reason = "It must be `Some`")]
                    let mut buffer = buf_cell.take().unwrap();

                    if buffer.buf_offset < buffer.valid_len {
                        let read_len = buf
                            .remaining()
                            .min(buffer.valid_len - buffer.buf_offset);
                        buf.put_slice(&buffer.buf[buffer.buf_offset..][..read_len]);
                        buffer.buf_offset += read_len;

                        *buf_cell = Some(buffer);
                        return Poll::Ready(Ok(()));
                    }

                    let f = Arc::clone(&self.file);
                    let file_offset = self.file_offset;

                    let jh = tokio::task::spawn_blocking(move || -> Result<_, std::io::Error> {
                        let readed = f.read_at(file_offset, &mut buffer.buf)?;
                        buffer.valid_len = readed;
                        buffer.buf_offset = 0;
                        Ok(buffer)
                    });
                    self.state = State::Busy(jh);
                },
                State::Busy(jh) => match ready!(Pin::new(jh).poll(cx))? {
                    Ok(buffer) => {
                        self.file_offset += buffer.valid_len as u64;
                        self.state = State::Idle(Some(buffer));
                        continue;
                    },
                    Err(e) => return Poll::Ready(Err(e)),
                },
            }
        }
    }
}
