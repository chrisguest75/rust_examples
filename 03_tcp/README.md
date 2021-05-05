# README
Demonstrates how to accept a TCP connection

## How it was created 
```sh
# create a new project
cargo new tcp --bin           
```

## Running
```sh
cd tcp
# run the code
cargo run
cargo run --release    
```

## Connecting with telnet
```sh
# install
brew install telnet    
# connect and send message (return will pull a message from the dequeue)
telnet 0.0.0.0 8080                
```

## Resources

[Arc Atomic Reference Count](https://doc.rust-lang.org/std/sync/struct.Arc.html)
[BufReader](https://doc.rust-lang.org/std/io/struct.BufReader.html)
[option_unwrap](https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html)

