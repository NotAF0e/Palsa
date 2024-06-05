use crate::cache;
use crate::parallel;
use crate::AlsData;

pub fn run(dir: &str) -> Result<Vec<AlsData>, String> {
    let all_als_data = parallel::parallel_parse(dir);
    match all_als_data {
        Ok(all_als_data) => {
            if let Err(e) = cache::cache(all_als_data.clone()) {
                eprintln!("Error creating cache: {:?}", e);
            }

            let all_als_data = cache::retrieve().expect("Failed to retreive cache!");

            if let Err(e) = cache::cache(all_als_data.clone()) {
                eprintln!("Error creating cache: {:?}", e);
            }
            Ok(all_als_data)
        }
        Err(error) => Err(error),
    }
}
