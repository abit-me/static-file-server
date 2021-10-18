use walkdir::{DirEntry, WalkDir};
use axum::{
    response::Html,
};
use crate::path::FILE_DIR;

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub async fn show_files() -> Html<&'static str> {

    let htlm_header = "<html><head><title>Static Server</title></head>";
    let html_tail = "</html>";
    let mut html_str = htlm_header.to_string();

    let walker = WalkDir::new(FILE_DIR);
    for entry in walker {
        if let Ok(entry) = entry {
            if entry.path().is_dir() { continue };
            if is_hidden(&entry) { continue };
            if let Some(file_name) = entry.file_name().to_str() {
                let li: String = format!("<li><a href=\"/{}\">{}</a></li>", file_name, file_name).to_owned();
                //println!("item: {:?}", li);
                html_str.push_str(&li);
            }
        }
    }

    html_str += html_tail;
    return Html(string_to_static_str(html_str))
}
