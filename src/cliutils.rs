use std::{env, fs, path::PathBuf};

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
    fs::write(&dir.join("src/main.usb"), "PRINT \"Hello, World\"").unwrap();
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
