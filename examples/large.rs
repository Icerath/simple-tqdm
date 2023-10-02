use simple_tqdm::Tqdm;

fn main() {
    for _ in (0..2 << 24).tqdm() {}
}
