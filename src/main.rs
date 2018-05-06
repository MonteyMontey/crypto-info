extern crate native_tls;

use std::io;
use std::net::TcpStream;
use std::io::prelude::*;
use native_tls::TlsConnector;


fn main() {
    println!("Welcome to Crypto Info! \nI can show you a list of the current top currencies. \n\
    How many currencies should the list contain?");

    let mut n: String = String::new();

    io::stdin().read_line(&mut n).expect("failed to read line");

    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    if n == 0 {
        println!("Input must be numeric!\nRestart and try again.");
    } else {
        println!("{}", coinmarketcap_request(n));
    }
}


fn coinmarketcap_request(n: u32) -> String {
    /*if let Ok(stream) = TcpStream::connect("api.coinmarketcap.com:443") {
        println!("Connected to the server!");
    } else {
        println!("Couldn't connect to server...");
    }*/

    // needed because coinmarketcap api only allows https so we need tls encryption
    let connector = TlsConnector::builder().unwrap().build().unwrap();

    let stream = TcpStream::connect("api.coinmarketcap.com:443").unwrap();
    let mut stream = connector.connect("api.coinmarketcap.com", stream).unwrap();

    let mut request = String::from(format!("GET /v2/ticker/?limit={} HTTP/1.1\r\nHost: api.coinmarketcap.com\r\nConnection: close\r\n\r\n", n));
    stream.write(request.as_bytes());

    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();

    return response;
}





