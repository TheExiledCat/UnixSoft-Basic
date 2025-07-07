use cliutils::Command;
use pico_args::Arguments;

mod cliutils;
mod usbcompiler;

fn main() {
    let args = Arguments::from_env();
    if let Ok(command) = Command::new(args) {
        println!("{:?}", command);
    }
}
