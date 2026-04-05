use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
};

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ChatMessage {
    #[serde(rename = "type")]
    msg_type: String,
    user: String,
    text: String,
}

fn validate_message(msg: &ChatMessage) -> Result<(), &'static str> {
    if msg.msg_type != "message" {
        return Err("field `type` must be \"message\"");
    }
    if msg.user.trim().is_empty() {
        return Err("field `user` must not be empty");
    }
    if msg.text.trim().is_empty() {
        return Err("field `text` must not be empty");
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    println!("New client connected: {}", stream.peer_addr().unwrap());

    let mut buffer = [0u8; 512];
    let mut pending = Vec::<u8>::new();

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected: {}", stream.peer_addr().unwrap());
                break;
            }
            Ok(bytes_read) => {
                pending.extend_from_slice(&buffer[..bytes_read]);

                while let Some(newline_pos) = pending.iter().position(|b| *b == b'\n') {
                    let mut message_bytes: Vec<u8> = pending.drain(..=newline_pos).collect();
                    if message_bytes.last() == Some(&b'\n') {
                        message_bytes.pop();
                    }
                    if message_bytes.last() == Some(&b'\r') {
                        message_bytes.pop();
                    }

                    if message_bytes.is_empty() {
                        continue;
                    }

                    let message_str = match std::str::from_utf8(&message_bytes) {
                        Ok(s) => s,
                        Err(_) => {
                            eprintln!("Rejected message: invalid UTF-8");
                            stream.write_all(b"ERROR invalid UTF-8\n")?;
                            continue;
                        }
                    };

                    println!("Received line: {}", message_str);

                    match serde_json::from_str::<ChatMessage>(message_str) {
                        Ok(msg) => match validate_message(&msg) {
                            Ok(()) => {
                                println!(
                                    "Accepted JSON message from {}: {}",
                                    msg.user, msg.text
                                );
                                stream.write_all(message_str.as_bytes())?;
                                stream.write_all(b"\n")?;
                            }
                            Err(reason) => {
                                eprintln!("Rejected message: {}", reason);
                                stream.write_all(format!("ERROR {}\n", reason).as_bytes())?;
                            }
                        },
                        Err(err) => {
                            eprintln!("Rejected message: invalid JSON ({})", err);
                            stream.write_all(b"ERROR invalid JSON format\n")?;
                        }
                    }
                }
            }
            Err(error) => {
                eprintln!("Error reading from client: {}", error);
                break;
            }
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        let stream = stream?;
        if let Err(err) = handle_client(stream) {
            eprintln!("Client handling error: {}", err);
        }
    }
    Ok(())
}
