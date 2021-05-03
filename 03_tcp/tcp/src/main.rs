use std::env::args;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::BufReader;
// Traits
use std::io::BufRead;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;

use std::collections::VecDeque;

// macros are always called with a !
// println!
// Macros are not a textual replacement as they are in C 
// Use todo! macro
// You cannot look at data and read it at the same time. 

// you can write custom ones of these. 
#[derive(Debug, Eq, PartialEq)]
enum Request {
    Publish(String), 
    Retrieve
}


fn read_line(stream: &TcpStream) -> String {
    let mut buffered_reader = BufReader::new(stream);

    let mut buf = String::new();
    buffered_reader.read_line(&mut buf).unwrap(); 

    // this is how you return values - no semi-colon.
    buf
}

fn parse_request(line: String) -> Request {
    let trimmed = line.trim_end();
    if trimmed == "" {
        Request::Retrieve
    } else {
        Request::Publish(String::from(trimmed))
    }
    //drop(line)
}

fn handle_client(mut stream: TcpStream, storage: &Mutex<VecDeque<String>>) -> () {
    println!("Client connected!");

    let input = read_line(&stream);
    let request = parse_request(input);

    // match request type
    match request {
        Request::Publish(msg) => {
            println!("Publising message {}", msg);
            let mut guard = storage.lock().unwrap();
            guard.push_back(msg);

            // the scope drops the guard
        }
        Request::Retrieve => {
            // single line scope for guard
            let maybe_msg = storage.lock().unwrap().pop_front();
            match maybe_msg {
                Some(msg) => {
                    // the string is seen as bytes - no copy involves
                    stream.write_all(msg.as_bytes()).unwrap();
                    println!("Retrieving message {}", msg);
                }
                None => {
                    // write on message    
                    stream.write_all(b"No message available").unwrap();
                }
            }
        }
    }

    //println!("{}", input);
    //todo!();
    //drop(stream);
}

fn main() {
    let _who = args().nth(1);

    // expect vs unwrap vs ?  
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    let dequeue = Arc::new(Mutex::new(VecDeque::new()));
    // incoming returns a iterator.   connection_attempt <, error>
    for connection_attempt in listener.incoming() {

        match connection_attempt {
            Ok(stream) => {
                // arc = atomic reference counter..  
                let thread_handle = Arc::clone(&dequeue);
                // how do we pass a mutable collection storage into a thread?
                std::thread::spawn(move || {
                    // ownership is handed to the handle_client function  (low coupling)
                    handle_client(stream, &thread_handle);

                    // ownership handover is complete.  Cannot uae it now
                    // println!("{:?}", stream);
                });
            }
            Err(e) => {
                eprintln!("Error connecting: {}", e);
            }
            
        }
    }
    //println!("Hello, world!");
}
