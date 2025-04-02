use std::{fs::File, io::{self, BufRead, BufReader, Read}};

pub struct FileReader {
    pub filename: String,
    pub reader: Option<BufReader<File>>,
    pub buffer: Vec<String>,
}

pub fn open_file(file_reader: &mut FileReader) -> io::Result<()> {
    let file = File::open(&file_reader.filename)?;
    file_reader.reader = Some(BufReader::new(file));
    Ok(())
}

pub fn read_char(file_reader: &mut FileReader) -> io::Result<Option<char>> {
    let mut byte_buff = [0u8; 4]; // UTF-8 chars can be up to 4 bytes
    let reader = match file_reader.reader.as_mut() {
        Some(r) => r,
        None => return Ok(None), // No reader available
    };

    let mut first_byte = [0u8; 1];
    if reader.read(&mut first_byte)? == 0 {
        return Ok(None); // End of file
    }

    byte_buff[0] = first_byte[0];
    let char_len = utf8_char_length(byte_buff[0]);

    for i in 1..char_len {
        if reader.read(&mut byte_buff[i..i + 1])? == 0 {
            return Ok(None);
        }
    }

    match std::str::from_utf8(&byte_buff[..char_len]) {
        Ok(s) => Ok(s.chars().next()),
        Err(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8")),
    }
}

pub fn read_buff(file_reader: &mut FileReader, buff: &mut String, size: usize) -> io::Result<usize> {
    buff.clear();
    let mut byte_buff = vec![0u8; size];
    let reader = match file_reader.reader.as_mut() {
        Some(r) => r,
        None => return Ok(0),
    };

    let bytes_read = reader.read(&mut byte_buff)?;
    if bytes_read == 0 {
        return Ok(0);
    }

    match String::from_utf8(byte_buff[..bytes_read].to_vec()) {
        Ok(s) => {
            buff.push_str(&s);
            Ok(s.chars().count())
        }
        Err(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8")),
    }
}

pub fn read_word(file_reader: &mut FileReader) -> io::Result<Option<String>> {
    let reader = match file_reader.reader.as_mut() {
        Some(r) => r,
        None => return Ok(None),
    };

    if file_reader.buffer.is_empty(){
        let mut buffer = String::new();
        reader.read_line(&mut buffer)?;
    
        for word in buffer.rsplit(' ').filter(|w| !w.is_empty()){
            file_reader.buffer.push(String::from(word));
        };
    };

    if file_reader.buffer.is_empty() {
        return Ok(None);
    }

    return Ok(Some(file_reader.buffer.remove(0)))

}

pub fn read_line(file_reader: &mut FileReader) -> io::Result<Option<String>> {
    if let Some(reader) = file_reader.reader.as_mut() {
        let mut buffer = String::new();
        let bytes_read = reader.read_line(&mut buffer)?;

        if bytes_read == 0 {
            return Ok(None); // End of file
        }

        Ok(Some(buffer.trim_end().to_string()))
    } else {
        Ok(None)
    }
}

fn utf8_char_length(byte: u8) -> usize {
    if byte & 0b1000_0000 == 0 {
        1 // 1-byte char (ASCII)
    } else if byte & 0b1110_0000 == 0b1100_0000 {
        2 // 2-byte char
    } else if byte & 0b1111_0000 == 0b1110_0000 {
        3 // 3-byte char
    } else if byte & 0b1111_1000 == 0b1111_0000 {
        4 // 4-byte char
    } else {
        1 // Invalid char
    }
}
