#![allow(warnings)]
#![feature(ascii_char)]

use std::ascii::Char;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{stdin, BufRead, BufReader, Error, Read, Write};
use std::str;

fn handle_client(mut stream: TcpStream) {

    let mut buf = [0 as u8; 64];

    stream.read(&mut buf);

    println!("buf: {:?}", buf);

}


fn main() {
    


    let port: String = "8080".to_string();
    let ip: String = "127.0.0.1".to_string();

    let listener: Result<TcpListener, Error> = TcpListener::bind(ip + ":" + &port);

    let listener: TcpListener = match listener { Ok(x) => { x }, Err(_) => { panic!("ERROR Create TcpListener") } };

    for stream in listener.incoming() {

        match stream {
            Ok(mut stream) => {

                thread::spawn(move || {
                    println!("{:?}", stream);
                    handle_client(stream);
                });

            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}