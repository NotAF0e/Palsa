use std::sync::mpsc;
use std::thread;

// All project modules
use parse::als::AlsData;
mod cache;
mod extract;
mod gui;
use gui::gui::Gui;
pub mod macros;
mod palsa;
mod parallel;
mod parse;

fn main() {
    let (sender, receiver) = mpsc::channel::<Result<Vec<AlsData>, String>>();
    let dir: &str = "als_files/";

    thread::spawn(move || {
        let all_als_data = palsa::run(dir);
        sender.send(all_als_data).unwrap();
    });

    Gui::run(receiver);
}
