// create dist directory
// save index.html
// save html for each md post.

use std::fs::{File};
use std::fs;
use serde_json;
use maud::{Render, html, PreEscaped, Markup};

use crate::render;

pub fn index_page () -> Markup {
    let icons = render::render_html::Icons();

    // Graph data
    let graph_data: render::render_html::Graph = render::render_html::helpers::generate_graph("md");
    let graph_json = serde_json::to_string(&graph_data).unwrap();
    let graph = render::render_html::JsObject(graph_json, "graph_data");

    let force_graph_script = render::render_html::Script("node_modules/force-graph/dist/force-graph.min.js");
    let graph_script = render::render_html::Script("js/graph.js");
    let logo = render::render_html::Logo();
    let default_css = render::render_html::Css("styles/default.css");

    // animejs
    let anime_js_script = render::render_html::Script("node_modules/animejs/lib/anime.min.js");
    let anime_script = render::render_html::Script("js/anime.js");
    let anime_css = render::render_html::Css("styles/anime.css");

    // Index page for visitors who have JS disabled.
    let ix = render::render_html::helpers::generate_index("md");
    let stripped = ix.iter().map(|pa|{
        let result = pa.strip_prefix("md/").unwrap().strip_suffix(".md").unwrap();
        String::from(result)
    }).collect();
    let posts_list = render::render_html::Index(&stripped);

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

pub fn md_page (contents: String) -> Markup {

    let logo = render::render_html::Logo();
    let anime_script = render::render_html::Script("js/anime.js");
    let anime_css = render::render_html::Css("styles/anime.css");
    let anime_js_script = render::render_html::Script("node_modules/animejs/lib/anime.min.js");

    let (frontmatter, md_contents) = render::render_html::helpers::split_contents(&contents);

    let meta = render::render_html::Meta(&frontmatter);
    let icons = render::render_html::Icons();
    let md_css = render::render_html::Css("styles/md.css");
    let md = render::render_html::Markdown(&md_contents);

    // Syntax highlighting
    let syntax_css = render::render_html::ExternalAsset("https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.6.0/styles/default.min.css", render::render_html::Asset::CSS);
    let syntax_script = render::render_html::ExternalAsset("https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.6.0/highlight.min.js", render::render_html::Asset::JS);
    let syntax_init = render::render_html::Script("js/syntax.js");

    html!{
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
/*

pub fn build() -> Result<(),()>  {
    // Create dist director...
    fs::create_dir("/dist")?;
    let index_html = index_page();
    // write file...
    let mut output = File::create("/dist/index.html")?;
    write!(output,"{}", index_html.to_string())?;

    Ok(())
}
*/
