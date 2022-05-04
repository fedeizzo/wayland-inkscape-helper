use std::process::Command;
use std::str::from_utf8;

pub fn get_clipboard_content() -> Option<String> {
    let out = Command::new("sh").arg("-c").arg("wl-paste").output().ok()?;
    let extracted = from_utf8(&out.stdout).ok()?;
    let extracted = extracted.to_string();
    let extracted = extracted
        .split("\n")
        .filter(|x| !x.starts_with("<!-"))
        .collect::<Vec<&str>>()
        .join("\n");
    Some(extracted)
}

pub fn insert_clipboard_content(content: &str) {
    let _ = Command::new("sh")
        .arg("-c")
        .arg(format!("wl-copy '{}'", content))
        .output();
}
