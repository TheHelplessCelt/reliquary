use std::io::prelude::*;
use byteorder::{ByteOrder, LittleEndian};
use std::{fs::File};
use std::io::BufReader;

pub struct Header {
    pub signature: u64,
    pub table_array_offset: u32,
    pub table_array_count: u32,
    pub data_index_array_offset: u32,
    pub data_index_array_count: u32,
    pub data_array_offset: u32,
    pub data_array_count: u32,
    pub string_section_offset: u32,
    pub string_section_length: u32,
}

impl Header {
    pub fn load<T: Read>(reader: &mut T) -> Header {

        let mut load_part = |size| {

            let mut buf = Vec::with_capacity(size);
            let mut part_reader = reader.take(size as u64);
            
            part_reader.read_to_end(&mut buf).unwrap();
            
            buf

        };

        Header{
            signature: LittleEndian::read_u64(&load_part(8)),
            table_array_offset: LittleEndian::read_u32(&load_part(4)),
            table_array_count: LittleEndian::read_u32(&load_part(4)),
            data_index_array_offset: LittleEndian::read_u32(&load_part(4)),
            data_index_array_count: LittleEndian::read_u32(&load_part(4)),
            data_array_offset: LittleEndian::read_u32(&load_part(4)),
            data_array_count: LittleEndian::read_u32(&load_part(4)),
            string_section_offset: LittleEndian::read_u32(&load_part(4)),
            string_section_length: LittleEndian::read_u32(&load_part(4)),
        }
    }
}

pub  fn construct_header(rbf_file: File) -> Header {

    let mut buf_reader = BufReader::new(rbf_file);

    let header = Header::load(&mut buf_reader);

    header
}