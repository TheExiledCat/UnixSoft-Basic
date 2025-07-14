use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
fn main() {
    //create stdlib
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_usblib.rs");
    let mut out_file = File::create(dest_path).unwrap();

    let file = File::open("src/stdlib/usblib.h").expect("Could not open usblib.h");
    let reader = BufReader::new(file);

    let mut current_section = "";
    let mut positional = vec![];
    let mut enclosed = vec![];
    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        if line.starts_with("//") {
            if line.contains("positional") {
                current_section = "positional";
            } else if line.contains("enclosed") {
                current_section = "enclosed";
            }
            continue;
        }

        if line.ends_with(";") {
            if let Some((ret_and_name, args_str)) = line.trim_end_matches(';').split_once('(') {
                let args = args_str
                    .trim_end_matches(')')
                    .split(',')
                    .filter(|s| !s.trim().is_empty())
                    .map(|s| s.trim().split_whitespace().next().unwrap())
                    .collect::<Vec<_>>();

                let mut parts = ret_and_name.trim().split_whitespace();
                let return_type = parts.next().unwrap();
                let name = parts.next().unwrap();

                let fn_line = format!(
                    r#"StdLibFunction {{ name: "{}", return_type: "{}", param_types: &[{}] }}"#,
                    name,
                    return_type,
                    args.iter()
                        .map(|a| format!(r#""{}""#, a))
                        .collect::<Vec<_>>()
                        .join(", ")
                );

                match current_section {
                    "positional" => positional.push(fn_line),
                    "enclosed" => enclosed.push(fn_line),
                    _ => {}
                }
            }
        }
    }

    writeln!(
        out_file,
        r#"

pub static POS_FUNCTIONS: &[StdLibFunction] = &[
    {}
];

pub static ENC_FUNCTIONS: &[StdLibFunction] = &[
    {}
];
"#,
        positional.join(",\n    "),
        enclosed.join(",\n    ")
    )
    .unwrap();
}
