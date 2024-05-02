use clap::{App, Arg};

fn main() {
    let app = App::new("echo")
        .version("0.0.1")
        .author("max")
        .about("simplifed echo in rust for learning")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_the_last_next_line")
                .short("n")
                .help("omit the last next line")
                .takes_value(false),
        )
        .get_matches();

    let text = app.values_of_lossy("text").unwrap();
    let omit_newline = app.is_present("omit_the_last_next_line");

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
