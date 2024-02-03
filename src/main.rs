use std::fs;
use std::io;
use std::path::PathBuf;
use regex::Regex;
use clap::{App, Arg};

fn remove_css_font_styling(file_path: &PathBuf) -> io::Result<()> {
    let content = fs::read_to_string(file_path)?;

    // Remove CSS Font Styling
    let re = Regex::new(r"<.*?>").unwrap();
    let cleaned_content : String = re.replace_all(&content, "").to_string();

    fs::write(file_path, cleaned_content)?;

    println!("CSS Font Styling removed from {}", file_path.to_str().unwrap());

    Ok(())
}


fn main() {
    let matches = App::new("SRT Subtitle Cleaner")
        .version("1.0")
        .author("claw726")
        .about("Remove CSS Font Styling from SRT Subtitle files")
        .arg(Arg::with_name("file_path")
            .help("Path to the SRT file")
            .required(true)
            .index(1))
        .get_matches();

    let file_path = PathBuf::from(matches.value_of("file_path").unwrap());

    remove_css_font_styling(&file_path).unwrap_or_else(|err|{
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }); 
}
