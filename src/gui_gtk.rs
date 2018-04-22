use std::time::{Instant, Duration};
use crossbeam_channel::{Sender, Receiver};
use clipboard::{ClipboardProvider, ClipboardContext};
use gtk::{
    self,
    prelude::*,
    Button,
    Window,
    WindowType::Toplevel,
    ListBox,
    ListBoxRow,
    Label,
    Box,
    Orientation::{Horizontal, Vertical},
    Entry, EntryBuffer,
};

use data::{MsgToController, MsgToGui, Game};

const TITLE: &str = "Shadowverse Lobby";
const WIDTH: u32 = 320;
const HEIGHT: u32 = 320;

pub fn start(sender: Sender<MsgToController>, receiver: Receiver<MsgToGui>) {
    if gtk::init().is_err() {
        println!("Failed to intialise GTK.");
        return;
    }
    // Setup
    let window = Window::new(Toplevel);
    window.set_title(TITLE);
    window.set_default_size(WIDTH as i32, HEIGHT as i32);
    let v_layout = Box::new(Vertical, 0);
    let mut widget_game_list = Vec::new();
    let list = ListBox::new();
    v_layout.pack_start(&list, true, true, 0);
    let add_bar = Box::new(Horizontal, 1);
    let buffer = EntryBuffer::new(Some("Description"));
    let description = Entry::new_with_buffer(&buffer);
    add_bar.pack_start(&description, true, true, 1);
    let buffer = EntryBuffer::new(Some("Name"));
    let name = Entry::new_with_buffer(&buffer);
    add_bar.pack_start(&name, false, false, 1);
    let buffer = EntryBuffer::new(Some("Join Code"));
    let join_code = Entry::new_with_buffer(&buffer);
    add_bar.pack_start(&join_code, false, false, 1);
    let add_button = Button::new_with_label("Add");
    add_bar.pack_start(&add_button, false, false, 1);
    v_layout.pack_start(&add_bar, false, false, 1);
    let status = Label::new(None);
    v_layout.pack_start(&status, false, false, 1);
    window.add(&v_layout);
    window.show_all();
    // Connections
    let close_sender = sender.clone();
    window.connect_delete_event(move |_,_|{
        close_sender.send(MsgToController::Shutdown);
        gtk::main_quit();
        gtk::Inhibit(false)
    });
    let update_sender = sender.clone();
    add_button.connect_clicked(move |_|{
        let game_name = description.get_buffer().get_text();
        let author = name.get_buffer().get_text();
        let code = join_code.get_buffer().get_text();
        update_sender.send(MsgToController::UpdateGame(Game{author, name: game_name, join_code: code}));
    });
    sender.send(MsgToController::GetCache);
    let mut time_sent = Instant::now();
    gtk::timeout_add(500, move ||{
        use MsgToGui::*;
        for msg in receiver.try_iter() {
            match msg {
                PopulateList(game_list) => {
                    populate_list(game_list, &mut widget_game_list, &list);
                },
                Error(text) => {
                    status.set_label(text.as_str());
                    status.show_all();
                },
                UpdateRequestSent => {
                    status.set_label("Game updated!");
                    status.show_all();
                },
            }
        };
        let now = Instant::now();
        if now > time_sent + Duration::from_secs(10) {
            sender.send(MsgToController::GetCache);
            time_sent = now;
        };
        gtk::Continue(true)
    });
    gtk::main();
}

fn populate_list(new_game_list: Vec<Game>, widget_game_list: &mut Vec<Game>, list: &ListBox) {
    let mut temp_list = Vec::new();
    let mut changed = false;
    for (i, game) in widget_game_list.iter().enumerate() {
        if !new_game_list.contains(game) {
            list.remove(&list.get_row_at_index(i as i32).unwrap());
            changed = true;
        } else {
            temp_list.push(game.clone())
        }
    };
    for game in new_game_list.iter() {
        if !widget_game_list.contains(game) {
            let entry = ListBoxRow::new();
            let hbox = Box::new(Horizontal, 10);
            let name = Label::new(Some(game.name.as_str()));
            hbox.pack_start(&name, true, true, 10);
            let author = Label::new(Some(game.author.as_str()));
            hbox.pack_start(&author, false, false, 10);
            let code = Label::new(Some(game.join_code.as_str()));
            hbox.pack_start(&code, false, false, 10);
            let but = Button::new_with_label("Copy");
            let join_code = game.join_code.clone();
            but.connect_clicked(move |_|{copy_to_clipboard(join_code.clone())});
            hbox.pack_start(&but, false, false, 10);
            entry.add(&hbox);
            list.add(&entry);
            temp_list.push(game.clone());
            changed = true;
        }
    };
    if changed {
        list.show_all();
        widget_game_list.clear();
        widget_game_list.append(&mut temp_list);
    };
}

fn copy_to_clipboard(text: String) {
    let mut clipboard: ClipboardContext = ClipboardProvider::new()
        .expect("Failed to create clipboard api. Possibly due to unkown OS.");
    clipboard.set_contents(text)
        .expect("Failed to copy text to clipboard.");
}