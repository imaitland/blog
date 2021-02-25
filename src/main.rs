use maud::{html};
use rouille::{router, Response};
mod renderers;
use std::fs;

fn main(){
    rouille::start_server("0.0.0.0:82", move |request| {
        //println!("{:?}", request);
        router!(request,
            (GET) ["/assets/{asset}", asset: String] => {
                rouille::match_assets(&request, ".")
            },
            (GET) ["/styles/{style}", style: String] => {
                rouille::match_assets(&request, ".")
            },
            (GET) ["/node_modules/force-graph/dist/force-graph.min.js"] => {
                rouille::match_assets(&request, ".")
            },
            (GET) ["/js/graph.js"] => {
                rouille::match_assets(&request, ".")
            },
            (GET) ["/graph"] => {
                let graph_data: renderers::Graph = renderers::generate_graph("md");
                Response::json(&graph_data)
            },
            (GET) ["/"] => {
                let icons = renderers::Icons();
                let force_graph_script = renderers::Script("/node_modules/force-graph/dist/force-graph.min.js");
                let graph_script = renderers::Script("/js/graph.js");

                Response::html(html! {
                    head {
                        (icons)
                        (force_graph_script)
                    }
                    div id="graph" {} 
                    (graph_script)
                })

            },
            (GET) ["/{id}", id: String] => {
                let file_path = ["md/", &id, ".md"].join("");

                match fs::read_to_string(file_path) {
                    Ok(contents) => {

                        let (frontmatter, md_contents) = renderers::split_contents(&contents);

                        let meta = renderers::Meta(&frontmatter);
                        let icons = renderers::Icons();
                        let md_css = renderers::Css("styles/md.css");
                        let logo = renderers::Logo();
                        let md = renderers::Markdown(&md_contents);

                        Response::html(html!{
                            head {
                                (icons)
                                (meta)
                                (md_css)
                            }
                            (logo)
                            div class="markdown-body" {
                                (md)
                            }
                        })
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


