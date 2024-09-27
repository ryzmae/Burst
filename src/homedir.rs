use home;

pub fn homedir() -> String {
    match home::home_dir() {
        Some(path) => {
            return path.display().to_string();
        }
        None => {
            panic!("Could not find home directory");
        }
    }
}