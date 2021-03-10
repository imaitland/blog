use maud::{html};
use rouille::{router, Response};
mod renderers;
use std::fs;

fn main(){
    rouille::start_server("0.0.0.0:8080", move |request| {
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
                let graph_data: renderers::Graph = renderers::helpers::generate_graph("md");
                Response::json(&graph_data)
            },
            (GET) ["/"] => {

                let icons = renderers::Icons();
                let force_graph_script = renderers::Script("node_modules/force-graph/dist/force-graph.min.js");
                let graph_script = renderers::Script("js/graph.js");
                let logo = renderers::Logo();
                let default_css = renderers::Css("styles/default.css");

                // animejs
                let anime_js_script = renderers::Script("node_modules/animejs/lib/anime.min.js");
                let anime_script = renderers::Script("js/anime.js");
                let anime_css = renderers::Css("styles/anime.css");

                // Index page for visitors who have JS disabled.
                let ix = renderers::helpers::generate_index("md");
                let stripped = ix.iter().map(|pa|{
                    let result = pa.strip_prefix("md/").unwrap().strip_suffix(".md").unwrap();
                    String::from(result)
                }).collect();
                let posts_list = renderers::Index(&stripped);

                Response::html(html! {
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
                })

            },
            (GET) ["/{id}", id: String] => {
                let file_path = ["md/", &id, ".md"].join("");

                match fs::read_to_string(file_path) {
                    Ok(contents) => {

                        let logo = renderers::Logo();
                        let anime_script = renderers::Script("js/anime.js");
                        let anime_css = renderers::Css("styles/anime.css");
                        let anime_js_script = renderers::Script("node_modules/animejs/lib/anime.min.js");

                        let (frontmatter, md_contents) = renderers::helpers::split_contents(&contents);

                        let meta = renderers::Meta(&frontmatter);
                        let icons = renderers::Icons();
                        let md_css = renderers::Css("styles/md.css");
                        let md = renderers::Markdown(&md_contents);

                        /// Syntax highlighting
                        let syntax_css = renderers::Css("https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.6.0/styles/default.min.css");
                        let syntax_script = renderers::Script("https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.6.0/highlight.min.js");
                        let syntax_init = renderers::Script("js/syntax.js");

                        Response::html(html!{
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


