fn get_title() -> String {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let description = env!("CARGO_PKG_DESCRIPTION");
    format!("{}, (v{}), {}", name, version, description)
}

fn print_short_banner() {
    println!("{}", get_title());
}

fn print_long_banner() {
    let owner = format!("Written by: {}", env!("CARGO_PKG_AUTHORS"));
    let homepage = format!("Homepage: {}", env!("CARGO_PKG_HOMEPAGE"));

    print_short_banner();
    println!("{}\n{}\n", owner, homepage)
}

fn usage(_message: String) {
    let usage = format!("Usage: {} <somefile>.md", env!("CARGO_BIN_NAME"));

    print_long_banner();
    println!("{}\n\n{}", _message, usage);
}

fn parse_markdown_file(_filename: &str) {
    print_short_banner();
    println!("[ INFO ] Trying to parse {}...", _filename);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => usage(String::from(
            "[ ERROR ] You forgot to specify the markdown file to parse!",
        )),
        2 => parse_markdown_file(&args[1]),
        _ => usage(String::from("[ ERROR ] Invalid invocation foo:penis!")),
    }
}
