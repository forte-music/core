use fixture_setup::load::load;

fn main() {
    if let Err(err) = load() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
