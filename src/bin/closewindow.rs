use serde::Deserialize;
use std::process::Command;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Deserialize, Debug)]
struct Window {
    title: String,
    app_id: String,
}

fn main() -> Result<()> {
    if let Err(e) = close_window() {
        eprintln!("Error: {:?}", e);
    }
    Ok(())
}

fn close_window() -> Result<()> {
    let output = Command::new("niri")
        .arg("msg")
        .arg("-j")
        .arg("focused-window")
        .output()?;
    let json = String::from_utf8(output.stdout)?;
    let window = serde_json::from_str::<Window>(&json)?;
    println!("window: {:?}", window);
    if window.app_id != "wlroots" {
        Command::new("niri")
            .arg("msg")
            .arg("action")
            .arg("close-window")
            .output()?;
    }
    Ok(())
}
