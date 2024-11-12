use std::io::{stdout, Write};

use color_eyre::owo_colors::OwoColorize;
use figlet_rs::FIGfont;

pub fn render_name(name: &str) {
    let font = FIGfont::standard().unwrap();

    let binding = font.convert(name);

    let randomcolor = match rand::random::<u8>() % 5 {
        1 => binding.unwrap().bright_red().bold().to_string(),
        2 => binding.unwrap().bright_green().bold().to_string(),
        3 => binding.unwrap().bright_blue().bold().to_string(),
        4 => binding.unwrap().bright_magenta().bold().to_string(),
        _ => binding.unwrap().bright_yellow().bold().to_string(), // Default if no match
    };

    // Uses less memory than println!() due to buffering
    stdout().write_all(randomcolor.as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_name() {
        render_name("Burst");
        assert_eq!(true, true);
    }
}
