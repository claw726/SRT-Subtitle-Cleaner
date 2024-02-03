use std::env;
use std::fs;
use std::io;
use regex::Regex;

fn remove_css_font_styling(file_path: &str) -> io::Result<()> {
    let content = fs::read_to_string(file_path)?;

    // Remove CSS Font Styling
    let re = Regex::new(r"<.*?>").unwrap();
    let cleaned_content : String = re.replace_all(&content, "").to_string();

    fs::write(file_path, cleaned_content)?;

    println!("CSS Font Styling removed from {}", file_path);

    Ok(())
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    if let Err(err) = remove_css_font_styling(file_path) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
