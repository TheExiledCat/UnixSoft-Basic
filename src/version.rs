const BINARY: &'static str = "UnixSoft-BASIC";
const VERSION: &'static str = "0.0.1";
const ARC: &'static str = "x86_64_linux";
pub struct Version;
impl Version {
    pub fn print() {
        println!("{} {} version {}", ARC, BINARY, VERSION);
    }
}
