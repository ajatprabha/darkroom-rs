mod config;
use config::Config;

fn main() {
    let cfg = Config::new();

    println!("{:#?}", cfg);
}
