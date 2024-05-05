use gtk::{prelude::*, Box, Button, Entry, Window, WindowType, ButtonsType, DialogFlags, MessageType, MessageDialog};

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

pub fn application() {
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