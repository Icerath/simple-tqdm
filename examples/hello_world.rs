use simple_tqdm::Tqdm;
use std::time::Duration;

fn main() {
    for _ in (0..250).tqdm() {
        std::thread::sleep(Duration::from_millis(10));
    }
}
