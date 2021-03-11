use std::fs;
use std::io::{Write, BufReader, BufRead, Error};
use std::env;
use maud::{html};
use rouille::{router, Response};
use serde_json;

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

    }

    rouille::start_server(address, move |request| {
        router!(request,
            (GET) ["/assets/{asset}", asset: String] => {
                rouille::match_assets(&request, ".")
            },
            (GET) ["/styles/{style}", style: String] => {
                rouille::match_assets(&request, ".")
            },
            (GET) ["/js/{script}", script: String] => {
                rouille::match_assets(&request, ".")
            },
            (GET) ["/node_modules/{module}/{dir}/{file}", module:String, dir: String, file: String] => {
                rouille::match_assets(&request, ".")
            },
            (GET) ["/graph"] => {
                let graph_data: render::Graph = render::helpers::generate_graph("md");
                // Save this a .json file
                let path = "graph_data.json";
                let mut output = fs::File::create(path).unwrap();

                // Parse to json.
                let graph_json = serde_json::to_string(&graph_data).unwrap();
                write!(output, "{}", graph_json).unwrap();
                Response::json(&graph_data)
            },
            (GET) ["/"] => {

                let icons = render::Icons();
                let force_graph_script = render::Script("node_modules/force-graph/dist/force-graph.min.js");
                let graph_script = render::Script("js/graph.js");
                let logo = render::Logo();
                let default_css = render::Css("styles/default.css");

                // animejs
                let anime_js_script = render::Script("node_modules/animejs/lib/anime.min.js");
                let anime_script = render::Script("js/anime.js");
                let anime_css = render::Css("styles/anime.css");

                // Index page for visitors who have JS disabled.
                let ix = render::helpers::generate_index("md");
                let stripped = ix.iter().map(|pa|{
                    let result = pa.strip_prefix("md/").unwrap().strip_suffix(".md").unwrap();
                    String::from(result)
                }).collect();
                let posts_list = render::Index(&stripped);
                let index_html = html! {
                    head {
                        (icons)
                        (force_graph_script)
                        (default_css)
                        (anime_js_script)
                        (anime_css)
                    }
                    div class="logo-front_page"{
                        (logo)
                    }
                    div id="graph" {} // div with this id is targeted by graph js...
                    div id="NoJS" class="posts-index" {
                        (posts_list)
                    }
                    (graph_script)
                    (anime_script)
                };
                Response::html(index_html)
            },
            (GET) ["/{id}", id: String] => {
                let file_path = ["md/", &id, ".md"].join("");

                match fs::read_to_string(file_path) {
                    Ok(contents) => {

                        let logo = render::Logo();
                        let anime_script = render::Script("js/anime.js");
                        let anime_css = render::Css("styles/anime.css");
                        let anime_js_script = render::Script("node_modules/animejs/lib/anime.min.js");

                        let (frontmatter, md_contents) = render::helpers::split_contents(&contents);

                        let meta = render::Meta(&frontmatter);
                        let icons = render::Icons();
                        let md_css = render::Css("styles/md.css");
                        let md = render::Markdown(&md_contents);

                        // Syntax highlighting
                        let syntax_css = render::Css("https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.6.0/styles/default.min.css");
                        let syntax_script = render::Script("https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.6.0/highlight.min.js");
                        let syntax_init = render::Script("js/syntax.js");

                        let post_html = html!{
                            head {
                                (icons)
                                (meta)
                                (md_css)
                                (anime_js_script)
                                (anime_css)
                                (syntax_css)
                                (syntax_script)
                            }
                            div class="logo-blog_page"{
                                (logo)
                            }
                            div class="markdown-body" {
                                (md)
                            }
                            (anime_script)
                            (syntax_init)
                        };

                        Response::html(post_html)
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


