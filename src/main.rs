mod filesystem;

fn main() {
    run();
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    for entry in walkdir::WalkDir::new("media") {
        println!("{}", entry?.path().display());
    };
    Ok(())
}
