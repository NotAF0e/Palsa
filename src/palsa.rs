use crate::cache;
use crate::parallel;
use crate::AlsData;

pub fn run(dir: &str) -> Vec<AlsData> {
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
