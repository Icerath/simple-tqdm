use simple_tqdm::{Config, Tqdm};

fn main() {
    let config = Config::new().with_progress_chars(">= ");
    std::thread::scope(|scope| {
        for _ in 0..3 {
            scope.spawn(|| for _ in (0..2 << 24).tqdm_config(config.clone()) {});
        }
    });
}
