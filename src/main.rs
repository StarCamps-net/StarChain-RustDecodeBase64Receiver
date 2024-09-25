extern crate base64;
extern crate reqwest;

use base64::decode;
use reqwest::blocking::get;
use std::io::{self, Read};
use std::net::{TcpListener, TcpStream};
use std::str;

fn decode_base64(encoded: &str) -> Result<String, base64::DecodeError> {
    let decoded_bytes = decode(encoded)?;
    let decoded_str = str::from_utf8(&decoded_bytes).map_err(|_| base64::DecodeError::InvalidByte(0, 0))?;
    Ok(decoded_str.to_string())
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).expect("Failed to read from stream");

    let encoded_input = String::from_utf8_lossy(&buffer);
    let encoded_input = encoded_input.trim();

    // Decode the input
    match decode_base64(encoded_input) {
        Ok(decoded) => {
            println!("Decoded: {}", decoded);

            // Validate the decoded URL
            if decoded.starts_with("http://") || decoded.starts_with("https://") {
                // Make an HTTP GET request to the decoded URL
                match get(&decoded) {
                    Ok(response) => {
                        if let Ok(body) = response.text() {
                            println!("HTTP Response: {}", body);
                        } else {
                            println!("Failed to read the response body");
                        }
                    }
                    Err(e) => println!("Failed to make HTTP request: {}", e),
                }
            } else {
                println!("Decoded string is not a valid URL: {}", decoded);
            }
        }
        Err(e) => println!("Failed to decode: {}", e),
    }
}

fn main() {
    // Start the TCP listener
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind to address");

    println!("StarChain Server listening on port 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Failed to accept connection: {}", e);
            }
        }
    }
}