/* Simple HTTP Server */
/* Author : Ramesh Vyas */
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {
    
    /* Creating a Local TcpListener at Port 8477 */
    const HOST : &str ="127.0.0.1";
    const PORT : &str ="8477";

    /* Concating Host address and Port to Create Final Endpoint */
    let end_point : String = HOST.to_owned() + ":" +  PORT;

    /*Creating TCP Listener at our end point */
    let listener = TcpListener::bind(end_point).unwrap();

    println!("Web server is listening at port {}",PORT);

    /* Conneting to any incoming connections */
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        // Call Function to process any incomming connections
        handle_connection(_stream);
    }
    
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let contents = fs::read_to_string("index.html").unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        // some other request
    }
}
