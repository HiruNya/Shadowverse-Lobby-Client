extern crate crossbeam_channel;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate conrod;
extern crate piston_window;
extern crate gtk;
extern crate clipboard;

use crossbeam_channel::{unbounded, Sender, Receiver};
use std::thread::spawn;

mod networking;
mod parse;
mod data;
//mod gui;
mod gui_gtk;
use gui_gtk as gui;
use networking::{get_cache, update_game};
use data::*;

fn main() {
    // Communicate to the controller
    let (sender, receiver): (Sender<MsgToController>, Receiver<MsgToController>) = unbounded();
    // Communicate to the GUI
    let (gui_sender, gui_receiver): (Sender<MsgToGui>, Receiver<MsgToGui>) = unbounded();

    let c_sender = sender.clone();
    spawn(||{gui::start(c_sender, gui_receiver);});

    // Main loop
    for msg in receiver.iter() {
        use MsgToController::*;
        match msg {
            Shutdown => break,
            GetCache => {
                let g_sender = gui_sender.clone();
                let c_sender = sender.clone();
                spawn(move || {
                    match get_cache() {
                        Ok(e) => {
                            match parse::cache(e) {
                                Ok(f) => {c_sender.send(MsgToController::CacheReceived(f));},
                                Err(f) => {g_sender.send(MsgToGui::Error(f.to_string()));},
                            }

                        },
                        Err(e) => {g_sender.send(MsgToGui::Error(e.to_string()));},
                    };
                });
            },
            CacheReceived(game_list) => {
                gui_sender.send(MsgToGui::PopulateList(game_list));
//                let mut current_list = Vec::new();
//                for (i, g) in game_list.iter().enumerate() {
//                    if i == 10 {break};
//                };
//                gui_sender.send(MsgToGui::PopulateList(current_list));
            },
            UpdateGame(game) => {
                match update_game(game) {
                    Ok(_) => {gui_sender.send(MsgToGui::UpdateRequestSent);},
                    Err(e) => {gui_sender.send(MsgToGui::Error(e.to_string()));},
                }
            }
        }
    };
}
