use color_eyre::owo_colors::OwoColorize;
use figlet_rs::FIGfont;

pub fn render_name(name: &str) {
    let font = FIGfont::standard().unwrap();

    let binding = font.convert(name);

    return println!{"{}", binding.unwrap().bright_yellow().bold()};
}
