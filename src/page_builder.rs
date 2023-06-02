// create dist directory
// save index.html
// save html for each md post.

use maud::{html, Markup};
use serde_json;
use std::fs;

use crate::render_html;

pub fn index_page() -> Markup {
    let icons = render_html::Icons();

    // Graph data
    let graph_data: render_html::Graph = render_html::helpers::generate_graph("md");
    let graph_json = serde_json::to_string(&graph_data).unwrap();
    let graph = render_html::JsObject(graph_json, "graph_data");

    let force_graph_script =
        render_html::Script("node_modules/force-graph/dist/force-graph.min.js");
    let graph_script = render_html::Script("js/graph.js");
    let logo = render_html::Logo();
    let default_css = render_html::Css("styles/default.css");

    // animejs
    let anime_js_script = render_html::Script("node_modules/animejs/lib/anime.min.js");
    let anime_script = render_html::Script("js/anime.js");
    let anime_css = render_html::Css("styles/anime.css");

    // Index page for visitors who have JS disabled.
    let ix = render_html::helpers::generate_index("md");
    let stripped = ix
        .iter()
        .map(|pa| {
            let result = pa.strip_prefix("md/").unwrap().strip_suffix(".md").unwrap();
            String::from(result)
        })
        .collect();
    let posts_list = render_html::Index(&stripped);

    html! {
        head {
            (icons)
            (force_graph_script)
            (default_css)
            (anime_js_script)
            (anime_css)
            (graph)
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
    }
}

pub fn md_page(contents: String) -> Markup {
    let logo = render_html::Logo();
    let anime_script = render_html::Script("js/anime.js");
    let anime_css = render_html::Css("styles/anime.css");
    let anime_js_script = render_html::Script("node_modules/animejs/lib/anime.min.js");

    let (frontmatter, md_contents) = render_html::helpers::split_contents(&contents);

    let meta = render_html::Meta(&frontmatter);
    let icons = render_html::Icons();
    let md_css = render_html::Css("styles/md.css");
    let md = render_html::Markdown(&md_contents);

    // Syntax highlighting
    let syntax_css = render_html::ExternalAsset(
        "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.6.0/styles/default.min.css",
        render_html::Asset::CSS,
    );
    let syntax_script = render_html::ExternalAsset(
        "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.6.0/highlight.min.js",
        render_html::Asset::JS,
    );
    let syntax_init = render_html::Script("js/syntax.js");

    html! {
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
    }
}

pub fn build() -> Result<(), std::io::Error> {
    match fs::create_dir("dist") {
        Ok(_) => {}
        Err(_) => {}
    };
    let index_html = index_page().into_string();
    fs::write("dist/index.html", index_html).expect("Unable to write file");

    // Generate index.html
    let file_index = render_html::helpers::generate_index("md");

    // generate .html for each post.
    for file_path in file_index {
        match fs::read_to_string(&file_path) {
            Ok(contents) => {
                let md_html = md_page(contents).into_string();
                let old_file_name = file_path
                    .strip_prefix("md/")
                    .unwrap()
                    .strip_suffix(".md")
                    .unwrap();
                let new_file_name = format!("dist/{}", String::from(old_file_name));

                fs::write(new_file_name, md_html).expect("Unable to write file");
            }
            Err(_why) => (),
        }
    }

    // copy assets directory.

    Ok(())
}
