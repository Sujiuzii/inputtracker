use std::thread;
use std::io::{BufReader, Cursor};
use std::time::Duration;
use rodio::{Decoder, Sink};

use super::super::AUDIO_FILE;

fn play_audio() {
    let source = Decoder::new(BufReader::new(Cursor::new(AUDIO_FILE))).unwrap();
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    
    sink.append(source);
    sink.play();
    thread::sleep(Duration::from_secs(10));
    sink.stop();
}

pub fn check_router() {
    loop {
        let mut system = sysinfo::System::new_all();
        system.refresh_all();
        for (_, process) in system.processes() {
            let cmdline = process.cmd().join(" ");
            if cmdline.contains("server.py") {
                play_audio();
                break;
            }
        }
    }
}