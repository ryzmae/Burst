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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_homedir() {
        let home = home::home_dir().unwrap();
        assert_eq!(homedir(), home.display().to_string());
    }
}
