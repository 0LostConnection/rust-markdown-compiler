use std::fmt::format;

fn parse_markdown_file() {}

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
    let usage = format!("Usage: {} <somefile>.md",env!("CARGO_BIN_NAME"));
    
    print_short_banner();
    println!("{}\n{}\n{}", owner, homepage, usage)
}

fn usage() {}

fn main() {
    print_long_banner();
    usage();
}