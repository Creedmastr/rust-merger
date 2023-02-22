mod backup;
mod bcommands;
mod merge;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    backup::backup(&args[2]);

    merge::merge(&args[1], &args[2]);
}
