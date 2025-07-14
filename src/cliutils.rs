use std::{collections::hash_map::Keys, env, error::Error, fs, path::PathBuf};

use pico_args::Arguments;

use crate::{
    usbcompiler::compiler::{self, compiler::Compiler},
    version::Version,
};
#[derive(Debug)]
pub enum Command {
    Init { working_dir: String },
    Build { entry: String },
    Run { entry: String },
    Version,
    Help,
}
pub fn show_help() {
    println!("usbasic - UnixSoft BASIC compiler and cli tool");
    println!("commands (use command --help to see details):\n");
    println!(
        r#"
            version - show version info
            init    - initialize a new USB project
            build   - build the current USB project
            run     - build and run the current USB project, or run a USB file as a script
        "#
    );
}
impl Command {
    pub fn new(args: Arguments) -> Result<Self, ()> {
        let mut args = args;
        match args.subcommand().unwrap() {
            Some(arg) => {
                if args.contains("--help") || args.contains("-h") {
                    help_message(arg.as_str());
                    return Err(());
                }
                return Ok(match arg.as_str() {
                    "init" => Command::Init {
                        working_dir: args
                            .opt_free_from_str()
                            .unwrap()
                            .unwrap_or_else(|| String::from("./")),
                    },
                    "build" => Command::Build {
                        entry: args
                            .opt_free_from_str()
                            .unwrap()
                            .unwrap_or_else(|| String::from("./")),
                    },
                    "version" => Command::Version,
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
            Command::Build { entry } => Compiler::new(PathBuf::from(entry)).compile().unwrap(),
            Command::Run { entry } => todo!(),
            Command::Help => show_help(),
            Command::Version => Version::print(),
        }
        return Ok(());
    }
}

fn help_message(command_name: &str) {
    println!("\n{}\n", command_name);
    match command_name {
        "init" => println!(
            "Create a new USB project in a given directory and create it if it does not exist.\nUSAGE: usbasic init [directory=./]\n"
        ),
        "build" => println!(
            "Build the current USB project and output the final binary into the build/ directory, must be used in an existing USB project.\nUSAGE: usbasic build"
        ),
        "run" => println!(
            "Builds the current USB project using usbasic build and runs the final binary or runs the given .usb file as if it were a script\nUSAGE: usbasic run [usb_file]"
        ),
        "version" => println!("Shows version information\nUSAGE: usbasic version"),
        _ => show_help(),
    }
    println!();
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
    fs::write(
        &dir.join(format!(
            "{}.usbp.json",
            dir.file_name().unwrap().to_string_lossy()
        )),
        r#"
{
    "entry_point": "src/main.usb"
}
    "#,
    )
    .unwrap();
    fs::write(&dir.join(".gitignore"), "obj\nbuild\n").unwrap();
}
pub fn generate_default_project(dir: PathBuf) {
    let dir = relative_to_absolute(dir);

    // generate directory if it doesnt exist yet
    if let Ok(exists) = fs::exists(&dir) {
        if exists {
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
