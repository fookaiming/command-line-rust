fn main() {
    if let Err(e) = cat::parse_args().and_then(cat::run) {
        eprint!("{}", e);
        std::process::exit(1);
    }
}
