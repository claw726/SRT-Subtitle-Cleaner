use std::fs;
use std::io;
use std::path::PathBuf;
use regex::Regex;
use rfd::{FileDialog, MessageDialog};

fn dialog_box(message: &str, title: &str, level: rfd::MessageLevel) {
    MessageDialog::new()
        .set_title(title)
        .set_description(message)
        .set_level(level)
        .show();
}

fn remove_css_font_styling(file_path: &PathBuf) -> io::Result<()> {
    let content = fs::read_to_string(file_path)?;

    // Remove CSS Font Styling
    let re = Regex::new(r"<.*?>").unwrap();
    let cleaned_content : String = re.replace_all(&content, "").to_string();

    fs::write(file_path, cleaned_content)?;

    dialog_box("CSS Font Styling removed successfully", "Success", rfd::MessageLevel::Info);

    Ok(())
}


fn main() {
    if let Some(file_path) = FileDialog::new()
        .add_filter("Subtitles", &["srt"])
        .pick_file()
    {
        remove_css_font_styling(&file_path).unwrap_or_else(|err|{
            eprintln!("Error: {}", err);
            dialog_box("Error removing CSS Font Styling", "Error", rfd::MessageLevel::Error);
            std::process::exit(1);
        });
    } else {
        dialog_box("No file selected", "Error", rfd::MessageLevel::Error);
        std::process::exit(1);
    }
}


