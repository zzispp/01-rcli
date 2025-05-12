use anyhow::Result;
use std::{fs::File, io::Read};

pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    // 从文件或标准输入读取
    let reader: Box<dyn Read> = if input == "-" {
        eprintln!("Reading from stdin. Type your content and press Ctrl+D (Unix) or Ctrl+Z (Windows) when done.");
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}
