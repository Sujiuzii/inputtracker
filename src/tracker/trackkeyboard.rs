use std::io::Write;
use std::net::TcpStream;
use rdev::{listen, EventType};

pub fn run() {
    let send_event = |event_data: &str| {
        if let Ok(mut stream) = TcpStream::connect("127.0.0.1:2718") {
            stream.write(event_data.as_bytes()).expect("Failed to write to stream");
        } else {
            eprintln!("Failed to connect to socket");
        }
    };
    loop {
        if let Err(e) = listen(move |event| {
            match event.event_type{
                EventType::KeyPress(key) => send_event(&format!("Key press: {:?}", key)),
                EventType::KeyRelease(key) => send_event(&format!("Key press: {:?}", key)),
                _ => ()
            }
        }) {
            eprintln!("Error: {:?}", e);
            break;
        }
    }
}