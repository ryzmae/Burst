use color_eyre::owo_colors::OwoColorize;
use figlet_rs::FIGfont;

pub fn render_name(name: &str) {
    let font = FIGfont::standard().unwrap();

    let binding = font.convert(name);

    let randomcolor = match rand::random::<u8>() % 6 {
        1 => binding.unwrap().red().bold().to_string(),
        2 => binding.unwrap().green().bold().to_string(),
        3 => binding.unwrap().yellow().bold().to_string(),
        4 => binding.unwrap().blue().bold().to_string(),
        5 => binding.unwrap().magenta().bold().to_string(),
        _ => binding.unwrap().bright_yellow().bold().to_string() // Default if no match
    };

    println! {"{}", randomcolor};
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
