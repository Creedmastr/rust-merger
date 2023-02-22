mod merging;
mod backup;

fn main() {
    let args = std::env::args().collect();

    backup::backup(args[1]);
}
