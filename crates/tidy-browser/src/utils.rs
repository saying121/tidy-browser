use std::io::{ErrorKind, IoSlice, Write};

/// Copy from nightly [`Write::write_all_vectored`]
/// TODO: Use std method
pub(crate) fn write_all_vectored(
    file: &mut std::fs::File,
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
