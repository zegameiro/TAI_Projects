use tai_first_project::*;
fn main(){
    let mut file_reader_struct = file_reader::FileReader{filename:String::from("example.txt"),reader:Option::None};

    if !file_reader::open_file(&mut file_reader_struct).is_ok(){
        println!("error ReadingFile");
        return;
    }
    let mut buff = [0u8;1];
    
    loop {
        match file_reader::read_byte(&mut file_reader_struct,&mut buff) {
            Ok(true) => {
                println!("{}", String::from_utf8_lossy(&buff)); // Print single byte
            }
            Ok(false) => break, // EOF reached
            Err(e) => {
                eprintln!("Error reading file: {}", e); // Print error and exit loop
                break;
            }
        }
    }
    
}