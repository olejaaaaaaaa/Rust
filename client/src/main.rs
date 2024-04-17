use std::net::TcpStream;
use std::io::{Read, Write, stdin};
use std::str::from_utf8;

fn main() {

    match TcpStream::connect("localhost:8080") {

        Ok(mut stream) => {
            println!("Connected to server");

            loop {
                let mut msg = String::new();
                stdin().read_line(&mut msg).unwrap();
                stream.write(msg.as_bytes()).unwrap();
            }

        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }

    println!("Terminated");
}