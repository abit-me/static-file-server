use std::io::Write;
use axum::{
    extract::{ContentLengthLimit, Multipart},
    response::Html,
};
use crate::path::FILE_DIR;
use tokio::task::block_in_place;

pub async fn show_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/upload" method="post" enctype="multipart/form-data">
                    <label>
                        Upload file:
                        <input type="file" name="file" multiple>
                    </label>
                    <input type="submit" value="Upload files">
                </form>
            </body>
        </html>
        "#,
    )
}

pub async fn accept_form(
    ContentLengthLimit(mut multipart): ContentLengthLimit<
        Multipart,
        {
            512 * 1024 * 1024 /* 512mb */
        },
    >,
) -> Html<&'static str> {

    while let Ok(Some(field)) = multipart.next_field().await {

        if let Some(file_name) = &field.file_name() {
            let name = file_name.to_string();
            if let Ok(data) = &field.bytes().await {
                let save_path = format!("{}/{}", FILE_DIR, name);
                if let Ok(mut file) = std::fs::File::create(&save_path) {
                    println!("write `{}` is {} bytes file {:?}", save_path, data.len(), file);
                    //file.write_all(&data).map(|_| file);
                    let _ = block_in_place(move || file.write_all(&data).map(|_| file));
                }
            }
        }
    }

    Html(
        r#"
        <!doctype html>
        <html>
        <meta http-equiv="refresh" content="0; url='/'">
        </html>
        "#,
    )
}