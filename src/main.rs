#![allow(unused)]
use std::net::{TcpListener, TcpStream};

fn handle_client(stream: TcpStream) {
    // ...
    println!("received package")
}

fn main() -> std::io::Result<()> {
    // based on https://doc.rust-lang.org/std/net/struct.TcpListener.html
    let TCP_SOCKET_ADDRESS = "0.0.0.0:8080";
    let listener = TcpListener::bind(TCP_SOCKET_ADDRESS)?;
    println!("Running TCP-Listener on: {TCP_SOCKET_ADDRESS}");

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}