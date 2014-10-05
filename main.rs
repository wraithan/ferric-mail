use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};
use std::io::BufferedReader;

fn main() {
    let listener = TcpListener::bind("127.0.0.1", 8080);

    // bind the listener to the specified address
    let mut acceptor = listener.listen();

    // accept connections and process them, spawning a new tasks for each one
    for stream in acceptor.incoming() {
        match stream {
            Err(e) => {
                println!("connection failed: {}", e);
            }
            Ok(stream) => spawn(proc() {
                handle_client(stream)
            })
        }
    }
}

fn handle_client(mut write_stream: TcpStream) {
    let read_stream = write_stream.clone();
    match write_stream.write(b"220 localhost SMTP shitmail\r\n") {
        Err(e) => {
            println!("outbound bummer dude: {}", e);
        }
        Ok(_) => {
            println!("220 localhost SMTP shitmail\r\n");
        }

    }

    // Wrap up in BufferedReader to handle utf8 in chunks correctly.
    let mut reader = BufferedReader::new(read_stream);

    // Infinitely read.
    loop {
        // Process input line-wise
        match reader.read_line() {
            Ok(line) => {
                println!("message: {}", line);
                match process_line(write_stream.clone(), line) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("could not process message: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("inbound bummer dude: {}", e);
                break;
            }
        }
    }
}

fn process_line(mut stream: TcpStream, line: String) -> Result<&'static str, &'static str> {
    let mut sliced = line.as_slice().split(' ');
    let command = sliced.next().unwrap();
    let host = sliced.next().unwrap();

    let response = format!("250 Hello {}, I am glad to meet you\r\n", host);
    match stream.write_str(response.as_slice()) {
        Err(e) => println!("outbound bummer dude: {}", e),
        Ok(_) => {
            println!("{}", response);
        }
    }
    println!("command: {}", command);
    return Ok("success");
}

fn remove_ending(mut line: String) -> Result<String, &'static str> {
    if line.as_slice().ends_with("\r\n") {
        return Err("shit went down");
    }
    return Ok(line);
}