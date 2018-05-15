extern crate native_tls;
extern crate minihttpse;
#[macro_use]
extern crate serde_json;

mod test;

use std::io;
use std::net::TcpStream;
use std::io::prelude::*;
use std::process;
use native_tls::TlsConnector;
use minihttpse::Response;
use serde_json::{Value, Error};


fn main() {
    println!("\nWelcome to Crypto Info! \nI can show you the top currencies on coinmarketcap ordered in three ways: \n\
        Market cap (1)\nPrice per coin (2)\nVolume last 24h (3)");

    let sort_attribute: u32 = receive_numerical_input();

    if sort_attribute < 1 || sort_attribute > 3 {
        println!("Input must be numeric and between 1 and 3!\nRestart and try again.");
        process::exit(1);
    }

    println!("Excellent! And how long should the list be?");

    let number_of_coins: u32 = receive_numerical_input();

    if number_of_coins < 1 || number_of_coins > 100 {
        println!("Input must be numeric and not over 100! Restart and try again.");
        process::exit(1);
    }

    show_list(number_of_coins, number_to_attribute(sort_attribute));
}


fn receive_numerical_input() -> u32 {
    let mut n: String = String::new();

    io::stdin().read_line(&mut n).expect("failed to read line");

    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    return n;
}


fn number_to_attribute(n: u32) -> String {
    match n {
        1 => return "market_cap".to_string(),
        2 => return "price".to_string(),
        3 => return "volume_24h".to_string(),
        _ => panic!("Invalid parameter: {}", n)
    }
}


fn parse_http_response_content(response: String) -> String {
    let resp_obj = Response::new(response.as_bytes().to_owned()).unwrap();
    let json: String = resp_obj.text();
    return json;
}


fn check_if_key_in_json(json: &Value, key: &String) -> bool {
    let ref value: Value = json["data"][key];
    match value.is_null() {
        true => return false,
        false => return true
    };
}


fn coinmarketcap_request() -> String {
    let connector = TlsConnector::builder().unwrap().build().unwrap();
    let stream = TcpStream::connect("api.coinmarketcap.com:443").unwrap();
    let mut stream = connector.connect("api.coinmarketcap.com", stream).unwrap();

    let mut request = String::from(format!("GET /v2/ticker/ HTTP/1.1\r\nHost: api.coinmarketcap.com\r\nConnection: close\r\n\r\n"));
    stream.write(request.as_bytes());

    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();

    return response;
}


#[derive(PartialEq)]
struct Node<'a> {
    // struct needs lifetime of borrowed values that's why <'a>
    left_node: Option<Box<Node<'a>>>,
    right_node: Option<Box<Node<'a>>>,
    value: &'a Value,
}

impl<'a> Node<'a> {
    pub fn insert(&mut self, coin: &'a Value, sort_attribute: &'a String) {
        let target_node = if coin["quotes"]["USD"][sort_attribute].to_string().parse::<f64>().unwrap_or(0.0) <=
            self.value["quotes"]["USD"][sort_attribute].to_string().parse::<f64>().unwrap_or(0.0) { &mut self.left_node } else { &mut self.right_node };

        match target_node {
            &mut Some(ref mut subnode) => subnode.insert(coin, sort_attribute),
            &mut None => {
                let new_node = Node { left_node: None, right_node: None, value: coin };
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node;
            }
        }
    }
    pub fn inorder(&mut self, sort_attribute: &'a String, recursion_depth: &mut u32) {
        if *recursion_depth > 0 {
            *recursion_depth -= 1;
            match &mut self.right_node {
                &mut Some(ref mut subnode) => subnode.inorder(&sort_attribute, recursion_depth),
                &mut None => {}
            }

            println!("{}: {}", self.value["name"], self.value["quotes"]["USD"][sort_attribute]);

            match &mut self.left_node {
                &mut Some(ref mut subnode) => subnode.inorder(&sort_attribute, recursion_depth),
                &mut None => {}
            }
        }
    }
}


fn store_content_in_tree<'a>(sort_attribute: &'a String, content: &'a Value) -> Node<'a> {
    let mut tree = Node { left_node: None, right_node: None, value: &content["data"]["1"] };
    let mut counter: u32 = 2;

    while counter < 2000 {
        if check_if_key_in_json(content, &counter.to_string()) {
            tree.insert(&content["data"][counter.to_string()], sort_attribute);
        }
        counter += 1;
    }

    return tree;
}

fn show_list(mut list_length: u32, sort_attribute: String) -> Result<(), Error> {
    let http_response: String = coinmarketcap_request();
    let json_content: String = parse_http_response_content(http_response);
    let json_obj: Value = serde_json::from_str(json_content.as_str())?;
    let mut tree: Node = store_content_in_tree(&sort_attribute, &json_obj);
    tree.inorder(&sort_attribute, &mut list_length);
    Ok(())
}
