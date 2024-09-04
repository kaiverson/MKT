mod config;
mod run;

use config::Config;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let config: Config = Config::build(args);

    // TODO: Change run to return Result<String, String> and we can print that stuff here.
    // TODO: Remove all exit() calls. They make the program less consistent.
    if let Err(e) = run::run(config) {
        eprintln!("We had a little no beuno: {e}");
    }
}
