use simple_tqdm::Tqdm;
use std::time::Duration;

fn main() {
    let config = simple_tqdm::Config::new()
        .with_desc("[]")
        .with_total(500)
        .with_disable(false)
        .with_leave(true)
        .with_unit("num")
        .with_scale(0.5)
        .with_postfix("hi")
        .with_color("cyan")
        .with_progress_chars(">= ");

    for _ in (0..250).tqdm_config(config) {
        std::thread::sleep(Duration::from_millis(10));
    }
}
