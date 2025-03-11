use std::{io::Cursor, path::Path};

pub fn read_file_as_cursor<P: AsRef<Path>>(path: P) -> Cursor<Vec<u8>> {
     Cursor::new(std::fs::read(path).unwrap())
}
