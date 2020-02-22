use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::prelude::*;
use std::io::{BufReader};


fn handle_client(stream: &mut TcpStream) {
    let mut reader = BufReader::new(stream);
    let mut buf: String = String::new();
    
    while let Ok(size) = reader.read_line(&mut buf) {
        if size == 0 {
            break;
        }
        println!("{}", buf);
    }

    println!("Connection closed.");
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:10000")?;
    println!("Listening on 127.0.0.1:10000");

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(&mut stream?);
    }
    Ok(())
}
