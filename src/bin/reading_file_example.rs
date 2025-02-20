use tai_first_project::*;
fn main(){
    let mut file_reader_struct = file_reader::FileReader{filename:String::from("example.txt"),reader:Option::None};

    if !file_reader::open_file(&mut file_reader_struct).is_ok(){
        println!("error ReadingFile");
        return;
    }
    let mut buff = [0u8;1];
    let value = file_reader::read_byte(&mut file_reader_struct, &mut buff);
    
    if value{
        println!("{}",String::from_utf8_lossy(&buff));
    }else{
        println!("error printing char");
    }

}