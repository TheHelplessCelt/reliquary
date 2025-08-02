use std::{fs::File, str::from_utf8};
use std::io::BufReader;
use std::error::Error;

mod relic_binary_format;
use crate::relic_binary_format::read_rbf_header::{construct_header, Header};


fn main() -> Result<(), Box<dyn Error>> {
    let rbf_file: File = File::options().read(true).open(
        "C:\\Users\\Sam\\Documents\\programming\\rust\\reliquary\\src\\tactical_marine.rbf"
    )?;

    let header: Header = construct_header(rbf_file);
        
    let byte_signature = &header.signature.to_le_bytes();
    let converted_string: &str = match str::from_utf8(byte_signature) {
        Ok(string) => string,
        Err(e) => panic!("error: {}", e),
    };

    println!("the fine signature is: {}", converted_string);

    Ok(())
}
