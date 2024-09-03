mod config;
mod run;

use config::Config;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let config: Config = Config::build(args);

    if let Err(e) = run::run(config) {
        eprintln!("We had a little no beuno: {e}");
    }
}
