mod backup;
mod bcommands;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    backup::backup(&args[1]);
}
