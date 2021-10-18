use std::fs::File;
use std::io::Read;
use axum::{
    extract::{Path},
    response::{IntoResponse},
    http::Response,
    http::StatusCode,
};
use axum::body::{Full};
use crate::path::FILE_DIR;
use std::env;
use crate::bad::handler_404;

pub async fn down(Path(file_name): Path<String>) -> impl IntoResponse {
// pub async fn down(Path(file_name): Path<String>) -> Response<BoxBody> {
    //println!("filename: {}", &file_name);
    let workdir = env::current_dir(); // may crash
    if let Ok(workdir) = workdir {
        let path = format!("{}/{}/{}", workdir.display(), FILE_DIR, file_name);
        if !std::path::Path::new(&path).exists() {
            return (StatusCode::NOT_FOUND, "file not exists").into_response()
        }
        if let Ok(buff) = read_a_file(&path) {
            let res = Response::new(Full::from(buff));
            // let res = Response::builder().header("Content-Type", "application/octet-stream").body(Full::from(buff)).unwrap();
            return res;
        }
    }
    return (StatusCode::NOT_FOUND, "inner error").into_response()
}

fn read_a_file(filename: &String) -> std::io::Result<Vec<u8>> {
    //println!("filename: {}", &filename);
    let mut data = Vec::new();
    let mut file = File::open(&filename).expect("no file found");
    file.read_to_end(&mut data).expect("unable read_to_end");
    return Ok(data);
}