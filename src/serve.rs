/*
    `serve.rs` - HTTP Request/Response Management, Route Validation & Serving
    validates routes & serves HTTP responses with the corresponding route files
*/
use actix_files::NamedFile;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, Result};

use crate::config::get_config;

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// directory of rendered templates
static TEMPLATE_DIR: &str = "rendered_templates";

pub async fn serve_content(req: HttpRequest) -> Result<NamedFile> {
    // get the HTTP request path
    let req_path = format!("/{}", req.match_info().query("route"));

    let config = get_config();

    let routes = config["routes"][req_path].to_string().replace("\"", "");

    let status_code;
    /*
        404 Not Found Handler
        `routes` returns 'null' if the route entry doesn't exist
    */
    let response_file = if routes == "null" {
        let page_404 = config["error_pages"]["404"].to_string().replace("\"", "");
        status_code = StatusCode::NOT_FOUND;
        format!("{}/{}", TEMPLATE_DIR, page_404)
    } else if routes == "comment.html" {
        let comment = format!("/{}", req.match_info().query("comment"));
        println!("{}", comment);

        let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("rendered_templates/read.html")
        .unwrap();

        let new_line = String::from("<div class=\"pt-6 md:grid md:grid-cols-12 md:gap-8\"><dd class=\"mt-2 md:mt-0 md:col-span-12\"><p class=\"text-base text-white\">TEST</p></dd></div>");

        if let Err(e) = writeln!(file, "{}", new_line) {
            eprintln!("Couldn't write to file: {}", e);
        }
        status_code = StatusCode::OK;
        format!("{}/{}", TEMPLATE_DIR, routes)
    } else if routes == "read.html" {
        println!("read");

        if let Ok(lines) = read_lines("static/comments.txt") {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(ip) = line {
                    println!("{}", ip);
                }
            }
        }
        status_code = StatusCode::OK;
        format!("{}/{}", TEMPLATE_DIR, routes)
    } else {
        status_code = StatusCode::OK;
        format!("{}/{}", TEMPLATE_DIR, routes)
    };

    Ok(NamedFile::open(response_file)?
        .set_status_code(status_code)
        .prefer_utf8(true)
        .use_last_modified(true))
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}