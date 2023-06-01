use std::fs;
use std::env;
use rouille::{router, Response};

mod render_html;
mod page_builder;

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
        match page_builder::build() {
            Ok(_r) => {
                println!("Build complete");
            },
            Err(_why) => {
                println!("Build Failed!")
                
            }
        }
    }

    rouille::start_server(address, move |request| {
        router!(request,
            //(GET) ["/resume"] => {
                //Response::redirect_307("/iain_maitland_resume.pdf")
            //},
            (GET) ["/iain_maitland_resume.pdf"] => {
                rouille::match_assets(&request, ".")
            },
            //(GET) ["/photos"] => {
                //Response::redirect_307("https://photos.google.com/share/AF1QipPgG8AHI2l56B7gtrN2GKYQjrJW05wOy1GPVqAGj2eR-RtTuoRFTFdCSN0Z13Zryw?key=U0VsTGFNSkM5VVBOeVFOajg4RXpRNFc2Q25KV1RR")
            //},
            //(GET) ["/notebooks"] => {
                //Response::redirect_307("")
            //},
            (GET) ["/assets/paintings/{painting}", _painting: String] => {
                rouille::match_assets(&request, ".")
            },
            (GET) ["/assets/{asset}", _asset: String] => {
                rouille::match_assets(&request, ".")
            },
            (GET) ["/"] => {
                let index_html = page_builder::index_page();
                Response::html(index_html)
            },
            (GET) ["/{id}", id: String] => {
                let file_path = ["md/", &id, ".md"].join("");
                match fs::read_to_string(file_path) {
                    Ok(contents) => {
                        let md_html = page_builder::md_page(contents);
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


