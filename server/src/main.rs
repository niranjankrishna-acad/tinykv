use std::net::{TcpStream, Shutdown, TcpListener};
use std::io::{ Read, Write};
use std::thread;

mod protocol;

const MAX_DATA_SIZE: usize = 2 * 1024 * 1024; // 2 MB

fn handle_client(mut stream: TcpStream) {
    let mut data = vec![0u8; MAX_DATA_SIZE];
    while match stream.read(&mut data) {
        Ok(size) => {
            // process request + send response
            protocol::read(&data);
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    drop(listener);
}