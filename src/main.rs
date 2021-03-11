use std::fs;
use std::env;
use maud::{html};
use rouille::{router, Response};

mod render;
mod build;

fn main(){

    let args: Vec<String> = env::args().collect();
    let mut port = String::from("8080");
    let mut build = false;
    for i in 0..args.len() {
        if args[i] == "--port" {
            port = args[i+1].to_owned();
        }
        if args[i] == "--build" {
            build = true
        }
    }

    let address = format!("0.0.0.0:{}", port);
    if build == true {
        // 
        // let index_page = indexfuncuton...
    }

    rouille::start_server(address, move |request| {
        router!(request,
            (GET) ["/"] => {
                let index_html = build::page_builder::index_page();
                Response::html(index_html)
            },
            (GET) ["/{id}", id: String] => {
                let file_path = ["md/", &id, ".md"].join("");
                match fs::read_to_string(file_path) {
                    Ok(contents) => {
                        let md_html = build::page_builder::md_page(contents);
                        Response::html(md_html)
                    }
                    Err(_why) => {
                        Response::text("Couldn't find that file!")
                    }
                }
            },
            _ => {
                    Response::text("2")
            }
        )
    });
}


