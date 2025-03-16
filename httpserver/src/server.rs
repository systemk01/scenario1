use super::router::Router;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;



pub struct Server<'a> {
    socket_addr: &'a str, //socket_adress = Host & Port e.g. "localhost:3000"
}

impl <'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server{socket_addr}
    }
    pub fn run(&self) {
        //start a server, listening on socket_address
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("running on {}", self.socket_addr);
        //listen to incoming connections in a loop
        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established");
            let mut read_buffer = [0; 90];
            stream.read(&mut read_buffer).unwrap();
            //convert HTTPRequest to Rust datastructure
            let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();
            //Route request to appropriate Handler
            Router::route(req, &mut stream);
        }
    }
}