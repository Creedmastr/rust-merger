mod backup;
mod bcommands;
mod merge;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args[1] == "--bak-dir" {
        bcommands::Commands::execute("cd /.rust-merger-bak/".to_string())
    } else {
        backup::backup(&args[2]);

        merge::merge(&args[1], &args[2]);
    }
}
