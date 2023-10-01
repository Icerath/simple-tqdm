use rayon::prelude::*;
use simple_tqdm::ParTqdm;
use std::time::Duration;

fn main() {
    (0..500).into_par_iter().tqdm().for_each(|_| {
        std::thread::sleep(Duration::from_millis(10));
    });
}
