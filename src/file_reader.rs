
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

pub fn read_byte(file_reader: &mut FileReader,buff: &mut [u8;1]) -> bool{
    if let Some(reader) = file_reader.reader.as_mut() {
        return reader.read(buff).is_ok();
    }
    false
}