#![allow(warnings)]
#![feature(ascii_char)]

use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write, stdin};

fn handle_client(mut stream: TcpStream) {



    let mut data = [0 as u8; 50]; // using 50 byte buffer

    while match stream.read(&mut data) {

        Ok(size) => {
            
            for i in data.as_ascii().unwrap() {
                if *i != '\0'.as_ascii().unwrap() {
                    print!("{}", i);
                }
            }
            true
        },

        Err(_) => {
            println!("error connection to {}, terminated", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 8080");
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("connecting new user: {}", stream.peer_addr().unwrap());

                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}