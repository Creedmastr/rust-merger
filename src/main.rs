mod backup;
mod bcommands;
mod merge;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args[1] == "-bak-dir" {
        std::env::set_current_dir("/.rust-merger-bak/").expect("Couldn't move dir");
    } else {
        backup::backup(&args[2]);

        merge::merge(&args[1], &args[2]);
    }
}
