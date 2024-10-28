use web_sys::window;
use chrono::NaiveDateTime;

pub fn is_dark_mode() -> bool {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(theme)) = storage.get_item("theme") {
                return theme == "dark";
            }
        }
    }
    false
}

pub fn toggle_dark_mode(is_dark: bool) {
    if let Some(document) = window().and_then(|w| w.document()) {
        if let Some(root) = document.document_element() {
            let mut classes: Vec<String> = root
                .class_name()
                .split_whitespace()
                .map(String::from)
                .collect();

            if is_dark {
                if !classes.contains(&"dark".to_string()) {
                    classes.push("dark".to_string());
                }
            } else {
                classes.retain(|c| c != "dark");
            }

            let _ = root.set_attribute("class", &classes.join(" "));

            if let Some(window) = window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.set_item("theme", if is_dark { "dark" } else { "light" });
                }
            }
        }
    }
}

pub fn format_local_time(time_str: &str) -> String {
    if let Ok(dt) = NaiveDateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M") {
        dt.format("%I:%M %p %B %e, %Y").to_string()
    } else {
        time_str.to_string()
    }
}