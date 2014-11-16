fn main() {
    // Don't mind this, these just make sure all the functions are used in
    // normal compile for now. Eventually this will actually start the server
    // and such.
    match parse_message(break_line("HELO relay.example.org")) {
        Some(msg) => println!("Got {} from {}", msg.command, msg.arguments),
        None => println!("got nothing?")
    }
    println!("hello world");
}

fn break_line(input: &str) -> Vec<String> {
    return input.split(' ').map(|s| s.to_string()).collect();
}

struct Message {
     command: String,
     arguments: Vec<String>
}

fn parse_message(mut input: Vec<String>) -> Option<Message> {
    match input.swap_remove(0) {
        Some(command) => {
            return Some(Message {
                command: command.to_string(),
                arguments: input
            })
        },
        None => return None
    }
}

#[test]
fn main_basic() {
    main();
}

#[test]
fn break_line_basic() {
    assert_eq!(
        break_line("HELO relay.example.org"),
        vec!["HELO".to_string(), "relay.example.org".to_string()]
    )
}

#[test]
fn parse_message_command_and_arg() {
    let input = vec!["HELO".to_string(), "relay.example.org".to_string()];
    match parse_message(input) {
        Some(msg) => {
            match msg {
                Message {command, arguments} => {
                    assert_eq!(command, "HELO".to_string());
                    assert_eq!(arguments, vec!["relay.example.org".to_string()])
                }
            }
        },
        None => unreachable!()
    }
}

#[test]
fn parse_message_only_command() {
    let expected_args: Vec<String> = Vec::new();
    match parse_message(vec!["HELO".to_string()]) {
        Some(msg) => {
            match msg {
                Message {command, arguments} => {
                    assert_eq!(command, "HELO".to_string());
                    assert_eq!(arguments, expected_args);
                }
            }
        },
        None => unreachable!()
    }
}

#[test]
fn parse_message_empty_vector() {
    match parse_message(vec![]) {
        Some(msg) => {
            println!("Got {},{} in unreachable branch", msg.command, msg.arguments);
            unreachable!();
        },
        None => {}
    }
}

// #[test]
// fn generate_response_helo() {
//     match generate_response(Message {
//         command: "HELO".to_string(),
//         arguments: vec!["reply.example.com".to_string()]
//     }) {
//         e
//     }
// }