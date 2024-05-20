use std::time::Instant;

use parse::als::AlsData;

mod cache;
mod extract;
pub mod macros;
mod parallel;
mod parse;

fn main() {
    let dir: &str = "test als files/";

    let start_time = Instant::now();
    let all_als_data = palsa(dir);
    let elapsed_time = start_time.elapsed();

    println!(
        "\nPalsa completed with {} files in {:?}     :)",
        all_als_data.len(),
        elapsed_time
    )
}

fn palsa(dir: &str) -> Vec<AlsData> {
    let all_als_data = parallel::parallel_parse(dir);
    if let Err(e) = cache::cache(all_als_data.clone()) {
        eprintln!("Error creating cache: {:?}", e);
    }

    let all_als_data = cache::retrieve().expect("Failed to retreive cache!");

    if let Err(e) = cache::cache(all_als_data.clone()) {
        eprintln!("Error creating cache: {:?}", e);
    }

    return all_als_data;
}
