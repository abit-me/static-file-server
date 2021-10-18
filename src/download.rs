use std::fs::File;
use std::io::Read;
use axum::{
    extract::{Path},
    response::{IntoResponse},
    http::{header::{HeaderMap, HeaderValue, CONTENT_TYPE}},
    http::Response,
    http::StatusCode,
};
use axum::body::{Full};
use axum::body::{Body, BoxBody};
use axum::http::{header, Request};
use crate::path::FILE_DIR;
use std::env;


pub async fn down(Path(file_name): Path<String>) -> impl IntoResponse {
// pub async fn down(Path(file_name): Path<String>) -> Response<BoxBody> {
    println!("filename: {}", &file_name);
    let workdir = env::current_dir(); // may crash
    if let Ok(workdir) = workdir {
        let path = format!("{}/{}/{}", workdir.display(), FILE_DIR, file_name);
        if !std::path::Path::new(&path).exists() {
            return (StatusCode::NOT_FOUND, "file not exists").into_response()
        }
        if let Ok(buff) = read_a_file(&path) {
            let mut headers = HeaderMap::new();
            headers.insert(CONTENT_TYPE, "application/octet-stream".parse().unwrap());
            let response = Response::new(Full::from(buff));
            let (mut parts, body) = response.into_parts();
            parts.status = StatusCode::OK;
            parts.headers = headers;
            let response = Response::from_parts(parts, body);
            return response;
        }
    }
    return (StatusCode::NOT_FOUND, "inner error").into_response();
}

pub async fn down2(req: Request<Body>) -> impl IntoResponse {
    let file_name = req.uri().path().to_string();
    let workdir = env::current_dir(); // may crash
    if let Ok(workdir) = workdir {
        let path = format!("{}/{}/{}", workdir.display(), FILE_DIR, file_name);

        if !std::path::Path::new(&path).exists() {
            return (StatusCode::NOT_FOUND, "file not exists").into_response()
        }
        if let Ok(buff) = read_a_file(&path) {
            let mut headers = HeaderMap::new();
            headers.insert(CONTENT_TYPE, "application/octet-stream".parse().unwrap());
            let response = Response::new(Full::from(buff));
            let (mut parts, body) = response.into_parts();
            parts.status = StatusCode::OK;
            parts.headers = headers;
            let response = Response::from_parts(parts, body);
            return response;
        }
    }

    return (StatusCode::NOT_FOUND, "inner error").into_response();
}

// fn find_last_word(s: &String) -> &str {
//     let mut begin = 0;
//     for (i, c) in s.char_indices() {
//         if c == ' ' {
//             begin = i + 1;
//         }
//     }
//     &s[begin..]
// }

fn read_a_file(filename: &String) -> std::io::Result<Vec<u8>> {
    //println!("filename: {}", &filename);
    let mut data = Vec::new();
    let mut file = File::open(&filename).expect("no file found");
    file.read_to_end(&mut data).expect("unable read_to_end");
    return Ok(data);
}