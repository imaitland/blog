use toml::value::Datetime;
use std::fs;
use std::io::{BufReader, BufRead, Error};
use maud::{Render, html, PreEscaped, Markup};
use pulldown_cmark::{Parser, LinkType, Tag, Event};
use ammonia;
use serde::{Deserialize, Serialize};
use toml;
use walkdir::WalkDir;

pub enum Asset {
    CSS,
    JS
}

/// Structs
#[derive(Deserialize, PartialEq, Debug)]
pub struct Frontmatter {
    title: String,
    id: String,
    author: String,
    description: String,
    date: Datetime, // "2015-09-05"
    tag: String,
    image: String,
    icon: String,
    draft: bool
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Node {
    id: String,
    title: Option<String>,
    tag: Option<String>,
    icon: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Edge {
    source: Node,
    target: Node
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Link{
    source: String,
    target: String
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Graph {
    nodes: Vec<Node>,
    links: Vec<Link>
}

pub struct JsObject(pub String, pub &'static str);
impl Render for JsObject{
    fn render(&self) -> Markup {
        //println!("args: {} {}", self.0, self.1);
        let mut js_object = &self.0;
        let mut js_object_name = &self.1;
        let mut js = format!("var {} = {}", js_object_name, js_object);
        html! {
            script {
               (PreEscaped(js)) 
            }
        }
    }
}


/// Links to a JS script at the given path.
pub struct Script(pub &'static str);

impl Render for Script{
    fn render(&self) -> Markup {
        match fs::read_to_string(self.0) {
            Ok(file_contents) => {
                html! {
                    script {
                        (PreEscaped(file_contents))
                    }
                }
            },
            Err(_err) => {
                html! {
                    div {
                        "Could not find that file:" (self.0)
                    }
                }
            }
        }
    }
}

/// Links to a CSS stylesheet at the given path.
pub struct Css(pub &'static str);

impl Render for Css {
    fn render(&self) -> Markup {
        // Read to string and embedd it instead of the reference.
        match fs::read_to_string(self.0) {
            Ok(file_contents) => {
                html! {
                    style media="all" {
                        (file_contents)
                    }
                }
            },
            Err(_err) => {
                html! {
                    div {
                        "Could not find that file:" (self.0)
                    }
                }
            }
        }
    }
}

/// Renders a block of Markdown using `pulldown-cmark`.
pub struct ExternalAsset<'a>(pub &'a str, pub Asset);

impl Render for ExternalAsset <'_> {
    fn render(&self) -> Markup {
        match self.1 {
            Asset::CSS => {
                html! {
                    link rel="stylesheet" type="text/css" href=(self.0);
                }
            },
            Asset::JS => {
                html! {
                    script src=(self.0) {};
                }
            }
        }
    }
}

/// Renders a block of Markdown using `pulldown-cmark`.
pub struct Markdown<'a>(pub &'a str);

impl Render for Markdown <'_> {
    fn render(&self) -> Markup {
        let parser = Parser::new(self.0);
         // Write to String buffer.
        let mut unsafe_html_output: String = String::with_capacity(self.0.len() * 3 / 2);
        pulldown_cmark::html::push_html(&mut unsafe_html_output, parser);
        // Sanitize it with ammonia
        let safe_html = ammonia::clean(&unsafe_html_output);
        PreEscaped(safe_html)
    }
}

/// Renders an unordered list of markdowns.
pub struct Index<'a>(pub &'a Vec<String>);

impl Render for Index<'_> {
    fn render(&self) -> Markup {
        html! {
            ul {
                @for md in self.0.iter() {
                    li {
                        a href=(md) {(md.split("_").collect::<Vec<&str>>().join(" "))}
                    }
                }
            }
        }
    }
}

/// Renders html head.
pub struct Meta<'a>(pub &'a Frontmatter);

impl Render for Meta<'_> {
    fn render(&self) -> Markup {
        // parse self to toml..
        let fm: &Frontmatter = self.0;

        html! {
            meta name="author" content=(fm.author) {}
            meta name="description" content=(fm.description) {}
            // Open Graph.
            meta property="og:description" content=(fm.description) {}
            meta property="og:image" content=(fm.image) {}
            // Twitter Cards.
            meta name="twitter:card" content="summary" {}
            meta name="twitter:title" content=(fm.title) {}
            meta name="twitter:description" content=(fm.description) {}
            meta name="twitter:image" content=(fm.image) {}
            title {(fm.title)}
        }
    }
}
/// Renders html head.
pub struct Icons();

