use std::fs::File;
use std::io::prelude::*;
use std::io;
use rfd::FileDialog;

fn main() -> io::Result<()>  {


    let file_path2 = FileDialog::new().set_title("select a file").pick_file();
    match &file_path2 {
        Some(file_path) => {
            println!("the path for the file that is selected is: {:?}", file_path);
        }
        None => {
            println!("an error occured there was no file selected");
        }
    }

    match File::open(file_path2.unwrap()) {
        Ok(file) => {
            println!("File successfully opened!");
            let mut buf_reader = io::BufReader::new(file);
            let mut file_conts = Vec::new();
            buf_reader.read_to_end(&mut file_conts).expect("Failed to read file");
            println!("File was successfully read!");
            println!("File conts was: {:?}", bytes_to_hex(&file_conts));


        }
        Err(e) => {
            println!("File open error : {}", e);
        }
    }

    Ok(())
}
fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{:02X}", byte)).collect::<String>()
}