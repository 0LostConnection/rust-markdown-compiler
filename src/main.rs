fn get_version() -> u16 {
    1000
}

fn usage() {
    // Long way
    // let version: u16;
    // version = get_version();
    // println("Version {}", version)

    // Short way
    let version: u16 = get_version();

    println!("Tinymd, a markdown compiler written in Rust by Geovane Saraiva da Silva");
    println!("Version {version}");
}

fn main() {
    usage()
}
