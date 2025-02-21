
use std::{fs::File, io::{self, BufReader, Read}};
pub struct FileReader{
    pub filename: String,
    pub reader: Option<BufReader<File>>,
}
pub fn open_file(file_reader: &mut FileReader) -> io::Result<()>{
    let file = File::open(&file_reader.filename)?;
    file_reader.reader = Some(BufReader::new(file));
    Ok(())
}

pub fn read_byte(file_reader: &mut FileReader, buff: &mut [u8; 1]) -> io::Result<bool> {
    if let Some(reader) = file_reader.reader.as_mut() {
        let bytes_read = reader.read(buff)?;
        return Ok(bytes_read > 0);
    }
    Ok(false) // No reader available
}

pub fn read_buff(file_reader: &mut FileReader, buff: &mut [u8]) -> io::Result<usize> {
    if let Some(reader) = file_reader.reader.as_mut() {
        let bytes_read = reader.read(buff)?;
        return Ok(bytes_read);
    }
    Ok(0) // No reader available
}
