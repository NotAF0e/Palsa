// All project modules
use parse::als::AlsData;
mod cache;
mod extract;
mod gui;
pub mod macros;
mod palsa;
mod parallel;
mod parse;

fn main() {
    let dir: &str = "test als files/";

    let all_als_data = palsa::run(dir);

    println!("\nPalsa completed with {} files     :)", all_als_data.len(),)
}
