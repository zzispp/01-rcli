use std::{fs::File, io::Read};

use anyhow::Error;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine,
};

use crate::cli::Base64Format;

pub fn process_encode(input: &str, format: Base64Format) -> Result<(), Error> {
    // 从文件或标准输入读取
    let mut reader: Box<dyn Read> = if input == "-" {
        eprintln!("Reading from stdin. Type your content and press Ctrl+D (Unix) or Ctrl+Z (Windows) when done.");
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buf),
    };
    println!("{}", encoded);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<(), Error> {
    // 从文件或标准输入读取
    let mut reader: Box<dyn Read> = if input == "-" {
        eprintln!("Reading from stdin. Type your content and press Ctrl+D (Unix) or Ctrl+Z (Windows) when done.");
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim().as_bytes().to_vec();

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };
    println!("{}", String::from_utf8(decoded)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        let result = process_encode(input, format);
        assert!(result.is_ok());
    }

    #[test]
    fn test_process_decode() {
        // Create a temporary file with some base64 content
        let temp_filename = "test_decode_temp.b64";
        let test_content = "SGVsbG8sIHdvcmxkIQ=="; // "Hello, world!" in base64

        {
            let mut file = File::create(temp_filename).expect("Failed to create test file");
            file.write_all(test_content.as_bytes())
                .expect("Failed to write to test file");
        }

        // Run the test
        let format = Base64Format::Standard;
        let result = process_decode(temp_filename, format);

        // Clean up the temporary file
        std::fs::remove_file(temp_filename).expect("Failed to remove test file");

        assert!(result.is_ok());
    }
}
