use figlet_rs::FIGfont;

pub fn render_name(name: &str) {
    let font = FIGfont::standard().unwrap();

    let figure = font.convert(name);

    return println!("{}", figure.unwrap());
}
