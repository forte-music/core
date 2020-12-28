use fixture_setup::load::load;

fn main() {
    if let Err(err) = load() {
        println!("{}", err);
    }
}
