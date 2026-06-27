use std::env;
use std::process;

use vamc::driver::Driver;
use vamc::host::SystemHost;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let source_path = match arguments.get(1) {
        Some(path) => path,
        None => {
            eprintln!("Usage: vamc SOURCE_FILE");
            process::exit(2);
        },
    };

    let driver = Driver::new(SystemHost);
    match driver.compile(source_path) {
        Ok(output) => println!("Wrote {}", output.display()),
        Err(message) => {
            eprintln!("{message}");
            process::exit(1);
        },
    }
}