impl Render for Icons {
    fn render(&self) -> Markup {
        // parse self to toml..
        html! {
                // third-generation iPad with high-resolution Retina display.
                link rel="apple-touch-icon-precomposed" sizes="144x144" href="/assets/apple-touch-icon.png" {}
                // first- and second-generation iPad.
                link rel="apple-touch-icon-precomposed" sizes="72x72" href="/assets/apple-touch-icon.png" {}
                // non-Retina iPhone, iPod Touch, and Android 2.1+ devices.
                link rel="apple-touch-icon-precomposed" href="/assets/android-chrome-192x192.png" {}
                // basic favicon.
                link rel="shortcut icon" href="/assets/favicon-32x32.png"{}
        }
    }
}

/// Renders site logo.
pub struct Logo();

impl Render for Logo {
    fn render(&self) -> Markup {
        html! {
            div class="logo-container" {
                a class="logo" href="/" {
                    span class="revealspring" data-x="30" data-y="-78" {"/ "}
                    span class="reveal" data-x="-40" data-y="104" {"/"}
                    span class="reveal" data-x="-60" data-y="-156" {"\\ "}
                    span class="reveal" data-x="34" data-y="-88" {"/ "}
                    span class="reveal" data-x="-16" data-y="42" {"/"}
                    span class="revealspring" data-x="104" data-y="272" {"\\"}
                    span class="reveal" data-x="75" data-y="-195" {"/ "}
                }
            }
        }
    }
}

pub mod helpers {
    use super::*;

    pub fn split_contents(contents: &String) -> (Frontmatter, String) {
        // Spit frontmatter and contents, create a frontmatter renderer.
        let split_contents: Vec<&str> = contents.split("+++").collect();

        let mut frontmatter = "";
        let mut md_contents = contents.as_str();

        if split_contents.len() > 1 {
            frontmatter = split_contents[1];
            md_contents = split_contents[2];
        }
        let fm: Frontmatter = toml::from_str(frontmatter).unwrap();
        (fm, md_contents.to_string())
    }

    pub fn generate_graph(dir: &str) -> Graph {

        let mut graph = Graph {
            nodes: vec![],
            links: vec![]
        };

        let paths = helpers::generate_index(dir);

        for i in 0..paths.len() {
            let p = &paths[i];

            match fs::read_to_string(p) {
                Ok(file_contents) => {
                    // Split frontmatter here...
                    let (fm, md) = helpers::split_contents(&file_contents);

                    graph.nodes.push(
                        Node{
                            id: fm.id.to_owned(),
                            title: Some(fm.title.to_owned()),
                            icon: Some(fm.icon.to_owned()),
                            tag: Some(fm.tag.to_owned())
                        }
                    );

                    let e = helpers::get_edges(&md, &fm);

                    let mut links = e.iter().map(|edge| {
                        Link {
                            source: edge.source.id.to_owned(),
                            target: edge.target.id.to_owned()
                        }
                    }).collect();
                    graph.links.append(&mut links);
                }
                Err(_why) => {

                }
            }

        }
        // Parse to json.
        //serde_json::to_string(&graph).unwrap();
        graph
    }

    pub fn generate_index(dir: &str) -> Vec<String> {
        let paths: Vec<String> = WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| {
                let path = e
                    .path()
                    .to_str()
                    .unwrap();
                    String::from(path)
            })
            .collect();
        paths
    }

    pub fn get_edges(contents: &String, fm: &Frontmatter) -> Vec<Edge> {

        let mut edges: Vec<Edge> = vec![];

        let mut parser = Parser::new(contents).map(|event| match event {
            Event::Start(ref tag) => {

                match tag {
                    Tag::Link(link_type, url, _title) => {
                        match link_type {
                            LinkType::Inline => {
                                // Relative links only. 
                                if url.as_ref().starts_with("/") {
                                    let edge = Edge {
                                        source: Node {
                                            id: fm.id.to_owned(),
                                            title: None,
                                            tag: None,
                                            icon: None
                                        },
                                        target: Node {
                                            id: url.as_ref().strip_prefix("/").unwrap().to_string(),
                                            title: None,
                                            tag: None,
                                            icon: None
                                        }
                                    };
                                    edges.push(edge);
                                }
                                event.to_owned()
                            },
                            _ => event.to_owned()
                        };
                    },
                    _ => ()
                }
                event
            }
            _ => event
        });
            
        loop {
            parser.next();
            if parser.next() == None {
                break;
            }
        };
        edges
    }
}