use std::path::Path;

// Used for multithreading of initial file loading to separate it from the gui
use std::sync::mpsc;
use std::thread;

// All project modules
use parse::als::Project;
mod cache;
mod extract;
mod gui;
use gui::gui::Gui;
pub mod macros;
mod palsa;
mod parallel;
mod parse;

fn main() {
    let (sender, receiver) = mpsc::channel::<Result<Vec<Project>, String>>();
    let dir: &Path = Path::new("als_files/");

    // Creates a thread to run palsa in parralel with ui for initial file loading
    thread::spawn(move || {
        let projects = palsa::run_palsa(dir);
        sender.send(projects).unwrap();
    });

    Gui::run(receiver);
}
