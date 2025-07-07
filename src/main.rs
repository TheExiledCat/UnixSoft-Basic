use pico_args::Arguments;

mod cliutils;

#[derive(Debug)]
enum Command {
    Init { working_dir: String },
    Build { entry: String },
    Run { entry: String },
    Help,
}
fn show_help() {
    println!("usbasic - UnixSoft BASIC compiler and cli tool");
}
impl Command {
    fn new(args: Arguments) -> Result<Self, ()> {
        let mut args = args;
        match args.subcommand().unwrap() {
            Some(arg) => {
                return Ok(match arg.as_str() {
                    "init" => Command::Init {
                        working_dir: args
                            .opt_free_from_str()
                            .unwrap()
                            .unwrap_or_else(|| String::from("./")),
                    },
                    _ => Command::Help,
                });
            }
            None => {
                show_help();
                return Err(());
            }
        };
    }
}
trait HelpDisplay {
    fn display_help(&self);
}
impl HelpDisplay for Command {
    fn display_help(&self) {
        todo!()
    }
}
fn main() {
    let args = Arguments::from_env();
    if let Ok(command) = Command::new(args) {
        println!("{:?}", command);
    }
}
