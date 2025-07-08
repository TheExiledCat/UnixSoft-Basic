use cliutils::Command;
use pico_args::Arguments;

mod cliutils;
mod usbcompiler;

fn main() -> Result<(), u8> {
    let args = Arguments::from_env();

    if let Ok(command) = Command::new(args) {
        return command.run();
    }

    return Ok(());
}
