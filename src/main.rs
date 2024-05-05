use inputtracker::*;
use std::thread;

fn main() {
    application();
    thread::spawn(run);
    thread::spawn(check_router);
    gtk::main();
}