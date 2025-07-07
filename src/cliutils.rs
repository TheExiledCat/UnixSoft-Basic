use std::{env, error::Error, fs, path::PathBuf};

use pico_args::Arguments;
#[derive(Debug)]
pub enum Command {
    Init { working_dir: String },
    Build { entry: String },
    Run { entry: String },
    Help,
}
pub fn show_help() {
    println!("usbasic - UnixSoft BASIC compiler and cli tool");
}
impl Command {
    pub fn new(args: Arguments) -> Result<Self, ()> {
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
    pub fn run(&self) -> Result<(), u8> {
        match self {
            Command::Init { working_dir } => generate_default_project(PathBuf::from(working_dir)),
            Command::Build { entry } => todo!(),
            Command::Run { entry } => todo!(),
            Command::Help => todo!(),
        }
        return Ok(());
    }
}
pub trait HelpDisplay {
    fn display_help(&self);
}
impl HelpDisplay for Command {
    fn display_help(&self) {
        todo!()
    }
}

fn relative_to_absolute(path: PathBuf) -> PathBuf {
    if path.is_absolute() {
        return path;
    } else {
        let cwd = env::current_dir().expect("read write access to the dir");
        return cwd.join(path);
    }
}
fn create_project_directory(dir: PathBuf) {
    fs::create_dir(&dir.join("build")).unwrap();
    fs::create_dir(&dir.join("obj")).unwrap();
    fs::create_dir(&dir.join("src")).unwrap();
    fs::write(&dir.join("src/main.usb"), "10 PRINT \"Hello, World\"").unwrap();
}
pub fn generate_default_project(dir: PathBuf) {
    let dir = relative_to_absolute(dir);

    // generate directory if it doesnt exist yet
    if let Ok(exists) = fs::exists(&dir) {
        if (exists) {
            if !&dir.is_dir() {
                panic!("expected directory")
            }
            //create files in that sub dir
            create_project_directory(dir);
        } else {
            fs::create_dir(&dir).unwrap();
            //create files in that sub dir
            create_project_directory(dir);
        }
        // create files in that sub dir
    } else {
        panic!("IO error");
    }
}
