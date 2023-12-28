use sha256::digest;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

const SATOSHI_SUPPLY: u64 = 21000000 * 100000000;

fn main() {

    // println!("Enter a value to hash: ");
    // let stdin = io::stdin();
    // for line in stdin.lock().lines() {
    //     let input = String::from(line.unwrap());
    //     let val = digest(input);
    //     println!("Hashed Value: {}", val);
    // }

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }

}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
}
