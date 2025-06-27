use std::{fs::File, io::Read};

pub trait CookieCursor {
    type Cursor<'a>: Read + 'a
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

impl CookieCursor for File {
    type Cursor<'a>
        = positioned_io::Cursor<&'a Self>
    where
        Self: 'a;

    fn cursor_at(&self, offset: u64) -> Self::Cursor<'_> {
        positioned_io::Cursor::new_pos(self, offset)
    }
}
