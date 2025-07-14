#[derive(Debug)]
pub struct StdLibFunction {
    pub name: &'static str,
    pub return_type: &'static str,
    pub param_types: &'static [&'static str], // use slice instead of Vec
}
include!(concat!(env!("OUT_DIR"), "/generated_usblib.rs"));
